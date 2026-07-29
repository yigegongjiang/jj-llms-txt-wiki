# GPT-2

[GPT-2](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf) is a scaled up version of GPT, a causal transformer language model, with 10x more parameters and training data. The model was pretrained on a 40GB dataset to predict the next word in a sequence based on all the previous words. This approach enabled the model to perform many downstream tasks in a zero-shot setting. The blog post released by OpenAI can be found [here](https://openai.com/index/better-language-models/).

The model architecture uses a unidirectional (causal) attention mechanism where each token can only attend to previous tokens, making it particularly effective for text generation tasks.

You can find all the original GPT-2 checkpoints under the [OpenAI community](https://huggingface.co/openai-community?search_models=gpt) organization.

> [!TIP]
> Click on the GPT-2 models in the right sidebar for more examples of how to apply GPT-2 to different language tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(task="text-generation", model="openai-community/gpt2", device=0)
pipeline("Hello, I'm a language model")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("openai-community/gpt2", device_map="auto", attn_implementation="sdpa")
tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")

input_ids = tokenizer("Hello, I'm a language model", return_tensors="pt").to(model.device)

output = model.generate(**input_ids, cache_implementation="static")
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

One can also serve the model using vLLM with the `transformers backend`.

```bash
vllm serve openai-community/gpt2 --model-imp transformers
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to 4-bits.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype="float16",
    bnb_4bit_use_double_quant=True
)

model = AutoModelForCausalLM.from_pretrained(
    "openai-community/gpt2-xl",
    quantization_config=quantization_config,
    device_map="auto"
)

tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2-xl")
inputs = tokenizer("Once upon a time, there was a magical forest", return_tensors="pt").to(model.device)
outputs = model.generate(**inputs, max_new_tokens=100)
print(tokenizer.decode(outputs[0], skip_special_tokens=True))
```

## Notes

- Pad inputs on the right because GPT-2 uses absolute position embeddings.
- GPT-2 can reuse previously computed key-value attention pairs. Access this feature with the [past_key_values](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Model.forward.past_key_values) parameter in [GPT2Model.forward()](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Model.forward).
- Enable the [scale_attn_by_inverse_layer_idx](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config.scale_attn_by_inverse_layer_idx) and [reorder_and_upcast_attn](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config.reorder_and_upcast_attn) parameters to apply the training stability improvements from [Mistral](./mistral).

## GPT2Config[[transformers.GPT2Config]]

- **vocab_size** (`int`, *optional*, defaults to `50257`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **n_positions** (`int`, *optional*, defaults to `1024`) --
  The maximum sequence length that this model might ever be used with.
- **n_embd** (`int`, *optional*, defaults to `768`) --
  Dimensionality of the embeddings and hidden states.
- **n_layer** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **n_head** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **n_inner** (`int`, *optional*) --
  Dimension of the MLP representations.
- **activation_function** (`str`, *optional*, defaults to `gelu_new`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **resid_pdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **embd_pdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the embeddings.
- **attn_pdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **layer_norm_epsilon** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **summary_type** (`string`, *optional*, defaults to `"cls_index"`) --
  Argument used when doing sequence summary, used in the models [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel).
  Has to be one of the following options:
  - `"last"`: Take the last token hidden state (like XLNet).
  - `"first"`: Take the first token hidden state (like BERT).
  - `"mean"`: Take the mean of all tokens hidden states.
  - `"cls_index"`: Supply a Tensor of classification token position (like GPT/GPT-2).
  - `"attn"`: Not implemented now, use multi-head attention.
- **summary_use_proj** (`bool`, *optional*, defaults to `True`) --
  Argument used when doing sequence summary, used in the models [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel).
  Whether or not to add a projection after the vector extraction.
- **summary_activation** (`str`, *optional*) --
  Argument used when doing sequence summary. Used in for the multiple choice head in
  [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel).
  Pass `"tanh"` for a tanh activation to the output, any other value will result in no activation.
- **summary_proj_to_labels** (`bool`, *optional*, defaults to `True`) --
  Argument used when doing sequence summary, used in the models [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel).
  Whether the projection outputs should have `config.num_labels` or `config.hidden_size` classes.
- **summary_first_dropout** (`float`, *optional*, defaults to 0.1) --
  Argument used when doing sequence summary, used in the models [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel).
  The dropout ratio to be used after the projection and activation.
- **scale_attn_weights** (`bool`, *optional*, defaults to `True`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **bos_token_id** (`int`, *optional*, defaults to `50256`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `50256`) --
  Token id used for end-of-stream in the vocabulary.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **scale_attn_by_inverse_layer_idx** (`bool`, *optional*, defaults to `False`) --
  Whether to additionally scale attention weights by `1 / layer_idx + 1`.
- **reorder_and_upcast_attn** (`bool`, *optional*, defaults to `False`) --
  Whether to scale keys (K) prior to computing attention (dot-product) and upcast attention
  dot-product/softmax to float() when training with mixed precision.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a GPT2Model. It is used to instantiate a Gpt2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openai-community/gpt2](https://huggingface.co/openai-community/gpt2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GPT2Config, GPT2Model

>>> # Initializing a GPT2 configuration
>>> configuration = GPT2Config()

>>> # Initializing a model (with random weights) from the configuration
>>> model = GPT2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GPT2Tokenizer[[transformers.GPT2Tokenizer]]

'"}, {"name": "bos_token", "val": ": tokenizers.AddedToken | str = '<|endoftext|>'"}, {"name": "eos_token", "val": ": tokenizers.AddedToken | str = '<|endoftext|>'"}, {"name": "pad_token", "val": ": tokenizers.AddedToken | str | None = None"}, {"name": "add_prefix_space", "val": " = False"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) --
  Path to the vocabulary file.
- **merges_file** (`str`) --
  Path to the merges file.
- **errors** (`str`, *optional*, defaults to `"replace"`) --
  Paradigm to follow when decoding bytes to UTF-8. See
  [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.
- **unk_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **bos_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The beginning of sequence token.
- **eos_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The end of sequence token.
- **pad_token** (`str`, *optional*) --
  The token used for padding, for example when batching sequences of different lengths.
- **add_prefix_space** (`bool`, *optional*, defaults to `False`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word. (GPT2 tokenizer detect beginning of words by the preceding space).
- **add_bos_token** (`bool`, *optional*, defaults to `False`) --
  Whether or not to add an initial beginning of sentence token to the input. This allows to treat the leading
  word just as any other word.
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file`.

Construct a GPT-2 tokenizer. Based on byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import GPT2Tokenizer

>>> tokenizer = GPT2Tokenizer.from_pretrained("openai-community/gpt2")
>>> tokenizer("Hello world")["input_ids"]
[15496, 995]

>>> tokenizer(" Hello world")["input_ids"]
[18435, 995]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer will add a space before each word (even the first one).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

## GPT2TokenizerFast[[transformers.GPT2Tokenizer]]

'"}, {"name": "bos_token", "val": ": tokenizers.AddedToken | str = '<|endoftext|>'"}, {"name": "eos_token", "val": ": tokenizers.AddedToken | str = '<|endoftext|>'"}, {"name": "pad_token", "val": ": tokenizers.AddedToken | str | None = None"}, {"name": "add_prefix_space", "val": " = False"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) --
  Path to the vocabulary file.
- **merges_file** (`str`) --
  Path to the merges file.
- **errors** (`str`, *optional*, defaults to `"replace"`) --
  Paradigm to follow when decoding bytes to UTF-8. See
  [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.
- **unk_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **bos_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The beginning of sequence token.
- **eos_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The end of sequence token.
- **pad_token** (`str`, *optional*) --
  The token used for padding, for example when batching sequences of different lengths.
- **add_prefix_space** (`bool`, *optional*, defaults to `False`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word. (GPT2 tokenizer detect beginning of words by the preceding space).
- **add_bos_token** (`bool`, *optional*, defaults to `False`) --
  Whether or not to add an initial beginning of sentence token to the input. This allows to treat the leading
  word just as any other word.
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file`.

Construct a GPT-2 tokenizer. Based on byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import GPT2Tokenizer

>>> tokenizer = GPT2Tokenizer.from_pretrained("openai-community/gpt2")
>>> tokenizer("Hello world")["input_ids"]
[15496, 995]

>>> tokenizer(" Hello world")["input_ids"]
[18435, 995]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer will add a space before each word (even the first one).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

## GPT2 specific outputs[[transformers.models.gpt2.modeling_gpt2.GPT2DoubleHeadsModelOutput]]

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) --
  Language modeling loss.
- **mc_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `mc_labels` is provided) --
  Multiple choice classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_choices, sequence_length, config.vocab_size)`) --
  Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **mc_logits** (`torch.FloatTensor` of shape `(batch_size, num_choices)`) --
  Prediction scores of the multiple choice classification head (scores for each choice before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) --
  It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Base class for outputs of models predicting if two sentences are consecutive or not.

## GPT2Model[[transformers.GPT2Model]]

- **config** ([GPT2Model](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Model)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Gpt2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2Model](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Model) forward method, overrides the `__call__` special method.

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
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.

## GPT2LMHeadModel[[transformers.GPT2LMHeadModel]]

- **config** ([GPT2LMHeadModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2LMHeadModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GPT2 Model transformer with a language modeling head on top (linear layer with weights tied to the input
embeddings).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **labels** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100`
  are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2LMHeadModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2LMHeadModel) forward method, overrides the `__call__` special method.

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
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Cross attentions weights after the attention softmax, used to compute the weighted average in the
  cross-attention heads.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer, GPT2LMHeadModel

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2LMHeadModel.from_pretrained("openai-community/gpt2")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")
>>> outputs = model(**inputs, labels=inputs["input_ids"])
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## GPT2DoubleHeadsModel[[transformers.GPT2DoubleHeadsModel]]

- **config** ([GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GPT2 Model transformer with a language modeling and a multiple-choice classification head on top e.g. for
RocStories/SWAG tasks. The two heads are two linear layers. The language modeling head has its weights tied to the
input embeddings, the classification head takes as input the input of a specified classification token index in the
input sequence).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **mc_token_ids** (`torch.LongTensor` of shape `(batch_size, num_choices)`, *optional*, default to index of the last token of the input) --
  Index of the classification token in each input sequence. Selected in the range `[0, input_ids.size(-1) -
  1]`.
- **labels** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids`. Indices are selected in `[-100, 0, ..., config.vocab_size - 1]`. All labels set to
  `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size - 1]`
- **mc_labels** (`torch.LongTensor` of shape `(batch_size)`, *optional*) --
  Labels for computing the multiple choice classification loss. Indices should be in `[0, ..., num_choices]`
  where *num_choices* is the size of the second dimension of the input tensors. (see *input_ids* above)
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[GPT2DoubleHeadsModelOutput](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.models.gpt2.modeling_gpt2.GPT2DoubleHeadsModelOutput) or `tuple(torch.FloatTensor)`A [GPT2DoubleHeadsModelOutput](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.models.gpt2.modeling_gpt2.GPT2DoubleHeadsModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2DoubleHeadsModel](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2DoubleHeadsModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **mc_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `mc_labels` is provided) -- Multiple choice classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_choices, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **mc_logits** (`torch.FloatTensor` of shape `(batch_size, num_choices)`) -- Prediction scores of the multiple choice classification head (scores for each choice before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer, GPT2DoubleHeadsModel

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2DoubleHeadsModel.from_pretrained("openai-community/gpt2")

>>> # Add a [CLS] to the vocabulary (we should train it also!)
>>> num_added_tokens = tokenizer.add_special_tokens({"cls_token": "[CLS]"})
>>> # Update the model embeddings with the new vocabulary size
>>> embedding_layer = model.resize_token_embeddings(len(tokenizer))

>>> choices = ["Hello, my dog is cute [CLS]", "Hello, my cat is cute [CLS]"]
>>> encoded_choices = [tokenizer.encode(s) for s in choices]
>>> cls_token_location = [tokens.index(tokenizer.cls_token_id) for tokens in encoded_choices]

>>> input_ids = torch.tensor(encoded_choices).unsqueeze(0)  # Batch size: 1, number of choices: 2
>>> mc_token_ids = torch.tensor([cls_token_location])  # Batch size: 1

>>> outputs = model(input_ids, mc_token_ids=mc_token_ids)
>>> lm_logits = outputs.logits
>>> mc_logits = outputs.mc_logits
```

## GPT2ForQuestionAnswering[[transformers.GPT2ForQuestionAnswering]]

- **config** ([GPT2ForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForQuestionAnswering)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Gpt2 transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **start_positions** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the start of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **end_positions** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the end of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.[QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`A [QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2ForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForQuestionAnswering) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.
- **start_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-start scores (before SoftMax).
- **end_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-end scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, GPT2ForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2ForQuestionAnswering.from_pretrained("openai-community/gpt2")

>>> question, text = "Who was Jim Henson?", "Jim Henson was a nice puppet"

>>> inputs = tokenizer(question, text, return_tensors="pt")
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> answer_start_index = outputs.start_logits.argmax()
>>> answer_end_index = outputs.end_logits.argmax()

>>> predict_answer_tokens = inputs.input_ids[0, answer_start_index : answer_end_index + 1]
>>> tokenizer.decode(predict_answer_tokens, skip_special_tokens=True)
...

>>> # target is "nice puppet"
>>> target_start_index = torch.tensor([14])
>>> target_end_index = torch.tensor([15])

>>> outputs = model(**inputs, start_positions=target_start_index, end_positions=target_end_index)
>>> loss = outputs.loss
>>> round(loss.item(), 2)
...
```

## GPT2ForSequenceClassification[[transformers.GPT2ForSequenceClassification]]

- **config** ([GPT2ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForSequenceClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GPT2 Model transformer with a sequence classification head on top (linear layer).

[GPT2ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForSequenceClassification) uses the last token in order to do the classification, as other causal models
(e.g. GPT-1) do.

Since it does classification on the last token, it requires to know the position of the last token. If a
`pad_token_id` is defined in the configuration, it finds the last token that is not a padding token in each row. If
no `pad_token_id` is defined, it simply takes the last value in each row of the batch. Since it cannot guess the
padding tokens when `inputs_embeds` are passed instead of `input_ids`, it does the same (take the last value in
each row of the batch).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the sequence classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`SequenceClassifierOutputWithPast` or `tuple(torch.FloatTensor)`A `SequenceClassifierOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
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

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, GPT2ForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2ForSequenceClassification.from_pretrained("openai-community/gpt2")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = GPT2ForSequenceClassification.from_pretrained("openai-community/gpt2", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, GPT2ForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2ForSequenceClassification.from_pretrained("openai-community/gpt2", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = GPT2ForSequenceClassification.from_pretrained(
...     "openai-community/gpt2", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## GPT2ForTokenClassification[[transformers.GPT2ForTokenClassification]]

- **config** ([GPT2ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForTokenClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Gpt2 transformer with a token classification head on top (a linear layer on top of the hidden-states
output) e.g. for Named-Entity-Recognition (NER) tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, input_ids_length)`) --
  `input_ids_length` = `sequence_length` if `past_key_values` is `None` else
  `past_key_values.get_seq_length()` (`sequence_length` of input past key value states). Indices of input
  sequence tokens in the vocabulary.

  If `past_key_values` is used, only `input_ids` that do not have their past calculated should be passed as
  `input_ids`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the sequence classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`A [TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GPT2Config](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Config)) and inputs.
The [GPT2ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2ForTokenClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, GPT2ForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("openai-community/gpt2")
>>> model = GPT2ForTokenClassification.from_pretrained("openai-community/gpt2")

>>> inputs = tokenizer(
...     "HuggingFace is a company based in Paris and New York", add_special_tokens=False, return_tensors="pt"
... )

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_token_class_ids = logits.argmax(-1)

>>> # Note that tokens are classified rather then input words which means that
>>> # there might be more predicted token classes than words.
>>> # Multiple token classes might account for the same word
>>> predicted_tokens_classes = [model.config.id2label[t.item()] for t in predicted_token_class_ids[0]]
>>> predicted_tokens_classes
...

>>> labels = predicted_token_class_ids
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```
