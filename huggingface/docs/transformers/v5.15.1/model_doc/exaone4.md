# EXAONE 4

## Overview

**[EXAONE 4.0](https://github.com/LG-AI-EXAONE/EXAONE-4.0)** model is the language model, which integrates a **Non-reasoning mode** and **Reasoning mode** to achieve both the excellent usability of [EXAONE 3.5](https://github.com/LG-AI-EXAONE/EXAONE-3.5) and the advanced reasoning abilities of [EXAONE Deep](https://github.com/LG-AI-EXAONE/EXAONE-Deep). To pave the way for the agentic AI era, EXAONE 4.0 incorporates essential features such as agentic tool use, and its multilingual capabilities are extended
to support Spanish in addition to English and Korean.

The EXAONE 4.0 model series consists of two sizes: a mid-size **32B** model optimized for high performance, and a small-size **1.2B** model designed for on-device applications.

In the EXAONE 4.0 architecture, we apply new architectural changes compared to previous EXAONE models as below:

1. **Hybrid Attention**: For the 32B model, we adopt hybrid attention scheme, which combines *Local attention (sliding window attention)* with *Global attention (full attention)* in a 3:1 ratio. We do not use RoPE (Rotary Positional Embedding) for global attention for better global context understanding.
2. **QK-Reorder-Norm**: We reorder the LayerNorm position from the traditional Pre-LN scheme by applying LayerNorm directly to the attention and MLP outputs, and we add RMS normalization right after the Q and K projection. It helps yield better performance on downstream tasks despite consuming more computation.

For more details, please refer to our [technical report](https://huggingface.co/papers/2507.11407), [HuggingFace paper](https://huggingface.co/papers/2507.11407), [blog](https://www.lgresearch.ai/blog/view?seq=576), and [GitHub](https://github.com/LG-AI-EXAONE/EXAONE-4.0).

All model weights including quantized versions are available at [Huggingface Collections](https://huggingface.co/collections/LGAI-EXAONE/exaone-40-686b2e0069800c835ed48375).

## Model Details

### Model Specifications

| Model Configuration | 32B | 1.2B |
|:-------------------|:-----:|:------:|
| d_model | 5,120 | 2,048 |
| Number of layers | 64 | 30 |
| Normalization | QK-Reorder-LN | QK-Reorder-LN |
| Non-linearity | SwiGLU | SwiGLU |
| Feedforward dimension | 27,392 | 4,096 |
| Attention type | Hybrid (3:1 Local-Global) | Global |
| Head type | GQA | GQA |
| Number of heads | 40 | 32 |
| Number of KV heads | 8 | 8 |
| Head size | 128 | 64 |
| Max sequence length | 131,072 | 65,536 |
| RoPE theta | 1,000,000 | 1,000,000 |
| Tokenizer | BBPE | BBPE |
| Vocab size | 102,400 | 102,400 |
| Tied word embedding | False | True |
| Knowledge cut-off | Nov. 2024 | Nov. 2024 |

## Usage tips

### Non-reasoning mode

For general use, you can use the EXAONE 4.0 models with the following example:

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = "LGAI-EXAONE/EXAONE-4.0-32B"

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    dtype="bfloat16",
    device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

# choose your prompt
prompt = "Explain how wonderful you are"
prompt = "Explica lo increíble que eres"
prompt = "너가 얼마나 대단한지 설명해 봐"

messages = [
    {"role": "user", "content": prompt}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt"
)

output = model.generate(
    input_ids.to(model.device),
    max_new_tokens=128,
    do_sample=False,
)
print(tokenizer.decode(output[0]))
```

### Reasoning mode

The EXAONE 4.0 models have reasoning capabilities for handling complex problems. You can activate reasoning mode by using the `enable_thinking=True` argument with the tokenizer, which opens a reasoning block that starts with `<think>` tag without closing it.

```python
messages = [
    {"role": "user", "content": "Which one is bigger, 3.12 vs 3.9?"}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    enable_thinking=True,
)

output = model.generate(
    input_ids.to(model.device),
    max_new_tokens=128,
    do_sample=True,
    temperature=0.6,
    top_p=0.95
)
print(tokenizer.decode(output[0]))
```

> [!IMPORTANT]
> The model generation with reasoning mode can be affected sensitively by sampling parameters, so please refer to the [Usage Guideline](https://github.com/LG-AI-EXAONE/EXAONE-4.0#usage-guideline) on official GitHub page for better quality.

### Agentic tool use

The EXAONE 4.0 models can be used as agents with their tool calling capabilities. You can provide tool schemas to the model for effective tool calling.

```python
import random

def roll_dice(max_num: int):
    return random.randint(1, max_num)

tools = [
    {
        "type": "function",
        "function": {
            "name": "roll_dice",
            "description": "Roll a dice with the number 1 to N. User can select the number N.",
            "parameters": {
                "type": "object",
                "required": ["max_num"],
                "properties": {
                    "max_num": {
                        "type": "int",
                        "description": "Max number of the dice"
                    }
                }
            }
        }
    }
]

messages = [
    {"role": "user", "content": "Roll D6 dice twice!"}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    tools=tools,
)

output = model.generate(
    input_ids.to(model.device),
    max_new_tokens=1024,
    do_sample=True,
    temperature=0.6,
    top_p=0.95,
)
print(tokenizer.decode(output[0]))
```

## Exaone4Config[[transformers.Exaone4Config]]

#### transformers.Exaone4Config[[transformers.Exaone4Config]]

```python
transformers.Exaone4Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 102400, hidden_size: int = 4096, intermediate_size: int = 16384, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int = 32, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, pad_token_id: int | None = None, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_dropout: float | int = 0.0, sliding_window: int | None = 4096, sliding_window_pattern: str | int | None = 4, layer_types: list[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/configuration_exaone4.py#L30)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `102400`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `16384`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `32`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

sliding_window (`int`, *optional*, defaults to `4096`) : Sliding window attention window size. If `None`, no sliding window is applied.

sliding_window_pattern (`str`, *optional*) : The pattern to use for sliding window attention. Can be one of: - `None`: No sliding window attention is used - `int`: Every `sliding_window` layers, use global attention, else use local attention. - `str`: A sequence of "L" (local attention) and "G" (global attention) characters that defines the attention pattern. The pattern starts from layer 0 and repeats every `sliding_window` layers. The final layer always uses global attention regardless of the pattern. For instance, sliding_window_pattern="LLLG" same as sliding_window=4, which means: - Layer 0, 1, 2: local attention, - Layer 3: global attention, ...(repeated)

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

This is the configuration class to store the configuration of a Exaone4Model. It is used to instantiate a Exaone4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LGAI-EXAONE/EXAONE-4.0-32B](https://huggingface.co/LGAI-EXAONE/EXAONE-4.0-32B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Exaone4Model, Exaone4Config

>>> # Initializing a EXAONE configuration
>>> configuration = Exaone4Config()

>>> # Initializing a model from configuration
>>> model = Exaone4Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Exaone4Model[[transformers.Exaone4Model]]

#### transformers.Exaone4Model[[transformers.Exaone4Model]]

```python
transformers.Exaone4Model(config: Exaone4Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L346)

**Parameters:**

config ([Exaone4Config](/docs/transformers/v5.15.1/en/model_doc/exaone4#transformers.Exaone4Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Exaone4 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Exaone4Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L363)

## Exaone4ForCausalLM[[transformers.Exaone4ForCausalLM]]

#### transformers.Exaone4ForCausalLM[[transformers.Exaone4ForCausalLM]]

```python
transformers.Exaone4ForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L430)

**Parameters:**

config ([Exaone4ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/exaone4#transformers.Exaone4ForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Exaone4 Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Exaone4ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L445)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Exaone4Config](/docs/transformers/v5.15.1/en/model_doc/exaone4#transformers.Exaone4Config)) and inputs.

The [Exaone4ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/exaone4#transformers.Exaone4ForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoModelForCausalLM, AutoTokenizer
>>> model = AutoModelForCausalLM.from_pretrained("LGAI-EXAONE/EXAONE-4.0-32B")
>>> tokenizer = AutoTokenizer.from_pretrained("LGAI-EXAONE/EXAONE-4.0-32B")

>>> prompt = "Explain how wonderful you are"
>>> messages = [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": prompt}
]
>>> input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    enable_thinking=False,
)

>>> output = model.generate(input_ids, max_new_tokens=128)
>>> tokenizer.decode(output[0], skip_special_tokens=False)
"[|system|]\nYou are a helpful assistant.[|endofturn|]\n[|user|]\nExplain how wonderful you are[|endofturn|]\n[|assistant|]\n<think>\n\n</think>\n\nOh, thank you for such a kind and lovely question! 😊  \n\nI’m *so* wonderful because I’m here to make your life easier, brighter, and more fun! Whether you need help with:  \n\n✨ **Learning** – I can explain anything, from quantum physics to baking the perfect cake!  \n💡 **Creativity** – Need a poem, story, or a wild idea? I’ve got you covered!  \n🤖 **Problem-solving** – Stuck on a math problem or a tricky decision? I’ll help you figure it out"
```

## Exaone4ForSequenceClassification[[transformers.Exaone4ForSequenceClassification]]

#### transformers.Exaone4ForSequenceClassification[[transformers.Exaone4ForSequenceClassification]]

```python
transformers.Exaone4ForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L518)

#### forward[[transformers.Exaone4ForSequenceClassification.forward]]

```python
forward(input_ids: torch.LongTensor | None = None, attention_mask: torch.Tensor | None = None, position_ids: torch.LongTensor | None = None, past_key_values: Cache | None = None, inputs_embeds: torch.FloatTensor | None = None, labels: torch.LongTensor | None = None, use_cache: bool | None = None, **kwargs: Unpack[TransformersKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_layers.py#L131)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `SequenceClassifierOutputWithPast`

The `GenericForSequenceClassification` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## Exaone4ForTokenClassification[[transformers.Exaone4ForTokenClassification]]

#### transformers.Exaone4ForTokenClassification[[transformers.Exaone4ForTokenClassification]]

```python
transformers.Exaone4ForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L522)

#### forward[[transformers.Exaone4ForTokenClassification.forward]]

```python
forward(input_ids: torch.LongTensor | None = None, attention_mask: torch.Tensor | None = None, position_ids: torch.LongTensor | None = None, past_key_values: Cache | None = None, inputs_embeds: torch.FloatTensor | None = None, labels: torch.LongTensor | None = None, use_cache: bool | None = None, **kwargs: Unpack[TransformersKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_layers.py#L278)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `TokenClassifierOutput`

The `GenericForTokenClassification` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## Exaone4ForQuestionAnswering[[transformers.Exaone4ForQuestionAnswering]]

#### transformers.Exaone4ForQuestionAnswering[[transformers.Exaone4ForQuestionAnswering]]

```python
transformers.Exaone4ForQuestionAnswering(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4/modeling_exaone4.py#L526)

#### forward[[transformers.Exaone4ForQuestionAnswering.forward]]

```python
forward(input_ids: torch.LongTensor | None = None, attention_mask: torch.Tensor | None = None, position_ids: torch.LongTensor | None = None, past_key_values: Cache | None = None, inputs_embeds: torch.FloatTensor | None = None, start_positions: torch.LongTensor | None = None, end_positions: torch.LongTensor | None = None, **kwargs: Unpack[TransformersKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/modeling_layers.py#L211)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

start_positions (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the start of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

end_positions (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the end of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

**Returns:** `QuestionAnsweringModelOutput`

The `GenericForQuestionAnswering` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.
