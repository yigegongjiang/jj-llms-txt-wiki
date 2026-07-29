# ModernBERT

[ModernBERT](https://huggingface.co/papers/2412.13663) is a modernized version of `BERT` trained on 2T tokens. It brings many improvements to the original architecture such as rotary positional embeddings to support sequences of up to 8192 tokens, unpadding to avoid wasting compute on padding tokens, GeGLU layers, and alternating attention.

You can find all the original ModernBERT checkpoints under the [ModernBERT](https://huggingface.co/collections/answerdotai/modernbert-67627ad707a4acbf33c41deb) collection.

> [!TIP]
> Click on the ModernBERT models in the right sidebar for more examples of how to apply ModernBERT to different language tasks.

The example below demonstrates how to predict the `[MASK]` token with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="fill-mask",
    model="answerdotai/ModernBERT-base",
    device=0
)
pipeline("Plants create [MASK] through a process known as photosynthesis.")
```

```python
import torch

from transformers import AutoModelForMaskedLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained(
    "answerdotai/ModernBERT-base",
)
model = AutoModelForMaskedLM.from_pretrained(
    "answerdotai/ModernBERT-base",
    device_map="auto",
    attn_implementation="sdpa"
)
inputs = tokenizer("Plants create [MASK] through a process known as photosynthesis.", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)
    predictions = outputs.logits

masked_index = torch.where(inputs['input_ids'] == tokenizer.mask_token_id)[1]
predicted_token_id = predictions[0, masked_index].argmax(dim=-1)
predicted_token = tokenizer.decode(predicted_token_id)

print(f"The predicted token is: {predicted_token}")
```

## Padding-free inference and training

ModernBERT supports padding-free inference and training. For example, you can leverage the [DataCollatorWithFlattening](/docs/transformers/v5.14.0/en/main_classes/data_collator#transformers.DataCollatorWithFlattening) to prepare your inputs:

> [!TIP]
> Padding-free inference and training requires `flash_attention_2` as the attention implementation. Since ModernBERT no longer defaults to FlashAttention2, you must explicitly set `attn_implementation="flash_attention_2"` when loading the model for padding-free usage.

```python
import torch

from transformers import AutoModelForMaskedLM, AutoTokenizer, DataCollatorWithFlattening

model_id = "answerdotai/ModernBERT-base"
tokenizer = AutoTokenizer.from_pretrained(model_id)
collator = DataCollatorWithFlattening(return_flash_attn_kwargs=True)

def prepare_text_for_padding_free(texts):
    # base tokenization with padding and subsequent flattening
    inputs_dict = tokenizer(texts, return_tensors="pt", padding=True).to(model.device)
    flattened_features = collator(
        [
            {"input_ids": i[a.bool()].tolist()}
            for i, a in zip(inputs_dict["input_ids"], inputs_dict["attention_mask"])
        ]
    )

    for k, v in flattened_features.items():
        if isinstance(v, torch.Tensor):
            flattened_features[k] = v.to(model.device)

    return flattened_features

inputs = prepare_text_for_padding_free(
    ["The capital of France is [MASK].", "ModernBERT is a [MASK] model."]
)
model = AutoModelForMaskedLM.from_pretrained(
    model_id, attn_implementation="flash_attention_2", device_map="cuda"
)

# Optional: use torch.compile for faster inference
# model.forward = torch.compile(model.forward, fullgraph=True)

out = model(**inputs)
```

## ModernBertConfig[[transformers.ModernBertConfig]]

- **vocab_size** (`int`, *optional*, defaults to `50368`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `1152`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `22`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **hidden_activation** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `8192`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_cutoff_factor** (`float`, *optional*, defaults to 2.0) --
  The cutoff factor for the truncated_normal_initializer for initializing all weight matrices.
- **norm_eps** (`float`, *optional*, defaults to 1e-05) --
  The epsilon used by the rms normalization layers.
- **norm_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use bias in the normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `50283`) --
  Token id used for padding in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `50282`) --
  Token id used for end-of-stream in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `50281`) --
  Token id used for beginning-of-stream in the vocabulary.
- **cls_token_id** (`int`, *optional*, defaults to `50281`) --
  Token id used for CLS in the vocabulary.
- **sep_token_id** (`int`, *optional*, defaults to `50282`) --
  Token id used for separator in the vocabulary.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.
- **rope_parameters** (`dict[Literal[full_attention, sliding_attention], dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **local_attention** (`int`, *optional*, defaults to 128) --
  The window size for local attention.
- **embedding_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the embeddings.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **mlp_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the MLP layers.
- **decoder_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the decoder layers.
- **classifier_pooling** (`str`, *optional*, defaults to `"cls"`) --
  The pooling method for the classifier. Should be either `"cls"` or `"mean"`. In local attention layers, the
  CLS token doesn't attend to all tokens on long sequences.
- **classifier_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for classifier.
- **classifier_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use bias in the classifier.
- **classifier_activation** (`str`, *optional*, defaults to `"gelu"`) --
  The activation function for the classifier.
- **deterministic_flash_attn** (`bool`, *optional*, defaults to `False`) --
  Whether to use deterministic flash attention. If `False`, inference will be faster but not deterministic.
- **sparse_prediction** (`bool`, *optional*, defaults to `False`) --
  Whether to use sparse prediction for the masked language model instead of returning the full dense logits.
- **sparse_pred_ignore_index** (`int`, *optional*, defaults to -100) --
  The index to ignore for the sparse prediction.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a ModernBertModel. It is used to instantiate a Modernbert
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [answerdotai/ModernBERT-base](https://huggingface.co/answerdotai/ModernBERT-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import ModernBertModel, ModernBertConfig

>>> # Initializing a ModernBert style configuration
>>> configuration = ModernBertConfig()

>>> # Initializing a model from the modernbert-base style configuration
>>> model = ModernBertModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ModernBertModel[[transformers.ModernBertModel]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Modernbert Model outputting raw hidden-states without any specific head on top.

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
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertModel](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## ModernBertForMaskedLM[[transformers.ModernBertForMaskedLM]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernBert Model with a decoder head on top that is used for masked language modeling.

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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.[MaskedLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`A [MaskedLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertForMaskedLM](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, ModernBertForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForMaskedLM.from_pretrained("answerdotai/ModernBERT-base")

>>> inputs = tokenizer("The capital of France is <mask>.", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # retrieve index of <mask>
>>> mask_token_index = (inputs.input_ids == tokenizer.mask_token_id)[0].nonzero(as_tuple=True)[0]

>>> predicted_token_id = logits[0, mask_token_index].argmax(axis=-1)
>>> tokenizer.decode(predicted_token_id)
...

>>> labels = tokenizer("The capital of France is Paris.", return_tensors="pt")["input_ids"]
>>> # mask labels of non-<mask> tokens
>>> labels = torch.where(inputs.input_ids == tokenizer.mask_token_id, labels, -100)

>>> outputs = model(**inputs, labels=labels)
>>> round(outputs.loss.item(), 2)
...
```

## ModernBertForSequenceClassification[[transformers.ModernBertForSequenceClassification]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernBert Model with a sequence classification head on top that performs pooling.

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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the sequence classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).[SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`A [SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
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
>>> from transformers import AutoTokenizer, ModernBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForSequenceClassification.from_pretrained("answerdotai/ModernBERT-base")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = ModernBertForSequenceClassification.from_pretrained("answerdotai/ModernBERT-base", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ModernBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForSequenceClassification.from_pretrained("answerdotai/ModernBERT-base", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = ModernBertForSequenceClassification.from_pretrained(
...     "answerdotai/ModernBERT-base", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## ModernBertForTokenClassification[[transformers.ModernBertForTokenClassification]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernBert Model with a token classification head on top, e.g. for Named Entity Recognition (NER) tasks.

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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the token classification loss. Indices should be in `[0, ..., config.num_labels - 1]`.[TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`A [TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertForTokenClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, ModernBertForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForTokenClassification.from_pretrained("answerdotai/ModernBERT-base")

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

## ModernBertForMultipleChoice[[transformers.ModernBertForMultipleChoice]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernBert Model with a multiple choice classification head on top (a linear layer on top of the pooled output and a softmax) e.g. for RocStories/SWAG tasks.

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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the multiple choice classification loss. Indices should be in `[0, ...,
  num_choices-1]` where `num_choices` is the size of the second dimension of the input tensors.</paramsdesc><rettype>[MultipleChoiceModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MultipleChoiceModelOutput) or `tuple(torch.FloatTensor)`A [MultipleChoiceModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MultipleChoiceModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertForMultipleChoice](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertForMultipleChoice) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape *(1,)*, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_choices)`) -- *num_choices* is the second dimension of the input tensors. (see *input_ids* above).

  Classification scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, ModernBertForMultipleChoice
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForMultipleChoice.from_pretrained("answerdotai/ModernBERT-base")

>>> prompt = "In Italy, pizza served in formal settings, such as at a restaurant, is presented unsliced."
>>> choice0 = "It is eaten with a fork and a knife."
>>> choice1 = "It is eaten while held in the hand."
>>> labels = torch.tensor(0).unsqueeze(0)  # choice0 is correct (according to Wikipedia ;)), batch size 1

>>> encoding = tokenizer([prompt, prompt], [choice0, choice1], return_tensors="pt", padding=True)
>>> outputs = model(**{k: v.unsqueeze(0) for k, v in encoding.items()}, labels=labels)  # batch size is 1

>>> # the linear classifier still needs to be trained
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## ModernBertForQuestionAnswering[[transformers.ModernBertForQuestionAnswering]]

- **config** ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Modernbert transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **start_positions** (`torch.Tensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the start of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **end_positions** (`torch.Tensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the end of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.[QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`A [QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernBertConfig](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertConfig)) and inputs.
The [ModernBertForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/modernbert#transformers.ModernBertForQuestionAnswering) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, ModernBertForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("answerdotai/ModernBERT-base")
>>> model = ModernBertForQuestionAnswering.from_pretrained("answerdotai/ModernBERT-base")

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

### Usage tips

The ModernBert model can be fine-tuned using the HuggingFace Transformers library with its [official script](https://github.com/huggingface/transformers/blob/main/examples/pytorch/question-answering/run_qa.py) for question-answering tasks.
