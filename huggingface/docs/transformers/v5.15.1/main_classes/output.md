# Model outputs

All models have outputs that are instances of subclasses of [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput). Those are
data structures containing all the information returned by the model, but that can also be used as tuples or
dictionaries.

Let's see how this looks in an example:

```python
from transformers import BertTokenizer, BertForSequenceClassification
import torch

tokenizer = BertTokenizer.from_pretrained("google-bert/bert-base-uncased")
model = BertForSequenceClassification.from_pretrained("google-bert/bert-base-uncased")

inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")
labels = torch.tensor([1]).unsqueeze(0)  # Batch size 1
outputs = model(**inputs, labels=labels)
```

The `outputs` object is a [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput), as we can see in the
documentation of that class below, it means it has an optional `loss`, a `logits`, an optional `hidden_states` and
an optional `attentions` attribute. Here we have the `loss` since we passed along `labels`, but we don't have
`hidden_states` and `attentions` because we didn't pass `output_hidden_states=True` or
`output_attentions=True`.

When passing `output_hidden_states=True` you may expect the `outputs.hidden_states[-1]` to match `outputs.last_hidden_state` exactly.
However, this is not always the case. Some models apply normalization or subsequent process to the last hidden state when it's returned.

You can access each attribute as you would usually do, and if that attribute has not been returned by the model, you
will get `None`. Here for instance `outputs.loss` is the loss computed by the model, and `outputs.attentions` is
`None`.

When considering our `outputs` object as tuple, it only considers the attributes that don't have `None` values.
Here for instance, it has two elements, `loss` then `logits`, so

```python
outputs[:2]
```

will return the tuple `(outputs.loss, outputs.logits)` for instance.

When considering our `outputs` object as dictionary, it only considers the attributes that don't have `None`
values. Here for instance, it has two keys that are `loss` and `logits`.

We document here the generic model outputs that are used by more than one model type. Specific output types are
documented on their corresponding model page.

## ModelOutput[[transformers.utils.ModelOutput]]

#### transformers.utils.ModelOutput[[transformers.utils.ModelOutput]]

```python
transformers.utils.ModelOutput(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L415)

Base class for all model outputs as dataclass. Has a `__getitem__` that allows indexing by integer or slice (like a
tuple) or strings (like a dictionary) that will ignore the `None` attributes. Otherwise behaves like a regular
python dictionary.

You can't unpack a `ModelOutput` directly. Use the [to_tuple()](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput.to_tuple) method to convert it to a tuple
before.

#### to_tuple[[transformers.utils.ModelOutput.to_tuple]]

```python
to_tuple()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/generic.py#L548)

Convert self to a tuple containing all the attributes/keys that are not `None`.

## BaseModelOutput[[transformers.modeling_outputs.BaseModelOutput]]

#### transformers.modeling_outputs.BaseModelOutput[[transformers.modeling_outputs.BaseModelOutput]]

```python
transformers.modeling_outputs.BaseModelOutput(last_hidden_state: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L24)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for model's outputs, with potential hidden states and attentions.

## BaseModelOutputWithPooling[[transformers.modeling_outputs.BaseModelOutputWithPooling]]

#### transformers.modeling_outputs.BaseModelOutputWithPooling[[transformers.modeling_outputs.BaseModelOutputWithPooling]]

```python
transformers.modeling_outputs.BaseModelOutputWithPooling(last_hidden_state: typing.Optional[torch.FloatTensor] = None, pooler_output: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L69)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.

pooler_output (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) : Last layer hidden-state of the first token of the sequence (classification token) after further processing through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns the classification token after processing through a linear layer and a tanh activation function. The linear layer weights are trained from the next sentence prediction (classification) objective during pretraining.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for model's outputs that also contains a pooling of the last hidden states.

## BaseModelOutputWithCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithCrossAttentions]]

#### transformers.modeling_outputs.BaseModelOutputWithCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithCrossAttentions]]

