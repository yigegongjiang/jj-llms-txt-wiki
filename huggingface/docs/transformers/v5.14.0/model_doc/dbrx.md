# DBRX

## Overview

DBRX is a [transformer-based](https://www.isattentionallyouneed.com/) decoder-only large language model (LLM) that was trained using next-token prediction.
It uses a *fine-grained* mixture-of-experts (MoE) architecture with 132B total parameters of which 36B parameters are active on any input.
It was pre-trained on 12T tokens of text and code data.
Compared to other open MoE models like Mixtral-8x7B and Grok-1, DBRX is fine-grained, meaning it uses a larger number of smaller experts. DBRX has 16 experts and chooses 4, while Mixtral-8x7B and Grok-1 have 8 experts and choose 2.
This provides 65x more possible combinations of experts and we found that this improves model quality.
DBRX uses rotary position encodings (RoPE), gated linear units (GLU), and grouped query attention (GQA).
It is a BPE based model and uses the GPT-4 tokenizer as described in the [tiktoken](https://github.com/openai/tiktoken) repository.
We made these choices based on exhaustive evaluation and scaling experiments.

DBRX was pretrained on 12T tokens of carefully curated data and a maximum context length of 32K tokens.
We estimate that this data is at least 2x better token-for-token than the data we used to pretrain the MPT family of models.
This new dataset was developed using the full suite of Databricks tools, including Apache Spark™ and Databricks notebooks for data processing, and Unity Catalog for data management and governance.
We used curriculum learning for pretraining, changing the data mix during training in ways we found to substantially improve model quality.

More detailed information about DBRX Instruct and DBRX Base can be found in our [technical blog post](https://www.databricks.com/blog/introducing-dbrx-new-state-art-open-llm).

This model was contributed by [eitan-turok](https://huggingface.co/eitanturok) and [abhi-db](https://huggingface.co/abhi-db).
Note: The original `databricks/dbrx-instruct` checkpoint was closed; [`transformers-community/dbrx-instruct`](https://huggingface.co/transformers-community/dbrx-instruct) is a re-upload for compatibility, and the snippets below use that re-upload.

## Usage Examples

The `generate()` method can be used to generate text using DBRX. You can generate using the standard attention implementation, flash-attention, and the PyTorch scaled dot product attention. The last two attention implementations give speed ups.

```python

from transformers import AutoTokenizer, DbrxForCausalLM

tokenizer = AutoTokenizer.from_pretrained("transformers-community/dbrx-instruct", token="YOUR_HF_TOKEN")
model = DbrxForCausalLM.from_pretrained(
    "transformers-community/dbrx-instruct",
    device_map="auto",
    token="YOUR_HF_TOKEN",
    )

input_text = "What does it take to build a great LLM?"
messages = [{"role": "user", "content": input_text}]
input_ids = tokenizer.apply_chat_template(messages, return_dict=True, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)

outputs = model.generate(**input_ids, max_new_tokens=200)
print(tokenizer.decode(outputs[0]))
```

If you have flash-attention installed (`pip install flash-attn`), it is possible to generate faster. (The HuggingFace documentation for flash-attention can be found [here](https://huggingface.co/docs/transformers/perf_infer_gpu_one#flashattention-2).)

```python

from transformers import AutoTokenizer, DbrxForCausalLM

tokenizer = AutoTokenizer.from_pretrained("transformers-community/dbrx-instruct", token="YOUR_HF_TOKEN")
model = DbrxForCausalLM.from_pretrained(
    "transformers-community/dbrx-instruct",
    device_map="auto",
    token="YOUR_HF_TOKEN",
    attn_implementation="flash_attention_2",
    )

input_text = "What does it take to build a great LLM?"
messages = [{"role": "user", "content": input_text}]
input_ids = tokenizer.apply_chat_template(messages, return_dict=True, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)

outputs = model.generate(**input_ids, max_new_tokens=200)
print(tokenizer.decode(outputs[0]))
```

You can also generate faster using the PyTorch scaled dot product attention. (The HuggingFace documentation for scaled dot product attention can be found [here](https://huggingface.co/docs/transformers/perf_infer_gpu_one#pytorch-scaled-dot-product-attention).)

```python

from transformers import AutoTokenizer, DbrxForCausalLM

tokenizer = AutoTokenizer.from_pretrained("transformers-community/dbrx-instruct", token="YOUR_HF_TOKEN")
model = DbrxForCausalLM.from_pretrained(
    "transformers-community/dbrx-instruct",
    device_map="auto",
    token="YOUR_HF_TOKEN",
    attn_implementation="sdpa",
    )

input_text = "What does it take to build a great LLM?"
messages = [{"role": "user", "content": input_text}]
input_ids = tokenizer.apply_chat_template(messages, return_dict=True, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)

outputs = model.generate(**input_ids, max_new_tokens=200)
print(tokenizer.decode(outputs[0]))
```

## DbrxConfig[[transformers.DbrxConfig]]

- **d_model** (`int`, *optional*, defaults to `2048`) --
  Size of the encoder layers and the pooler layer.
- **n_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **n_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **max_seq_len** (`int`, *optional*, defaults to 2048) --
  The maximum sequence length of the model.
- **vocab_size** (`int`, *optional*, defaults to `32000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **resid_pdrop** (`float`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **emb_pdrop** (`float`, *optional*, defaults to `0.0`) --
  The dropout ratio for the embeddings.
- **attn_config** (`dict`, *optional*) --
  A dictionary used to configure the model's attention module.
- **ffn_config** (`dict`, *optional*) --
  A dictionary used to configure the model's FFN module.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a DbrxModel. It is used to instantiate a Dbrx
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [transformers-community/dbrx-instruct](https://huggingface.co/transformers-community/dbrx-instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import DbrxConfig, DbrxModel

>>> # Initializing a Dbrx configuration
>>> configuration = DbrxConfig(n_layers=2, d_model=256, n_heads=8, vocab_size=128)

>>> # Initializing a model (with random weights) from the configuration
>>> model = DbrxModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DbrxModel[[transformers.DbrxModel]]

- **config** ([DbrxConfig](/docs/transformers/v5.14.0/en/model_doc/dbrx#transformers.DbrxConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Dbrx Model outputting raw hidden-states without any specific head on top.

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
elements depending on the configuration ([DbrxConfig](/docs/transformers/v5.14.0/en/model_doc/dbrx#transformers.DbrxConfig)) and inputs.
The [DbrxModel](/docs/transformers/v5.14.0/en/model_doc/dbrx#transformers.DbrxModel) forward method, overrides the `__call__` special method.

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

## DbrxForCausalLM[[transformers.DbrxForCausalLM]]

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
elements depending on the configuration ([DbrxConfig](/docs/transformers/v5.14.0/en/model_doc/dbrx#transformers.DbrxConfig)) and inputs.
The [DbrxForCausalLM](/docs/transformers/v5.14.0/en/model_doc/dbrx#transformers.DbrxForCausalLM) forward method, overrides the `__call__` special method.

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
>> from transformers import AutoTokenizer, DbrxForCausalLM

>> model = DbrxForCausalLM.from_pretrained("transformers-community/dbrx-instruct")
>> tokenizer = AutoTokenizer.from_pretrained("transformers-community/dbrx-instruct")

>> prompt = "Hey, are you conscious? Can you talk to me?"
>> inputs = tokenizer(prompt, return_tensors="pt")

>> # Generate
>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
