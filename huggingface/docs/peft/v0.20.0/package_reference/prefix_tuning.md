# Prefix tuning

    

Optimize the prefix parameters for each task (image source).

[Prefix tuning](https://hf.co/papers/2101.00190) prefixes a series of task-specific vectors to the input sequence that can be learned while keeping the pretrained model frozen. The prefix parameters are inserted in all of the model layers.

The abstract from the paper is:

*Fine-tuning is the de facto way to leverage large pretrained language models to perform downstream tasks. However, it modifies all the language model parameters and therefore necessitates storing a full copy for each task. In this paper, we propose prefix-tuning, a lightweight alternative to fine-tuning for natural language generation tasks, which keeps language model parameters frozen, but optimizes a small continuous task-specific vector (called the prefix). Prefix-tuning draws inspiration from prompting, allowing subsequent tokens to attend to this prefix as if it were "virtual tokens". We apply prefix-tuning to GPT-2 for table-to-text generation and to BART for summarization. We find that by learning only 0.1\% of the parameters, prefix-tuning obtains comparable performance in the full data setting, outperforms fine-tuning in low-data settings, and extrapolates better to examples with topics unseen during training*.

**Note** For encoder-decoder models (seq2seq), the prefix is only applied to the decoder, which does not correspond to the paper specification (see e.g. Figure 2). Prefix tuning can still be fine-tuned on these model architectures but the performance could be sub-par; consider using other PEFT methods for encoder-decoder models.

Prefix tuning is very similar to [prompt tuning](../package_reference/prompt_tuning). The main difference is that the prefix parameters are inserted in **all** of the model layers, whereas prompt tuning only adds the prompt parameters to the model input embeddings. The prefix parameters are also optimized by a separate feed-forward network (FFN) instead of training directly on the soft prompts because it causes instability and hurts performance. The FFN is discarded after updating the soft prompts.

As a result, the authors found that prefix tuning demonstrates comparable performance to fully finetuning a model, despite having 1000x fewer parameters, and it performs even better in low-data settings.

## Basic Usage

Create a [PrefixTuningConfig](/docs/peft/v0.20.0/en/package_reference/prefix_tuning#peft.PrefixTuningConfig) with the task type and number of virtual tokens to add and learn.

```py
from peft import PrefixTuningConfig, get_peft_model

peft_config = PrefixTuningConfig(task_type="CAUSAL_LM", num_virtual_tokens=20)
model = get_peft_model(model, peft_config)
model.print_trainable_parameters()
"trainable params: 983,040 || all params: 560,197,632 || trainable%: 0.1754809274167014"
```

## Possible Initializations

By default, prefix tuning uses randomly initialized virtual tokens. There's also the option to initialize the vectors
to be close to a no-op (initialized to zero, it will still shift the probability mass a bit).
This means that the KV-cache injected prefixes have less impact from the beginning and reduces the variance in training
performance.

PEFT also provides utilities to initialize a prefix-tuning adapter from an existing KV cache prefix (for example, from
the first `p` tokens of a prompt/corpus). This is only supported when `prefix_projection=False` (the default), because
in that case the learned parameters are the KV prefix itself.

```py
from transformers import AutoModelForCausalLM, AutoTokenizer

from peft import PrefixTuningConfig, get_peft_model, initialize_kv_prefix_from_text

base = AutoModelForCausalLM.from_pretrained("gpt2")
tok = AutoTokenizer.from_pretrained("gpt2")

peft_cfg = PrefixTuningConfig(task_type="CAUSAL_LM", num_virtual_tokens=20, prefix_projection=False)
model = get_peft_model(base, peft_cfg)

initialize_kv_prefix_from_text(
    model,
    tok,
    text="...a long context with at least num_virtual_tokens tokens...",
    use_chat_template=False,
)m peft import PrefixTuningConfig, get_peft_model, initialize_kv_prefix_from_text

base = AutoModelForCausalLM.from_pretrained("gpt2")
tok = AutoTokenizer.from_pretrained("gpt2")

peft_cfg = PrefixTuningConfig(task_type="CAUSAL_LM", num_virtual_tokens=20, prefix_projection=False)
model = get_peft_model(base, peft_cfg)

initialize_kv_prefix_from_text(
    model,
    tok,
    text="...a long context with at least num_virtual_tokens tokens...",
    use_chat_template=False,
)

```

Make sure the text is long enough to produce at least `num_virtual_tokens` tokens, otherwise initialization will fail.

As a guideline:

* start with a neutral starting sequence using `initialize_kv_prefix_from_text`, it can be a very short string like
  "Question: "
* if that doesn't help, use a longer sequence with task relevance (i.e. an engineered prompt), giving you more virtual
  tokens to fit but also more steering of the model
* if it is not possible to use an initialization text or you want to quickly check if prefix tuning is viable at all,
  use a zero init without projection

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=PREFIX_TUNING"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PrefixTuningConfig[[peft.PrefixTuningConfig]]

- **init_weights** (`Optional[str]`) -- If not set, weights are initialized at random, if set to "zero"
  the weights are initialized so that the activations will be a no-op (zero).
- **encoder_hidden_size** (`int`) -- The hidden size of the prompt encoder.
- **prefix_projection** (`bool`) -- Whether to project the prefix embeddings.

This is the configuration class to store the configuration of a [PrefixEncoder](/docs/peft/v0.20.0/en/package_reference/prefix_tuning#peft.PrefixEncoder).

## PrefixEncoder[[peft.PrefixEncoder]]

- **config** ([PrefixTuningConfig](/docs/peft/v0.20.0/en/package_reference/prefix_tuning#peft.PrefixTuningConfig)) -- The configuration of the prefix encoder.

The `torch.nn` model to encode the prefix.

Example:

```py
>>> from peft import PrefixEncoder, PrefixTuningConfig

>>> config = PrefixTuningConfig(
...     peft_type="PREFIX_TUNING",
...     task_type="SEQ_2_SEQ_LM",
...     num_virtual_tokens=20,
...     token_dim=768,
...     num_transformer_submodules=1,
...     num_attention_heads=12,
...     num_layers=12,
...     encoder_hidden_size=768,
... )
>>> prefix_encoder = PrefixEncoder(config)
```

**Attributes**:
- **embedding** (`torch.nn.Embedding`) -- The embedding layer of the prefix encoder.
- **transform** (`torch.nn.Sequential`) -- The two-layer MLP to transform the prefix embeddings if
  `prefix_projection` is `True`.
- **prefix_projection** (`bool`) -- Whether to project the prefix embeddings.

Input shape: (`batch_size`, `num_virtual_tokens`)

Output shape: (`batch_size`, `num_virtual_tokens`, `2*layers*hidden`)

Load the flattened prompt embeddings saved by PEFT (`prompt_embeddings`).

For prefix tuning, this is only supported when `prefix_projection=False`, because in that case the learned
parameters are the KV prefix itself (`embedding.weight` has shape `[num_virtual_tokens,
num_layers*2*token_dim]`).

If `prefix_projection=True`, the parameters are (virtual token embeddings + an MLP) and there is no general way
to invert the projection to recover those parameters from a flattened KV prefix.
