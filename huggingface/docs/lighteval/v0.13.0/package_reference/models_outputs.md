# Model's Output[[lighteval.models.model_output.ModelResponse]]

All models will generate an ouput per Doc supplied to the `generation` or `loglikelihood` fuctions.

#### lighteval.models.model_output.ModelResponse[[lighteval.models.model_output.ModelResponse]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/model_output.py#L29)

A class to represent the response from a model during evaluation.

This dataclass contains all the information returned by a model during inference,
including generated text, log probabilities, token information, and metadata.
Different attributes are required for different types of evaluation metrics.

Usage Examples:

**For generative tasks (text completion, summarization):**

```python
response = ModelResponse(
    text=["The capital of France is Paris."],
    input_tokens=[1, 2, 3, 4],
    output_tokens=[[5, 6, 7, 8]]
)
```

**For multiple choice tasks:**

```python
response = ModelResponse(
    logprobs=[-0.5, -1.2, -2.1, -1.8],  # Logprobs for each choice
    argmax_logits_eq_gold=[False, False, False, False],  # Whether correct choice was selected
    input_tokens=[1, 2, 3, 4],
    output_tokens=[[5], [6], [7], [8]]
)
```

**For perplexity calculation:**

```python
response = ModelResponse(
    text=["The model generated this text."],
    logprobs=[-1.2, -0.8, -1.5, -0.9, -1.1],  # Logprobs for each token
    input_tokens=[1, 2, 3, 4, 5],
    output_tokens=[[6], [7], [8], [9], [10]]
)
```

**For PMI analysis:**

```python
response = ModelResponse(
    text=["The answer is 42."],
    logprobs=[-1.1, -0.9, -1.3, -0.7],  # Conditioned logprobs
    unconditioned_logprobs=[-2.1, -1.8, -2.3, -1.5],  # Unconditioned logprobs
    input_tokens=[1, 2, 3, 4],
    output_tokens=[[5], [6], [7], [8]]
)
```

Notes:
- For most evaluation tasks, only a subset of attributes is required
- The `text` attribute is the most commonly used for generative tasks
- `logprobs` are essential for probability-based metrics like perplexity

**Parameters:**

input (str | list | None) : The original input prompt or context that was fed to the model. Used for debugging and analysis purposes. 

input_tokens (list[int]) : The tokenized representation of the input prompt. Useful for understanding how the model processes the input. 

text (list[str]) : The generated text responses from the model. Each element represents one generation (useful when num_samples > 1). **Required for**: Generative metrics, exact match, llm as a judge, etc. 

text_post_processed (Optional[list[str]]) : The generated text responses from the model, but post processed. Atm, post processing removes thinking/reasoning steps.  Careful! This is not computed by default, but in a separate step by calling `post_process` on the ModelResponse object. **Required for**: Generative metrics that require direct answers. 

logprobs (list[float]) : Log probabilities of the generated tokens or sequences. **Required for**: loglikelihood and perplexity metrics. 

argmax_logits_eq_gold (list[bool]) : Whether the argmax logits match the gold/expected text. Used for accuracy calculations in multiple choice and classification tasks. **Required for**: certain loglikelihood metrics.  

unconditioned_logprobs (Optional[list[float]]) : Log probabilities from an unconditioned model (e.g., without context). Used for PMI (Pointwise Mutual Information) normalization. **Required for**: PMI metrics.
