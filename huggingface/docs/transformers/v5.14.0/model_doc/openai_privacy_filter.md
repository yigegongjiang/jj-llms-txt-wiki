# OpenAI Privacy Filter

OpenAI Privacy Filter is a bidirectional token-classification model for personally identifiable information (PII) detection and masking in text. It is intended for high-throughput data sanitization workflows where teams need a model that they can run on-premises that is fast, context-aware, and tunable.

OpenAI Privacy Filter is pretrained autoregressively to arrive at a checkpoint with similar architecture to gpt-oss, albeit of a smaller size.  We  then converted that checkpoint into a bidirectional token classifier over a privacy label taxonomy, and post-trained with a supervised classification loss. (For architecture details about gpt-oss, please see the gpt-oss model card.) Instead of generating text token-by-token, this model labels an input sequence in a single forward pass, then decodes coherent spans with a constrained Viterbi procedure. For each input token, the model predicts a probability distribution over the label taxonomy which consists of 8 output categories described below.

Highlights:

- Permissive Apache 2.0 license: ideal for experimentation, customization, and commercial deployment.
- Small size: Runs in a web browser or on a laptop – 1.5B parameters total and 50M active parameters.
- Fine-tunable: Adapt the model to specific data distributions through easy and data efficient finetuning.
- Long-context: 128,000-token context window enables processing long text with high throughput and no chunking.
- Runtime control: configure precision/recall tradeoffs and detected span lengths through preset operating points.

The example below demonstrates how to detect privacy-sensitive tokens with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModelForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModelForTokenClassification) class.

```python
from transformers import pipeline

classifier = pipeline(
    task="token-classification",
    model="openai/privacy-filter",
)
classifier("My name is Alice Smith")
```

```python
import torch

from transformers import AutoModelForTokenClassification, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("openai/privacy-filter")
model = AutoModelForTokenClassification.from_pretrained("openai/privacy-filter", device_map="auto")

inputs = tokenizer("My name is Alice Smith", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

predicted_token_class_ids = outputs.logits.argmax(dim=-1)
predicted_token_classes = [model.config.id2label[token_id.item()] for token_id in predicted_token_class_ids[0]]
print(predicted_token_classes)
```

- Developed by: OpenAI
- Funded by: OpenAI
- Shared by: OpenAI
- Model type: Bidirectional token classification model for privacy span detection
- Language(s): Primarily English; selected multilingual robustness evaluation reported
- License: [Apache 2.0](LICENSE)

- Source repository: https://github.com/openai/privacy-filter
- Model weights: https://huggingface.co/openai/privacy-filter
- Demo: https://huggingface.co/spaces/openai/privacy-filter

## Resources

- [Token classification task guide](../tasks/token_classification)

## OpenAIPrivacyFilterConfig[[transformers.OpenAIPrivacyFilterConfig]]

- **num_hidden_layers** (`int`, *optional*, defaults to `8`) --
  Number of hidden layers in the Transformer decoder.
- **num_local_experts** (`int`, *optional*, defaults to `128`) --
  Number of local experts on each device. `num_experts` should be divisible by `num_local_experts`.
- **vocab_size** (`int`, *optional*, defaults to `200064`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `640`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `640`) --
  Dimension of the MLP representations.
- **head_dim** (`int`, *optional*, defaults to `64`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **num_attention_heads** (`int`, *optional*, defaults to `14`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `2`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **sliding_window** (`int`, *optional*, defaults to `128`) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **max_position_embeddings** (`int`, *optional*, defaults to `131072`) --
  The maximum sequence length that this model might ever be used with.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_experts_per_tok** (`int`, *optional*, defaults to `4`) --
  Number of experts to route each token to. This is the top-k value for the token-choice routing.
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.
- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `199999`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `199999`) --
  Token id used for end-of-stream in the vocabulary.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **classifier_dropout** (`float`, *optional*, defaults to `0.0`) --
  The dropout ratio for classifier.

This is the configuration class to store the configuration of a OpenAIPrivacyFilterModel. It is used to instantiate a Openai Privacy Filter
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openai/privacy-filter](https://huggingface.co/openai/privacy-filter)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## OpenAIPrivacyFilterModel[[transformers.OpenAIPrivacyFilterModel]]

- **config** ([OpenAIPrivacyFilterConfig](/docs/transformers/v5.14.0/en/model_doc/openai_privacy_filter#transformers.OpenAIPrivacyFilterConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Openai Privacy Filter Model outputting raw hidden-states without any specific head on top.

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([OpenAIPrivacyFilterConfig](/docs/transformers/v5.14.0/en/model_doc/openai_privacy_filter#transformers.OpenAIPrivacyFilterConfig)) and inputs.
The [OpenAIPrivacyFilterModel](/docs/transformers/v5.14.0/en/model_doc/openai_privacy_filter#transformers.OpenAIPrivacyFilterModel) forward method, overrides the `__call__` special method.

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

## OpenAIPrivacyFilterForTokenClassification[[transformers.OpenAIPrivacyFilterForTokenClassification]]

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
- **past_key_values** (`Cache`, *optional*) --
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
  `past_key_values`).`TokenClassifierOutput`
The `GenericForTokenClassification` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.
