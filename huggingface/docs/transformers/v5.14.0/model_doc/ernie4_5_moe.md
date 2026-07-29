# Ernie 4.5 Moe

## Overview

The Ernie 4.5 Moe model was released in the [Ernie 4.5 Model Family](https://ernie.baidu.com/blog/posts/ernie4.5/) release by baidu.
This family of models contains multiple different architectures and model sizes. This model in specific targets the base text
model with mixture of experts (moe) - one with 21B total, 3B active parameters and another one with 300B total, 47B active parameters.
It uses the standard [Llama](./llama) at its core combined with a specialized MoE based on [Mixtral](./mixtral) with additional shared
experts.

Other models from the family can be found at [Ernie 4.5](./ernie4_5) and [Ernie 4.5 VL MoE](./ernie4_5_vl_moe).

    

## Usage Tips

### Generate text

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = "baidu/ERNIE-4.5-21B-A3B-PT"

# load the tokenizer and the model
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    device_map="auto",
)

# prepare the model input
inputs = tokenizer("Hey, are you conscious? Can you talk to me?", return_tensors="pt").to(model.device)
prompt = "Hey, are you conscious? Can you talk to me?"
messages = [
    {"role": "user", "content": prompt}
]
text = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True
)
model_inputs = tokenizer([text], add_special_tokens=False, return_tensors="pt").to(model.device)

# conduct text completion
generated_ids = model.generate(
    **model_inputs,
    max_new_tokens=32,
)
output_ids = generated_ids[0][len(model_inputs.input_ids[0]):].tolist()

# decode the generated ids
generate_text = tokenizer.decode(output_ids, skip_special_tokens=True)
```

### Distributed Generation with Tensor Parallelism

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = "baidu/ERNIE-4.5-21B-A3B-PT"

# load the tokenizer and the model
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    device_map="auto",
    tp_plan="auto",
)

# prepare the model input
inputs = tokenizer("Hey, are you conscious? Can you talk to me?", return_tensors="pt").to(model.device)
prompt = "Hey, are you conscious? Can you talk to me?"
messages = [
    {"role": "user", "content": prompt}
]
text = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True
)
model_inputs = tokenizer([text], add_special_tokens=False, return_tensors="pt").to(model.device)

# conduct text completion
generated_ids = model.generate(
    **model_inputs,
    max_new_tokens=32,
)
output_ids = generated_ids[0][len(model_inputs.input_ids[0]):].tolist()

# decode the generated ids
generate_text = tokenizer.decode(output_ids, skip_special_tokens=True)
```

### Quantization with Bitsandbytes

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

model_name = "baidu/ERNIE-4.5-21B-A3B-PT"

# load the tokenizer and the model
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    device_map="auto",
    quantization_config=BitsAndBytesConfig(load_in_4bit=True),
)

# prepare the model input
inputs = tokenizer("Hey, are you conscious? Can you talk to me?", return_tensors="pt").to(model.device)
prompt = "Hey, are you conscious? Can you talk to me?"
messages = [
    {"role": "user", "content": prompt}
]
text = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True
)
model_inputs = tokenizer([text], add_special_tokens=False, return_tensors="pt").to(model.device)

# conduct text completion
generated_ids = model.generate(
    **model_inputs,
    max_new_tokens=32,
)
output_ids = generated_ids[0][len(model_inputs.input_ids[0]):].tolist()

# decode the generated ids
generate_text = tokenizer.decode(output_ids, skip_special_tokens=True)
```

This model was contributed by [Anton Vlasjuk](https://huggingface.co/AntonV).
The original code can be found [here](https://github.com/PaddlePaddle/ERNIE).

## Ernie4_5_MoeConfig[[transformers.Ernie4_5_MoeConfig]]

- **vocab_size** (`int`, *optional*, defaults to `103424`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **hidden_size** (`int`, *optional*, defaults to `2560`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `12288`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `28`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `20`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `4`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `131072`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **use_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in any of the projections including mlp and attention for example.
- **moe_intermediate_size** (`int`, *optional*, defaults to `1536`) --
  Intermediate size of the routed expert MLPs.
- **moe_k** (`int`, *optional*, defaults to 6) --
  Number of selected experts.
- **moe_num_experts** (`int`, *optional*, defaults to 64) --
  Number of routed experts.
- **moe_num_shared_experts** (`int`, *optional*, defaults to 2) --
  The number of experts that are shared for all MoE forwards.
- **moe_layer_start_index** (`int`, *optional*, defaults to 1) --
  The first index at which MoE layers start to appear.
- **moe_layer_end_index** (`int`, *optional*, defaults to -1) --
  The last possible index for a MoE layer.
- **moe_layer_interval** (`int`, *optional*, defaults to 1) --
  The intervals between MoE layers to appear.
- **moe_norm_min** (`float`, *optional*, defaults to 1e-12) --
  Minimum division value during routing normalization.
- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

This is the configuration class to store the configuration of a Ernie4_5_MoeModel. It is used to instantiate a Ernie4 5 Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [baidu/ERNIE-4.5-21B-A3B-PT](https://huggingface.co/baidu/ERNIE-4.5-21B-A3B-PT)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Ernie4_5_MoeModel, Ernie4_5_MoEConfig

>>> # Initializing a Ernie4_5_MoE style configuration
>>> configuration = Ernie4_5_MoEConfig()

>>> # Initializing a model from the ERNIE-4.5-21B-A3B style configuration
>>> model = Ernie4_5_MoeModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Ernie4_5_MoeModel[[transformers.Ernie4_5_MoeModel]]

- **config** ([Ernie4_5_MoeConfig](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Ernie4 5 Moe Model outputting raw hidden-states without any specific head on top.

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

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ernie4_5_MoeConfig](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeConfig)) and inputs.
The [Ernie4_5_MoeModel](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## Ernie4_5_MoeForCausalLM[[transformers.Ernie4_5_MoeForCausalLM]]

- **config** ([Ernie4_5_MoeForCausalLM](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Ernie4 5 Moe Model for causal language modeling.

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

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_router_logits** (`bool`, *optional*) --
  Whether or not to return the logits of all the routers. They are useful for computing the router loss, and
  should not be returned during inference.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`MoeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ernie4_5_MoeConfig](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeConfig)) and inputs.
