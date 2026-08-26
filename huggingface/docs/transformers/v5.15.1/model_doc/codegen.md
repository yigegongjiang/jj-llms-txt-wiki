# CodeGen

## Overview

The CodeGen model was proposed in [A Conversational Paradigm for Program Synthesis](https://huggingface.co/papers/2203.13474) by Erik Nijkamp, Bo Pang, Hiroaki Hayashi, Lifu Tu, Huan Wang, Yingbo Zhou, Silvio Savarese, and Caiming Xiong.

CodeGen is an autoregressive language model for program synthesis trained sequentially on [The Pile](https://pile.eleuther.ai/), BigQuery, and BigPython.

The abstract from the paper is the following:

*Program synthesis strives to generate a computer program as a solution to a given problem specification. We propose a conversational program synthesis approach via large language models, which addresses the challenges of searching over a vast program space and user intent specification faced in prior approaches. Our new approach casts the process of writing a specification and program as a multi-turn conversation between a user and a system. It treats program synthesis as a sequence prediction problem, in which the specification is expressed in natural language and the desired program is conditionally sampled. We train a family of large language models, called CodeGen, on natural language and programming language data. With weak supervision in the data and the scaling up of data size and model size, conversational capacities emerge from the simple autoregressive language modeling. To study the model behavior on conversational program synthesis, we develop a multi-turn programming benchmark (MTPB), where solving each problem requires multi-step synthesis via multi-turn conversation between the user and the model. Our findings show the emergence of conversational capabilities and the effectiveness of the proposed conversational program synthesis paradigm. In addition, our model CodeGen (with up to 16B parameters trained on TPU-v4) outperforms OpenAI's Codex on the HumanEval benchmark. We make the training library JaxFormer including checkpoints available as open source contribution: [this https URL](https://github.com/salesforce/codegen).*

This model was contributed by [Hiroaki Hayashi](https://huggingface.co/rooa).
The original code can be found [here](https://github.com/salesforce/codegen).

## Checkpoint Naming

* CodeGen model [checkpoints](https://huggingface.co/models?other=codegen) are available on different pre-training data with variable sizes.
* The format is: `Salesforce/codegen-{size}-{data}`, where
  * `size`: `350M`, `2B`, `6B`, `16B`
  * `data`:
    * `nl`: Pre-trained on the Pile
    * `multi`: Initialized with `nl`, then further pre-trained on multiple programming languages data
    * `mono`: Initialized with `multi`, then further pre-trained on Python data
* For example, `Salesforce/codegen-350M-mono` offers a 350 million-parameter checkpoint pre-trained sequentially on the Pile, multiple programming languages, and Python.

## Usage example

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

checkpoint = "Salesforce/codegen-350M-mono"
model = AutoModelForCausalLM.from_pretrained(checkpoint, device_map="auto")
tokenizer = AutoTokenizer.from_pretrained(checkpoint)

text = "def hello_world():"

completion = model.generate(**tokenizer(text, return_tensors="pt").to(model.device))

print(tokenizer.decode(completion[0]))
def hello_world():
    print("Hello World")

hello_world()
```

## Resources

- [Causal language modeling task guide](../tasks/language_modeling)

## CodeGenConfig[[transformers.CodeGenConfig]]

#### transformers.CodeGenConfig[[transformers.CodeGenConfig]]

```python
transformers.CodeGenConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 50400, n_positions: int = 2048, n_ctx: int = 2048, n_embd: int = 4096, n_layer: int = 28, n_head: int = 16, rotary_dim: int = 64, n_inner: int | None = None, activation_function: str = 'gelu_new', resid_pdrop: float | int = 0.0, embd_pdrop: float | int = 0.0, attn_pdrop: float | int = 0.0, layer_norm_epsilon: float = 1e-05, initializer_range: float = 0.02, use_cache: bool = True, bos_token_id: int | None = 50256, eos_token_id: int | list[int] | None = 50256, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/configuration_codegen.py#L24)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `50400`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

n_positions (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

n_ctx (`int`, *optional*, defaults to 2048) : This attribute is used in `CodeGenModel.__init__` without any real effect. The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

n_embd (`int`, *optional*, defaults to `4096`) : Dimensionality of the embeddings and hidden states.

n_layer (`int`, *optional*, defaults to `28`) : Number of hidden layers in the Transformer decoder.

n_head (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

rotary_dim (`int`, *optional*, defaults to 64) : Number of dimensions in the embedding that Rotary Position Embedding is applied to.

n_inner (`int`, *optional*) : Dimensionality of the inner feed-forward layers. `None` will set it to 4 times n_embd

activation_function (`str`, *optional*, defaults to `gelu_new`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

resid_pdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

embd_pdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the embeddings.

attn_pdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

layer_norm_epsilon (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

bos_token_id (`int`, *optional*, defaults to `50256`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `50256`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a CodeGenModel. It is used to instantiate a Codegen
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Salesforce/codegen-2B-mono](https://huggingface.co/Salesforce/codegen-2B-mono)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import CodeGenConfig, CodeGenModel

>>> # Initializing a CodeGen 6B configuration
>>> configuration = CodeGenConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = CodeGenModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CodeGenTokenizer[[transformers.CodeGenTokenizer]]

#### transformers.CodeGenTokenizer[[transformers.CodeGenTokenizer]]

```python
transformers.CodeGenTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, unk_token: str = '<|endoftext|>', bos_token: str = '<|endoftext|>', eos_token: str = '<|endoftext|>', pad_token = None, add_prefix_space: bool = False, return_token_type_ids: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/tokenization_codegen.py#L37)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

unk_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

bos_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The beginning of sequence token.

eos_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The end of sequence token.

pad_token (`str`, *optional*) : The token used for padding, for example when batching sequences of different lengths.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not to add an initial space to the input. This allows to treat the leading word just as any other word. (CodeGen tokenizer detect beginning of words by the preceding space).

return_token_type_ids (`bool`, *optional*, defaults to `False`) : Whether to return token type IDs.

Construct a CodeGen tokenizer (backed by HuggingFace's *tokenizers* library). Based on byte-level
Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import CodeGenTokenizer

>>> tokenizer = CodeGenTokenizer.from_pretrained("Salesforce/codegen-350M-mono")
>>> tokenizer("Hello world")["input_ids"]
[15496, 995]

>>> tokenizer(" Hello world")["input_ids"]
[18435, 995]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer, but since
the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer needs to be instantiated with `add_prefix_space=True`.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### save_vocabulary[[transformers.CodeGenTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_tokenizers.py#L509)

## CodeGenTokenizerFast[[transformers.CodeGenTokenizer]]

#### transformers.CodeGenTokenizer[[transformers.CodeGenTokenizer]]

```python
transformers.CodeGenTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, unk_token: str = '<|endoftext|>', bos_token: str = '<|endoftext|>', eos_token: str = '<|endoftext|>', pad_token = None, add_prefix_space: bool = False, return_token_type_ids: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/tokenization_codegen.py#L37)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

unk_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

bos_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The beginning of sequence token.

eos_token (`str`, *optional*, defaults to `"<|endoftext|>"`) : The end of sequence token.

pad_token (`str`, *optional*) : The token used for padding, for example when batching sequences of different lengths.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not to add an initial space to the input. This allows to treat the leading word just as any other word. (CodeGen tokenizer detect beginning of words by the preceding space).

return_token_type_ids (`bool`, *optional*, defaults to `False`) : Whether to return token type IDs.

Construct a CodeGen tokenizer (backed by HuggingFace's *tokenizers* library). Based on byte-level
Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import CodeGenTokenizer

>>> tokenizer = CodeGenTokenizer.from_pretrained("Salesforce/codegen-350M-mono")
>>> tokenizer("Hello world")["input_ids"]
[15496, 995]

>>> tokenizer(" Hello world")["input_ids"]
[18435, 995]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer, but since
the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer needs to be instantiated with `add_prefix_space=True`.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### decode[[transformers.CodeGenTokenizer.decode]]

```python
decode(token_ids: typing.Union[int, list[int], numpy.ndarray, ForwardRef('torch.Tensor')], skip_special_tokens: bool = False, clean_up_tokenization_spaces: bool | None = None, truncate_before_pattern: list[str] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/tokenization_codegen.py#L141)

**Parameters:**

token_ids (`Union[int, List[int], np.ndarray, torch.Tensor]`) : List of tokenized input ids. Can be obtained using the `__call__` method.

skip_special_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to remove special tokens in the decoding.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether or not to clean up the tokenization spaces. If `None`, will default to `self.clean_up_tokenization_spaces` (available in the `tokenizer_config`).

truncate_before_pattern (`List[str]`, *optional*, defaults to `None`) : A list of regular expression strings that will be used to truncate the returned string. This can be used to remove extra pieces of code (e.g. truncate if observing a comment symbol "#" at the beginning of a new line). An example pattern could be `["^#", re.escape("<|endoftext|>"), "^'''", "

**Returns:** `str`

The decoded sentence.

Converts a sequence of ids in a string, using the tokenizer and vocabulary with options to remove special
tokens and clean up tokenization spaces.

Similar to doing `self.convert_tokens_to_string(self.convert_ids_to_tokens(token_ids))`.

"]`.
kwargs (additional keyword arguments, *optional*):
Will be passed to the underlying model specific decode method.

## CodeGenModel[[transformers.CodeGenModel]]

#### transformers.CodeGenModel[[transformers.CodeGenModel]]

```python
transformers.CodeGenModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/modeling_codegen.py#L276)

**Parameters:**

config ([CodeGenModel](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Codegen Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CodeGenModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/modeling_codegen.py#L299)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_dim)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert *input_ids* indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CodeGenConfig](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenConfig)) and inputs.

The [CodeGenModel](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## CodeGenForCausalLM[[transformers.CodeGenForCausalLM]]

#### transformers.CodeGenForCausalLM[[transformers.CodeGenForCausalLM]]

```python
transformers.CodeGenForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/modeling_codegen.py#L411)

**Parameters:**

config ([CodeGenForCausalLM](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The CodeGen Model transformer with a language modeling head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CodeGenForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/codegen/modeling_codegen.py#L422)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_dim)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert *input_ids* indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CodeGenConfig](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenConfig)) and inputs.

The [CodeGenForCausalLM](/docs/transformers/v5.15.1/en/model_doc/codegen#transformers.CodeGenForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
