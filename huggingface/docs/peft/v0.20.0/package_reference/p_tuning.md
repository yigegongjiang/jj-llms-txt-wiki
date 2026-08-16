# P-tuning

    

Prompt tokens can be inserted anywhere in the input sequence, and they are optimized by a prompt encoder (image source).

[P-tuning](https://hf.co/papers/2103.10385) is designed for natural language understanding (NLU) tasks and all language models.

The abstract from the paper is:

*While GPTs with traditional fine-tuning fail to achieve strong results on natural language understanding (NLU), we show that GPTs can be better than or comparable to similar-sized BERTs on NLU tasks with a novel method P-tuning -- which employs trainable continuous prompt embeddings. On the knowledge probing (LAMA) benchmark, the best GPT recovers 64\% (P@1) of world knowledge without any additional text provided during test time, which substantially improves the previous best by 20+ percentage points. On the SuperGlue benchmark, GPTs achieve comparable and sometimes better performance to similar-sized BERTs in supervised learning. Importantly, we find that P-tuning also improves BERTs' performance in both few-shot and supervised settings while largely reducing the need for prompt engineering. Consequently, P-tuning outperforms the state-of-the-art approaches on the few-shot SuperGlue benchmark.*.

The method adds trainable prompt embeddings to the input that is optimized by a prompt encoder to find a better prompt, eliminating the need to manually design prompts. The prompt tokens can be added anywhere in the input sequence, and p-tuning also introduces anchor tokens for improving performance. A prompt encoder (a bidirectional long-short term memory network or LSTM) is used to optimize the prompt parameters. Unlike prefix tuning:

- the prompt tokens can be inserted anywhere in the input sequence, and it isn't restricted to only the beginning
- the prompt tokens are only added to the input instead of adding them to every layer of the model
- introducing *anchor* tokens can improve performance because they indicate characteristics of a component in the input sequence

The paper's results suggest that P-tuning is more efficient than manually crafting prompts, and it enables GPT-like models to compete with BERT-like models on NLU tasks.

## Usage

Create a [PromptEncoderConfig](/docs/peft/v0.20.0/en/package_reference/p_tuning#peft.PromptEncoderConfig) with the task type, the number of virtual tokens to add and learn, and the hidden size of the encoder for learning the prompt parameters.

```py
from peft import PromptEncoderConfig, get_peft_model

peft_config = PromptEncoderConfig(task_type="CAUSAL_LM", num_virtual_tokens=20, encoder_hidden_size=128)
model = get_peft_model(model, peft_config)
model.print_trainable_parameters()
"trainable params: 300,288 || all params: 559,514,880 || trainable%: 0.05366935013417338"
```

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=P_TUNING"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PromptEncoderConfig[[peft.PromptEncoderConfig]]

#### peft.PromptEncoderConfig[[peft.PromptEncoderConfig]]

```python
peft.PromptEncoderConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, num_virtual_tokens: int = None, token_dim: int = None, num_transformer_submodules: Optional[int] = None, num_attention_heads: Optional[int] = None, num_layers: Optional[int] = None, modules_to_save: Optional[list[str]] = None, encoder_reparameterization_type: typing.Union[str, peft.tuners.p_tuning.config.PromptEncoderReparameterizationType] = <PromptEncoderReparameterizationType.MLP: 'MLP'>, encoder_hidden_size: int = None, encoder_num_layers: int = 2, encoder_dropout: float = 0.0)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/p_tuning/config.py#L29)

**Parameters:**

encoder_reparameterization_type (Union[`PromptEncoderReparameterizationType`, `str`]) : The type of reparameterization to use.

encoder_hidden_size (`int`) : The hidden size of the prompt encoder.

encoder_num_layers (`int`) : The number of layers of the prompt encoder.

encoder_dropout (`float`) : The dropout probability of the prompt encoder.

This is the configuration class to store the configuration of a [PromptEncoder](/docs/peft/v0.20.0/en/package_reference/p_tuning#peft.PromptEncoder).

## PromptEncoder[[peft.PromptEncoder]]

#### peft.PromptEncoder[[peft.PromptEncoder]]

```python
peft.PromptEncoder(config)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/p_tuning/model.py#L24)

**Parameters:**

config ([PromptEncoderConfig](/docs/peft/v0.20.0/en/package_reference/p_tuning#peft.PromptEncoderConfig)) : The configuration of the prompt encoder.

The prompt encoder network that is used to generate the virtual token embeddings for p-tuning.

Example:

```py
>>> from peft import PromptEncoder, PromptEncoderConfig

>>> config = PromptEncoderConfig(
...     peft_type="P_TUNING",
...     task_type="SEQ_2_SEQ_LM",
...     num_virtual_tokens=20,
...     token_dim=768,
...     num_transformer_submodules=1,
...     num_attention_heads=12,
...     num_layers=12,
...     encoder_reparameterization_type="MLP",
...     encoder_hidden_size=768,
... )

>>> prompt_encoder = PromptEncoder(config)
```

**Attributes**:
- **embedding** (`torch.nn.Embedding`) -- The embedding layer of the prompt encoder.
- **mlp_head** (`torch.nn.Sequential`) -- The MLP head of the prompt encoder if `inference_mode=False`.
- **lstm_head** (`torch.nn.LSTM`) -- The LSTM head of the prompt encoder if `inference_mode=False` and
`encoder_reparameterization_type="LSTM"`.
- **token_dim** (`int`) -- The hidden embedding dimension of the base transformer model.
- **input_size** (`int`) -- The input size of the prompt encoder.
- **output_size** (`int`) -- The output size of the prompt encoder.
- **hidden_size** (`int`) -- The hidden size of the prompt encoder.
- **total_virtual_tokens** (`int`): The total number of virtual tokens of the
prompt encoder.
- **encoder_type** (Union[`PromptEncoderReparameterizationType`, `str`]): The encoder type of the prompt
  encoder.

Input shape: (`batch_size`, `total_virtual_tokens`)

Output shape: (`batch_size`, `total_virtual_tokens`, `token_dim`)
