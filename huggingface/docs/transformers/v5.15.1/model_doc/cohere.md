# Cohere

Cohere [Command-R](https://cohere.com/blog/command-r) is a 35B parameter multilingual large language model designed for long context tasks like retrieval-augmented generation (RAG) and calling external APIs and tools. The model is specifically trained for grounded generation and supports both single-step and multi-step tool use. It supports a context length of 128K tokens.

You can find all the original Command-R checkpoints under the [Command Models](https://huggingface.co/collections/CohereForAI/command-models-67652b401665205e17b192ad) collection.

> [!TIP]
> Click on the Cohere models in the right sidebar for more examples of how to apply Cohere to different language tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="text-generation",
    model="CohereForAI/c4ai-command-r-v01",
    device=0
)
pipeline("Plants create energy through a process known as")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("CohereForAI/c4ai-command-r-v01")
model = AutoModelForCausalLM.from_pretrained("CohereForAI/c4ai-command-r-v01", device_map="auto", attn_implementation="sdpa")

# format message with the Command-R chat template
messages = [{"role": "user", "content": "How do plants make energy?"}]
input_ids = tokenizer.apply_chat_template(messages, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)
output = model.generate(
    input_ids,
    max_new_tokens=100,
    do_sample=True,
    temperature=0.3,
    cache_implementation="static",
)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

```bash
# pip install -U flash-attn --no-build-isolation
transformers chat CohereForAI/c4ai-command-r-v01 --dtype auto --attn_implementation flash_attention_2
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to quantize the weights to 4-bits.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

bnb_config = BitsAndBytesConfig(load_in_4bit=True)
tokenizer = AutoTokenizer.from_pretrained("CohereForAI/c4ai-command-r-v01")
model = AutoModelForCausalLM.from_pretrained("CohereForAI/c4ai-command-r-v01", device_map="auto", quantization_config=bnb_config, attn_implementation="sdpa")

# format message with the Command-R chat template
messages = [{"role": "user", "content": "How do plants make energy?"}]
input_ids = tokenizer.apply_chat_template(messages, tokenize=True, add_generation_prompt=True, return_tensors="pt").to(model.device)
output = model.generate(
    input_ids,
    max_new_tokens=100,
    do_sample=True,
    temperature=0.3,
    cache_implementation="static",
)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

Use the [AttentionMaskVisualizer](https://github.com/huggingface/transformers/blob/beb9b5b02246b9b7ee81ddf938f93f44cfeaad19/src/transformers/utils/attention_visualizer.py#L139) to better understand what tokens the model can and cannot attend to.

```python
from transformers.utils.attention_visualizer import AttentionMaskVisualizer

visualizer = AttentionMaskVisualizer("CohereForAI/c4ai-command-r-v01")
visualizer("Plants create energy through a process known as")
```

    

## Notes

- Don't use the dtype parameter in [from_pretrained()](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel.from_pretrained) if you're using FlashAttention-2 because it only supports fp16 or bf16. You should use [Automatic Mixed Precision](https://pytorch.org/tutorials/recipes/recipes/amp_recipe.html), set fp16 or bf16 to True if using [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer), or use [torch.autocast](https://pytorch.org/docs/stable/amp.html#torch.autocast).

## CohereConfig[[transformers.CohereConfig]]

#### transformers.CohereConfig[[transformers.CohereConfig]]

```python
transformers.CohereConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 256000, hidden_size: int = 8192, intermediate_size: int = 22528, logit_scale: float | None = 0.0625, num_hidden_layers: int = 40, num_attention_heads: int = 64, num_key_value_heads: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 8192, initializer_range: float = 0.02, layer_norm_eps: float | None = 1e-05, use_cache: bool = True, pad_token_id: int | None = 0, bos_token_id: int | None = 5, eos_token_id: int | list[int] | None = 255001, tie_word_embeddings: bool = True, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | int | None = 0.0, use_qk_norm: bool | None = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/configuration_cohere.py#L30)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `256000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `8192`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `22528`) : Dimension of the MLP representations.

logit_scale (`float`, *optional*, defaults to 0.0625) : The scaling factor for the output logits.

num_hidden_layers (`int`, *optional*, defaults to `40`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `64`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `8192`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `5`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `255001`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

use_qk_norm (`bool`, *optional*, defaults to `False`) : Whether to use query-key normalization in the attention.

This is the configuration class to store the configuration of a CohereModel. It is used to instantiate a Cohere
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [CohereForAI/c4ai-command-r-v01](https://huggingface.co/CohereForAI/c4ai-command-r-v01)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import CohereModel, CohereConfig

>>> # Initializing a Cohere model configuration
>>> configuration = CohereConfig()

>>> # Initializing a model from the Cohere configuration
>>> model = CohereModel(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## CohereTokenizer[[transformers.CohereTokenizer]]

#### transformers.CohereTokenizer[[transformers.CohereTokenizer]]

```python
transformers.CohereTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, errors: str = 'replace', unk_token: str = '<UNK>', bos_token: str = '<BOS_TOKEN>', eos_token: str = '<|END_OF_TURN_TOKEN|>', pad_token: str = '<PAD>', cls_token: str = '<CLS>', sep_token: str = '<SEP>', mask_token: str = '<MASK_TOKEN>', use_default_system_prompt: bool = False, add_prefix_space: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/tokenization_cohere.py#L45)

**Parameters:**

vocab_file (`str`, *optional*) : Path to the vocabulary file.

merges_file (`str`, *optional*) : Path to the merges file.

tokenizer_file (`str`, *optional*) : [tokenizers](https://github.com/huggingface/tokenizers) file (generally has a .json extension) that contains everything needed to load the tokenizer.

clean_up_tokenization_spaces (`bool`, *optional*, defaults to `False`) : Whether or not to cleanup spaces after decoding, cleanup consists in removing potential artifacts like extra spaces.

unk_token (`str` or `tokenizers.AddedToken`, *optional*, defaults to `"<UNK>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

bos_token (`str` or `tokenizers.AddedToken`, *optional*, defaults to `"<BOS_TOKEN>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.

eos_token (`str` or `tokenizers.AddedToken`, *optional*, defaults to `"<|END_OF_TURN_TOKEN|>"`) : The end of sequence token.

add_bos_token (`bool`, *optional*, defaults to `True`) : Whether or not to add an `bos_token` at the start of sequences.

add_eos_token (`bool`, *optional*, defaults to `False`) : Whether or not to add an `eos_token` at the end of sequences.

use_default_system_prompt (`bool`, *optional*, defaults to `False`) : Whether or not the default system prompt for Cohere tokenizer should be used.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not the tokenizer should automatically add a prefix space

vocab (`str`, `dict` or `list`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from vocab_file.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

Construct a Cohere tokenizer. Based on byte-level Byte-Pair-Encoding.

This uses notably ByteFallback and NFC normalization.

```python
>>> from transformers import AutoTokenizer

>>> tokenizer = AutoTokenizer.from_pretrained("CohereForAI/c4ai-command-r-v01")
>>> tokenizer.encode("Hello this is a test")
[5, 28339, 2075, 1801, 1671, 3282]
```

If you want to change the `bos_token` or the `eos_token`, make sure to specify them when initializing the model, or
call `tokenizer.update_post_processor()` to make sure that the post-processing is correctly done (otherwise the
values of the first token and final token of an encoded sequence will not be correct). For more details, checkout
[post-processors] (https://huggingface.co/docs/tokenizers/api/post-processors) documentation.

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer, but since
the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer needs to be instantiated with `add_prefix_space=True`.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### apply_grounded_generation_template[[transformers.CohereTokenizer.apply_grounded_generation_template]]

```python
apply_grounded_generation_template(conversation: list, documents: list, citation_mode: typing.Literal['fast', 'accurate'] = 'accurate', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/tokenization_cohere.py#L297)

**Parameters:**

conversation (list[dict[str, str]]) : A list of dicts with "role" and "content" keys, representing the chat history so far.

documents (list[dict[str, str]) : A list of dicts, representing documents or tool outputs to ground your generation on. A document is a semistructured dict, with a string to string mapping. Common fields are `url`, `title`, `snippet` etc but should be descriptive of the key. They will get rendered into the prompt.

citation_mode : either "accurate" (prompt the model to generate an answer first, then rewrite it with citation spans in) or "fast", where the prompt instructs the model to generate an answer with citations in directly. The former has higher quality citations, the latter requires fewer tokens to be generated.

add_generation_prompt (bool, *optional*) : Whether to end the prompt with the token(s) that indicate the start of an assistant message. This is useful when you want to generate a response from the model. Note that this argument will be passed to the chat template, and so it must be supported in the template for this argument to have any effect.

tokenize (`bool`, defaults to `True`) : Whether to tokenize the output. If `False`, the output will be a string.

padding (`bool`, defaults to `False`) : Whether to pad sequences to the maximum length. Has no effect if tokenize is `False`.

truncation (`bool`, defaults to `False`) : Whether to truncate sequences at the maximum length. Has no effect if tokenize is `False`.

max_length (`int`, *optional*) : Maximum length (in tokens) to use for padding or truncation. Has no effect if tokenize is `False`. If not specified, the tokenizer's `max_length` attribute will be used as a default.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Has no effect if tokenize is `False`. Acceptable values are: - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

return_dict (`bool`, *optional*, defaults to `False`) : Whether to return a dictionary with named outputs. Has no effect if tokenize is `False`.

- ****tokenizer_kwargs** : Additional kwargs to pass to the tokenizer.

**Returns:** `str`

A rendered prompt string.
or if tokenize=True:
`list[int]`: A list of token ids representing the tokenized chat so far, including control tokens. This
output is ready to pass to the model, either directly or via methods like `generate()`.

Create a Command-R grounded generation (aka RAG) prompt.

Once rendered, the prompt instructs the model to generate a response with citations in, based on supplied documents.

Conceptually, this works in the same way as `apply_chat_format`, but takes additional `documents`
and parameter `citation_mode` parameters.

Converts a list of dictionaries with `"role"` and `"content"` keys and a list of
documents for the model to ground its response on into a prompt string, or a list of token ids.
This method will use the tokenizer's `grounded_generation_template` template specified at the class level.
You can override the default template using the `grounded_generation_template` kwarg but the quality of your results may decrease.

Examples:

```python
>> tokenizer = CohereTokenizer.from_pretrained('CohereForAI/c4ai-command-r-v01')

>> # define documents:
>> documents = [
    { "title": "Tall penguins", "text": "Emperor penguins are the tallest." },
    { "title": "Penguin habitats", "text": "Emperor penguins only live in Antarctica."}
]
>> # define a conversation:
>> conversation = [
    {"role": "user", "content": "What's the biggest penguin in the world?"}
]
>> # render the prompt, ready for user to inspect, or for input into the model:
>> grounded_generation_prompt = tokenizer.apply_grounded_generation_template(conversation, documents=documents, tokenize=False, add_generation_prompt=True)
>> print(grounded_generation_prompt)
>> inputs = tokenizer.encode(prompt, add_special_tokens=False, return_tensors='pt')
>> outputs = model.generate(inputs, max_new_tokens=128)
>> print(tokenizer.decode(outputs[0]))
```

#### apply_tool_use_template[[transformers.CohereTokenizer.apply_tool_use_template]]

```python
apply_tool_use_template(conversation: list, tools: list, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/tokenization_cohere.py#L186)

**Parameters:**

conversation (list[dict[str, str]]) : A list of dicts with "role" and "content" keys, representing the chat history so far.

tools (list[Dict]) : a list of tools to render into the prompt for the model to choose from. See an example at the bottom of the docstring. The format should be: * name (str): The name of the tool to be called. Valid names contain only the characters a-z, A-Z, 0-9, _ and must not begin with a digit. * description (str): The description of what the tool does, the model uses the description to choose when and how to call the function. * parameter_definitions (list[Dict]): The input parameters of the tool. Accepts a dictionary where the key is the name of the parameter and the value is the parameter spec. Valid parameter names contain only the characters a-z, A-Z, 0-9, _ and must not begin with a digit. Parameter specs are as follows: * description (str): The description of the parameter. * type (str): the type of the parameter - most effective for python builtin data types, such as 'str', 'bool' * required: boolean: Denotes whether the parameter is always present (required) or not. Defaults to not required.

add_generation_prompt (bool, *optional*) : Whether to end the prompt with the token(s) that indicate the start of an assistant message. This is useful when you want to generate a response from the model. Note that this argument will be passed to the chat template, and so it must be supported in the template for this argument to have any effect.

tokenize (`bool`, defaults to `True`) : Whether to tokenize the output. If `False`, the output will be a string.

padding (`bool`, defaults to `False`) : Whether to pad sequences to the maximum length. Has no effect if tokenize is `False`.

truncation (`bool`, defaults to `False`) : Whether to truncate sequences at the maximum length. Has no effect if tokenize is `False`.

max_length (`int`, *optional*) : Maximum length (in tokens) to use for padding or truncation. Has no effect if tokenize is `False`. If not specified, the tokenizer's `max_length` attribute will be used as a default.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Has no effect if tokenize is `False`. Acceptable values are: - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

return_dict (`bool`, *optional*, defaults to `False`) : Whether to return a dictionary with named outputs. Has no effect if tokenize is `False`.

- ****tokenizer_kwargs** : Additional kwargs to pass to the tokenizer.

**Returns:** `str`

A rendered prompt string.
or if tokenize=True:
`list[int]`: A list of token ids representing the tokenized chat so far, including control tokens. This
output is ready to pass to the model, either directly or via methods like `generate()`.

Create a Command-R tool-use prompt.

Once rendered, the prompt instructs the model to generate a list of actions to perform on a set of user supplied tools
to help carry out the user's requests.

Conceptually, this works in the same way as `apply_chat_format`, but takes an additional `tools` parameter.

Converts a chat in the form of a list of dictionaries with `"role"` and `"content"` keys and a list of available
tools for the model to use into a prompt string, or a list of token ids.
This method will use the tokenizer's `default_tool_use_template` template specified at the class level.
You can override the default template using the `tool_use_template` kwarg but the quality of your results may decrease.

Examples:

```python
>> tokenizer = CohereTokenizer.from_pretrained("CohereForAI/c4ai-command-r-v01")
>> tools = [
    {
        "name": "internet_search",
        "description": "Returns a list of relevant document snippets for a textual query retrieved from the internet",
        "parameter_definitions": {
            "query": {
                "description": "Query to search the internet with",
                "type": "str",
                "required": True,
            }
        },
    },
    {
        "name": "directly_answer",
        "description": "Calls a standard (un-augmented) AI chatbot to generate a response given the conversation history",
        "parameter_definitions": {},
    },
]
>> conversation = [
    {"role": "user", "content": "What's the biggest penguin in the world?"},
]
>> # Render the prompt, ready for user to inspect, or for input into the model
>> prompt = tokenizer.apply_tool_use_template(conversation, tools=tools, tokenize=False, add_generation_prompt=True)
>> print(prompt)
>> inputs = tokenizer.encode(grounded_generation_prompt, add_special_tokens=False, return_tensors='pt')
>> outputs = model.generate(inputs, max_new_tokens=128)
>> print(tokenizer.decode(outputs[0]))
[
    {
        "tool_name": "internet_search",
        "parameters": {
            "query": "biggest penguin in the world"
        }
    }
]
```

## CohereModel[[transformers.CohereModel]]

#### transformers.CohereModel[[transformers.CohereModel]]

```python
transformers.CohereModel(config: CohereConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/modeling_cohere.py#L368)

**Parameters:**

config ([CohereConfig](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Cohere Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CohereModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/modeling_cohere.py#L385)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CohereConfig](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereConfig)) and inputs.

The [CohereModel](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereModel) forward method, overrides the `__call__` special method.

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

## CohereForCausalLM[[transformers.CohereForCausalLM]]

#### transformers.CohereForCausalLM[[transformers.CohereForCausalLM]]

```python
transformers.CohereForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/modeling_cohere.py#L442)

**Parameters:**

config ([CohereForCausalLM](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Cohere Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.CohereForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/cohere/modeling_cohere.py#L459)

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
elements depending on the configuration ([CohereConfig](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereConfig)) and inputs.

The [CohereForCausalLM](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereForCausalLM) forward method, overrides the `__call__` special method.

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
>> from transformers import AutoTokenizer, CohereForCausalLM

>> model = CohereForCausalLM.from_pretrained("CohereForAI/c4ai-command-r-v01")
>> tokenizer = AutoTokenizer.from_pretrained("CohereForAI/c4ai-command-r-v01")

>> prompt = "Hey, are you conscious? Can you talk to me?"
>> inputs = tokenizer(prompt, return_tensors="pt")

>> # Generate
>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
