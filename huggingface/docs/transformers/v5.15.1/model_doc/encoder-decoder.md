# Encoder Decoder Models

[`EncoderDecoderModel`](https://huggingface.co/papers/1706.03762) initializes a sequence-to-sequence model with any pretrained autoencoder and pretrained autoregressive model. It is effective for sequence generation tasks as demonstrated in [Text Summarization with Pretrained Encoders](https://huggingface.co/papers/1908.08345) which uses [BertModel](/docs/transformers/v5.15.1/en/model_doc/bert#transformers.BertModel) as the encoder and decoder.

> [!TIP]
> This model was contributed by [thomwolf](https://huggingface.co/thomwolf).
>
> Click on the Encoder Decoder models in the right sidebar for more examples of how to apply Encoder Decoder to different language tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("patrickvonplaten/bert2bert-cnn_dailymail-fp16")
model = AutoModelForCausalLM.from_pretrained("patrickvonplaten/bert2bert-cnn_dailymail-fp16", device_map="auto",attn_implementation="sdpa")

text = "Plants create energy through a process known as photosynthesis. This involves capturing sunlight and converting carbon dioxide and water into glucose and oxygen."

inputs = tokenizer(text, return_tensors="pt", truncation=True, padding=True).to(model.device)

summary = model.generate(**inputs, max_length=60, num_beams=4, early_stopping=True)
print(tokenizer.decode(summary[0], skip_special_tokens=True))
```

## Notes

- [EncoderDecoderModel](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderModel) can be initialized using any pretrained encoder and decoder. But depending on the decoder architecture, the cross-attention layers may be randomly initialized.

These models require downstream fine-tuning, as discussed in this [blog post](https://huggingface.co/blog/warm-starting-encoder-decoder). Use [from_encoder_decoder_pretrained()](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderModel.from_encoder_decoder_pretrained) to combine encoder and decoder checkpoints.

```python
from transformers import BertTokenizer, EncoderDecoderModel

tokenizer = BertTokenizer.from_pretrained("google-bert/bert-base-uncased")
model = EncoderDecoderModel.from_encoder_decoder_pretrained(
    "google-bert/bert-base-uncased",
    "google-bert/bert-base-uncased"
)
```

- Encoder Decoder models can be fine-tuned like BART, T5 or any other encoder-decoder model. Only 2 inputs are required to compute a loss, `input_ids` and `labels`. Refer to this [notebook](https://colab.research.google.com/drive/1WIk2bxglElfZewOHboPFNj8H44_VAyKE?usp=sharing#scrollTo=ZwQIEhKOrJpl) for a more detailed training example.

```python
from transformers import BertTokenizer, EncoderDecoderModel

tokenizer = BertTokenizer.from_pretrained("google-bert/bert-base-uncased")
model = EncoderDecoderModel.from_encoder_decoder_pretrained("google-bert/bert-base-uncased", "google-bert/bert-base-uncased")

model.config.decoder_start_token_id = tokenizer.cls_token_id
model.config.pad_token_id = tokenizer.pad_token_id

input_ids = tokenizer(
    "The tower is 324 metres (1,063 ft) tall, about the same height as an 81-storey building, and the tallest structure in Paris. Its base is square, measuring 125 metres (410 ft) on each side.During its construction, the Eiffel Tower surpassed the Washington Monument to become the tallest man-made structure in the world, a title it held for 41 years until the Chrysler Building in New York City was  finished in 1930. It was the first structure to reach a height of 300 metres. Due to the addition of a broadcasting aerial at the top of the tower in 1957, it is now taller than the Chrysler Building by 5.2 metres (17 ft).Excluding transmitters, the Eiffel Tower is the second tallest free-standing structure in France after the Millau Viaduct.",
    return_tensors="pt",
).input_ids

labels = tokenizer(
    "the eiffel tower surpassed the washington monument to become the tallest structure in the world. it was the first structure to reach a height of 300 metres in paris in 1930. it is now taller than the chrysler building by 5. 2 metres ( 17 ft ) and is the second tallest free - standing structure in paris.",
    return_tensors="pt",
).input_ids

# the forward function automatically creates the correct decoder_input_ids
loss = model(input_ids=input_ids, labels=labels).loss
```

- [EncoderDecoderModel](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderModel) can be randomly initialized from an encoder and a decoder config as shown below.

```python
from transformers import BertConfig, EncoderDecoderConfig, EncoderDecoderModel

config_encoder = BertConfig()
config_decoder = BertConfig()

config = EncoderDecoderConfig.from_encoder_decoder_configs(config_encoder, config_decoder)
model = EncoderDecoderModel(config=config)
```

- The Encoder Decoder Model can also be used for translation as shown below.

```python
from transformers import AutoTokenizer, EncoderDecoderModel

# Load a pre-trained translation model
model_name = "google/bert2bert_L-24_wmt_en_de"
tokenizer = AutoTokenizer.from_pretrained(model_name, pad_token="<pad>", eos_token="</s>", bos_token="<s>")
model = EncoderDecoderModel.from_pretrained(model_name, device_map="auto")

# Input sentence to translate
input_text = "Plants create energy through a process known as"

# Encode the input text
inputs = tokenizer(input_text, return_tensors="pt", add_special_tokens=False).to(model.device).input_ids

# Generate the translated output
outputs = model.generate(inputs)[0]

# Decode the output tokens to get the translated sentence
translated_text = tokenizer.decode(outputs, skip_special_tokens=True)

print("Translated text:", translated_text)
```

## EncoderDecoderConfig[[transformers.EncoderDecoderConfig]]

#### transformers.EncoderDecoderConfig[[transformers.EncoderDecoderConfig]]

```python
transformers.EncoderDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool | None = True, pad_token_id: int | None = None, decoder_start_token_id: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/encoder_decoder/configuration_encoder_decoder.py#L29)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

decoder_start_token_id (`int`, *optional*) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

This is the configuration class to store the configuration of a Encoder DecoderModel. It is used to instantiate a Encoder Decoder
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [](https://huggingface.co/)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import BertConfig, EncoderDecoderConfig, EncoderDecoderModel

>>> # Initializing a BERT google-bert/bert-base-uncased style configuration
>>> config_encoder = BertConfig()
>>> config_decoder = BertConfig()

>>> config = EncoderDecoderConfig.from_encoder_decoder_configs(config_encoder, config_decoder)

>>> # Initializing a Bert2Bert model (with random weights) from the google-bert/bert-base-uncased style configurations
>>> model = EncoderDecoderModel(config=config)

>>> # Accessing the model configuration
>>> config_encoder = model.config.encoder
>>> config_decoder = model.config.decoder
>>> # set decoder config to causal lm
>>> config_decoder.is_decoder = True
>>> config_decoder.add_cross_attention = True

>>> # Saving the model, including its configuration
>>> model.save_pretrained("my-model")

>>> # loading model and config from pretrained folder
>>> encoder_decoder_config = EncoderDecoderConfig.from_pretrained("my-model")
>>> model = EncoderDecoderModel.from_pretrained("my-model", config=encoder_decoder_config)
```

#### from_encoder_decoder_configs[[transformers.EncoderDecoderConfig.from_encoder_decoder_configs]]

```python
from_encoder_decoder_configs(encoder_config: PreTrainedConfig, decoder_config: PreTrainedConfig, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/encoder_decoder/configuration_encoder_decoder.py#L84)

**Returns:** [EncoderDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderConfig)

An instance of a configuration object

Instantiate a [EncoderDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderConfig) (or a derived class) from a pre-trained encoder model configuration and
decoder model configuration.

## EncoderDecoderModel[[transformers.EncoderDecoderModel]]

#### transformers.EncoderDecoderModel[[transformers.EncoderDecoderModel]]

```python
transformers.EncoderDecoderModel(config: transformers.configuration_utils.PreTrainedConfig | None = None, encoder: transformers.modeling_utils.PreTrainedModel | None = None, decoder: transformers.modeling_utils.PreTrainedModel | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/encoder_decoder/modeling_encoder_decoder.py#L65)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig), *optional*) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

encoder (`PreTrainedModel`, *optional*) : The encoder model to use.

decoder (`PreTrainedModel`, *optional*) : The decoder model to use.

The bare Encoder Decoder Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EncoderDecoderModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.BoolTensor] = None, encoder_outputs: tuple[torch.FloatTensor] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/encoder_decoder/modeling_encoder_decoder.py#L317)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)  If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).  For training, `decoder_input_ids` are automatically created by the model by shifting the `labels` to the right, replacing -100 by the `pad_token_id` and prepending them with the `decoder_start_token_id`.

decoder_attention_mask (`torch.BoolTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.

encoder_outputs (`tuple[torch.FloatTensor]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss for the decoder. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EncoderDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderConfig)) and inputs.

The [EncoderDecoderModel](/docs/transformers/v5.15.1/en/model_doc/encoder-decoder#transformers.EncoderDecoderModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

Examples:

```python
>>> from transformers import EncoderDecoderModel, BertTokenizer
>>> import torch

>>> tokenizer = BertTokenizer.from_pretrained("google-bert/bert-base-uncased")
>>> model = EncoderDecoderModel.from_encoder_decoder_pretrained(
...     "google-bert/bert-base-uncased", "google-bert/bert-base-uncased"
... )  # initialize Bert2Bert from pre-trained checkpoints

>>> # training
>>> model.config.decoder_start_token_id = tokenizer.cls_token_id
>>> model.config.pad_token_id = tokenizer.pad_token_id
>>> model.config.vocab_size = model.config.decoder.vocab_size

>>> input_ids = tokenizer("This is a really long text", return_tensors="pt").input_ids
>>> labels = tokenizer("This is the corresponding summary", return_tensors="pt").input_ids
>>> outputs = model(input_ids=input_ids, labels=labels)
>>> loss, logits = outputs.loss, outputs.logits

>>> # save and load from pretrained
>>> model.save_pretrained("bert2bert")
>>> model = EncoderDecoderModel.from_pretrained("bert2bert")

>>> # generation
>>> generated = model.generate(input_ids)
```

#### from_encoder_decoder_pretrained[[transformers.EncoderDecoderModel.from_encoder_decoder_pretrained]]

```python
from_encoder_decoder_pretrained(encoder_pretrained_model_name_or_path: str | None = None, decoder_pretrained_model_name_or_path: str | None = None, *model_args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/encoder_decoder/modeling_encoder_decoder.py#L182)

**Parameters:**

encoder_pretrained_model_name_or_path (`str`, *optional*) : Information necessary to initiate the encoder. Can be either:  - A string, the *model id* of a pretrained model hosted inside a model repo on huggingface.co. - A path to a *directory* containing model weights saved using [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `./my_model_directory/`. 

decoder_pretrained_model_name_or_path (`str`, *optional*, defaults to `None`) : Information necessary to initiate the decoder. Can be either:  - A string, the *model id* of a pretrained model hosted inside a model repo on huggingface.co. - A path to a *directory* containing model weights saved using [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `./my_model_directory/`. 

model_args (remaining positional arguments, *optional*) : All remaining positional arguments will be passed to the underlying model's `__init__` method. 

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to update the configuration object (after it being loaded) and initiate the model (e.g., `output_attentions=True`).  - To update the encoder configuration, use the prefix *encoder_* for each configuration parameter. - To update the decoder configuration, use the prefix *decoder_* for each configuration parameter. - To update the parent model configuration, do not use a prefix for each configuration parameter.  Behaves differently depending on whether a `config` is provided or automatically loaded.

Instantiate an encoder and a decoder from one or two base classes of the library from pretrained model
checkpoints.

The model is set in evaluation mode by default using `model.eval()` (Dropout modules are deactivated). To train
the model, you need to first set it back in training mode with `model.train()`.

Example:

```python
>>> from transformers import EncoderDecoderModel

>>> # initialize a bert2bert from two pretrained BERT models. Note that the cross-attention layers will be randomly initialized
>>> model = EncoderDecoderModel.from_encoder_decoder_pretrained("google-bert/bert-base-uncased", "google-bert/bert-base-uncased")
>>> # saving model after fine-tuning
>>> model.save_pretrained("./bert2bert")
>>> # load fine-tuned model
>>> model = EncoderDecoderModel.from_pretrained("./bert2bert")
```