```python
transformers.modeling_outputs.BaseModelOutputWithCrossAttentions(last_hidden_state: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L159)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

Base class for model's outputs, with potential hidden states and attentions.

## BaseModelOutputWithPoolingAndCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions]]

#### transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions]]

```python
transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions(last_hidden_state: typing.Optional[torch.FloatTensor] = None, pooler_output: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L192)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.

pooler_output (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) : Last layer hidden-state of the first token of the sequence (classification token) after further processing through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns the classification token after processing through a linear layer and a tanh activation function. The linear layer weights are trained from the next sentence prediction (classification) objective during pretraining.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

Base class for model's outputs that also contains a pooling of the last hidden states.

## BaseModelOutputWithPast[[transformers.modeling_outputs.BaseModelOutputWithPast]]

#### transformers.modeling_outputs.BaseModelOutputWithPast[[transformers.modeling_outputs.BaseModelOutputWithPast]]

```python
transformers.modeling_outputs.BaseModelOutputWithPast(last_hidden_state: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L123)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1, hidden_size)` is output.

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for model's outputs that may also contain a past key/values (to speed up sequential decoding).

## BaseModelOutputWithPastAndCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions]]

#### transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions[[transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions]]

```python
transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions(last_hidden_state: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L238)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1, hidden_size)` is output.

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

Base class for model's outputs that may also contain a past key/values (to speed up sequential decoding).

## Seq2SeqModelOutput[[transformers.modeling_outputs.Seq2SeqModelOutput]]

#### transformers.modeling_outputs.Seq2SeqModelOutput[[transformers.modeling_outputs.Seq2SeqModelOutput]]

```python
transformers.modeling_outputs.Seq2SeqModelOutput(last_hidden_state: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L452)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the decoder of the model.  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1, hidden_size)` is output.

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for model encoder's outputs that also contains : pre-computed hidden states that can speed up sequential
decoding.

## CausalLMOutput[[transformers.modeling_outputs.CausalLMOutput]]

#### transformers.modeling_outputs.CausalLMOutput[[transformers.modeling_outputs.CausalLMOutput]]

```python
transformers.modeling_outputs.CausalLMOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L581)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Language modeling loss (for next-token prediction).

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for causal language model (or autoregressive) outputs.

## CausalLMOutputWithCrossAttentions[[transformers.modeling_outputs.CausalLMOutputWithCrossAttentions]]

#### transformers.modeling_outputs.CausalLMOutputWithCrossAttentions[[transformers.modeling_outputs.CausalLMOutputWithCrossAttentions]]

```python
transformers.modeling_outputs.CausalLMOutputWithCrossAttentions(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L645)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Language modeling loss (for next-token prediction).

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Cross attentions weights after the attention softmax, used to compute the weighted average in the cross-attention heads.

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

Base class for causal language model (or autoregressive) outputs.

## CausalLMOutputWithPast[[transformers.modeling_outputs.CausalLMOutputWithPast]]

#### transformers.modeling_outputs.CausalLMOutputWithPast[[transformers.modeling_outputs.CausalLMOutputWithPast]]

```python
transformers.modeling_outputs.CausalLMOutputWithPast(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L610)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Language modeling loss (for next-token prediction).

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for causal language model (or autoregressive) outputs.

## MaskedLMOutput[[transformers.modeling_outputs.MaskedLMOutput]]

#### transformers.modeling_outputs.MaskedLMOutput[[transformers.modeling_outputs.MaskedLMOutput]]

```python
transformers.modeling_outputs.MaskedLMOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L722)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Masked language modeling (MLM) loss.

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for masked language models outputs.

## Seq2SeqLMOutput[[transformers.modeling_outputs.Seq2SeqLMOutput]]

#### transformers.modeling_outputs.Seq2SeqLMOutput[[transformers.modeling_outputs.Seq2SeqLMOutput]]

```python
transformers.modeling_outputs.Seq2SeqLMOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L751)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Language modeling loss.

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for sequence-to-sequence language models outputs.

## NextSentencePredictorOutput[[transformers.modeling_outputs.NextSentencePredictorOutput]]

#### transformers.modeling_outputs.NextSentencePredictorOutput[[transformers.modeling_outputs.NextSentencePredictorOutput]]

```python
transformers.modeling_outputs.NextSentencePredictorOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L882)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `next_sentence_label` is provided) : Next sequence prediction (classification) loss.

