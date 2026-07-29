# Blenderbot

## Overview

The Blender chatbot model was proposed in [Recipes for building an open-domain chatbot](https://huggingface.co/papers/2004.13637) Stephen Roller, Emily Dinan, Naman Goyal, Da Ju, Mary Williamson, Yinhan Liu,
Jing Xu, Myle Ott, Kurt Shuster, Eric M. Smith, Y-Lan Boureau, Jason Weston on 30 Apr 2020.

The abstract of the paper is the following:

*Building open-domain chatbots is a challenging area for machine learning research. While prior work has shown that
scaling neural models in the number of parameters and the size of the data they are trained on gives improved results,
we show that other ingredients are important for a high-performing chatbot. Good conversation requires a number of
skills that an expert conversationalist blends in a seamless way: providing engaging talking points and listening to
their partners, and displaying knowledge, empathy and personality appropriately, while maintaining a consistent
persona. We show that large scale models can learn these skills when given appropriate training data and choice of
generation strategy. We build variants of these recipes with 90M, 2.7B and 9.4B parameter models, and make our models
and code publicly available. Human evaluations show our best models are superior to existing approaches in multi-turn
dialogue in terms of engagingness and humanness measurements. We then discuss the limitations of this work by analyzing
failure cases of our models.*

This model was contributed by [sshleifer](https://huggingface.co/sshleifer). The authors' code can be found [here](https://github.com/facebookresearch/ParlAI) .

## Usage tips and example

Blenderbot is a model with absolute position embeddings so it's usually advised to pad the inputs on the right
rather than the left.

An example:

```python
from transformers import BlenderbotForConditionalGeneration, BlenderbotTokenizer

mname = "facebook/blenderbot-400M-distill"
model = BlenderbotForConditionalGeneration.from_pretrained(mname, device_map="auto")
tokenizer = BlenderbotTokenizer.from_pretrained(mname)
UTTERANCE = "My friends are cool but they eat too many carbs."
inputs = tokenizer([UTTERANCE], return_tensors="pt").to(model.device)
reply_ids = model.generate(**inputs)
print(tokenizer.batch_decode(reply_ids))
["<s> That's unfortunate. Are they trying to lose weight or are they just trying to be healthier?</s>"]
```

## Implementation Notes

- Blenderbot uses a standard [seq2seq model transformer](https://huggingface.co/papers/1706.03762) based architecture.
- Available checkpoints can be found in the [model hub](https://huggingface.co/models?search=blenderbot).
- This is the *default* Blenderbot model class. However, some smaller checkpoints, such as
  `facebook/blenderbot_small_90M`, have a different architecture and consequently should be used with
  [BlenderbotSmall](blenderbot-small).

## Resources

- [Causal language modeling task guide](../tasks/language_modeling)
- [Translation task guide](../tasks/translation)
- [Summarization task guide](../tasks/summarization)

## BlenderbotConfig[[transformers.BlenderbotConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **vocab_size** (`int`, *optional*, defaults to `8008`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `128`) --
  The maximum sequence length that this model might ever be used with.
- **encoder_layers** (`int`, *optional*, defaults to `2`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `10240`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **encoder_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **decoder_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `10240`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **encoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
  for more details.
- **decoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
  for more details.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **d_model** (`int`, *optional*, defaults to `2560`) --
  Size of the encoder layers and the pooler layer.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **decoder_start_token_id** (`int`, *optional*, defaults to `1`) --
  If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.
- **scale_embedding** (`bool`, *optional*, defaults to `False`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **encoder_no_repeat_ngram_size** (`int`, *optional*, defaults to 3) --
  Number of ngrams to not be repeated in the encoder.
- **forced_eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  The id of the token to force as the last generated token when `max_length` is reached. Usually set to
  `eos_token_id`.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a BlenderbotModel. It is used to instantiate a Blenderbot
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/blenderbot-3B](https://huggingface.co/facebook/blenderbot-3B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import BlenderbotConfig, BlenderbotModel

>>> # Initializing a Blenderbot facebook/blenderbot-3B style configuration
>>> configuration = BlenderbotConfig()

>>> # Initializing a model (with random weights) from the facebook/blenderbot-3B style configuration
>>> model = BlenderbotModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## BlenderbotTokenizer[[transformers.BlenderbotTokenizer]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "add_prefix_space", "val": " = True"}, {"name": "vocab", "val": " = None"}, {"name": "merges", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.

  

  When building a sequence using special tokens, this is not the token that is used for the beginning of
  sequence. The token used is the `cls_token`.

  

- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.

  

  When building a sequence using special tokens, this is not the token that is used for the end of sequence.
  The token used is the `sep_token`.

  

- **sep_token** (`str`, *optional*, defaults to `"</s>"`) --
  The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for
  sequence classification or for a text and a question for question answering. It is also used as the last
  token of a sequence built with special tokens.
- **cls_token** (`str`, *optional*, defaults to `"<s>"`) --
  The classifier token which is used when doing sequence classification (classification of the whole sequence
  instead of per-token classification). It is the first token of the sequence when built with special tokens.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **mask_token** (`str`, *optional*, defaults to `"<mask>"`) --
  The token used for masking values. This is the token used when training this model with masked language
  modeling. This is the token which the model will try to predict.
- **add_prefix_space** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word. (Blenderbot tokenizer detect beginning of words by the preceding space).
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file`.

Construct a "fast" Blenderbot tokenizer (backed by HuggingFace's *tokenizers* library), derived from the GPT-2
tokenizer, using byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import BlenderbotTokenizerFast

>>> tokenizer = BlenderbotTokenizerFast.from_pretrained("facebook/blenderbot-3B")
>>> tokenizer("Hello world")["input_ids"]
[6950, 1085, 2]

>>> tokenizer(" Hello world")["input_ids"]
[6950, 1085, 2]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer needs to be instantiated with `add_prefix_space=True`.

This tokenizer inherits from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

## BlenderbotTokenizerFast[[transformers.BlenderbotTokenizer]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "add_prefix_space", "val": " = True"}, {"name": "vocab", "val": " = None"}, {"name": "merges", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.

  

  When building a sequence using special tokens, this is not the token that is used for the beginning of
  sequence. The token used is the `cls_token`.

  

- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.

  

  When building a sequence using special tokens, this is not the token that is used for the end of sequence.
  The token used is the `sep_token`.

  

- **sep_token** (`str`, *optional*, defaults to `"</s>"`) --
  The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for
  sequence classification or for a text and a question for question answering. It is also used as the last
  token of a sequence built with special tokens.
- **cls_token** (`str`, *optional*, defaults to `"<s>"`) --
  The classifier token which is used when doing sequence classification (classification of the whole sequence
  instead of per-token classification). It is the first token of the sequence when built with special tokens.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **mask_token** (`str`, *optional*, defaults to `"<mask>"`) --
  The token used for masking values. This is the token used when training this model with masked language
  modeling. This is the token which the model will try to predict.
- **add_prefix_space** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word. (Blenderbot tokenizer detect beginning of words by the preceding space).
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file`.

Construct a "fast" Blenderbot tokenizer (backed by HuggingFace's *tokenizers* library), derived from the GPT-2
tokenizer, using byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import BlenderbotTokenizerFast

>>> tokenizer = BlenderbotTokenizerFast.from_pretrained("facebook/blenderbot-3B")
>>> tokenizer("Hello world")["input_ids"]
[6950, 1085, 2]

>>> tokenizer(" Hello world")["input_ids"]
[6950, 1085, 2]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer needs to be instantiated with `add_prefix_space=True`.

This tokenizer inherits from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

## BlenderbotModel[[transformers.BlenderbotModel]]

See [BartModel](/docs/transformers/v5.14.0/en/model_doc/bart#transformers.BartModel) for arguments to *forward* and *generate*

- **config** ([BlenderbotConfig](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Blenderbot Model outputting raw hidden-states without any specific head on top.

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
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)

  Blenderbot uses the `bos_token_id` as the starting token for `decoder_input_ids` generation. If
  `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see
  `past_key_values`).
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also
  be used by default.
- **encoder_outputs** (`~modeling_outputs.BaseModelOutput`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
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
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[Seq2SeqModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`A [Seq2SeqModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([BlenderbotConfig](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotConfig)) and inputs.
The [BlenderbotModel](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Example:

```python
>>> from transformers import AutoTokenizer, BlenderbotModel

>>> model = BlenderbotModel.from_pretrained("facebook/blenderbot-400M-distill")
>>> tokenizer = AutoTokenizer.from_pretrained("facebook/blenderbot-400M-distill")

>>> inputs = tokenizer("Studies have been shown that owning a dog is good for you", return_tensors="pt")
>>> decoder_input_ids = tokenizer("Studies show that", return_tensors="pt").input_ids  # Batch size 1
>>> outputs = model(input_ids=inputs.input_ids, decoder_input_ids=decoder_input_ids)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 6, 1280]
```

## BlenderbotForConditionalGeneration[[transformers.BlenderbotForConditionalGeneration]]

See [BartForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/bart#transformers.BartForConditionalGeneration) for arguments to *forward* and *generate*

- **config** ([BlenderbotConfig](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Blenderbot Model with a language modeling head. Can be used for summarization.

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
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)

  Blenderbot uses the `bos_token_id` as the starting token for `decoder_input_ids` generation. If
  `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see
  `past_key_values`).
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also
  be used by default.
- **encoder_outputs** (`~modeling_outputs.BaseModelOutput`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
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
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[Seq2SeqLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`A [Seq2SeqLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([BlenderbotConfig](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotConfig)) and inputs.
The [BlenderbotForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Example conversation:

```python
>>> from transformers import AutoTokenizer, BlenderbotForConditionalGeneration

>>> mname = "facebook/blenderbot-400M-distill"
>>> model = BlenderbotForConditionalGeneration.from_pretrained(mname)
>>> tokenizer = AutoTokenizer.from_pretrained(mname)
>>> UTTERANCE = "My friends are cool but they eat too many carbs."
>>> print("Human: ", UTTERANCE)
Human:  My friends are cool but they eat too many carbs.

>>> inputs = tokenizer([UTTERANCE], return_tensors="pt")
>>> reply_ids = model.generate(**inputs)
>>> print("Bot: ", tokenizer.batch_decode(reply_ids, skip_special_tokens=True)[0])
Bot: That's unfortunate. Are they trying to lose weight or are they just trying to be healthier?

>>> REPLY = "I'm not sure"
>>> print("Human: ", REPLY)
Human: I'm not sure

>>> NEXT_UTTERANCE = (
...     "My friends are cool but they eat too many carbs.</s> <s>That's unfortunate. "
...     "Are they trying to lose weight or are they just trying to be healthier?</s> "
...     "<s> I'm not sure."
... )
>>> inputs = tokenizer([NEXT_UTTERANCE], return_tensors="pt")
>>> next_reply_ids = model.generate(**inputs)
>>> print("Bot: ", tokenizer.batch_decode(next_reply_ids, skip_special_tokens=True)[0])
Bot:   I see. Well, it's good that they're trying to change their eating habits.
```

## BlenderbotForCausalLM[[transformers.BlenderbotForCausalLM]]

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
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([BlenderbotConfig](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotConfig)) and inputs.
The [BlenderbotForCausalLM](/docs/transformers/v5.14.0/en/model_doc/blenderbot#transformers.BlenderbotForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, BlenderbotForCausalLM

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/blenderbot-400M-distill")
>>> model = BlenderbotForCausalLM.from_pretrained("facebook/blenderbot-400M-distill")
>>> assert model.config.is_decoder, f"{model.__class__} has to be configured as a decoder."
>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")
>>> outputs = model(**inputs)

>>> logits = outputs.logits
>>> expected_shape = [1, inputs.input_ids.shape[-1], model.config.vocab_size]
>>> list(logits.shape) == expected_shape
True
```
