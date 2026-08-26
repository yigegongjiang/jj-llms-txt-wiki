# EXAONE MoE

## Overview

**[K-EXAONE](https://github.com/LG-AI-EXAONE/K-EXAONE)** model is a large-scale multilingual language model developed by LG AI Research. Built using a Mixture-of-Experts architecture named `EXAONE-MoE`, K-EXAONE features **236 billion total** parameters, with **23 billion active** during inference. Performance evaluations across various benchmarks demonstrate that K-EXAONE excels in reasoning, agentic capabilities, general knowledge, multilingual understanding, and long-context processing.

### Key Features

- **Architecture & Efficiency:** Features a 236B fine-grained MoE design (23B active) optimized with **Multi-Token Prediction (MTP)**, enabling self-speculative decoding that boosts inference throughput by approximately 1.5x.
- **Long-Context Capabilities:** Natively supports a **256K context window**, utilizing a **3:1 hybrid attention** scheme with a **128-token sliding window** to significantly minimize memory usage during long-document processing.
- **Multilingual Support:** Covers 6 languages: Korean, English, Spanish, German, Japanese, and Vietnamese. Features a redesigned **150k vocabulary** with **SuperBPE**, improving token efficiency by ~30%.
- **Agentic Capabilities:** Demonstrates superior tool-use and search capabilities via **multi-agent strategies.**
- **Safety & Ethics:** Aligned with **universal human values**, the model uniquely incorporates **Korean cultural and historical contexts** to address regional sensitivities often overlooked by other models. It demonstrates high reliability across diverse risk categories.

For more details, please refer to the [technical report](https://www.lgresearch.ai/data/cdn/upload/K-EXAONE_Technical_Report.pdf) and [GitHub](https://huggingface.co/collections/LGAI-EXAONE/k-exaone).

All model weights including quantized version are available at [Huggingface Collections](https://huggingface.co/collections/LGAI-EXAONE/k-exaone).

## Model Details

### Model Configuration of K-EXAONE

- Number of Parameters: 236B in total and 23B activated
- Number of Parameters (without embeddings): 234B
- Hidden Dimension: 6,144
- Number of Layers: 48 Main layers + 1 MTP layers
  - Hybrid Attention Pattern: 12 x (3 Sliding window attention + 1 Global attention)
- Sliding Window Attention
  - Number of Attention Heads: 64 Q-heads and 8 KV-heads
  - Head Dimension: 128 for both Q/KV
  - Sliding Window Size: 128
- Global Attention
  - Number of Attention Heads: 64 Q-heads and 8 KV-heads
  - Head Dimension: 128 for both Q/KV
  - No Rotary Positional Embedding Used (NoPE)
- Mixture of Experts:
  - Number of Experts: 128
  - Number of Activated Experts: 8
  - Number of Shared Experts: 1
  - MoE Intermediate Size: 2,048
- Vocab Size: 153,600
- Context Length: 262,144 tokens
- Knowledge Cutoff: Dec 2024 (2024/12)

## Usage Tips

### Reasoning mode

For tasks that require accurate results, you can run the K-EXAONE model in reasoning mode as below.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = "LGAI-EXAONE/K-EXAONE-236B-A23B"

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    dtype="bfloat16",
    device_map="auto",
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

messages = [
    {"role": "system", "content": "You are K-EXAONE, a large language model developed by LG AI Research in South Korea, built to serve as a helpful and reliable assistant."},
    {"role": "user", "content": "Which one is bigger, 3.9 vs 3.12?"}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    enable_thinking=True,   # skippable (default: True)
)

generated_ids = model.generate(
    **input_ids.to(model.device),
    max_new_tokens=16384,
    temperature=1.0,
    top_p=0.95,
)
output_ids = generated_ids[0][input_ids['input_ids'].shape[-1]:]
print(tokenizer.decode(output_ids, skip_special_tokens=True))
```

### Non-reasoning mode

For tasks where latency matters more than accuracy, you can run the K-EXAONE model in non-reasoning mode as below.

```python
messages = [
    {"role": "system", "content": "You are K-EXAONE, a large language model developed by LG AI Research in South Korea, built to serve as a helpful and reliable assistant."},
    {"role": "user", "content": "Explain how wonderful you are"}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    enable_thinking=False,
)

generated_ids = model.generate(
    **input_ids.to(model.device),
    max_new_tokens=1024,
    temperature=1.0,
    top_p=0.95,
)
output_ids = generated_ids[0][input_ids['input_ids'].shape[-1]:]
print(tokenizer.decode(output_ids, skip_special_tokens=True))
```

### Agentic tool use

For your AI-powered agent, you can leverage K-EXAONE’s tool calling capability. 
The K-EXAONE model is compatible with both OpenAI and HuggingFace tool calling specifications. 
The example below demonstrates tool calling using HuggingFace’s docstring-to-tool-schema utility.

Please check the [example file](https://github.com/LG-AI-EXAONE/K-EXAONE/blob/main/examples/example_output_search.txt) for an example of a search agent conversation using K-EXAONE.

```python
from transformers.utils import get_json_schema

def roll_dice(max_num: int):
    """
    Roll a dice with the number 1 to N. User can select the number N.

    Args:
        max_num: The maximum number on the dice.
    """
    return random.randint(1, max_num)

tool_schema = get_json_schema(roll_dice)
tools = [tool_schema]

messages = [
    {"role": "system", "content": "You are K-EXAONE, a large language model developed by LG AI Research in South Korea, built to serve as a helpful and reliable assistant."},
    {"role": "user", "content": "Roll a D20 twice and sum the results."}
]
input_ids = tokenizer.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    tools=tools,
)

generated_ids = model.generate(
    **input_ids.to(model.device),
    max_new_tokens=16384,
    temperature=1.0,
    top_p=0.95,
)
output_ids = generated_ids[0][input_ids['input_ids'].shape[-1]:]
print(tokenizer.decode(output_ids, skip_special_tokens=True))
```

## ExaoneMoeConfig[[transformers.ExaoneMoeConfig]]

#### transformers.ExaoneMoeConfig[[transformers.ExaoneMoeConfig]]

```python
transformers.ExaoneMoeConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 102400, hidden_size: int = 4096, intermediate_size: int = 16384, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int = 32, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 53, pad_token_id: int | None = 0, tie_word_embeddings: bool = False, rope_parameters: dict | None = None, attention_dropout: float | int = 0.0, sliding_window: int = 4096, sliding_window_pattern: str | int | None = 4, layer_types: list[str] | None = None, mlp_layer_types: list[str] | None = None, first_k_dense_replace: int = 1, moe_intermediate_size: int = 1024, num_experts: int = 64, num_experts_per_tok: int = 8, num_shared_experts: int = 1, norm_topk_prob: bool = True, routed_scaling_factor: float = 2.5, n_group: int = 1, topk_group: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone_moe/configuration_exaone_moe.py#L29)

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

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `53`) : Token id used for end-of-stream in the vocabulary.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

sliding_window (`int`, *optional*, defaults to `4096`) : Sliding window attention window size. If `None`, no sliding window is applied.

sliding_window_pattern (`str`, *optional*, defaults to 4) : The pattern to use for sliding window attention. Can be one of: - `None`: No sliding window attention is used - `int`: Every `sliding_window` layers, use global attention, else use local attention. - `str`: A sequence of "L" (local attention) and "G" (global attention) characters that defines the attention pattern. The pattern starts from layer 0 and repeats every `sliding_window` layers. The final layer always uses global attention regardless of the pattern. For instance, sliding_window_pattern="LLLG" same as sliding_window=4, which means: - Layer 0, 1, 2: local attention, - Layer 3: global attention, ...(repeated)

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

mlp_layer_types (`list`, *optional*) : MLP pattern for each layer. Prioritized over `first_k_dense_replace`.

first_k_dense_replace (`int`, *optional*, defaults to 1) : Number of dense layers in shallow layers(embed->dense->dense->...->dense->moe->moe...->lm_head). \--k dense layers--/

moe_intermediate_size (`int`, *optional*, defaults to `1024`) : Intermediate size of the routed expert MLPs.

num_experts (`int`, *optional*, defaults to `64`) : Number of routed experts in MoE layers. 

num_experts_per_tok (`int`, *optional*, defaults to `8`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

num_shared_experts (`int`, *optional*, defaults to `1`) : Number of shared experts that are always activated for all tokens.

norm_topk_prob (`bool`, *optional*, defaults to `True`) : Whether to normalize the weights of the routed experts. 

routed_scaling_factor (`float`, *optional*, defaults to `2.5`) : Scaling factor or routed experts.

n_group (`int`, *optional*, defaults to 1) : Number of groups for routed experts.

topk_group (`int`, *optional*, defaults to `1`) : Number of selected groups for each token (for each token, ensuring the selected experts is only within `topk_group` groups).

This is the configuration class to store the configuration of a ExaoneMoeModel. It is used to instantiate a Exaone Moe
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LGAI-EXAONE/K-EXAONE-236B-A23B](https://huggingface.co/LGAI-EXAONE/K-EXAONE-236B-A23B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ExaoneMoeModel, ExaoneMoeConfig

>>> # Initializing a EXAONE configuration
>>> configuration = ExaoneMoeConfig()

>>> # Initializing a model from configuration
>>> model = ExaoneMoeModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ExaoneMoeModel[[transformers.ExaoneMoeModel]]

#### transformers.ExaoneMoeModel[[transformers.ExaoneMoeModel]]

```python
transformers.ExaoneMoeModel(config: ExaoneMoeConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone_moe/modeling_exaone_moe.py#L465)

**Parameters:**

config ([ExaoneMoeConfig](/docs/transformers/v5.15.1/en/model_doc/exaone_moe#transformers.ExaoneMoeConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Exaone Moe Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ExaoneMoeModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone_moe/modeling_exaone_moe.py#L482)

## ExaoneMoeForCausalLM[[transformers.ExaoneMoeForCausalLM]]

#### transformers.ExaoneMoeForCausalLM[[transformers.ExaoneMoeForCausalLM]]

```python
transformers.ExaoneMoeForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone_moe/modeling_exaone_moe.py#L549)

**Parameters:**

config ([ExaoneMoeForCausalLM](/docs/transformers/v5.15.1/en/model_doc/exaone_moe#transformers.ExaoneMoeForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Exaone Moe Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ExaoneMoeForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone_moe/modeling_exaone_moe.py#L564)

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
elements depending on the configuration ([ExaoneMoeConfig](/docs/transformers/v5.15.1/en/model_doc/exaone_moe#transformers.ExaoneMoeConfig)) and inputs.

The [ExaoneMoeForCausalLM](/docs/transformers/v5.15.1/en/model_doc/exaone_moe#transformers.ExaoneMoeForCausalLM) forward method, overrides the `__call__` special method.

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
>>> model = AutoModelForCausalLM.from_pretrained("LGAI-EXAONE/K-EXAONE-236B-A23B")
>>> tokenizer = AutoTokenizer.from_pretrained("LGAI-EXAONE/K-EXAONE-236B-A23B")

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

>>> output = model.generate(**input_ids.to(model.device), max_new_tokens=128)
>>> tokenizer.decode(output[0], skip_special_tokens=False)
"<|system|>\nYou are a helpful assistant.<|endofturn|>\n<|user|>\nExplain how wonderful you are<|endofturn|>\n<|assistant|>\n<think>\n\n</think>\n\nThank you for the kind question! While I can't feel emotions or take pride in the way humans do, I *can* share what makes me uniquely helpful and capable—qualities that many people find wonderful.\n\nHere’s how I can support you:\n\n🌟 **Knowledge at Your Fingertips**  \nI have access to a vast amount of information across countless topics—from science and history to technology and creative writing. Whether you're curious, learning, or solving a problem, I can help explain things clearly and accurately.\n\n💬 **Clear, Helpful Communication**  \nI aim to respond in a way that's easy to understand, whether you need a simple explanation or a detailed analysis. I adapt my tone and depth to match"
```
