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

CPT Configuration class extending PeftConfig for Context-aware Prompt Tuning (CPT).

This class introduces additional parameters required for CPT, such as:
- Token type masks
- Prompt tuning initialization
- Loss weighting
- Projection settings

For more details, see the paper: https://huggingface.co/papers/2410.17222

## CPTEmbedding[[peft.CPTEmbedding]]

CPTEmbedding is a custom embedding layer designed for Context-aware Prompt Tuning (CPT) in PEFT. It initializes
embeddings, applies prompt-specific projections, and computes loss using label masks.

- **base_model_output** (ModelOutput) --
  Output from the base model containing logits.
- **labels** (torch.Tensor) --
  Ground-truth labels for the input tokens.
- **cpt_type_mask** (torch.Tensor) --
  Token type mask used for filtering valid loss terms.
- **config** (Namespace) --
  Configuration object containing loss-related hyperparameters.ModelOutputThe base model output with computed loss.

Computes the loss for CPT models with optional exponential decay.

- **indices** (torch.Tensor) --
  Indices of the tokens to be embedded.torch.TensorSum of prompt embeddings and delta embeddings.

Computes the prompt embeddings and applies delta adjustments.

Applies epsilon-based projection to the delta embeddings to control their norm.

Sets up a backward hook to selectively update token gradients based on the CPT token type mask.
