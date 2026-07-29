# Evolla

## Overview

The Evolla model was proposed in [Decoding the Molecular Language of Proteins with Evolla](https://doi.org/10.1101/2025.01.05.630192) by [Zhou et al.](https://doi.org/10.1101/2025.01.05.630192).

Evolla is an advanced 80-billion-parameter protein-language generative model designed to decode the molecular language of proteins. It integrates information from protein sequences, structures, and user queries to generate precise and contextually nuanced insights into protein function. Trained on an unprecedented AI-generated dataset of 546 million protein question-answer pairs and 150 billion word tokens, Evolla significantly advances research in proteomics and functional genomics, providing expert-level insights and shedding light on the molecular logic encoded in proteins.

The abstract from the paper is the following:

*Proteins, nature's intricate molecular machines, are the products of billions of years of evolution and play fundamental roles in sustaining life. Yet, deciphering their molecular language - that is, understanding how protein sequences and structures encode and determine biological functions - remains a corner-stone challenge in modern biology. Here, we introduce Evolla, an 80 billion frontier protein-language generative model designed to decode the molecular language of proteins. By integrating information from protein sequences, structures, and user queries, Evolla generates precise and contextually nuanced insights into protein function. A key innovation of Evolla lies in its training on an unprecedented AI-generated dataset: 546 million protein question-answer pairs and 150 billion word tokens, designed to reflect the immense complexity and functional diversity of proteins. Post-pretraining, Evolla integrates Direct Preference Optimization (DPO) to refine the model based on preference signals and Retrieval-Augmented Generation (RAG) for external knowledge incorporation, improving response quality and relevance. To evaluate its performance, we propose a novel framework, Instructional Response Space (IRS), demonstrating that Evolla delivers expert-level insights, advancing research in proteomics and functional genomics while shedding light on the molecular logic encoded in proteins. The online demo is available at http://www.chat-protein.com/.*

Examples:

```python
processor = EvollaProcessor.from_pretrained("westlake-repl/Evolla-10B-DPO-hf")
model = EvollaForProteinText2Text.from_pretrained("westlake-repl/Evolla-10B-DPO-hf", device_map="auto")
# aa_seq should have same length as foldseek
protein_inputs = [
    {

        "aa_seq": "MATGGRRG...",
        "foldseek": "###lqpfd...", # hashtag means the low-confidence foldseek tokens
    },
    {
        "aa_seq": "MLPGLALL...",
        "foldseek": "dfwwkwad...",
    }
]
message_list = [
    [
        {
            "role": "system",
            "content": "You are an AI expert that can answer any questions about protein.",
        },
        {"role": "user", "content": "What is the function of this protein?"},
    ],
    [
        {
            "role": "system",
            "content": "You are an AI expert that can answer any questions about protein.",
        },
        {"role": "user", "content": "What is the function of this protein?"},
    ]
]
input_dict = processor(
    protein_inputs, messages_list, return_tensors="pt", text_max_length=512, protein_max_length=1024
)
with torch.no_grad():
    generated_ids = hf_model.generate(**input_dict)
generated_texts = processor.batch_decode(
    generated_ids, skip_special_tokens=True
)
```

Tips:

- This model was contributed by [Xibin Bayes Zhou](https://huggingface.co/XibinBayesZhou).
- The original code can be found [here](https://github.com/westlake-repl/Evolla).

## EvollaConfig[[transformers.EvollaConfig]]

- **protein_encoder_config** (`dict`, *optional*) --
  Dictionary of configuration options used to initialize `SaProtConfig`.
- **vocab_size** (`int`, *optional*, defaults to `128256`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `14336`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `8`) --
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
- **max_position_embeddings** (`int`, *optional*, defaults to `8192`) --
  The maximum sequence length that this model might ever be used with.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **aligner_ffn_mult** (`int`, *optional*, defaults to 4) --
  The FFN multiplier for the aligner layer.
- **aligner_enable_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the aligner layer.
- **aligner_attention_probs_dropout_prob** (`float`, *optional*, defaults to 0.1) --
  The dropout ratio for the attention probabilities in the aligner layer.
- **aligner_num_add_layers** (`int`, *optional*, defaults to 8) --
  The number of additional layers for the aligner layer.
- **resampler_depth** (`int`, *optional*, defaults to 6) --
  The depth of the resampler layer in the llama model.
- **resampler_dim_head** (`int`, *optional*, defaults to 64) --
  The dimension of the heads in the resampler layer in the llama model.
- **resampler_heads** (`int`, *optional*, defaults to 8) --
  The number of heads in the resampler layer in the llama model.
- **resampler_num_latents** (`int`, *optional*, defaults to 64) --
  The number of latents in the resampler layer in the llama model.
- **resampler_ff_mult** (`int`, *optional*, defaults to 4) --
  The FFN multiplier for the resampler layer.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `128000`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `128009`) --
  Token id used for end-of-stream in the vocabulary.
- **use_cache** (`bool`, *optional*, defaults to `False`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.

This is the configuration class to store the configuration of a EvollaModel. It is used to instantiate a Evolla
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [westlake-repl/Evolla-10B-hf](https://huggingface.co/westlake-repl/Evolla-10B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import EvollaModel, EvollaConfig

>>> # Initializing a Evolla evolla-10b style configuration
>>> configuration = EvollaConfig()

>>> # Initializing a model from the evolla-10b style configuration
>>> model = EvollaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## EvollaModel[[transformers.EvollaModel]]

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
  `past_key_values`).
- **protein_input_ids** (`torch.LongTensor`, *optional*) --
  The input IDs for the protein sequence in structure-aware tokens. Should be of shape `(batch_size, protein_seq_length)` and type `torch.LongTensor`.
- **protein_attention_mask** (`torch.Tensor`, *optional*) --
  The attention mask for the protein sequence. Should be of shape `(batch_size, protein_seq_length)` and type `torch.Tensor`.
- **structure_feats** (`torch.FloatTensor`, *optional*) --
  The input IDs for purely structure-based features. Should be of shape `(batch_size, structure_seq_length, structure_feat_dim)` and type `torch.FloatTensor`. Dummy input for now.
- **msa_feats** (`torch.FloatTensor`, *optional*) --
  The input IDs for purely MSA-based features. Should be of shape `(batch_size, msa_seq_length, msa_feat_dim)` and type `torch.FloatTensor`. Dummy input for now.
- **structure_batch_mask** (`torch.Tensor`, *optional*) --
  The batch mask to decide which protein sequences are purely structure-based. Should be of shape `(batch_size)` and type `torch.Tensor`. Should be paired with `structure_feats`. Dummpy input for now.
- **msa_batch_mask** (`torch.Tensor`, *optional*) --
  The batch mask to decide which protein sequences are purely MSA-based. Should be of shape `(batch_size)` and type `torch.Tensor`. Should be paired with `msa_feats`. Dummpy input for now.[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [EvollaModel](/docs/transformers/v5.14.0/en/model_doc/evolla#transformers.EvollaModel) forward method, overrides the `__call__` special method.

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

## EvollaForProteinText2Text[[transformers.EvollaForProteinText2Text]]

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **protein_input_ids** (`torch.LongTensor`, *optional*) --
  The input IDs for the protein sequence. Should be of shape `(batch_size, protein_seq_length)` and type `torch.LongTensor`.
- **protein_attention_mask** (`torch.Tensor`, *optional*) --
  The attention mask for the protein sequence. Should be of shape `(batch_size, protein_seq_length)` and type `torch.Tensor`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).
The [EvollaForProteinText2Text](/docs/transformers/v5.14.0/en/model_doc/evolla#transformers.EvollaForProteinText2Text) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from transformers import EvollaProcessor, EvollaForProteinText2Text
>>> model = EvollaForProteinText2Text.from_pretrained("westlake/Evolla-10B-hf")
>>> processor = EvollaProcessor.from_pretrained("westlake/Evolla-10B-hf")

>>> protein_information = {
    "aa_seq": "your amino acid sequence",
    "foldseek": "your foldseek sequence",
}
>>> question = "What is the function of this protein?"
>>> message = [
    {"role": "system", "content": "You are an AI expert that can answer any questions about protein."},
    {"role": "user", "content": question},
]

>>> inputs = processor(proteins=[protein_information], messages_list=[message], return_tensors="pt", padding="longest")
>>> outputs = model.generate(**inputs)

>>> print(processor.batch_decode(outputs, skip_special_tokens=True))
```

## EvollaProcessor[[transformers.EvollaProcessor]]

- **protein_tokenizer** (`EsmTokenizer`) --
  An instance of [EsmTokenizer](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmTokenizer). The protein tokenizer is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **protein_max_length** (`int`, *optional*, defaults to 1024) --
  The maximum length of the sequence to be generated.
- **text_max_length** (`int`, *optional*, defaults to 512) --
  The maximum length of the text to be generated.
Constructs a EvollaProcessor which wraps a protein tokenizer and a tokenizer into a single processor.

[EvollaProcessor](/docs/transformers/v5.14.0/en/model_doc/evolla#transformers.EvollaProcessor) offers all the functionalities of `{protein_tokenizer_class}` and `tokenizer_class`. See the
`~{protein_tokenizer_class}` and `~tokenizer_class` for more information.

- **proteins** (`Union[List[dict], dict]`) --
  A list of dictionaries or a single dictionary containing the following keys:
  - `"aa_seq"` (`str`) -- The amino acid sequence of the protein.
  - `"foldseek"` (`str`) -- The foldseek string of the protein.
- **messages_list** (`Union[List[List[dict]], List[dict]]`) --
  A list of lists of dictionaries or a list of dictionaries containing the following keys:
  - `"role"` (`str`) -- The role of the message.
  - `"content"` (`str`) -- The content of the message.
- **protein_max_length** (`int`, *optional*, defaults to 1024) --
  The maximum length of the sequence to be generated.
- **text_max_length** (`int`, *optional*, defaults to 512) --
  The maximum length of the text.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.a dict with following keys- `protein_input_ids` (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The input IDs for the protein sequence.
- `protein_attention_mask` (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The attention mask for the protein sequence.
- `text_input_ids` (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The input IDs for the text sequence.
- `text_attention_mask` (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The attention mask for the text sequence.
