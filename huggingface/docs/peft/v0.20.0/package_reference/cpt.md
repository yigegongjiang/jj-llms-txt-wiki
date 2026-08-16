# Context-aware Prompt Tuning: Advancing In-Context Learning with Adversarial Methods

    

CPT optimizing only specific token embeddings while keeping the rest of the model frozen (image source).

[Context-Aware Prompt Tuning (CPT)](https://huggingface.co/papers/2410.17222) is designed to enhance few-shot classification by refining only context embeddings.
This approach combines ideas from In-Context Learning (ICL), [Prompt Tuning](../package_reference/prompt_tuning) (PT), and adversarial optimization, focusing on making model adaptation both parameter-efficient and effective.
In CPT, only specific context token embeddings are optimized, while the rest of the model remains frozen.
To prevent overfitting and maintain stability, CPT uses controlled perturbations to limit the allowed changes to context embeddings within a defined range.
Additionally, to address the phenomenon of recency bias—where examples near the end of the context tend to be prioritized over earlier ones—CPT applies a decay loss factor.

The abstract from the paper is:

> Large Language Models (LLMs) can perform few-shot learning using either optimization-based approaches or In-Context Learning (ICL). Optimization-based methods often suffer from overfitting, as they require updating a large number of parameters with limited data. In contrast, ICL avoids overfitting but typically underperforms compared to optimization-based methods and is highly sensitive to the selection, order, and format of demonstration examples. To overcome these challenges, we introduce Context-aware Prompt Tuning (CPT), a method inspired by ICL, Prompt Tuning (PT), and adversarial attacks. CPT builds on the ICL strategy of concatenating examples before the input, extending it by incorporating PT-like learning to refine the context embedding through iterative optimization, extracting deeper insights from the training examples. Our approach carefully modifies specific context tokens, considering the unique structure of the examples within the context. In addition to updating the context with PT-like optimization, CPT draws inspiration from adversarial attacks, adjusting the input based on the labels present in the context while preserving the inherent value of the user-provided data. To ensure robustness and stability during optimization, we employ a projected gradient descent algorithm, constraining token embeddings to remain close to their original values and safeguarding the quality of the context. Our method has demonstrated superior accuracy across multiple classification tasks using various LLM models, outperforming existing baselines and effectively addressing the overfitting challenge in few-shot learning.

Take a look at [Example](https://github.com/huggingface/peft/blob/main/examples/cpt_finetuning/README.md) for a step-by-step guide on how to train a model with CPT.

## Benchmark overview

There is no benchmark for this method yet. Feel free to contribute an experiment
configuration but make sure to first create an issue
[here](https://github.com/huggingface/peft/issues).

# API

## CPTConfig[[peft.CPTConfig]]

#### peft.CPTConfig[[peft.CPTConfig]]

```python
peft.CPTConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, num_virtual_tokens: int = None, token_dim: int = None, num_transformer_submodules: Optional[int] = None, num_attention_heads: Optional[int] = None, num_layers: Optional[int] = None, modules_to_save: Optional[list[str]] = None, cpt_token_ids: typing.Optional[list[int]] = None, cpt_mask: typing.Optional[list[int]] = None, cpt_tokens_type_mask: typing.Optional[list[int]] = None, opt_weighted_loss_type: typing.Optional[typing.Literal['none', 'decay']] = 'none', opt_loss_decay_factor: typing.Optional[float] = 1.0, opt_projection_epsilon: typing.Optional[float] = 0.1, opt_projection_format_epsilon: typing.Optional[float] = 0.1, tokenizer_name_or_path: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/config.py#L23)

CPT Configuration class extending PeftConfig for Context-aware Prompt Tuning (CPT).

This class introduces additional parameters required for CPT, such as:
- Token type masks
- Prompt tuning initialization
- Loss weighting
- Projection settings

For more details, see the paper: https://huggingface.co/papers/2410.17222

## CPTEmbedding[[peft.CPTEmbedding]]

#### peft.CPTEmbedding[[peft.CPTEmbedding]]

```python
peft.CPTEmbedding(config, word_embeddings)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/model.py#L23)

CPTEmbedding is a custom embedding layer designed for Context-aware Prompt Tuning (CPT) in PEFT. It initializes
embeddings, applies prompt-specific projections, and computes loss using label masks.

#### calculate_loss[[peft.CPTEmbedding.calculate_loss]]

```python
calculate_loss(base_model_output, labels, cpt_type_mask, config)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/model.py#L143)

**Parameters:**

base_model_output (ModelOutput) : Output from the base model containing logits.

labels (torch.Tensor) : Ground-truth labels for the input tokens.

cpt_type_mask (torch.Tensor) : Token type mask used for filtering valid loss terms.

config (Namespace) : Configuration object containing loss-related hyperparameters.

**Returns:** `ModelOutput`

The base model output with computed loss.

Computes the loss for CPT models with optional exponential decay.

#### forward[[peft.CPTEmbedding.forward]]

```python
forward(indices)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/model.py#L65)

**Parameters:**

indices (torch.Tensor) : Indices of the tokens to be embedded.

**Returns:** `torch.Tensor`

Sum of prompt embeddings and delta embeddings.

Computes the prompt embeddings and applies delta adjustments.

#### get_projection[[peft.CPTEmbedding.get_projection]]

```python
get_projection()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/model.py#L125)

Applies epsilon-based projection to the delta embeddings to control their norm.

#### set_updated_tokens[[peft.CPTEmbedding.set_updated_tokens]]

```python
set_updated_tokens()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/cpt/model.py#L86)

Sets up a backward hook to selectively update token gradients based on the CPT token type mask.
