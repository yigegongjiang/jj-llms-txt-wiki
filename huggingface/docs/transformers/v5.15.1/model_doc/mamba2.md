# Mamba 2

[Mamba 2](https://huggingface.co/papers/2405.21060) is based on the state space duality (SSD) framework which connects structured state space models (SSMs) and attention variants. It uses a more efficient SSD algorithm that is 2-8x faster than Mamba and modifies the architecture to enable tensor parallelism and a grouped-value attention (GVA) head structure.

You can find all the original Mamba 2 checkpoints under the [State Space Models](https://huggingface.co/state-spaces) organization, but the examples shown below use [mistralai/Mamba-Codestral-7B-v0.1](https://huggingface.co/mistralai/Mamba-Codestral-7B-v0.1) because a Hugging Face implementation isn't supported yet for the original checkpoints.

Other Mamba 2-based architectures include [Bamba](./bamba), [FalconH1](./falcon_h1), and [Zamba2](./zamba2).

> [!TIP]
> This model was contributed by [ArthurZ](https://huggingface.co/ArthurZ).
> Click on the Mamba models in the right sidebar for more examples of how to apply Mamba to different language tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="text-generation",
    model="mistralai/Mamba-Codestral-7B-v0.1",
    device=0
)
pipeline("Plants create energy through a process known as")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("mistralai/Mamba-Codestral-7B-v0.1")
model = AutoModelForCausalLM.from_pretrained("mistralai/Mamba-Codestral-7B-v0.1", device_map="auto")
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to 4-bit integers.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, TorchAoConfig

quantization_config = TorchAoConfig("int4_weight_only", group_size=128)
tokenizer = AutoTokenizer.from_pretrained("mistralai/Mamba-Codestral-7B-v0.1")
model = AutoModelForCausalLM.from_pretrained("mistralai/Mamba-Codestral-7B-v0.1", quantization_config=quantization_config, device_map="auto")
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## Notes

- Codestral Mamba has `groups=8` which are similar to the number of kv heads in an attention-based model.
- Codestral Mamba has two different forward passes, `torch_forward` or `cuda_kernels_forward`, and their results are expected to be slightly different.
  - `torch_forward` without compilation is 3-4x faster than `cuda_kernels_forward`.
  - `cuda_kernels_forward` uses the original CUDA kernels if they're available in your environment. It is slower during prefill because it requires a "warmup run" due to the higher CPU overhead (see [these](https://github.com/state-spaces/mamba/issues/389#issuecomment-2171755306) [comments](https://github.com/state-spaces/mamba/issues/355#issuecomment-2147597457) for more details).

- There are no positional embeddings in this model, but there is an `attention_mask` and a specific logic to mask out hidden states in two places in the case of batched generation (see this [comment](https://github.com/state-spaces/mamba/issues/66#issuecomment-1863563829) for more details). This (and the addition of the reimplemented Mamba 2 kernels) results in a slight discrepancy between batched and cached generation.

- The SSM algorithm heavily relies on tensor contractions, which have matmul equivalents but the order of operations is slightly different. This makes the difference greater at smaller precisions.

- Hidden states that correspond to padding tokens is shutdown in 2 places and is mostly tested with left-padding. Right-padding propagates noise down the line and is not guaranteed to yield satisfactory results. `tokenizer.padding_side = "left"` ensures you are using the correct padding side.

- The example below demonstrates how to fine-tune Mamba 2 with [PEFT](https://huggingface.co/docs/peft).

```python
from datasets import load_dataset
from peft import LoraConfig
from trl import SFTConfig, SFTTrainer

model_id = "mistralai/Mamba-Codestral-7B-v0.1"
dataset = load_dataset("Abirate/english_quotes", split="train")
training_args = SFTConfig(dataset_text_field="quote", gradient_checkpointing=True, per_device_train_batch_size=4)
lora_config =  LoraConfig(target_modules=["x_proj", "embeddings", "in_proj", "out_proj"])
trainer = SFTTrainer(
    model=model_id,
    args=training_args,
    train_dataset=dataset,
    peft_config=lora_config,
)
trainer.train()
```

## Mamba2Config[[transformers.Mamba2Config]]

#### transformers.Mamba2Config[[transformers.Mamba2Config]]

```python
transformers.Mamba2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_heads: int = 128, head_dim: int = 64, vocab_size: int = 32768, hidden_size: int = 4096, state_size: int = 128, num_hidden_layers: int = 64, layer_norm_epsilon: float = 1e-05, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, expand: int = 2, conv_kernel: int = 4, n_groups: int = 8, use_bias: bool = False, use_conv_bias: bool = True, hidden_act: str = 'silu', initializer_range: float = 0.1, residual_in_fp32: bool = True, time_step_rank: str | int = 'auto', time_step_min: float = 0.001, time_step_max: float = 0.1, time_step_floor: float = 0.0001, time_step_limit: list[float] | tuple[float, ...] = (0.0, inf), rescale_prenorm_residual: bool = False, use_cache: bool = True, chunk_size: int = 256, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba2/configuration_mamba2.py#L26)

**Parameters:**

num_heads (`int`, *optional*, defaults to `128`) : Number of attention heads for each attention layer in the Transformer decoder.

head_dim (`int`, *optional*, defaults to `64`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

vocab_size (`int`, *optional*, defaults to `32768`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

state_size (`int`, *optional*, defaults to `128`) : Size of the SSM state (latent state dimension) in the Mamba layers.

num_hidden_layers (`int`, *optional*, defaults to `64`) : Number of hidden layers in the Transformer decoder.

layer_norm_epsilon (`float`, *optional*, defaults to 1e-05) : The epsilon to use in the layer normalization layers..

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

expand (`int`, *optional*, defaults to 2) : Expanding factor used to determine the intermediate size.

conv_kernel (`int`, *optional*, defaults to `4`) : The size of the convolutional kernel.

n_groups (`int`, *optional*, defaults to 8) : Number of groups for the evolution matrices of mamba 2.

use_bias (`bool`, *optional*, defaults to `False`) : Whether or not to use bias in ["in_proj", "out_proj"] of the mixer block

use_conv_bias (`bool`, *optional*, defaults to `True`) : Whether or not to use bias in the convolution layer of the mixer block.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.1`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

residual_in_fp32 (`bool`, *optional*, defaults to `True`) : Whether or not residuals should be in `float32`. If set to `False` residuals will keep the same `dtype` as the rest of the model

time_step_rank (`Union[str, int]`, *optional*, defaults to `auto`) : Rank of the delta (time step) projection. Can be `"auto"` to set it automatically.

time_step_min (`float`, *optional*, defaults to `0.001`) : Minimum `time_step` used to bound `dt_proj.bias`.

time_step_max (`float`, *optional*, defaults to `0.1`) : Maximum `time_step` used to bound `dt_proj.bias`.

time_step_floor (`float`, *optional*, defaults to `0.0001`) : Minimum allowed value for the discrete time step delta after softplus activation.

time_step_limit (`Union[list[float], tuple[float, ...]]`, *optional*, defaults to `(0.0, inf)`) : Accepted range of time step values for clamping.

rescale_prenorm_residual (`bool`, *optional*, defaults to `False`) : Whether or not to rescale `out_proj` weights when initializing.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

chunk_size (`int`, *optional*, defaults to 256) : Size of the chunks that will comprise the sequence.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Mamba2Model. It is used to instantiate a Mamba2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [state-spaces/mamba2-2.8b](https://huggingface.co/state-spaces/mamba2-2.8b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Mamba2Config, Mamba2Model

>>> # Initializing a Mamba2 configuration
>>> configuration = Mamba2Config()

>>> # Initializing a model (with random weights) from the configuration
>>> model = Mamba2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Mamba2Model[[transformers.Mamba2Model]]

#### transformers.Mamba2Model[[transformers.Mamba2Model]]

```python
transformers.Mamba2Model(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba2/modeling_mamba2.py#L720)

**Parameters:**

config ([Mamba2Model](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2Model)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Mamba2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Mamba2Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.LongTensor] = None, cache_params: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba2/modeling_mamba2.py#L745)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

inputs_embeds (`torch.LongTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

cache_params (`Cache`, *optional*) : If passed along, the model uses the previous state in all the blocks (which will give the output for the `input_ids` provided as if the model add `state_input_ids + input_ids` as context).

use_cache (`bool`, *optional*) : If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** `Mamba2Output` or `tuple(torch.FloatTensor)`

A `Mamba2Output` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mamba2Config](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2Config)) and inputs.

The [Mamba2Model](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **cache_params** (`~cache_utils.Cache`, *optional*) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.

  Includes both the State space model state matrices after the selective scan, and the Convolutional states
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

## Mamba2LMHeadModel[[transformers.Mamba2ForCausalLM]]

#### transformers.Mamba2ForCausalLM[[transformers.Mamba2ForCausalLM]]

```python
transformers.Mamba2ForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba2/modeling_mamba2.py#L822)

**Parameters:**

config ([Mamba2ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2ForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MAMBA2 Model transformer with a language modeling head on top (linear layer with weights not tied to the input
embeddings).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Mamba2ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, cache_params: transformers.cache_utils.Cache | None = None, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, use_cache: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba2/modeling_mamba2.py#L863)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

cache_params (`Cache`, *optional*) : If passed along, the model uses the previous state in all the blocks (which will give the output for the `input_ids` provided as if the model add `state_input_ids + input_ids` as context).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

use_cache (`bool`, *optional*) : If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `Mamba2CausalLMOutput` or `tuple(torch.FloatTensor)`

A `Mamba2CausalLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mamba2Config](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2Config)) and inputs.

The [Mamba2ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/mamba2#transformers.Mamba2ForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **cache_params** (`~cache_utils.Cache`, *optional*) -- The state of the model at the last time step. Can be used in a forward method with the next `input_ids` to
  avoid providing the old `input_ids`.

  Includes both the State space model state matrices after the selective scan, and the Convolutional states
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Example:

```python
```
