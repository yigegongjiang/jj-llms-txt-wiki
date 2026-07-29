# LongCatFlash

## Overview

The LongCatFlash model was proposed in [LongCat-Flash Technical Report](https://huggingface.co/papers/2509.01322) by the Meituan LongCat Team.
LongCat-Flash is a 560B parameter Mixture-of-Experts (MoE) model that activates 18.6B-31.3B parameters dynamically (average ~27B). The model features a shortcut-connected architecture enabling high inference speed (>100 tokens/second) and advanced reasoning capabilities.

The abstract from the paper is the following:

*We present LongCat-Flash, a 560 billion parameter Mixture-of-Experts (MoE) language model featuring a dynamic computation mechanism that activates 18.6B-31.3B parameters based on context (average ~27B). The model incorporates a shortcut-connected architecture enabling high inference speed (>100 tokens/second) and demonstrates strong performance across multiple benchmarks including 89.71% accuracy on MMLU and exceptional agentic tool use capabilities.*

Tips:

- LongCat-Flash uses a unique shortcut-connected MoE architecture that enables faster inference compared to traditional MoE models
- The model supports up to 128k context length for long-form tasks
- Dynamic parameter activation makes it computationally efficient while maintaining high performance
- Best suited for applications requiring strong reasoning, coding, and tool-calling capabilities
- The MoE architecture includes zero experts (nn.Identity modules) which act as skip connections, allowing tokens to bypass expert computation when appropriate

This model was contributed by [Molbap](https://huggingface.co/Molbap).
The original code can be found [here](https://huggingface.co/meituan-longcat/LongCat-Flash-Chat).

## Usage examples

The model is large: you will need 2x8 H100 to run inference.

```python
# launch_longcat.py

from transformers import AutoTokenizer, LongcatFlashForCausalLM

model_id = "meituan-longcat/LongCat-Flash-Chat"

tokenizer = AutoTokenizer.from_pretrained(model_id)

chat = [
      {"role": "user", "content": "Hello! What is the capital of France? What can you tell me about it?"},
]

model = LongcatFlashForCausalLM.from_pretrained(
      model_id,
      tp_plan="auto",
 device_map="auto")

inputs = tokenizer.apply_chat_template(
      chat, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)

outputs = model.generate(inputs, max_new_tokens=30)
print(tokenizer.batch_decode(outputs))
```

To run with TP, you will need torchrun:

```bash
torchrun  --nproc_per_node=8 --nnodes=2 --node_rank=0 | 1  --rdzv-id <an_id> --rdzv-backend c10d --rdzv-endpoint $NODE_ID:$NODE_PORT  --log-dir ./logs_longcat launch_longcat.py
```

And you'll get a nice generation:

```json
[Round 0] USER:Hello! What is the capital of France? What can you tell me about it? ASSISTANT:Hello! 😊 The capital of France is Paris, one of the most famous and beloved cities in the world. Here’s a quick overview of what makes Paris special:
1. Iconic Landmarks

    Eiffel Tower – The global symbol of France, built in 1889 for the World's Fair.
    Notre-Dame Cathedral – A masterpiece of Gothic architecture (currently under restoration after the 2019 fire).
    Louvre Museum – The world’s largest art museum, home to the Mona Lisa and Venus de Milo.
    Sacré-Cœur Basilica – A stunning white church atop Montmartre with panoramic views.
    Arc de Triomphe – Honors French military victories, with the Tomb of the Unknown Soldier beneath it.
    Champs-Élysées – A glamorous avenue leading to the Arc de Triomphe, lined with shops and cafés.

2. Culture & Arts

    Paris is the "City of Light" (La Ville Lumière), a nickname from its early adoption of street lighting and its role as a center of enlightenment.
    It’s a global hub for fashion (haute couture, Paris Fashion Week) and art (Impressionism, Picasso, Dali).
    Famous literary figures like Hemingway, Fitzgerald, and Sartre lived and wrote here.

3. Food & Cuisine

    Croissants, baguettes, macarons, and crème brûlée are just a few of its culinary delights.
    Paris has over 100 Michelin-starred restaurants and countless cozy bistros.
    The Marché d’Aligre and Rue Mouffetard are great for fresh produce and local flavors.

4. History & Politics

    Founded in the 3rd century BC by the Parisii tribe, it became a major European city under the Romans.
    The French Revolution (1789–1799) began here, leading to the fall of the monarchy.
    Today, it’s the political and economic heart of France, housing the French President’s residence (Élysée Palace) and the National Assembly.

**
```

## LongcatFlashConfig[[transformers.LongcatFlashConfig]]

- **vocab_size** (`int`, *optional*, defaults to `131072`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `6144`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `56`) --
  Number of hidden layers in the Transformer decoder.
- **num_layers** (`int`, *optional*, defaults to `28`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `64`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*) --
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
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **ffn_hidden_size** (`int`, *optional*, defaults to 12288) --
  Dimension of the MLP representations.
- **q_lora_rank** (`int`, *optional*, defaults to `1536`) --
  Rank of the LoRA matrices for query projections.
- **kv_lora_rank** (`int`, *optional*, defaults to `512`) --
  Rank of the LoRA matrices for key and value projections.
- **qk_nope_head_dim** (`int`, *optional*, defaults to `128`) --
  Dimension of the query/key heads that don't use rotary position embeddings.
- **qk_rope_head_dim** (`int`, *optional*, defaults to `64`) --
  Dimension of the query/key heads that use rotary position embeddings.
- **head_dim** (`int`, *optional*, defaults to `64`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **v_head_dim** (`int`, *optional*, defaults to `128`) --
  Dimension of the value heads.
- **qk_head_dim** (`int`, *optional*) --
  The total dimension of query/key heads. If not specified, set to `qk_nope_head_dim + qk_rope_head_dim`.
- **moe_topk** (`int`, *optional*, defaults to 12) --
  Number of experts to route to for each token in the MoE layer.
- **n_routed_experts** (`int`, *optional*, defaults to `512`) --
  Number of routed experts.
- **zero_expert_num** (`int`, *optional*, defaults to 256) --
  Number of zero experts (identity function) to add to the expert pool.
- **expert_ffn_hidden_size** (`int`, *optional*, defaults to 2048) --
  Hidden size of individual expert FFN layers.
- **routed_scaling_factor** (`float`, *optional*, defaults to `6.0`) --
  Scaling factor or routed experts.

This is the configuration class to store the configuration of a LongcatFlashModel. It is used to instantiate a Longcat Flash
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meituan-longcat/LongCat-Flash-Chat](https://huggingface.co/meituan-longcat/LongCat-Flash-Chat)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import LongcatFlashModel, LongcatFlashConfig

>>> # Initializing a LongCat Flash style configuration
>>> configuration = LongcatFlashConfig()

>>> # Initializing a model from the configuration
>>> model = LongcatFlashModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LongcatFlashPreTrainedModel[[transformers.LongcatFlashPreTrainedModel]]

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

## LongcatFlashModel[[transformers.LongcatFlashModel]]

- **config** ([LongcatFlashModel](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Longcat Flash Model outputting raw hidden-states without any specific head on top.

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
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LongcatFlashConfig](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashConfig)) and inputs.
The [LongcatFlashModel](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
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

## LongcatFlashForCausalLM[[transformers.LongcatFlashForCausalLM]]

- **config** ([LongcatFlashForCausalLM](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Longcat Flash Model for causal language modeling.

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
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LongcatFlashConfig](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashConfig)) and inputs.
The [LongcatFlashForCausalLM](/docs/transformers/v5.14.0/en/model_doc/longcat_flash#transformers.LongcatFlashForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
>>> from transformers import AutoTokenizer, LongcatFlashForCausalLM

>>> model = LongcatFlashForCausalLM.from_pretrained("meta-longcat_flash/LongcatFlash-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-longcat_flash/LongcatFlash-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
