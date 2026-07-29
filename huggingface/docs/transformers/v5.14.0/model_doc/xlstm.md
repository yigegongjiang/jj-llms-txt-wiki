# xLSTM

## Overview

The xLSTM model was proposed in [xLSTM: Extended Long Short-Term Memory](https://huggingface.co/papers/2405.04517) by Maximilian Beck*, Korbinian Pöppel*, Markus Spanring, Andreas Auer, Oleksandra Prudnikova, Michael Kopp, Günter Klambauer, Johannes Brandstetter and Sepp Hochreiter.
xLSTM updates the original LSTM architecture to be competitive with Transformer models by introducing exponential gating, matrix memory expansion, and parallelizable training and ingestion.

The [7B model](https://huggingface.co/NX-AI/xLSTM-7b) variant was trained by the xLSTM team Maximilian Beck, Korbinian Pöppel, Phillip Lippe, Richard Kurle, Patrick Blies, Sebastian Böck and Sepp Hochreiter at NXAI.

The abstract from the paper is the following:

*In the 1990s, the constant error carousel and gating were introduced as the central ideas of the Long Short-Term Memory (LSTM). Since then, LSTMs have stood the test of time and contributed to numerous deep learning success stories, in particular they constituted the first Large Language Models (LLMs). However, the advent of the Transformer technology with parallelizable self-attention at its core marked the dawn of a new era, outpacing LSTMs at scale. We now raise a simple question: How far do we get in language modeling when scaling LSTMs to billions of parameters, leveraging the latest techniques from modern LLMs, but mitigating known limitations of LSTMs? Firstly, we introduce exponential gating with appropriate normalization and stabilization techniques. Secondly, we modify the LSTM memory structure, obtaining: (i) sLSTM with a scalar memory, a scalar update, and new memory mixing, (ii) mLSTM that is fully parallelizable with a matrix memory and a covariance update rule. Integrating these LSTM extensions into residual block backbones yields xLSTM blocks that are then residually stacked into xLSTM architectures. Exponential gating and modified memory structures boost xLSTM capabilities to perform favorably when compared to state-of-the-art Transformers and State Space Models, both in performance and scaling.*

This model was contributed by [NX-AI](https://huggingface.co/NX-AI).
The original code can be found [here](https://github.com/NX-AI/xlstm).

## xLSTMConfig[[transformers.xLSTMConfig]]

- **vocab_size** (`int`, *optional*, defaults to `50304`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **embedding_dim** (`int`, *optional*) --
  Dimensionality of the embeddings and hidden states.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_blocks** (`int`, optional, *optional*, defaults to 32) --
  Number of blocks of the xLSTM model, use num_hidden_layers if None.
- **num_heads** (`int`, optional, *optional*, defaults to 8) --
  Number of heads for the xLSTM Layer/Cell.
- **use_bias** (`bool`, optional, *optional*, defaults to `False`) --
  Whether to use biases in the xLSTM model.
- **norm_reduction_force_float32** (`bool`, optional, *optional*, defaults to `True`) --
  Whether to force the float32 norm reduction op to be done in fp32 precision.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **add_out_norm** (`bool`, optional, *optional*, defaults to `True`) --
  Whether to add an output norm after the blocks before the LMHead.
- **norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **qk_dim_factor** (`float`, optional, *optional*, defaults to 0.5) --
  Scale factor for the query and key dimension.
- **v_dim_factor** (`float`, optional, *optional*, defaults to 1.0) --
  Scale factor for the value dimension.
- **chunkwise_kernel** (`ChunkwiseKernelType`, optional, *optional*, defaults to `"chunkwise--native_autograd"`) --
  Kernel type for chunkwise processing mode.
- **sequence_kernel** (`SequenceKernelType`, optional, *optional*, defaults to `"native_sequence__native"`) --
  Kernel type for sequence processing mode.
- **step_kernel** (`StepKernelType`, optional, *optional*, defaults to `"native"`) --
  Kernel type for step processing mode.
- **mode** (`BackendModeType`, optional, *optional*, defaults to `"inference"`) --
  Operation mode (inference is needed for generation).
- **chunk_size** (`int`, optional, *optional*, defaults to 64) --
  Internal chunk size.
- **return_last_states** (`bool`, optional, *optional*, defaults to `True`) --
  If to return the last states / cache internally. Needed as True for generation.
- **autocast_kernel_dtype** (`DtypeType`, optional, *optional*, defaults to `"bfloat16"`) --
  Kernel dtype for the states.
- **eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **inference_state_dtype** (`DtypeType`, optional, *optional*, defaults to `"float32"`) --
  Kernel dtype for states in inference.
- **ffn_proj_factor** (`float`, optional, *optional*, defaults to 2.667) --
  Size factor of the post-up projection gated Feed Forward network.
- **ffn_round_up_to_multiple_of** (`int`, optional, *optional*, defaults to 64) --
  Size factor round value of the post-up projection gated Feed Forward network.
- **gate_soft_cap** (`float`, optional, *optional*, defaults to 15.0) --
  Gate soft cap scale.
- **output_logit_soft_cap** (`float`, optional, *optional*, defaults to 30.0) --
  Output logit soft cap scale.
- **weight_mode** (`Literal`, *optional*, defaults to `"single"`) --
  Whether parallel linear layers are separated or fused (single).
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **max_inference_chunksize** (`int`, optional, *optional*, defaults to 16384) --
  Limit the chunk size for inference to save memory.

This is the configuration class to store the configuration of a xLSTMModel. It is used to instantiate a Xlstm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [NX-AI/xLSTM-7b](https://huggingface.co/NX-AI/xLSTM-7b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import xLSTMConfig, xLSTMModel

>>> # Initializing a xLSTM configuration
>>> configuration = xLSTMConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = xLSTMModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## xLSTMModel[[transformers.xLSTMModel]]

- **config** ([xLSTMModel](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Xlstm Model outputting raw hidden-states without any specific head on top.

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
- **inputs_embeds** (`torch.LongTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **cache_params** (`xLSTMCache`, *optional*) --
  The xLSTMCache that carries the RNN states.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`xLSTMOutput` or `tuple(torch.FloatTensor)`A `xLSTMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([xLSTMConfig](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMConfig)) and inputs.
The [xLSTMModel](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **cache_params** (`~models.xlstm.modeling_xlstm.xLSTMCache`, *optional*) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## xLSTMLMHeadModel[[transformers.xLSTMForCausalLM]]

- **config** ([xLSTMForCausalLM](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Xlstm Model for causal language modeling.

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **cache_params** (`xLSTMCache`, *optional*) --
  The xLSTMCache that carries the RNN states.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`xLSTMCausalLMOutput` or `tuple(torch.FloatTensor)`A `xLSTMCausalLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([xLSTMConfig](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMConfig)) and inputs.
The [xLSTMForCausalLM](/docs/transformers/v5.14.0/en/model_doc/xlstm#transformers.xLSTMForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **cache_params** (`xLSTMCache`, *optional*, carrying the RNN states) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```
