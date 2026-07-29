# Prompt tuning

    

Only train and store a significantly smaller set of task-specific prompt parameters (image source).

[Prompt tuning](https://hf.co/papers/2104.08691) adds a task-specific, virtual prompt to the input that consists of trainable vectors in the embedding space. The virtual token parameters are updated independently of the pretrained model parameters which are frozen.

The abstract from the paper is:

*In this work, we explore "prompt tuning", a simple yet effective mechanism for learning "soft prompts" to condition frozen language models to perform specific downstream tasks. Unlike the discrete text prompts used by GPT-3, soft prompts are learned through backpropagation and can be tuned to incorporate signal from any number of labeled examples. Our end-to-end learned approach outperforms GPT-3's "few-shot" learning by a large margin. More remarkably, through ablations on model size using T5, we show that prompt tuning becomes more competitive with scale: as models exceed billions of parameters, our method "closes the gap" and matches the strong performance of model tuning (where all model weights are tuned). This finding is especially relevant in that large models are costly to share and serve, and the ability to reuse one frozen model for multiple downstream tasks can ease this burden. Our method can be seen as a simplification of the recently proposed "prefix tuning" of Li and Liang (2021), and we provide a comparison to this and other similar approaches. Finally, we show that conditioning a frozen model with soft prompts confers benefits in robustness to domain transfer, as compared to full model tuning*.

In contrast to [prefix tuning](../package_reference/prefix_tuning), only the
input of the first layer receives the virtual tokens.

## Usage

There are two decisions to take: how many virtual tokens are added to the
input of the model (`num_virtual_tokens`) - this will define how many
trainable parameters there will be - and how these tokens are initialized.

Create a [PromptTuningConfig](/docs/peft/v0.20.0/en/package_reference/prompt_tuning#peft.PromptTuningConfig) with the task type, the initial prompt tuning text to train the model with, the number of virtual tokens to add and learn, and a tokenizer.

```py
from peft import PromptTuningConfig, PromptTuningInit, get_peft_model

prompt_tuning_init_text = "Classify if the tweet is a complaint or no complaint.\n"
peft_config = PromptTuningConfig(
    task_type="CAUSAL_LM",
    prompt_tuning_init=PromptTuningInit.TEXT,
    num_virtual_tokens=len(tokenizer(prompt_tuning_init_text)["input_ids"]),
    prompt_tuning_init_text=prompt_tuning_init_text,
    tokenizer_name_or_path="bigscience/bloomz-560m",
)
model = get_peft_model(model, peft_config)
model.print_trainable_parameters()
"trainable params: 8,192 || all params: 559,222,784 || trainable%: 0.0014648902430985358"
```

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=PROMPT_TUNING"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PromptTuningConfig[[peft.PromptTuningConfig]]

"}, {"name": "prompt_tuning_init_text", "val": ": typing.Optional[str] = None"}, {"name": "tokenizer_name_or_path", "val": ": typing.Optional[str] = None"}, {"name": "tokenizer_kwargs", "val": ": typing.Optional[dict] = None"}]}>
- **prompt_tuning_init** (Union[`PromptTuningInit`, `str`]) --
  The initialization of the prompt embedding. `TEXT` will initialize with your text. `SAMPLE_VOCAB` will
  initialize with randomly sampled tokens from the model's vocabulary. `RANDOM` will initialize with randomly
  sampled continuous, soft tokens (warning: sampled soft tokens may fall outside of embedding manifold)
- **prompt_tuning_init_text** (`str`, *optional*) --
  The text to initialize the prompt embedding. Only used if `prompt_tuning_init` is `TEXT`.
- **tokenizer_name_or_path** (`str`, *optional*) --
  The name or path of the tokenizer. Only used if `prompt_tuning_init` is `TEXT`.
- **tokenizer_kwargs** (`dict`, *optional*) --
  The keyword arguments to pass to `AutoTokenizer.from_pretrained`. Only used if `prompt_tuning_init` is
  `TEXT`.

This is the configuration class to store the configuration of a [PromptEmbedding](/docs/peft/v0.20.0/en/package_reference/prompt_tuning#peft.PromptEmbedding).

## PromptEmbedding[[peft.PromptEmbedding]]

- **config** ([PromptTuningConfig](/docs/peft/v0.20.0/en/package_reference/prompt_tuning#peft.PromptTuningConfig)) -- The configuration of the prompt embedding.
- **word_embeddings** (`torch.nn.Module`) -- The word embeddings of the base transformer model.

The model to encode virtual tokens into prompt embeddings.

**Attributes**:
- **embedding** (`torch.nn.Embedding`) -- The embedding layer of the prompt embedding.

Example:

```py
>>> from peft import PromptEmbedding, PromptTuningConfig

>>> config = PromptTuningConfig(
...     peft_type="PROMPT_TUNING",
...     task_type="SEQ_2_SEQ_LM",
...     num_virtual_tokens=20,
...     token_dim=768,
...     num_transformer_submodules=1,
...     num_attention_heads=12,
...     num_layers=12,
...     prompt_tuning_init="TEXT",
...     prompt_tuning_init_text="Predict if sentiment of this review is positive, negative or neutral",
...     tokenizer_name_or_path="t5-base",
... )

>>> # t5_model.shared is the word embeddings of the base model
>>> prompt_embedding = PromptEmbedding(config, t5_model.shared)
```

Input Shape: (`batch_size`, `total_virtual_tokens`)

Output Shape: (`batch_size`, `total_virtual_tokens`, `token_dim`)
