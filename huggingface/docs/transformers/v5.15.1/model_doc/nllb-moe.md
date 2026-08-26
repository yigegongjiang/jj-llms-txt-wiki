# NLLB-MOE

## Overview

The NLLB model was presented in [No Language Left Behind: Scaling Human-Centered Machine Translation](https://huggingface.co/papers/2207.04672) by Marta R. Costa-jussà, James Cross, Onur Çelebi,
Maha Elbayad, Kenneth Heafield, Kevin Heffernan, Elahe Kalbassi, Janice Lam, Daniel Licht, Jean Maillard, Anna Sun, Skyler Wang, Guillaume Wenzek, Al Youngblood, Bapi Akula,
Loic Barrault, Gabriel Mejia Gonzalez, Prangthip Hansanti, John Hoffman, Semarley Jarrett, Kaushik Ram Sadagopan, Dirk Rowe, Shannon Spruit, Chau Tran, Pierre Andrews,
Necip Fazil Ayan, Shruti Bhosale, Sergey Edunov, Angela Fan, Cynthia Gao, Vedanuj Goswami, Francisco Guzmán, Philipp Koehn, Alexandre Mourachko, Christophe Ropers,
Safiyyah Saleem, Holger Schwenk, and Jeff Wang.

The abstract of the paper is the following:

*Driven by the goal of eradicating language barriers on a global scale, machine translation has solidified itself as a key focus of artificial intelligence research today.
However, such efforts have coalesced around a small subset of languages, leaving behind the vast majority of mostly low-resource languages. What does it take to break the
200 language barrier while ensuring safe, high quality results, all while keeping ethical considerations in mind? In No Language Left Behind, we took on this challenge by
first contextualizing the need for low-resource language translation support through exploratory interviews with native speakers. Then, we created datasets and models aimed
at narrowing the performance gap between low and high-resource languages. More specifically, we developed a conditional compute model based on Sparsely Gated Mixture of
Experts that is trained on data obtained with novel and effective data mining techniques tailored for low-resource languages. We propose multiple architectural and training
improvements to counteract overfitting while training on thousands of tasks. Critically, we evaluated the performance of over 40,000 different translation directions using
a human-translated benchmark, Flores-200, and combined human evaluation with a novel toxicity benchmark covering all languages in Flores-200 to assess translation safety.
Our model achieves an improvement of 44% BLEU relative to the previous state-of-the-art, laying important groundwork towards realizing a universal translation system.*

This model was contributed by [Arthur Zucker](https://huggingface.co/ArthurZ).
The original code can be found [here](https://github.com/facebookresearch/fairseq).

## Usage tips

- M2M100ForConditionalGeneration is the base model for both NLLB and NLLB MoE
- The NLLB-MoE is very similar to the NLLB model, but it's feed forward layer is based on the implementation of SwitchTransformers.
- The tokenizer is the same as the NLLB models.

## Implementation differences with SwitchTransformers

The biggest difference is the way the tokens are routed. NLLB-MoE uses a `top-2-gate` which means that for each input, only the top two experts are selected based on the
highest predicted probabilities from the gating network, and the remaining experts are ignored. In `SwitchTransformers`, only the top-1 probabilities are computed,
which means that tokens have less probability of being forwarded. Moreover, if a token is not routed to any expert, `SwitchTransformers` still adds its unmodified hidden
states (kind of like a residual connection) while they are masked in `NLLB`'s top-2 routing mechanism.

## Generating with NLLB-MoE

The available checkpoints require around 350GB of storage. Make sure to use `accelerate` if you do not have enough RAM on your machine.

While generating the target text set the `forced_bos_token_id` to the target language id. The following
example shows how to translate English to French using the *facebook/nllb-200-distilled-600M* model.

Note that we're using the BCP-47 code for French `fra_Latn`. See [here](https://github.com/facebookresearch/flores/blob/main/flores200/README.md#languages-in-flores-200)
for the list of all BCP-47 in the Flores 200 dataset.

```python
from transformers import AutoModelForSeq2SeqLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("facebook/nllb-moe-54b")
model = AutoModelForSeq2SeqLM.from_pretrained("facebook/nllb-moe-54b", device_map="auto")

article = "Previously, Ring's CEO, Jamie Siminoff, remarked the company started when his doorbell wasn't audible from his shop in his garage."
inputs = tokenizer(article, return_tensors="pt").to(model.device)

translated_tokens = model.generate(
    **inputs, forced_bos_token_id=tokenizer.lang_code_to_id["fra_Latn"], max_length=50
)
tokenizer.batch_decode(translated_tokens, skip_special_tokens=True)[0]
"Auparavant, le PDG de Ring, Jamie Siminoff, a fait remarquer que la société avait commencé lorsque sa sonnette n'était pas audible depuis son magasin dans son garage."
```

### Generating from any other language than English

English (`eng_Latn`) is set as the default language from which to translate. In order to specify that you'd like to translate from a different language,
you should specify the BCP-47 code in the `src_lang` keyword argument of the tokenizer initialization.

See example below for a translation from romanian to german:

```python
from transformers import AutoModelForSeq2SeqLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("facebook/nllb-moe-54b", src_lang="ron_Latn")
model = AutoModelForSeq2SeqLM.from_pretrained("facebook/nllb-moe-54b", device_map="auto")

article = "Şeful ONU spune că nu există o soluţie militară în Siria"
inputs = tokenizer(article, return_tensors="pt").to(model.device)

translated_tokens = model.generate(
    **inputs, forced_bos_token_id=tokenizer.lang_code_to_id["deu_Latn"], max_length=30
)
tokenizer.batch_decode(translated_tokens, skip_special_tokens=True)[0]
```

## Resources

- [Translation task guide](../tasks/translation)
- [Summarization task guide](../tasks/summarization)

## NllbMoeConfig[[transformers.NllbMoeConfig]]

#### transformers.NllbMoeConfig[[transformers.NllbMoeConfig]]

```python
transformers.NllbMoeConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 128112, max_position_embeddings: int = 1024, encoder_layers: int = 12, encoder_ffn_dim: int = 4096, encoder_attention_heads: int = 16, decoder_layers: int = 12, decoder_ffn_dim: int = 4096, decoder_attention_heads: int = 16, encoder_layerdrop: float | int = 0.05, decoder_layerdrop: float | int = 0.05, use_cache: bool = True, activation_function: str = 'relu', d_model: int = 1024, dropout: float | int = 0.1, attention_dropout: float | int = 0.1, activation_dropout: float | int = 0.0, init_std: float = 0.02, decoder_start_token_id: int | None = 2, scale_embedding: bool = True, router_bias: bool = False, router_dtype: typing.Literal['float32', 'float16', 'bfloat16'] = 'float32', router_ignore_padding_tokens: bool = False, num_experts: int = 128, expert_capacity: int = 64, encoder_sparse_step: int = 4, decoder_sparse_step: int = 4, router_z_loss_coef: float = 0.001, router_aux_loss_coef: float = 0.001, second_expert_policy: str = 'all', normalize_router_prob_before_dropping: bool = False, batch_prioritized_routing: bool = False, moe_eval_capacity_token_fraction: float = 1.0, moe_token_dropout: float | int = 0.2, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = True, output_router_logits: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/configuration_nllb_moe.py#L26)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `128112`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

max_position_embeddings (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

encoder_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `4096`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `4096`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.05`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.05`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

d_model (`int`, *optional*, defaults to `1024`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

decoder_start_token_id (`int`, *optional*, defaults to `2`) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

scale_embedding (`bool`, *optional*, defaults to `True`) : Whether to scale embeddings by dividing by sqrt(d_model).

router_bias (`bool`, *optional*, defaults to `False`) : Whether or not the classifier of the router should have a bias.

router_dtype (`str`, *optional*, default to `"float32"`) : The `dtype` used for the routers. It is preferable to keep the `dtype` to `"float32"` as specified in the *selective precision* discussion in [the paper](https://huggingface.co/papers/2101.03961).

router_ignore_padding_tokens (`bool`, *optional*, defaults to `False`) : Whether to ignore padding tokens when routing. if `False`, the padding tokens are not routed to any experts.

num_experts (`int`, *optional*, defaults to `128`) : Number of routed experts in MoE layers. 

expert_capacity (`int`, *optional*, defaults to 64) : Number of tokens that can be stored in each expert.

encoder_sparse_step (`int`, *optional*, defaults to 4) : Frequency of the sparse layers in the encoder. 4 means that one out of 4 layers will be sparse.

decoder_sparse_step (`int`, *optional*, defaults to 4) : Frequency of the sparse layers in the decoder. 4 means that one out of 4 layers will be sparse.

router_z_loss_coef (`float`, *optional*, defaults to `0.001`) : Coefficient for the router z-loss, which penalizes large router logits to improve training stability.

router_aux_loss_coef (`float`, *optional*, defaults to `0.001`) : Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

second_expert_policy (`str`, *optional*, default to `"all"`) : The policy used for the sampling the probability of being sampled to a second expert for each token.

normalize_router_prob_before_dropping (`bool`, *optional*, defaults to `True`) : Whether or not to normalize the router probabilities before applying a mask based on the experts capacity (capacity dropping).

batch_prioritized_routing (`bool`, *optional*, defaults to `True`) : Whether or not to orders the tokens by their router probabilities before capacity dropping. This means that the tokens that have the highest probabilities will be routed before other tokens that might be further in the sequence.

moe_eval_capacity_token_fraction (`float`, *optional*, defaults to 1.0) : Fraction of tokens as capacity during validation, if set to negative, uses the same as training. Should be in range: (0.0, 1.0].

moe_token_dropout (`float`, *optional*, default to 0.2) : Masking rate for MoE expert output masking (EOM), which is implemented via a Dropout2d on the expert outputs.

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

output_router_logits (`bool`, *optional*, defaults to `False`) : Whether or not the router logits should be returned by the model. Enabling this will also allow the model to output the auxiliary loss, including load balancing loss and router z-loss.

This is the configuration class to store the configuration of a Nllb MoeModel. It is used to instantiate a Nllb Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/nllb-moe-54b](https://huggingface.co/facebook/nllb-moe-54b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import NllbMoeModel, NllbMoeConfig

>>> # Initializing a NllbMoe facebook/nllb-moe-54b style configuration
>>> configuration = NllbMoeConfig()

>>> # Initializing a model from the facebook/nllb-moe-54b style configuration
>>> model = NllbMoeModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## NllbMoeTop2Router[[transformers.NllbMoeTop2Router]]

#### transformers.NllbMoeTop2Router[[transformers.NllbMoeTop2Router]]

```python
transformers.NllbMoeTop2Router(config: NllbMoeConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L165)

Router using tokens choose top-2 experts assignment.

This router uses the same mechanism as in NLLB-MoE from the fairseq repository. Items are sorted by router_probs
and then routed to their choice of expert until the expert's expert_capacity is reached. **There is no guarantee
that each token is processed by an expert**, or that each expert receives at least one token.

The router combining weights are also returned to make sure that the states that are not updated will be masked.

#### route_tokens[[transformers.NllbMoeTop2Router.route_tokens]]

```python
route_tokens(router_logits: Tensor, input_dtype: dtype = torch.float32, padding_mask: typing.Optional[torch.LongTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L206)

Computes the `dispatch_mask` and the `dispatch_weights` for each experts. The masks are adapted to the expert
capacity.

#### forward[[transformers.NllbMoeTop2Router.forward]]

```python
forward(hidden_states: Tensor, padding_mask: typing.Optional[torch.LongTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L291)

**Parameters:**

hidden_states (`torch.Tensor`) : (batch_size, sequence_length, hidden_dim) from which router probabilities are computed.

**Returns:** top_1_mask (`torch.Tensor` of shape (batch_size, sequence_length))

Index tensor of shape [batch_size, sequence_length] corresponding to the expert selected for each token
using the top1 probabilities of the router.
router_probabilities (`torch.Tensor` of shape (batch_size, sequence_length, nump_experts)):
Tensor of shape (batch_size, sequence_length, num_experts) corresponding to the probabilities for each
token and expert. Used for routing tokens to experts.
router_logits (`torch.Tensor` of shape (batch_size, sequence_length))):
Logits tensor of shape (batch_size, sequence_length, num_experts) corresponding to raw router logits.
This is used later for computing router z-loss.

The hidden states are reshaped to simplify the computation of the router probabilities (combining weights for
each experts.)

## NllbMoeSparseMLP[[transformers.NllbMoeSparseMLP]]

#### transformers.NllbMoeSparseMLP[[transformers.NllbMoeSparseMLP]]

```python
transformers.NllbMoeSparseMLP(config: NllbMoeConfig, ffn_dim: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L367)

Implementation of the NLLB-MoE sparse MLP module.

#### forward[[transformers.NllbMoeSparseMLP.forward]]

```python
forward(hidden_states: Tensor, padding_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L378)

## NllbMoeModel[[transformers.NllbMoeModel]]

#### transformers.NllbMoeModel[[transformers.NllbMoeModel]]

```python
transformers.NllbMoeModel(config: NllbMoeConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L860)

**Parameters:**

config ([NllbMoeConfig](/docs/transformers/v5.15.1/en/model_doc/nllb-moe#transformers.NllbMoeConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Nllb Moe Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.NllbMoeModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L887)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `Seq2SeqMoEModelOutput` or `tuple(torch.FloatTensor)`

A `Seq2SeqMoEModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [NllbMoeModel](/docs/transformers/v5.15.1/en/model_doc/nllb-moe#transformers.NllbMoeModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **decoder_router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_logits=True` is passed or when `config.add_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Router logits of the decoder model, useful to compute the auxiliary loss for Mixture of Experts models.
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
- **encoder_router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_logits=True` is passed or when `config.add_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Router logits of the encoder model, useful to compute the auxiliary loss and the z_loss for the sparse
  modules.

## NllbMoeForConditionalGeneration[[transformers.NllbMoeForConditionalGeneration]]

#### transformers.NllbMoeForConditionalGeneration[[transformers.NllbMoeForConditionalGeneration]]

```python
transformers.NllbMoeForConditionalGeneration(config: NllbMoeConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L1039)

**Parameters:**

config ([NllbMoeConfig](/docs/transformers/v5.15.1/en/model_doc/nllb-moe#transformers.NllbMoeConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The NllbMoe Model with a language modeling head. Can be used for summarization.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.NllbMoeForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_router_logits: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nllb_moe/modeling_nllb_moe.py#L1055)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_router_logits (`bool`, *optional*) : Whether or not to return the logits of all the routers. They are useful for computing the router loss, and should not be returned during inference.

**Returns:** `Seq2SeqMoEOutput` or `tuple(torch.FloatTensor)`

A `Seq2SeqMoEOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([NllbMoeConfig](/docs/transformers/v5.15.1/en/model_doc/nllb-moe#transformers.NllbMoeConfig)) and inputs.

The [NllbMoeForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/nllb-moe#transformers.NllbMoeForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **decoder_router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_logits=True` is passed or when `config.add_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Router logits of the decoder model, useful to compute the auxiliary loss for Mixture of Experts models.
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
- **encoder_router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_logits=True` is passed or when `config.add_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Router logits of the encoder model, useful to compute the auxiliary loss and z_loss for Mixture of Experts
  models.

Example:

```python
```
