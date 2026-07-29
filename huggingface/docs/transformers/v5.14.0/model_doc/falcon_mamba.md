# FalconMamba

[FalconMamba](https://huggingface.co/papers/2410.05355) is a 7B large language model, available as pretrained and instruction-tuned variants, based on the [Mamba](./mamba). This model implements a pure Mamba design that focuses on computational efficiency while maintaining strong performance. FalconMamba is significantly faster at inference and requires substantially less memory for long sequence generation. The models are pretrained on a diverse 5.8T token dataset including [RefinedWeb](https://huggingface.co/datasets/tiiuae/falcon-refinedweb), technical content, code, and mathematical data.

You can find the official FalconMamba checkpoints in the [FalconMamba 7B](https://huggingface.co/collections/tiiuae/falconmamba-7b-66b9a580324dd1598b0f6d4a) collection.

> [!TIP]
> Click on the FalconMamba models in the right sidebar for more examples of how to apply FalconMamba to different language tasks.

The examples below demonstrate how to generate text with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    "text-generation",
    model="tiiuae/falcon-mamba-7b-instruct",
    device=0
)
pipeline(
    "Explain the difference between transformers and SSMs",
    max_length=100,
    do_sample=True,
    temperature=0.7
)
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("tiiuae/falcon-mamba-7b-instruct")
model = AutoModelForCausalLM.from_pretrained(
    "tiiuae/falcon-mamba-7b-instruct",
    device_map="auto"
)

input_ids = tokenizer("Explain the difference between transformers and SSMs", return_tensors="pt").to(model.device)

output = model.generate(**input_ids, max_new_tokens=100, cache_implementation="static")
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

```bash
transformers chat tiiuae/falcon-mamba-7b-instruct --dtype auto --device 0
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to quantize the weights to 4-bits.

```python
from transformers import AutoTokenizer, BitsAndBytesConfig, FalconMambaForCausalLM

quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_use_double_quant=True,
)

tokenizer = AutoTokenizer.from_pretrained("tiiuae/falcon-mamba-7b")
model = FalconMambaForCausalLM.from_pretrained(
    "tiiuae/falcon-mamba-7b",
    device_map="auto",
    quantization_config=quantization_config,
)

inputs = tokenizer("Explain the concept of state space models in simple terms", return_tensors="pt").to(model.device)
outputs = model.generate(**inputs, max_new_tokens=100)
print(tokenizer.decode(outputs[0], skip_special_tokens=True))
```

## FalconMambaConfig[[transformers.FalconMambaConfig]]

- **vocab_size** (`int`, *optional*, defaults to `50280`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **state_size** (`int`, *optional*, defaults to `16`) --
  Size of the SSM state (latent state dimension) in the Mamba layers.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **layer_norm_epsilon** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `0`) --
  Token id used for end-of-stream in the vocabulary.
- **expand** (`int`, *optional*, defaults to 2) --
  Expanding factor used to determine the intermediate size.
- **conv_kernel** (`int`, *optional*, defaults to 4) --
  Size of the convolution kernel.
- **use_bias** (`bool`, *optional*, defaults to `False`) --
  Whether or not to use bias in ["in_proj", "out_proj"] of the mixer block
- **use_conv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether or not to use bias in the convolution layer of the mixer block.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **initializer_range** (`float`, *optional*, defaults to `0.1`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **residual_in_fp32** (`bool`, *optional*, defaults to `True`) --
  Whether or not residuals should be in `float32`. If set to `False` residuals will keep the same `dtype` as the rest of the model
- **time_step_rank** (`Union[str, int]`, *optional*, defaults to `auto`) --
  Rank of the delta (time step) projection. Can be `"auto"` to set it automatically.
- **time_step_scale** (`float`, *optional*, defaults to `1.0`) --
  Scale applied to the time step delta before discretization.
- **time_step_min** (`float`, *optional*, defaults to `0.001`) --
  Minimum `time_step` used to bound `dt_proj.bias`.
- **time_step_max** (`float`, *optional*, defaults to `0.1`) --
  Maximum `time_step` used to bound `dt_proj.bias`.
- **time_step_init_scheme** (`str`, *optional*, defaults to `random`) --
  Initialization scheme for the time step delta. Can be `"random"` or `"uniform"`.
- **time_step_floor** (`float`, *optional*, defaults to `0.0001`) --
  Minimum allowed value for the discrete time step delta after softplus activation.
- **rescale_prenorm_residual** (`bool`, *optional*, defaults to `False`) --
  Whether or not to rescale `out_proj` weights when initializing.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **use_falcon_mambapy** (`bool`, *optional*, defaults to `False`) --
  This argument corresponds to `use_mambapy` in MambaConfig.
  Determines the fallback strategy during training if the CUDA-based official implementation of Mamba is not available. If `True`, the mamba.py implementation is used. If `False`, the naive and slower implementation is used. Consider switching to the naive version if memory is limited.
- **use_associative_scan** (`bool`, *optional*, defaults to `True`) --
  Whether to use PyTorch's `torch._higher_order_ops.associative_scan` for the parallel scan instead of the naive
  sequential implementation. The associative scan is only active during `torch.compile` tracing and
  requires torch >= 2.9.0. Both paths are tested to produce numerically identical results (see
  `test_associative_scan_matches_sequential`). Set to `False` to fall back to the sequential loop.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **mixer_rms_eps** (`float`, *optional*, defaults to 1e-06) --
  The RMS norm epsilon value that is used in the Mixer RMS norm for B, C and dt states.

This is the configuration class to store the configuration of a FalconMambaModel. It is used to instantiate a Falcon Mamba
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tiiuae/falcon-mamba-7b](https://huggingface.co/tiiuae/falcon-mamba-7b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FalconMambaConfig, FalconMambaModel

>>> # Initializing a FalconMamba configuration
>>> configuration = FalconMambaConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = FalconMambaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## FalconMambaModel[[transformers.FalconMambaModel]]

- **config** ([FalconMambaModel](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Falcon Mamba Model outputting raw hidden-states without any specific head on top.

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
- **cache_params** (`Cache`, *optional*) --
  If passed along, the model uses the previous state in all the blocks (which will give the output for the
  `input_ids` provided as if the model add `state_input_ids + input_ids` as context).
- **use_cache** (`bool`, *optional*) --
  If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`FalconMambaOutput` or `tuple(torch.FloatTensor)`A `FalconMambaOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FalconMambaConfig](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaConfig)) and inputs.
The [FalconMambaModel](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **cache_params** (`~cache_utils.Cache`, *optional*) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.

  Includes both the State space model state matrices after the selective scan, and the Convolutional states
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## FalconMambaLMHeadModel[[transformers.FalconMambaForCausalLM]]

- **config** ([FalconMambaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The FALCON_MAMBA Model transformer with a language modeling head on top (linear layer with weights tied to the input
embeddings).

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
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **cache_params** (`Cache`, *optional*) --
  If passed along, the model uses the previous state in all the blocks (which will give the output for the
  `input_ids` provided as if the model add `state_input_ids + input_ids` as context).
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100`
  are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`FalconMambaCausalLMOutput` or `tuple(torch.FloatTensor)`A `FalconMambaCausalLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FalconMambaConfig](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaConfig)) and inputs.
The [FalconMambaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/falcon_mamba#transformers.FalconMambaForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **cache_params** (`~cache_utils.Cache`, *optional*) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.

  Includes both the State space model state matrices after the selective scan, and the Convolutional states
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```