The [Ernie4_5_MoeForCausalLM](/docs/transformers/v5.14.0/en/model_doc/ernie4_5_moe#transformers.Ernie4_5_MoeForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
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

- **inputs** (`torch.Tensor` of varying shape depending on the modality, *optional*) --
  The sequence used as a prompt for the generation or as model inputs to the encoder. If `None` the
  method initializes it with `bos_token_id` and a batch size of 1. For decoder-only models `inputs`
  should be in the format of `input_ids`. For encoder-decoder models *inputs* can represent any of
  `input_ids`, `input_values`, `input_features`, or `pixel_values`.
- **generation_config** ([GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) --
  The generation configuration to be used as base parametrization for the generation call. `**kwargs`
  passed to generate matching the attributes of `generation_config` will override them. If
  `generation_config` is not provided, the default will be used, which has the following loading
  priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model
  configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig)'s
  default values, whose documentation should be checked to parameterize generation.
- **logits_processor** (`LogitsProcessorList`, *optional*) --
  Custom logits processors that complement the default logits processors built from arguments and
  generation config. If a logit processor is passed that is already created with the arguments or a
  generation config an error is thrown. This feature is intended for advanced users.
- **stopping_criteria** (`StoppingCriteriaList`, *optional*) --
  Custom stopping criteria that complements the default stopping criteria built from arguments and a
  generation config. If a stopping criteria is passed that is already created with the arguments or a
  generation config an error is thrown. If your stopping criteria depends on the `scores` input, make
  sure you pass `return_dict_in_generate=True, output_scores=True` to `generate`. This feature is
  intended for advanced users.
- **prefix_allowed_tokens_fn** (`Callable[[int, torch.Tensor], list[int]]`, *optional*) --
  If provided, this function constraints the beam search to allowed tokens only at each step. If not
  provided no constraint is applied. This function takes 2 arguments: the batch ID `batch_id` and
  `input_ids`. It has to return a list with the allowed tokens for the next generation step conditioned
  on the batch ID `batch_id` and the previously generated tokens `inputs_ids`. This argument is useful
  for constrained generation conditioned on the prefix, as described in [Autoregressive Entity
  Retrieval](https://huggingface.co/papers/2010.00904).
- **synced_gpus** (`bool`, *optional*) --
  Whether to continue running the while loop until max_length. Unless overridden, this flag will be set
  to `True` if using `FullyShardedDataParallel` or DeepSpeed ZeRO Stage 3 with multiple GPUs to avoid
  deadlocking if one GPU finishes generating before other GPUs. Otherwise, defaults to `False`.
- **assistant_model** (`PreTrainedModel`, *optional*) --
  An assistant model that can be used to accelerate generation. The assistant model must have the exact
  same tokenizer. The acceleration is achieved when forecasting candidate tokens with the assistant model
  is much faster than running generation with the model you're calling generate from. As such, the
  assistant model should be much smaller.
- **streamer** (`BaseStreamer`, *optional*) --
  Streamer object that will be used to stream the generated sequences. Generated tokens are passed
  through `streamer.put(token_ids)` and the streamer is responsible for any further processing.
- **negative_prompt_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  The negative prompt needed for some processors such as CFG. The batch size must match the input batch
  size. This is an experimental feature, subject to breaking API changes in future versions.
- **negative_prompt_attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Attention_mask for `negative_prompt_ids`.
- **custom_generate** (`str` or `Callable`, *optional*) --
  One of the following:
  - `str` (Hugging Face Hub repository name): runs the custom `generate` function defined at
    `custom_generate/generate.py` in that repository instead of the standard `generate` method. The
    repository fully replaces the generation logic, and the return type may differ.
  - `str` (local repository path): same as above but from a local path. Local directories also
    require `trust_remote_code=True` because the local `custom_generate/generate.py` is executed.
  - `Callable`: `generate` will perform the usual input preparation steps, then call the provided callable to
    run the decoding loop.
  For more information, see [the docs](../../generation_strategies#custom-generation-methods).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Ad hoc parametrization of `generation_config` and/or additional model-specific kwargs that will be
  forwarded to the `forward` function of the model. If the model is an encoder-decoder model, encoder
  specific kwargs should not be prefixed and decoder specific kwargs should be prefixed with *decoder_*.[ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`A [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.LongTensor`.

If the model is *not* an encoder-decoder model (`model.config.is_encoder_decoder=False`), the possible
[ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateDecoderOnlyOutput](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.generation.GenerateDecoderOnlyOutput),
- [GenerateBeamDecoderOnlyOutput](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.generation.GenerateBeamDecoderOnlyOutput)

If the model is an encoder-decoder model (`model.config.is_encoder_decoder=True`), the possible
[ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateEncoderDecoderOutput](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids for models with a language modeling head.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](../generation_strategies).