logits (`torch.FloatTensor` of shape `(batch_size, 2)`) : Prediction scores of the next sequence prediction (classification) head (scores of True/False continuation before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of models predicting if two sentences are consecutive or not.

## SequenceClassifierOutput[[transformers.modeling_outputs.SequenceClassifierOutput]]

#### transformers.modeling_outputs.SequenceClassifierOutput[[transformers.modeling_outputs.SequenceClassifierOutput]]

```python
transformers.modeling_outputs.SequenceClassifierOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L912)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification (or regression if config.num_labels==1) loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) : Classification (or regression if config.num_labels==1) scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of sentence classification models.

## Seq2SeqSequenceClassifierOutput[[transformers.modeling_outputs.Seq2SeqSequenceClassifierOutput]]

#### transformers.modeling_outputs.Seq2SeqSequenceClassifierOutput[[transformers.modeling_outputs.Seq2SeqSequenceClassifierOutput]]

```python
transformers.modeling_outputs.Seq2SeqSequenceClassifierOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L941)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `label` is provided) : Classification (or regression if config.num_labels==1) loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) : Classification (or regression if config.num_labels==1) scores (before SoftMax).

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of sequence-to-sequence sentence classification models.

## MultipleChoiceModelOutput[[transformers.modeling_outputs.MultipleChoiceModelOutput]]

#### transformers.modeling_outputs.MultipleChoiceModelOutput[[transformers.modeling_outputs.MultipleChoiceModelOutput]]

```python
transformers.modeling_outputs.MultipleChoiceModelOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L999)

**Parameters:**

loss (`torch.FloatTensor` of shape *(1,)*, *optional*, returned when `labels` is provided) : Classification loss.

logits (`torch.FloatTensor` of shape `(batch_size, num_choices)`) : *num_choices* is the second dimension of the input tensors. (see *input_ids* above).  Classification scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of multiple choice models.

## TokenClassifierOutput[[transformers.modeling_outputs.TokenClassifierOutput]]

#### transformers.modeling_outputs.TokenClassifierOutput[[transformers.modeling_outputs.TokenClassifierOutput]]

```python
transformers.modeling_outputs.TokenClassifierOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1030)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification loss.

logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) : Classification scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of token classification models.

## QuestionAnsweringModelOutput[[transformers.modeling_outputs.QuestionAnsweringModelOutput]]

#### transformers.modeling_outputs.QuestionAnsweringModelOutput[[transformers.modeling_outputs.QuestionAnsweringModelOutput]]

```python
transformers.modeling_outputs.QuestionAnsweringModelOutput(loss: typing.Optional[torch.FloatTensor] = None, start_logits: typing.Optional[torch.FloatTensor] = None, end_logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1059)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.

start_logits (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Span-start scores (before SoftMax).

end_logits (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Span-end scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of question answering models.

## Seq2SeqQuestionAnsweringModelOutput[[transformers.modeling_outputs.Seq2SeqQuestionAnsweringModelOutput]]

#### transformers.modeling_outputs.Seq2SeqQuestionAnsweringModelOutput[[transformers.modeling_outputs.Seq2SeqQuestionAnsweringModelOutput]]

```python
transformers.modeling_outputs.Seq2SeqQuestionAnsweringModelOutput(loss: typing.Optional[torch.FloatTensor] = None, start_logits: typing.Optional[torch.FloatTensor] = None, end_logits: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1091)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.

