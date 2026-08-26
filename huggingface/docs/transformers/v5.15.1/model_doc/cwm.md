# Code World Model (CWM)

## Overview

The Code World Model (CWM) model was proposed in [CWM: An Open-Weights LLM for Research on Code
Generation with World Models](https://ai.facebook.com/research/publications/cwm) by Meta FAIR CodeGen Team.
CWM is an LLM for code generation and reasoning about code that has, in particular, been trained
to better represent and reason about how code and commands affect the state of a program or system.
Specifically, we mid-trained CWM on a large number of observation-action trajectories from Python
execution traces and agentic interactions in containerized environments. We post-trained with
extensive multi-task RL in verifiable coding, math, and multi-turn software engineering environments.

The abstract from the paper is the following:

> *We release Code World Model (CWM), a 32-billion-parameter open-weights LLM, to advance research
on code generation with world models. To improve code understanding beyond what can be learned
from training on static code alone, we mid-train CWM on a large amount of observation-action
trajectories from Python interpreter and agentic Docker environments, and perform extensive multi-
task reasoning RL in verifiable coding, math, and multi-turn software engineering environments. With
CWM, we provide a strong testbed for researchers to explore the opportunities world modeling affords
for improving code generation with reasoning and planning in computational environments. We
present first steps of how world models can benefit agentic coding, enable step-by-step simulation of
Python code execution, and show early results of how reasoning can benefit from the latter. CWM is
a dense, decoder-only LLM trained with a context size of up to 131 k tokens. Independent of its world
modeling capabilities, CWM offers strong performance on general coding and math tasks: it reaches
pass@1 scores of 65.8 % on SWE-bench Verified (with test-time scaling), 68.9 % on LiveCodeBench,
96.6 % on Math-500, and 76.0 % on AIME 2024. To support further research on code world modeling,
we release model checkpoints after mid-training, SFT, and RL.*

Tips:

CWM requires a dedicated system prompt to function optimally during inference. Without proper prompt
configuration, CWM's output quality may be significantly degraded. The following serves as the default
system prompt for reasoning tasks. For agentic workflows, append the relevant tool specifications
after this base prompt. Checkout the original code repository for more details.

```text
You are a helpful AI assistant. You always reason before responding, using the following format:

<think>
your internal reasoning
</think>
your external response
```

This model was contributed by [Meta FAIR CodeGen Team](https://huggingface.co/facebook).
The original code can be found [here](https://github.com/facebookresearch/cwm).

## Usage examples

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_name = 'facebook/cwm'
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    device_map="auto"
)

system_prompt = """
You are a helpful AI assistant. You always reason before responding, using the following format:

<think>
your internal reasoning
</think>
your external response
""".strip()

messages = [
    {"role": "system", "content": system_prompt},
    {"role": "user", "content": "Write a haiku about recursion in programming."}
]

text = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True,
    enable_thinking=True, # Switches between thinking and non-thinking modes. Default is True.
    preserve_previous_think=True, # Switches between keeping thinking blocks from previous messages or not. Default is True.
)

model_inputs = tokenizer([text], return_tensors="pt").to(model.device)

generated_ids = model.generate(
    **model_inputs,
    max_new_tokens=1024
)
output_ids = generated_ids[0][len(model_inputs.input_ids[0]):].tolist()
print(tokenizer.decode(output_ids))
```

Produces the following output:

    Okay, I need to write a haiku about recursion in programming. Let's start by recalling what recursion is. Recursion is a programming technique where a function calls itself in its own definition, often used to solve problems that can be broken down into smaller, similar sub-problems.

    A haiku is a three-line poem with a syllable count of 5-7-5. So, I need to capture the essence of recursion in a concise and poetic way. Let's think about the key elements of recursion: the function calling itself, the base case that stops the recursion, and the idea of breaking down problems into smaller parts.

    Maybe I can use imagery related to mirrors or reflections, since recursion often involves a function reflecting or repeating itself. Alternatively, I could use natural imagery like a spiral or a tree, which suggests self-similar patterns.

    Let me try to come up with some ideas. For example, "Function calls itself" could be a starting point. But how to make that into a haiku? Maybe something like:

    Function calls itself
    Base case stops the endless loop
    Spiral of code

    Hmm, that's a start. But maybe I can make it more poetic. Let's think about the idea of a function calling itself as a kind of infinite loop, but with a base case that stops it. That's like a spiral that unwinds at the center.

    Another angle: recursion is often used for problems that have a recursive structure, like tree traversals or factorial calculations. Maybe I can use a natural image to represent that.

    How about:

    Branches split again
    Leaves unfold in endless dance
    Roots hold the base case

    That's a bit more poetic. The branches splitting represent the recursive calls, the leaves unfolding could represent the processing of each sub-problem, and the roots holding the base case are what stops the recursion.

    Alternatively, maybe something simpler:

    Mirror within mirror
    Reflections infinite deep
    Base case breaks the glass

    This uses the mirror metaphor to represent the function calling itself, with the base case being the point where the recursion stops, breaking the infinite reflection.

    I think I like the mirror idea better. Let me refine that:

    Mirror within mirror
    Infinite reflections dance
    Base case shatters glass

    Yes, that seems to capture the idea of recursion well. The mirror represents the function calling itself, the infinite reflections are the recursive calls, and the base case shattering the glass is what stops the recursion.

    Let me check the syllable count:

    Mirror within mirror (5 syllables: Mir-ror-with-in-mir-ror)
    Infinite reflections dance (7 syllables: In-fi-nite-ref-lec-tions-dance)
    Base case shatters glass (5 syllables: Base-case-shat-ters-glass)

    Yes, that fits the 5-7-5 structure. I think that's a good haiku about recursion.
    
    Mirror within mirror
    Infinite reflections dance
    Base case shatters glass<|eot_id|>

## CwmConfig[[transformers.CwmConfig]]

#### transformers.CwmConfig[[transformers.CwmConfig]]

```python
transformers.CwmConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 128256, hidden_size: int = 6144, intermediate_size: int = 21504, num_hidden_layers: int = 64, num_attention_heads: int = 48, num_key_value_heads: int = 8, hidden_act: str = 'silu', max_position_embeddings: int = 131072, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int = 128000, eos_token_id: int | list[int] | None = None, pretraining_tp: int = 1, tie_word_embeddings: bool = False, rope_parameters: dict | None = None, attention_dropout: float | int = 0.0, mlp_bias: bool = False, head_dim: int = 128, sliding_window: int = 8192, layer_types: list[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/configuration_cwm.py#L30)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `128256`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `6144`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `21504`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `64`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `48`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `131072`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `128000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

pretraining_tp (`int`, *optional*, defaults to `1`) : Experimental feature. Tensor parallelism rank used during pretraining. Please refer to [this document](https://huggingface.co/docs/transformers/main/perf_train_gpu_many#tensor-parallelism) to understand more about it. This value is necessary to ensure exact reproducibility of the pretraining results. Please refer to [this issue](https://github.com/pytorch/pytorch/issues/76232).

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

sliding_window (`int`, *optional*, defaults to `8192`) : Sliding window attention window size. If `None`, no sliding window is applied.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

This is the configuration class to store the configuration of a CwmModel. It is used to instantiate a Cwm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/cwm](https://huggingface.co/facebook/cwm)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CwmModel, CwmConfig

>>> # Initializing a Cwm cwm-7b style configuration
>>> configuration = CwmConfig()

>>> # Initializing a model from the cwm-7b style configuration
>>> model = CwmModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CwmPreTrainedModel[[transformers.CwmPreTrainedModel]]

#### transformers.CwmPreTrainedModel[[transformers.CwmPreTrainedModel]]

```python
transformers.CwmPreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/modeling_cwm.py#L311)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CwmPreTrainedModel.forward]]

```python
forward(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## CwmModel[[transformers.CwmModel]]

#### transformers.CwmModel[[transformers.CwmModel]]

```python
transformers.CwmModel(config: CwmConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/modeling_cwm.py#L334)

**Parameters:**

config ([CwmConfig](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Cwm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CwmModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/modeling_cwm.py#L353)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `CwmModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `CwmModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CwmConfig](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmConfig)) and inputs.

The [CwmModel](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmModel) forward method, overrides the `__call__` special method.

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

## CwmForCausalLM[[transformers.CwmForCausalLM]]

#### transformers.CwmForCausalLM[[transformers.CwmForCausalLM]]

```python
transformers.CwmForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/modeling_cwm.py#L416)

**Parameters:**

config ([CwmForCausalLM](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Cwm Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CwmForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cwm/modeling_cwm.py#L431)

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
elements depending on the configuration ([CwmConfig](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmConfig)) and inputs.

The [CwmForCausalLM](/docs/transformers/v5.15.1/en/model_doc/cwm#transformers.CwmForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, CwmForCausalLM

>>> model = CwmForCausalLM.from_pretrained("meta-cwm/Cwm-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-cwm/Cwm-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
