# Mamba

[Mamba](https://huggingface.co/papers/2312.00752) is a selective structured state space model (SSMs) designed to work around Transformers computational inefficiency when dealing with long sequences.  It is a completely attention-free architecture, and comprised of a combination of H3 and gated MLP blocks (Mamba block). Mamba's "content-based reasoning" allows it to focus on specific parts of an input depending on the current token. Mamba also uses a new hardware-aware parallel algorithm to compensate for the lack of convolutional operations. As a result, Mamba has fast inference and can scale to very long sequences.

You can find all the original Mamba checkpoints under the [State Space Models](https://huggingface.co/state-spaces) organization.

> [!TIP]
> This model was contributed by [Molbap](https://huggingface.co/Molbap) and [AntonV](https://huggingface.co/AntonV).
> Click on the Mamba models in the right sidebar for more examples of how to apply Mamba to different language tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="text-generation",
    model="state-spaces/mamba-130m-hf",
    device=0
)
pipeline("Plants create energy through a process known as")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("state-spaces/mamba-130m-hf")
model = AutoModelForCausalLM.from_pretrained("state-spaces/mamba-130m-hf", device_map="auto")
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to 4-bit integers.

```python
from torchao.quantization import Int4WeightOnlyConfig

from transformers import AutoModelForCausalLM, AutoTokenizer, TorchAoConfig

quantization_config = Int4WeightOnlyConfig(group_size=128)
quantization_config = TorchAoConfig(quant_type=quant_config)
tokenizer = AutoTokenizer.from_pretrained("state-spaces/mamba-2.8b-hf")
model = AutoModelForCausalLM.from_pretrained("state-spaces/mamba-2.8b-hf", quantization_config=quantization_config, device_map="auto")
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## Notes

- The current implementation uses the original CUDA kernels. The FlashAttention equivalent implementation is hosted in the [mamba-ssm](https://github.com/state-spaces/mamba) and [causal_conv1d](https://github.com/Dao-AILab/causal-conv1d) repositories. Make sure to install them if your hardware supports it!
- Mamba stacks `mixer` layers which are equivalent to `Attention` layers. You can find the main logic of Mamba in the `MambaMixer` class.
- The example below demonstrates how to fine-tune Mamba with [PEFT](https://huggingface.co/docs/peft).

  ```py
  from datasets import load_dataset
  from trl import SFTConfig, SFTTrainer
  from peft import LoraConfig

  model_id = "state-spaces/mamba-130m-hf"
  dataset = load_dataset("Abirate/english_quotes", split="train")
  training_args = SFTConfig(dataset_text_field="quote")
  lora_config =  LoraConfig(target_modules=["x_proj", "embeddings", "in_proj", "out_proj"])
  trainer = SFTTrainer(
      model=model_id,
      args=training_args,
      train_dataset=dataset,
      peft_config=lora_config,
  )
  trainer.train()
   ```

## MambaConfig[[transformers.MambaConfig]]

#### transformers.MambaConfig[[transformers.MambaConfig]]

```python
transformers.MambaConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 50280, hidden_size: int = 768, state_size: int = 16, num_hidden_layers: int = 32, layer_norm_epsilon: float = 1e-05, pad_token_id: int | None = 0, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 0, expand: int = 2, conv_kernel: int = 4, use_bias: bool = False, use_conv_bias: bool = True, hidden_act: str = 'silu', initializer_range: float = 0.1, residual_in_fp32: bool = True, time_step_rank: str | int = 'auto', time_step_scale: float = 1.0, time_step_min: float = 0.001, time_step_max: float = 0.1, time_step_init_scheme: str = 'random', time_step_floor: float = 0.0001, rescale_prenorm_residual: bool = False, use_cache: bool = True, use_mambapy: bool = False, use_associative_scan: bool = True, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba/configuration_mamba.py#L26)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `50280`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

state_size (`int`, *optional*, defaults to `16`) : Size of the SSM state (latent state dimension) in the Mamba layers.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

layer_norm_epsilon (`float`, *optional*, defaults to 1e-05) : The epsilon to use in the layer normalization layers.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `0`) : Token id used for end-of-stream in the vocabulary.

expand (`int`, *optional*, defaults to 2) : Expanding factor used to determine the intermediate size.

conv_kernel (`int`, *optional*, defaults to `4`) : The size of the convolutional kernel.

use_bias (`bool`, *optional*, defaults to `False`) : Whether or not to use bias in ["in_proj", "out_proj"] of the mixer block

use_conv_bias (`bool`, *optional*, defaults to `True`) : Whether or not to use bias in the convolution layer of the mixer block.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.1`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

residual_in_fp32 (`bool`, *optional*, defaults to `True`) : Whether or not residuals should be in `float32`. If set to `False` residuals will keep the same `dtype` as the rest of the model

time_step_rank (`Union[str, int]`, *optional*, defaults to `auto`) : Rank of the delta (time step) projection. Can be `"auto"` to set it automatically.

time_step_scale (`float`, *optional*, defaults to `1.0`) : Scale applied to the time step delta before discretization.

time_step_min (`float`, *optional*, defaults to `0.001`) : Minimum `time_step` used to bound `dt_proj.bias`.

time_step_max (`float`, *optional*, defaults to `0.1`) : Maximum `time_step` used to bound `dt_proj.bias`.

time_step_init_scheme (`str`, *optional*, defaults to `random`) : Initialization scheme for the time step delta. Can be `"random"` or `"uniform"`.

time_step_floor (`float`, *optional*, defaults to `0.0001`) : Minimum allowed value for the discrete time step delta after softplus activation.

rescale_prenorm_residual (`bool`, *optional*, defaults to `False`) : Whether or not to rescale `out_proj` weights when initializing.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

use_mambapy (`bool`, *optional*, defaults to `False`) : Determines the fallback strategy during training if the CUDA-based official implementation of Mamba is not available. If `True`, the mamba.py implementation is used. If `False`, the naive and slower implementation is used. Consider switching to the naive version if memory is limited.

use_associative_scan (`bool`, *optional*, defaults to `True`) : Whether to use PyTorch's `torch._higher_order_ops.associative_scan` for the parallel scan instead of the naive sequential implementation. The associative scan is only active during `torch.compile` tracing and requires torch >= 2.9.0. Both paths are tested to produce numerically identical results (see `test_associative_scan_matches_sequential`). Set to `False` to fall back to the sequential loop.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a MambaModel. It is used to instantiate a Mamba
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [state-spaces/mamba-2.8b](https://huggingface.co/state-spaces/mamba-2.8b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MambaConfig, MambaModel

>>> # Initializing a Mamba configuration
>>> configuration = MambaConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = MambaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MambaModel[[transformers.MambaModel]]

#### transformers.MambaModel[[transformers.MambaModel]]

```python
transformers.MambaModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba/modeling_mamba.py#L614)

**Parameters:**

config ([MambaModel](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Mamba Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MambaModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.LongTensor] = None, cache_params: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, attention_mask: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba/modeling_mamba.py#L639)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

inputs_embeds (`torch.LongTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

cache_params (`Cache`, *optional*) : If passed along, the model uses the previous state in all the blocks (which will give the output for the `input_ids` provided as if the model add `state_input_ids + input_ids` as context).

use_cache (`bool`, *optional*) : If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** `MambaOutput` or `tuple(torch.FloatTensor)`

A `MambaOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MambaConfig](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaConfig)) and inputs.

The [MambaModel](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaModel) forward method, overrides the `__call__` special method.

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

## MambaLMHeadModel[[transformers.MambaForCausalLM]]

#### transformers.MambaForCausalLM[[transformers.MambaForCausalLM]]

```python
transformers.MambaForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba/modeling_mamba.py#L709)

**Parameters:**

config ([MambaForCausalLM](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MAMBA Model transformer with a language modeling head on top (linear layer with weights tied to the input
embeddings).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MambaForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, cache_params: transformers.cache_utils.Cache | None = None, labels: typing.Optional[torch.LongTensor] = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mamba/modeling_mamba.py#L750)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

cache_params (`Cache`, *optional*) : If passed along, the model uses the previous state in all the blocks (which will give the output for the `input_ids` provided as if the model add `state_input_ids + input_ids` as context).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

use_cache (`bool`, *optional*) : If set to `True`, the `cache_params` is returned and can be used to quickly generate the next logits.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `MambaCausalLMOutput` or `tuple(torch.FloatTensor)`

A `MambaCausalLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MambaConfig](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaConfig)) and inputs.

The [MambaForCausalLM](/docs/transformers/v5.15.1/en/model_doc/mamba#transformers.MambaForCausalLM) forward method, overrides the `__call__` special method.

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