start_logits (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Span-start scores (before SoftMax).

end_logits (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Span-end scores (before SoftMax).

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of sequence-to-sequence question answering models.

## Seq2SeqSpectrogramOutput[[transformers.modeling_outputs.Seq2SeqSpectrogramOutput]]

#### transformers.modeling_outputs.Seq2SeqSpectrogramOutput[[transformers.modeling_outputs.Seq2SeqSpectrogramOutput]]

```python
transformers.modeling_outputs.Seq2SeqSpectrogramOutput(loss: typing.Optional[torch.FloatTensor] = None, spectrogram: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1422)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Spectrogram generation loss.

spectrogram (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_bins)`) : The predicted spectrogram.

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for sequence-to-sequence spectrogram outputs.

## SemanticSegmenterOutput[[transformers.modeling_outputs.SemanticSegmenterOutput]]

#### transformers.modeling_outputs.SemanticSegmenterOutput[[transformers.modeling_outputs.SemanticSegmenterOutput]]

```python
transformers.modeling_outputs.SemanticSegmenterOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1152)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification (or regression if config.num_labels==1) loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.num_labels, logits_height, logits_width)`) : Classification scores for each pixel.    The logits returned do not necessarily have the same size as the `pixel_values` passed as inputs. This is to avoid doing two interpolations and lose some quality when a user needs to resize the logits to the original image size as post-processing. You should always check your logits shape and resize as needed.   

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, patch_size, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of semantic segmentation models.

## ImageClassifierOutput[[transformers.modeling_outputs.ImageClassifierOutput]]

#### transformers.modeling_outputs.ImageClassifierOutput[[transformers.modeling_outputs.ImageClassifierOutput]]

```python
transformers.modeling_outputs.ImageClassifierOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1190)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification (or regression if config.num_labels==1) loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) : Classification (or regression if config.num_labels==1) scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states (also called feature maps) of the model at the output of each stage.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of image classification models.

## ImageClassifierOutputWithNoAttention[[transformers.modeling_outputs.ImageClassifierOutputWithNoAttention]]

#### transformers.modeling_outputs.ImageClassifierOutputWithNoAttention[[transformers.modeling_outputs.ImageClassifierOutputWithNoAttention]]

```python
transformers.modeling_outputs.ImageClassifierOutputWithNoAttention(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1218)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification (or regression if config.num_labels==1) loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) : Classification (or regression if config.num_labels==1) scores (before SoftMax).

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each stage) of shape `(batch_size, num_channels, height, width)`. Hidden-states (also called feature maps) of the model at the output of each stage.

Base class for outputs of image classification models.

## DepthEstimatorOutput[[transformers.modeling_outputs.DepthEstimatorOutput]]

#### transformers.modeling_outputs.DepthEstimatorOutput[[transformers.modeling_outputs.DepthEstimatorOutput]]

```python
transformers.modeling_outputs.DepthEstimatorOutput(loss: typing.Optional[torch.FloatTensor] = None, predicted_depth: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1239)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification (or regression if config.num_labels==1) loss.

predicted_depth (`torch.FloatTensor` of shape `(batch_size, height, width)`) : Predicted depth for each pixel. 

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for outputs of depth estimation models.

## Wav2Vec2BaseModelOutput[[transformers.modeling_outputs.Wav2Vec2BaseModelOutput]]

#### transformers.modeling_outputs.Wav2Vec2BaseModelOutput[[transformers.modeling_outputs.Wav2Vec2BaseModelOutput]]

```python
transformers.modeling_outputs.Wav2Vec2BaseModelOutput(last_hidden_state: typing.Optional[torch.FloatTensor] = None, extract_features: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1297)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the model.

extract_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, conv_dim[-1])`) : Sequence of extracted feature vectors of the last convolutional layer of the model.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Base class for models that have been trained with the Wav2Vec2 loss objective.

## XVectorOutput[[transformers.modeling_outputs.XVectorOutput]]

#### transformers.modeling_outputs.XVectorOutput[[transformers.modeling_outputs.XVectorOutput]]

```python
transformers.modeling_outputs.XVectorOutput(loss: typing.Optional[torch.FloatTensor] = None, logits: typing.Optional[torch.FloatTensor] = None, embeddings: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor, ...] | None = None, attentions: tuple[torch.FloatTensor, ...] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1326)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) : Classification loss.

logits (`torch.FloatTensor` of shape `(batch_size, config.xvector_output_dim)`) : Classification hidden states before AMSoftmax.

embeddings (`torch.FloatTensor` of shape `(batch_size, config.xvector_output_dim)`) : Utterance embeddings used for vector similarity-based retrieval.

hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the initial embedding outputs.

attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Output type of [Wav2Vec2ForXVector](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2ForXVector).

## Seq2SeqTSModelOutput[[transformers.modeling_outputs.Seq2SeqTSModelOutput]]

#### transformers.modeling_outputs.Seq2SeqTSModelOutput[[transformers.modeling_outputs.Seq2SeqTSModelOutput]]

```python
transformers.modeling_outputs.Seq2SeqTSModelOutput(last_hidden_state: typing.Optional[torch.FloatTensor] = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None, loc: typing.Optional[torch.FloatTensor] = None, scale: typing.Optional[torch.FloatTensor] = None, static_features: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1480)

**Parameters:**

last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) : Sequence of hidden-states at the output of the last layer of the decoder of the model.  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1, hidden_size)` is output.

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

loc (`torch.FloatTensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) : Shift values of each time series' context window which is used to give the model inputs of the same magnitude and then used to shift back to the original magnitude.

scale (`torch.FloatTensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) : Scaling values of each time series' context window which is used to give the model inputs of the same magnitude and then used to rescale back to the original magnitude.

static_features (`torch.FloatTensor` of shape `(batch_size, feature size)`, *optional*) : Static features of each time series' in a batch which are copied to the covariates at inference time.

Base class for time series model's encoder outputs that also contains pre-computed hidden states that can speed up
sequential decoding.

## Seq2SeqTSPredictionOutput[[transformers.modeling_outputs.Seq2SeqTSPredictionOutput]]

#### transformers.modeling_outputs.Seq2SeqTSPredictionOutput[[transformers.modeling_outputs.Seq2SeqTSPredictionOutput]]

```python
transformers.modeling_outputs.Seq2SeqTSPredictionOutput(loss: typing.Optional[torch.FloatTensor] = None, params: tuple[torch.FloatTensor, ...] | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, decoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, decoder_attentions: tuple[torch.FloatTensor, ...] | None = None, cross_attentions: tuple[torch.FloatTensor, ...] | None = None, encoder_last_hidden_state: typing.Optional[torch.FloatTensor] = None, encoder_hidden_states: tuple[torch.FloatTensor, ...] | None = None, encoder_attentions: tuple[torch.FloatTensor, ...] | None = None, loc: typing.Optional[torch.FloatTensor] = None, scale: typing.Optional[torch.FloatTensor] = None, static_features: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1550)

**Parameters:**

loss (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when a `future_values` is provided) : Distributional loss.

params (`torch.FloatTensor` of shape `(batch_size, num_samples, num_params)`) : Parameters of the chosen distribution.

past_key_values (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

decoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.

decoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

cross_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the weighted average in the cross-attention heads.

encoder_last_hidden_state (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder of the model.

encoder_hidden_states (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.

encoder_attentions (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the self-attention heads.

loc (`torch.FloatTensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) : Shift values of each time series' context window which is used to give the model inputs of the same magnitude and then used to shift back to the original magnitude.

scale (`torch.FloatTensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) : Scaling values of each time series' context window which is used to give the model inputs of the same magnitude and then used to rescale back to the original magnitude.

static_features (`torch.FloatTensor` of shape `(batch_size, feature size)`, *optional*) : Static features of each time series' in a batch which are copied to the covariates at inference time.

Base class for time series model's decoder outputs that also contain the loss as well as the parameters of the
chosen distribution.

## SampleTSPredictionOutput[[transformers.modeling_outputs.SampleTSPredictionOutput]]

#### transformers.modeling_outputs.SampleTSPredictionOutput[[transformers.modeling_outputs.SampleTSPredictionOutput]]

```python
transformers.modeling_outputs.SampleTSPredictionOutput(sequences: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_outputs.py#L1620)

**Parameters:**

sequences (`torch.FloatTensor` of shape `(batch_size, num_samples, prediction_length)` or `(batch_size, num_samples, prediction_length, input_size)`) : Sampled values from the chosen distribution.

Base class for time series model's predictions outputs that contains the sampled values from the chosen
distribution.
