# Callbacks

## RichProgressCallback[[trl.RichProgressCallback]]

#### trl.RichProgressCallback[[trl.RichProgressCallback]]

```python
trl.RichProgressCallback()
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/callbacks.py#L145)

A `TrainerCallback` that displays the progress of training or evaluation using Rich.

## LogCompletionsCallback[[trl.LogCompletionsCallback]]

#### trl.LogCompletionsCallback[[trl.LogCompletionsCallback]]

```python
trl.LogCompletionsCallback(trainer: Trainer, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, num_prompts: int | None = None, freq: int | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/callbacks.py#L256)

**Parameters:**

trainer (`Trainer`) : Trainer to which the callback will be attached. The trainer's evaluation dataset must include a `"prompt"` column containing the prompts for generating completions.

generation_config ([GenerationConfig](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : The generation config to use for generating completions.

num_prompts (`int`, *optional*) : The number of prompts to generate completions for. If not provided, defaults to the number of examples in the evaluation dataset.

freq (`int`, *optional*) : The frequency at which to log completions. If not provided, defaults to the trainer's `eval_steps`.

A [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback) that logs completions to Weights & Biases and/or Comet.

Usage:
```python
>>> trainer = DPOTrainer(...)
>>> completions_callback = LogCompletionsCallback(trainer=trainer)
>>> trainer.add_callback(completions_callback)
```

## BEMACallback[[trl.BEMACallback]]

#### trl.BEMACallback[[trl.BEMACallback]]

```python
trl.BEMACallback(update_freq: int = 400, ema_power: float = 0.5, bias_power: float = 0.2, lag: int = 10, update_after: int = 0, multiplier: float = 1.0, min_ema_multiplier: float = 0.0, device: str = 'cpu')
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/callbacks.py#L577)

**Parameters:**

update_freq (`int`, *optional*, defaults to `400`) : Update the BEMA weights every X steps. Denoted this as  \\( \phi \\) in the paper.

ema_power (`float`, *optional*, defaults to `0.5`) : Power for the EMA decay factor. Denoted  \\( \kappa \\) in the paper. To disable EMA, set this to `0.0`.

bias_power (`float`, *optional*, defaults to `0.2`) : Power for the BEMA scaling factor. Denoted  \\( \eta \\) in the paper. A large value (e.g. `8.0`) makes  \\( \alpha_t \\) decay to `0`, approximating disabled bias-correction; `0.0` instead pins  \\( \alpha_t \\) at `1` for every step (maximum, undecayed correction).

lag (`int`, *optional*, defaults to `10`) : Initial offset in the weight decay schedule that controls early-stage smoothness by acting as a virtual starting age for the updates. Denoted as  \\( \rho \\) in the paper.

update_after (`int`, *optional*, defaults to `0`) : Burn-in time before starting to update the BEMA weights. Denoted  \\( \tau \\) in the paper.

multiplier (`float`, *optional*, defaults to `1.0`) : Initial value for the EMA decay factor. Denoted as  \\( \gamma \\) in the paper.

min_ema_multiplier (`float`, *optional*, defaults to `0.0`) : Minimum value for the EMA decay factor.

device (`str`, *optional*, defaults to `"cpu"`) : Device to use for the BEMA buffers, e.g. `"cpu"` or `"cuda"`. Note that in most cases, this device SHOULD BE DIFFERENT from the device used for training in order to avoid OOM.

A [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback) that implements [BEMA](https://huggingface.co/papers/2508.00180)
(Bias-Corrected Exponential Moving Average) by [Adam Block](https://huggingface.co/abblock) and [Cyril
Zhang](https://huggingface.co/cyrilzhang). Code from https://github.com/abblock/bema under MIT license.

BEMA computes model weights that scale like:

$$
\theta_t' = \alpha_t \cdot (\theta_t - \theta_0) + \text{EMA}_t
$$

where  \\( \theta_t \\) is the current model weights,  \\( \theta_0 \\) is a snapshot of the model weights at the
first `update_after` step,  \\( \text{EMA}_t  \\) is the exponential moving average of the model weights, and
\\( \alpha_t \\) is a scaling factor that decays with the number of steps  \\( t \\) as

$$
\alpha_t = (\rho + \gamma \cdot t)^{-\eta}.
$$

The EMA is computed as:

$$
\text{EMA}_t = (1 - \beta_t) \cdot \text{EMA}_{t-1} + \beta_t \cdot \theta_t
$$

where  \\( \beta_t \\) is a decay factor that decays with the number of steps  \\( t \\) as

$$
\beta_t = (\rho + \gamma \cdot t)^{-\kappa}.
$$

Example:

```python
>>> from trl import BEMACallback

>>> trainer = Trainer(..., callbacks=[BEMACallback()])
```

## WeaveCallback[[trl.WeaveCallback]]

#### trl.WeaveCallback[[trl.WeaveCallback]]

```python
trl.WeaveCallback(trainer: Trainer, project_name: str | None = None, scorers: dict[str, callable] | None = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, num_prompts: int | None = None, dataset_name: str = 'eval_dataset', model_name: str | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/callbacks.py#L348)

**Parameters:**

trainer (`Trainer`) : Trainer to which the callback will be attached. The trainer's evaluation dataset must include a `"prompt"` column containing the prompts for generating completions.

project_name (`str`, *optional*) : Name of the Weave project where data will be logged. If not provided, will try to use existing weave client or fall back to the active wandb run's project name. Raises an error if none of these are available.

scorers (`dict[str, Callable]`, *optional*) : Dictionary mapping scorer names to scorer functions. If `None`, operates in tracing mode (predictions only). If provided, operates in evaluation mode (predictions + scores + summary). Scorer functions should have signature: `scorer(prompt: str, completion: str) -> float | int`

generation_config ([GenerationConfig](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : Generation config to use for generating completions.

num_prompts (`int` or `None`, *optional*) : Number of prompts to generate completions for. If not provided, defaults to the number of examples in the evaluation dataset.

dataset_name (`str`, *optional*, defaults to `"eval_dataset"`) : Name for the dataset metadata in Weave.

model_name (`str`, *optional*) : Name for the model metadata in Weave. If not provided, attempts to extract from model config.

A [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback) that logs traces and evaluations to W&B Weave. The callback uses
https://weave-docs.wandb.ai/guides/evaluation/evaluation_logger/ to log traces and evaluations at each evaluation
step.

Supports two modes based on the `scorers` parameter:
- **Tracing Mode** (when scorers=None): Logs predictions for data exploration and analysis
- **Evaluation Mode** (when scorers provided): Logs predictions with scoring and summary metrics

Both modes use Weave's EvaluationLogger for structured, consistent data logging.

The callback logs data during evaluation phases (`on_evaluate`) rather than training steps, making it more
efficient and semantically correct. It gracefully handles missing weave installation by logging warnings and
skipping weave-specific functionality. It also checks for existing weave clients before initializing new ones.

Usage:
```python
# Tracing mode (just log predictions)
trainer = DPOTrainer(...)
weave_callback = WeaveTraceCallback(trainer=trainer)  # project_name optional
trainer.add_callback(weave_callback)

# Or specify a project name
weave_callback = WeaveTraceCallback(trainer=trainer, project_name="my-llm-training")
trainer.add_callback(weave_callback)

# Evaluation mode (log predictions + scores + summary)
def accuracy_scorer(prompt: str, completion: str) -> float:
    # Your scoring logic here (metadata available via eval_attributes)
    return score

weave_callback = WeaveTraceCallback(
    trainer=trainer,
    project_name="my-llm-training",  # optional and needed only if weave client is not initialized
    scorers={"accuracy": accuracy_scorer},
)
trainer.add_callback(weave_callback)
```

#### on_train_begin[[trl.WeaveCallback.on_train_begin]]

```python
on_train_begin(args, state, control, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/callbacks.py#L479)

Initialize Weave when training begins.
