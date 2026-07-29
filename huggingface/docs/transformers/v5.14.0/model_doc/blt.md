# Byte Latent Transformer (BLT)

## Overview

The BLT model was proposed in [Byte Latent Transformer: Patches Scale Better Than Tokens](https://huggingface.co/papers/2412.09871) by Artidoro Pagnoni, Ram Pasunuru, Pedro Rodriguez, John Nguyen, Benjamin Muller, Margaret Li1, Chunting Zhou, Lili Yu, Jason Weston, Luke Zettlemoyer, Gargi Ghosh, Mike Lewis, Ari Holtzman†, Srinivasan Iyer.
BLT is a byte-level LLM that achieves tokenization-level performance through entropy-based dynamic patching.

The abstract from the paper is the following:

*We introduce the Byte Latent Transformer (BLT), a new byte-level LLM architecture that, for the first time, matches tokenization-based LLM performance at scale with significant improvements in inference
efficiency and robustness. BLT encodes bytes into dynamically sized patches, which serve as the primary units of computation. Patches are segmented based on the entropy of the next byte, allocating
more compute and model capacity where increased data complexity demands it. We present the first flop controlled scaling study of byte-level models up to 8B parameters and 4T training bytes. Our results demonstrate the feasibility of scaling models trained on raw bytes without a fixed vocabulary. Both training and inference efficiency improve due to dynamically selecting long patches when data is predictable, along with qualitative improvements on reasoning and long tail generalization. Overall, for fixed inference costs, BLT shows significantly better scaling than tokenization-based models, by simultaneously growing both patch and model size.*

## Usage Tips

- **Dual Model Architecture**: BLT consists of two separate trained models:
  - **Patcher (Entropy Model)**: A smaller transformer model that predicts byte-level entropy to determine patch boundaries and segment input.
  - **Main Transformer Model**: The primary model that processes the patches through a Local Encoder, Global Transformer, and Local Decoder.

- **Dynamic Patching**: The model uses entropy-based dynamic patching where:
  - High-entropy regions (complex data) get shorter patches with more computational attention
  - Low-entropy regions (predictable data) get longer patches for efficiency
  - This allows the model to allocate compute resources where they're most needed

- **Local Encoder**: Processes byte sequences with cross-attention to patch embeddings
- **Global Transformer**: Processes patch-level representations with full attention across patches
- **Local Decoder**: Generates output with cross-attention back to the original byte sequence

- **Byte-Level Tokenizer**: Unlike traditional tokenizers that use learned vocabularies, BLT's tokenizer simply converts text to UTF-8 bytes and maps each byte to a token ID. There is no need for a vocabulary.

The model can be loaded via:

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("itazap/blt-1b-hf")
model = AutoModelForCausalLM.from_pretrained(
    "itazap/blt-1b-hf",
    device_map="auto",
)

inputs = tokenizer(prompt, return_tensors="pt").to(model.device)

prompt = "my name is"
generated_ids = model.generate(
    **inputs, max_new_tokens=NUM_TOKENS_TO_GENERATE, do_sample=False, use_cache=False
)

print(tokenizer.decode(generated_ids[0]))
```

This model was contributed by [itazap](https://huggingface.co/).
The original code can be found [here]().

## BltConfig[[transformers.BltConfig]]

- **vocab_size** (`int`, *optional*, defaults to `260`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `4096`) --
  The maximum sequence length that this model might ever be used with.
- **patch_in_forward** (`bool`, *optional*, defaults to `True`) --
  Whether to perform patching during the forward pass.
- **patch_size** (`int`, *optional*, defaults to 4) --
  Size of the patches used in the patching mechanism.
- **patching_mode** (`str`, *optional*, defaults to `"entropy"`) --
  The mode used for patching, such as entropy-based patching.
- **patching_threshold** (`float`, *optional*, defaults to 1.34) --
  Threshold value used for determining when to apply patches.
- **patching_batch_size** (`int`, *optional*, defaults to 1) --
  Batch size used during the patching process.
- **max_patch_length** (`int`, *optional*) --
  Maximum length of patches that can be generated.
- **cross_attn_k** (`int`, *optional*, defaults to 2) --
  Number of cross-attention heads used in the model.
- **encoder_hash_byte_group_size** (`list`, *optional*) --
  List of byte group sizes used in the encoder hash function.
- **encoder_hash_byte_group_vocab** (`int`, *optional*, defaults to 500002) --
  Vocabulary size for the encoder hash byte groups.
- **encoder_hash_byte_group_nb_functions** (`int`, *optional*, defaults to 1) --
  Number of hash functions used in the encoder byte grouping.
- **patcher_config** (`BltPatcherConfig`, *optional*) --
  Configuration for the patcher component of the model.
- **encoder_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the encoder backbone.
- **decoder_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the decoder backbone.
- **global_config** (`BltGlobalTransformerConfig`, *optional*) --
  Configuration for the global transformer component of the model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a BltModel. It is used to instantiate a Blt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [itazap/blt-1b-hf](https://huggingface.co/itazap/blt-1b-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import BltModel, BltConfig

>>> # Initializing a Blt configuration
>>> configuration = BltConfig()

>>> # Initializing a model from the configuration
>>> model = BltModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## BltForCausalLM[[transformers.BltForCausalLM]]

- **config** ([BltConfig](/docs/transformers/v5.14.0/en/model_doc/blt#transformers.BltConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Blt Text Model with a language modeling head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>, )>] | None = None"}, {"name": "past_key_values", "val": ": transformers.cache_utils.Cache | None = None"}, {"name": "inputs_embeds", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "labels", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "use_cache", "val": ": bool | None = None"}, {"name": "logits_to_keep", "val": ": typing.Union[int, torch.Tensor] = 0"}, {"name": "**kwargs", "val": ": Unpack"}]}>
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
- **cross_attention_states** (`torch.FloatTensor`, *optional*) --
  Output of the vision model, used for cross-attention. This tensor contains the processed image features that
  the language model will attend to.
- **cross_attention_mask** (`torch.Tensor` of shape `(batch_size, seq_length, max_num_images, max_num_tiles)`, *optional*) --
  Cross-attention mask to control the interaction between text tokens and image tiles.
  This 4D tensor defines which image tiles each text token should attend to.

  For each text token (in seq_length):
  - 1 indicates the token **should attend** to the corresponding image tile
  - 0 indicates the token **should not attend** to the corresponding image tile
- **full_text_row_masked_out_mask** (`tuple[torch.Tensor, torch.Tensor]`, *optional*) --
  A tuple containing two tensors that mask out rows in the cross-attention mechanism:
  - The first tensor has shape `(batch_size, 1, seq_length, 1)` and contains values of 0 or 1.
    A value of 0 indicates that the corresponding text token's entire row in the cross-attention
    matrix should be masked out (all image tokens ignored).
  - The second tensor has the same shape and is used internally to apply the masking during
    the forward pass of cross-attention layers.
  This mask is derived from the cross_attention_mask and is used to handle cases where a text token
  should not attend to any image token.
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
elements depending on the configuration ([BltConfig](/docs/transformers/v5.14.0/en/model_doc/blt#transformers.BltConfig)) and inputs.
The [BltForCausalLM](/docs/transformers/v5.14.0/en/model_doc/blt#transformers.BltForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, BltForCausalLM

>>> model = BltForCausalLM.from_pretrained("itazap/blt-1b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("itazap/blt-1b-hf")

>>> prompt = "If I had to write a haiku, it would be:"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=40, do_sample=True, temperature=0.6)
>>> result = tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
>>> print(result)
If I had to write a haiku, it would be: "Snowflakes gently fall" - simple, yet peaceful.
I love the idea of snowflakes gently falling, each one
```
