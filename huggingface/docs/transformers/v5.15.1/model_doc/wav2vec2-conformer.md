# Wav2Vec2-Conformer

## Overview

The Wav2Vec2-Conformer was added to an updated version of [fairseq S2T: Fast Speech-to-Text Modeling with fairseq](https://huggingface.co/papers/2010.05171) by Changhan Wang, Yun Tang, Xutai Ma, Anne Wu, Sravya Popuri, Dmytro Okhonko, Juan Pino.

The official results of the model can be found in Table 3 and Table 4 of the paper.

The Wav2Vec2-Conformer weights were released by the Meta AI team within the [Fairseq library](https://github.com/pytorch/fairseq/blob/main/examples/wav2vec/README.md#pre-trained-models).

This model was contributed by [patrickvonplaten](https://huggingface.co/patrickvonplaten).
The original code can be found [here](https://github.com/pytorch/fairseq/tree/main/examples/wav2vec).

Note: Meta (FAIR) released a new version of [Wav2Vec2-BERT 2.0](https://huggingface.co/docs/transformers/en/model_doc/wav2vec2-bert) - it's pretrained on 4.5M hours of audio. We especially recommend using it for fine-tuning tasks, e.g. as per [this guide](https://huggingface.co/blog/fine-tune-w2v2-bert).

## Usage tips

- Wav2Vec2-Conformer follows the same architecture as Wav2Vec2, but replaces the *Attention*-block with a *Conformer*-block
  as introduced in [Conformer: Convolution-augmented Transformer for Speech Recognition](https://huggingface.co/papers/2005.08100).
- For the same number of layers, Wav2Vec2-Conformer requires more parameters than Wav2Vec2, but also yields
an improved word error rate.
- Wav2Vec2-Conformer uses the same tokenizer and feature extractor as Wav2Vec2.
- Wav2Vec2-Conformer can use either no relative position embeddings, Transformer-XL-like position embeddings, or
  rotary position embeddings by setting the correct `config.position_embeddings_type`.

## Resources

- [Audio classification task guide](../tasks/audio_classification)
- [Automatic speech recognition task guide](../tasks/asr)

## Wav2Vec2ConformerConfig[[transformers.Wav2Vec2ConformerConfig]]

#### transformers.Wav2Vec2ConformerConfig[[transformers.Wav2Vec2ConformerConfig]]

```python
transformers.Wav2Vec2ConformerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int | None = None, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout: float | int = 0.1, activation_dropout: float | int = 0.1, attention_dropout: float | int = 0.1, feat_proj_dropout: float | int = 0.0, feat_quantizer_dropout: float | int = 0.0, final_dropout: float | int = 0.1, layerdrop: float | int = 0.1, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, feat_extract_norm: str = 'group', feat_extract_activation: str = 'gelu', conv_dim: list[int] | tuple[int, ...] = (512, 512, 512, 512, 512, 512, 512), conv_stride: list[int] | tuple[int, ...] = (5, 2, 2, 2, 2, 2, 2), conv_kernel: list[int] | tuple[int, ...] = (10, 3, 3, 3, 3, 2, 2), conv_bias: bool = False, num_conv_pos_embeddings: int = 128, num_conv_pos_embedding_groups: int = 16, apply_spec_augment: bool = True, mask_time_prob: float | int = 0.05, mask_time_length: int = 10, mask_time_min_masks: int = 2, mask_feature_prob: float | int = 0.0, mask_feature_length: int = 10, mask_feature_min_masks: int = 0, num_codevectors_per_group: int = 320, num_codevector_groups: int = 2, contrastive_logits_temperature: float = 0.1, num_negatives: int = 100, codevector_dim: int = 256, proj_codevector_dim: int = 256, diversity_loss_weight: float = 0.1, ctc_loss_reduction: str = 'sum', ctc_zero_infinity: bool = False, use_weighted_layer_sum: bool = False, classifier_proj_size: int = 256, tdnn_dim: list[int] | tuple[int, ...] = (512, 512, 512, 512, 1500), tdnn_kernel: list[int] | tuple[int, ...] = (5, 3, 3, 1, 1), tdnn_dilation: list[int] | tuple[int, ...] = (1, 2, 3, 1, 1), xvector_output_dim: int = 512, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, add_adapter: bool = False, adapter_kernel_size: int = 3, adapter_stride: int = 2, num_adapter_layers: int = 3, output_hidden_size: int | None = None, position_embeddings_type: str | None = 'relative', rotary_embedding_base: int = 10000, max_source_positions: int = 5000, conv_depthwise_kernel_size: int = 31, conformer_conv_dropout: float | int = 0.1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/configuration_wav2vec2_conformer.py#L27)

**Parameters:**

vocab_size (`int`, *optional*) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

feat_proj_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for output of the feature encoder.

feat_quantizer_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for the output of the feature encoder that's used by the quantizer.

final_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for the final projection layer of [Wav2Vec2ConformerForCTC](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForCTC).

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

apply_spec_augment (`bool`, *optional*, defaults to `True`) : Whether to apply *SpecAugment* data augmentation to the outputs of the feature encoder. For reference see [SpecAugment: A Simple Data Augmentation Method for Automatic Speech Recognition](https://huggingface.co/papers/1904.08779).

mask_time_prob (`float`, *optional*, defaults to 0.05) : Percentage (between 0 and 1) of all feature vectors along the time axis which will be masked. The masking procedure generates ''mask_time_prob*len(time_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_time_prob* should be `prob_vector_start*mask_time_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_time_length (`int`, *optional*, defaults to 10) : Length of vector span along the time axis.

mask_time_min_masks (`int`, *optional*, defaults to 2), : The minimum number of masks of length `mask_feature_length` generated along the time axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_time_prob*len(time_axis)/mask_time_length < mask_time_min_masks''

mask_feature_prob (`float`, *optional*, defaults to 0.0) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates ''mask_feature_prob*len(feature_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_feature_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates `mask_feature_prob*len(feature_axis)/mask_time_length` independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment` is `True`.

mask_feature_length (`int`, *optional*, defaults to 10) : Length of vector span along the feature axis.

mask_feature_min_masks (`int`, *optional*, defaults to 0), : The minimum number of masks of length `mask_feature_length` generated along the feature axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_feature_prob*len(feature_axis)/mask_feature_length < mask_feature_min_masks''

num_codevectors_per_group (`int`, *optional*, defaults to 320) : Number of entries in each quantization codebook (group).

num_codevectors_per_group (`int`, *optional*, defaults to 320) : Number of entries in each quantization codebook (group).

num_codevector_groups (`int`, *optional*, defaults to 2) : Number of codevector groups for product codevector quantization.

contrastive_logits_temperature (`float`, *optional*, defaults to 0.1) : The temperature *kappa* in the contrastive loss.

num_negatives (`int`, *optional*, defaults to 100) : Number of negative samples for the contrastive loss.

codevector_dim (`int`, *optional*, defaults to 256) : Dimensionality of the quantized feature vectors.

proj_codevector_dim (`int`, *optional*, defaults to 256) : Dimensionality of the final projection of both the quantized and the transformer features.

diversity_loss_weight (`int`, *optional*, defaults to 0.1) : The weight of the codebook diversity loss component.

ctc_loss_reduction (`str`, *optional*, defaults to `sum`) : Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training.

ctc_zero_infinity (`bool`, *optional*, defaults to `False`) : Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance of [Wav2Vec2ConformerForCTC](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForCTC).

use_weighted_layer_sum (`bool`, *optional*, defaults to `False`) : Whether to use a weighted average of layer outputs with learned weights. Only relevant when using an instance of [Wav2Vec2ConformerForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForSequenceClassification).

classifier_proj_size (`int`, *optional*, defaults to 256) : Dimensionality of the projection before token mean-pooling for classification.

tdnn_dim (`tuple[int]` or `list[int]`, *optional*, defaults to `(512, 512, 512, 512, 1500)`) : A tuple of integers defining the number of output channels of each 1D convolutional layer in the *TDNN* module of the *XVector* model. The length of *tdnn_dim* defines the number of *TDNN* layers.

tdnn_kernel (`tuple[int]` or `list[int]`, *optional*, defaults to `(5, 3, 3, 1, 1)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the *TDNN* module of the *XVector* model. The length of *tdnn_kernel* has to match the length of *tdnn_dim*.

tdnn_dilation (`tuple[int]` or `list[int]`, *optional*, defaults to `(1, 2, 3, 1, 1)`) : A tuple of integers defining the dilation factor of each 1D convolutional layer in *TDNN* module of the *XVector* model. The length of *tdnn_dilation* has to match the length of *tdnn_dim*.

xvector_output_dim (`int`, *optional*, defaults to 512) : Dimensionality of the *XVector* embedding vectors.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

add_adapter (`bool`, *optional*, defaults to `False`) : Whether a convolutional network should be stacked on top of the Wav2Vec2Conformer Encoder. Can be very useful for warm-starting Wav2Vec2Conformer for SpeechEncoderDecoder models.

adapter_kernel_size (`int`, *optional*, defaults to 3) : Kernel size of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

adapter_stride (`int`, *optional*, defaults to 2) : Stride of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

num_adapter_layers (`int`, *optional*, defaults to 3) : Number of convolutional layers that should be used in the adapter network. Only relevant if `add_adapter is True`.

output_hidden_size (`int`, *optional*) : Dimensionality of the encoder output layer. If not defined, this defaults to *hidden-size*. Only relevant if `add_adapter is True`.

position_embeddings_type (`str`, *optional*, defaults to `"relative"`) : Can be specified to `relative` or `rotary` for relative or rotary position embeddings respectively. If left `None` no relative position embedding is applied.

rotary_embedding_base (`int`, *optional*, defaults to 10000) : If `"rotary"` position embeddings are used, defines the size of the embedding base.

max_source_positions (`int`, *optional*, defaults to 5000) : if `"relative"` position embeddings are used, defines the maximum source input positions.

conv_depthwise_kernel_size (`int`, *optional*, defaults to 31) : Kernel size of convolutional depthwise 1D layer in Conformer blocks.

conformer_conv_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for all convolutional layers in Conformer blocks.

This is the configuration class to store the configuration of a Wav2Vec2 ConformerModel. It is used to instantiate a Wav2Vec2 Conformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/wav2vec2-conformer-rel-pos-large](https://huggingface.co/facebook/wav2vec2-conformer-rel-pos-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Wav2Vec2ConformerConfig, Wav2Vec2ConformerModel

>>> # Initializing a Wav2Vec2Conformer facebook/wav2vec2-conformer-rel-pos-large style configuration
>>> configuration = Wav2Vec2ConformerConfig()

>>> # Initializing a model (with random weights) from the facebook/wav2vec2-conformer-rel-pos-large style configuration
>>> model = Wav2Vec2ConformerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Wav2Vec2Conformer specific outputs[[transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput]]

#### transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput[[transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput]]

```python
transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput(loss: typing.Optional[torch.FloatTensor] = None, projected_states: typing.Optional[torch.FloatTensor] = None, projected_quantized_states: typing.Optional[torch.FloatTensor] = None, codevector_perplexity: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor] | None = None, attentions: tuple[torch.FloatTensor] | None = None, contrastive_loss: typing.Optional[torch.FloatTensor] = None, diversity_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L40)

**Parameters:**

loss (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) : Total loss as the sum of the contrastive loss (L_m) and the diversity loss (L_d) as stated in the [official paper](https://huggingface.co/papers/2006.11477).

projected_states (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.proj_codevector_dim)`) : Hidden-states of the model projected to *config.proj_codevector_dim* that can be used to predict the masked projected quantized states.

projected_quantized_states (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.proj_codevector_dim)`) : Quantized extracted feature vectors projected to *config.proj_codevector_dim* representing the positive target vectors for contrastive loss.

codevector_perplexity (`torch.FloatTensor` of shape `(1,)`) : The perplexity of the codevector distribution, used to measure the diversity of the codebook.

hidden_states (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

contrastive_loss (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) : The contrastive loss (L_m) as stated in the [official paper](https://huggingface.co/papers/2006.11477).

diversity_loss (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) : The diversity loss (L_d) as stated in the [official paper](https://huggingface.co/papers/2006.11477).

Output type of [Wav2Vec2ConformerForPreTraining](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForPreTraining), with potential hidden states and attentions.

## Wav2Vec2ConformerModel[[transformers.Wav2Vec2ConformerModel]]

#### transformers.Wav2Vec2ConformerModel[[transformers.Wav2Vec2ConformerModel]]

```python
transformers.Wav2Vec2ConformerModel(config: Wav2Vec2ConformerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1061)

**Parameters:**

config ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Wav2Vec2 Conformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerModel.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, mask_time_indices: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1132)

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
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerModel](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerModel) forward method, overrides the `__call__` special method.

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

## Wav2Vec2ConformerForCTC[[transformers.Wav2Vec2ConformerForCTC]]

#### transformers.Wav2Vec2ConformerForCTC[[transformers.Wav2Vec2ConformerForCTC]]

```python
transformers.Wav2Vec2ConformerForCTC(config, target_lang: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1414)

**Parameters:**

config ([Wav2Vec2ConformerForCTC](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForCTC)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

target_lang (`str`, *optional*) : Language id of adapter weights. Adapter weights are stored in the format adapter..safetensors or adapter..bin. Only relevant when using an instance of [UniSpeechSatForCTC](/docs/transformers/v5.15.1/en/model_doc/unispeech-sat#transformers.UniSpeechSatForCTC) with adapters. Uses 'eng' by default.

Wav2Vec2Conformer Model with a `language modeling` head on top for Connectionist Temporal Classification (CTC).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerForCTC.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1451)

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
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerForCTC](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForCTC) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, Wav2Vec2ConformerForCTC
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")
>>> model = Wav2Vec2ConformerForCTC.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")

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

## Wav2Vec2ConformerForSequenceClassification[[transformers.Wav2Vec2ConformerForSequenceClassification]]

#### transformers.Wav2Vec2ConformerForSequenceClassification[[transformers.Wav2Vec2ConformerForSequenceClassification]]

```python
transformers.Wav2Vec2ConformerForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1530)

**Parameters:**

config ([Wav2Vec2ConformerForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Wav2Vec2Conformer Model with a sequence classification head on top (a linear layer over the pooled output) for tasks like
SUPERB Keyword Spotting.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerForSequenceClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1563)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `Wav2Vec2ConformerProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForSequenceClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, Wav2Vec2ConformerForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")
>>> model = Wav2Vec2ConformerForSequenceClassification.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = Wav2Vec2ConformerForSequenceClassification.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, Wav2Vec2ConformerForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")
>>> model = Wav2Vec2ConformerForSequenceClassification.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = Wav2Vec2ConformerForSequenceClassification.from_pretrained(
...     "facebook/wav2vec2-conformer-rel-pos-large", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## Wav2Vec2ConformerForAudioFrameClassification[[transformers.Wav2Vec2ConformerForAudioFrameClassification]]

#### transformers.Wav2Vec2ConformerForAudioFrameClassification[[transformers.Wav2Vec2ConformerForAudioFrameClassification]]

```python
transformers.Wav2Vec2ConformerForAudioFrameClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1635)

**Parameters:**

config ([Wav2Vec2ConformerForAudioFrameClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForAudioFrameClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Wav2Vec2 Conformer Model with a frame classification head on top for tasks like Speaker Diarization.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerForAudioFrameClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1667)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `Wav2Vec2ConformerProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`

A [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerForAudioFrameClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForAudioFrameClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoFeatureExtractor, Wav2Vec2ConformerForAudioFrameClassification
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")
>>> model = Wav2Vec2ConformerForAudioFrameClassification.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")

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

## Wav2Vec2ConformerForXVector[[transformers.Wav2Vec2ConformerForXVector]]

#### transformers.Wav2Vec2ConformerForXVector[[transformers.Wav2Vec2ConformerForXVector]]

```python
transformers.Wav2Vec2ConformerForXVector(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1789)

**Parameters:**

config ([Wav2Vec2ConformerForXVector](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForXVector)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Wav2Vec2Conformer Model with an XVector feature extraction head on top for tasks like Speaker Verification.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerForXVector.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1839)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `Wav2Vec2ConformerProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [XVectorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.XVectorOutput) or `tuple(torch.FloatTensor)`

A [XVectorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.XVectorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerForXVector](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForXVector) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoFeatureExtractor, Wav2Vec2ConformerForXVector
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")
>>> model = Wav2Vec2ConformerForXVector.from_pretrained("facebook/wav2vec2-conformer-rel-pos-large")

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

## Wav2Vec2ConformerForPreTraining[[transformers.Wav2Vec2ConformerForPreTraining]]

#### transformers.Wav2Vec2ConformerForPreTraining[[transformers.Wav2Vec2ConformerForPreTraining]]

```python
transformers.Wav2Vec2ConformerForPreTraining(config: Wav2Vec2ConformerConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1197)

**Parameters:**

config ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Wav2Vec2Conformer Model with a quantizer and `VQ` head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Wav2Vec2ConformerForPreTraining.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, mask_time_indices: typing.Optional[torch.BoolTensor] = None, sampled_negative_indices: typing.Optional[torch.BoolTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wav2vec2_conformer/modeling_wav2vec2_conformer.py#L1245)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [Wav2Vec2Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2Processor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

mask_time_indices (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices to mask extracted features for contrastive loss. When in training mode, model learns to predict masked extracted features in *config.proj_codevector_dim* space.

sampled_negative_indices (`torch.BoolTensor` of shape `(batch_size, sequence_length, num_negatives)`, *optional*) : Indices indicating which quantized target vectors are used as negative sampled vectors in contrastive loss. Required input for pre-training.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Wav2Vec2ConformerForPreTrainingOutput](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput) or `tuple(torch.FloatTensor)`

A [Wav2Vec2ConformerForPreTrainingOutput](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer.Wav2Vec2ConformerForPreTrainingOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Wav2Vec2ConformerConfig](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerConfig)) and inputs.

The [Wav2Vec2ConformerForPreTraining](/docs/transformers/v5.15.1/en/model_doc/wav2vec2-conformer#transformers.Wav2Vec2ConformerForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) -- Total loss as the sum of the contrastive loss (L_m) and the diversity loss (L_d) as stated in the [official
  paper](https://huggingface.co/papers/2006.11477).
- **projected_states** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.proj_codevector_dim)`) -- Hidden-states of the model projected to *config.proj_codevector_dim* that can be used to predict the masked
  projected quantized states.
- **projected_quantized_states** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.proj_codevector_dim)`) -- Quantized extracted feature vectors projected to *config.proj_codevector_dim* representing the positive
  target vectors for contrastive loss.
- **codevector_perplexity** (`torch.FloatTensor` of shape `(1,)`) -- The perplexity of the codevector distribution, used to measure the diversity of the codebook.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **contrastive_loss** (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) -- The contrastive loss (L_m) as stated in the [official paper](https://huggingface.co/papers/2006.11477).
- **diversity_loss** (`*optional*`, returned when `sample_negative_indices` are passed, `torch.FloatTensor` of shape `(1,)`) -- The diversity loss (L_d) as stated in the [official paper](https://huggingface.co/papers/2006.11477).

Example:

```python
>>> import torch
>>> from transformers import AutoFeatureExtractor, Wav2Vec2ConformerForPreTraining
>>> from transformers.models.wav2vec2_conformer.modeling_wav2vec2_conformer import _compute_mask_indices, _sample_negative_indices
>>> from datasets import load_dataset

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("facebook/wav2vec2_conformer-base")
>>> model = Wav2Vec2ConformerForPreTraining.from_pretrained("facebook/wav2vec2_conformer-base")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> input_values = feature_extractor(ds[0]["audio"]["array"], return_tensors="pt").input_values  # Batch size 1

>>> # compute masked indices
>>> batch_size, raw_sequence_length = input_values.shape
>>> sequence_length = model._get_feat_extract_output_lengths(raw_sequence_length).item()
>>> mask_time_indices = _compute_mask_indices(
...     shape=(batch_size, sequence_length), mask_prob=0.2, mask_length=2
... )
>>> sampled_negative_indices = _sample_negative_indices(
...     features_shape=(batch_size, sequence_length),
...     num_negatives=model.config.num_negatives,
...     mask_time_indices=mask_time_indices,
... )
>>> mask_time_indices = torch.tensor(data=mask_time_indices, device=input_values.device, dtype=torch.long)
>>> sampled_negative_indices = torch.tensor(
...     data=sampled_negative_indices, device=input_values.device, dtype=torch.long
... )

>>> with torch.no_grad():
...     outputs = model(input_values, mask_time_indices=mask_time_indices)

>>> # compute cosine similarity between predicted (=projected_states) and target (=projected_quantized_states)
>>> cosine_sim = torch.cosine_similarity(outputs.projected_states, outputs.projected_quantized_states, dim=-1)

>>> # show that cosine similarity is much higher than random
>>> cosine_sim[mask_time_indices.to(torch.bool)].mean() > 0.5
tensor(True)

>>> # for contrastive loss training model should be put into train mode
>>> model = model.train()
>>> loss = model(
...     input_values, mask_time_indices=mask_time_indices, sampled_negative_indices=sampled_negative_indices
... ).loss
```
