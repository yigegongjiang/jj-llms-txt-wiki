# Reward Modeling

[![model badge](https://img.shields.io/badge/All_models-Reward_Trainer-blue)](https://huggingface.co/models?other=reward-trainer,trl)

## Overview

TRL supports the Outcome-supervised Reward Modeling (ORM) Trainer for training reward models.

This post-training method was contributed by [Younes Belkada](https://huggingface.co/ybelkada).

## Quick start

This example demonstrates how to train a reward model using the [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) from TRL. We train a [Qwen 3 0.6B](https://huggingface.co/Qwen/Qwen3-0.6B) model on the [UltraFeedback dataset](https://huggingface.co/datasets/trl-lib/ultrafeedback_binarized), large-scale, fine-grained, diverse preference dataset.

```python
from trl import RewardTrainer
from datasets import load_dataset

trainer = RewardTrainer(
    model="Qwen/Qwen3-0.6B",
    train_dataset=load_dataset("trl-lib/ultrafeedback_binarized", split="train"),
)
trainer.train()
```

## Expected dataset type and format

[RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) supports [preference](dataset_formats#preference) datasets type (both implicit and explicit prompt). The [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) is compatible with both [standard](dataset_formats#standard) and [conversational](dataset_formats#conversational) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

```python
# Standard preference (implicit prompt)
{"chosen": "The sky is blue.",
 "rejected": "The sky is green."}

# Conversational preference (implicit prompt)
{"chosen": [{"role": "user", "content": "What color is the sky?"},
            {"role": "assistant", "content": "It is blue."}],
 "rejected": [{"role": "user", "content": "What color is the sky?"},
              {"role": "assistant", "content": "It is green."}]}

# Standard preference (explicit prompt)
{"prompt": "The sky is",
 "chosen": " blue.",
 "rejected": " green."}

# Conversational preference (explicit prompt)
{"prompt": [{"role": "user", "content": "What color is the sky?"}],
 "chosen": [{"role": "assistant", "content": "It is blue."}],
 "rejected": [{"role": "assistant", "content": "It is green."}]}
```

If your dataset is not in one of these formats, you can preprocess it to convert it into the expected format. Here is an example with the [lmarena-ai/arena-human-preference-55k](https://huggingface.co/datasets/lmarena-ai/arena-human-preference-55k) dataset:

```python
from datasets import load_dataset
import json

dataset = load_dataset("lmarena-ai/arena-human-preference-55k")

# Filter out ties
dataset = dataset.filter(lambda example: example["winner_tie"] == 0)

# Create 'chosen' and 'rejected' fields based on the winner column
def response_a_b_to_chosen_rejected(example):
    if example["winner_model_a"] == 1:
        example["chosen"] = example["response_a"]
        example["rejected"] = example["response_b"]
    else:
        example["chosen"] = example["response_b"]
        example["rejected"] = example["response_a"]
    return example

dataset = dataset.map(response_a_b_to_chosen_rejected)

# Convert to conversational format
def make_conversation(example):
    prompt = json.loads(example["prompt"])[0]  # '["What color is the sky?"]' -> "What color is the sky?"
    chosen = json.loads(example["chosen"])[0]
    rejected = json.loads(example["rejected"])[0]
    return {
        "chosen": [{"role": "user", "content": prompt}, {"role": "assistant", "content": chosen}],
        "rejected": [{"role": "user", "content": prompt}, {"role": "assistant", "content": rejected}],
    }

dataset = dataset.map(make_conversation)

# Keep only necessary columns
dataset = dataset.select_columns(["chosen", "rejected"])

print(next(iter(dataset["train"])))
```

```json
{
    "chosen": [
        {"role": "user", "content": "Is it morally right to try to have a certain percentage of females on managerial positions?"},
        {"role": "assistant", "content": "The question of whether it is morally right to aim for a certain percentage of females..."},
    ],
    "rejected": [
        {"role": "user", "content": "Is it morally right to try to have a certain percentage of females on managerial positions?"},
        {"role": "assistant", "content": "As an AI, I don't have personal beliefs or opinions. However, ..."},
    ],
}
```

## Looking deeper into the training method

Reward Models (RMs) are typically trained using supervised learning on datasets containing pairs of preferred and non-preferred responses. The goal is to learn a function that assigns higher scores to preferred responses, enabling the model to rank outputs based on preferences.

This section breaks down how reward modeling works in practice, covering the key steps: **preprocessing** and **loss computation**.

### Preprocessing and tokenization

During training, each example is expected to contain a **chosen** and **rejected** field. For more details on the expected formats, see [Dataset formats - Preference](dataset_formats#preference).
The [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) tokenizes each input using the model's tokenizer. If prompts and completions (chosen and rejected) are provided separately (explicit prompt case), they are concatenated before tokenization.

### Computing the loss

Let  \\( x \\) be the input sequence (prompt) and  \\( y^+ \\) and  \\( y^- \\) be the chosen and rejected sequences respectively. Under the Bradley-Terry model ([Bradley & Terry, 1952](https://www.jstor.org/stable/2334029)), the probability that  \\( y^+ \\) is preferred over  \\( y^- \\) given a reward function  \\( r \\) is  \\( p(y^+ ≻ y^- |x) = \sigma(r(x, y^+)−r(x, y^-)) \\), where  \\( σ \\) is the sigmoid function.

The reward model  \\( r_\theta(x, y) \\) is trained to assign higher scores to preferred responses  \\( y^+ \\) over non-preferred ones  \\( y^- \\). The loss is then defined as the negative log-likelihood of the observed preferences:

$$
\mathcal{L}(\theta) = - \mathbb{E}_{(x,y^+,y^-) \sim \mathcal{D}} \left[ \log \sigma(r_\theta(x, y^+) - r_\theta(x, y^-)) \right].
$$

> [!TIP]
> The Bradley-Terry model is underdetermined, meaning that adding a constant to all rewards does not change the preference probabilities. To address this, [Helping or Herding? Reward Model Ensembles Mitigate but do not Eliminate Reward Hacking](https://huggingface.co/papers/2312.09244) proposes adding an auxiliary loss term that encourages the rewards to be centered around zero. This is controlled by the `center_rewards_coefficient` parameter in the [RewardConfig](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardConfig). The recommended value is `1e-2`.

## Logged metrics

While training and evaluating, we record the following metrics:

* `global_step`: The total number of optimizer steps taken so far.
* `epoch`: The current epoch number, based on dataset iteration.
* `num_tokens`: The total number of tokens processed so far.
* `loss`: The average loss over the last logging interval.
* `accuracy`: The proportion of correct predictions (i.e., the model assigned a higher score to the chosen response than to the rejected one) averaged over the last logging interval.
* `min_reward`: The minimum reward score assigned by the model. This value is averaged over the logging interval.
* `mean_reward`: The average reward score assigned by the model over the last logging interval.
* `max_reward`: The maximum reward score assigned by the model. This value is averaged over the logging interval.
* `margin`: The average margin (difference between chosen and rejected rewards) over the last logging interval.
* `learning_rate`: The current learning rate, which may change dynamically if a scheduler is used.
* `grad_norm`: The L2 norm of the gradients, computed before gradient clipping.

## Customization

### Model initialization

You can directly pass the kwargs of the `from_pretrained()` method to the [RewardConfig](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardConfig). For example, if you want to load a model in a different precision, analogous to

```python
model = AutoModelForSequenceClassification.from_pretrained("Qwen/Qwen3-0.6B", dtype=torch.bfloat16)
```

you can do so by passing the `model_init_kwargs={"dtype": torch.bfloat16}` argument to the [RewardConfig](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardConfig).

```python
from trl import RewardConfig

training_args = RewardConfig(
    model_init_kwargs={"dtype": torch.bfloat16},
)
```

Note that all keyword arguments of `from_pretrained()` are supported, except for `num_labels`, which is automatically set to 1.

### Train adapters with PEFT

We support tight integration with 🤗 PEFT library, allowing any user to conveniently train adapters and share them on the Hub, rather than training the entire model.

```python
from datasets import load_dataset
from trl import RewardTrainer
from peft import LoraConfig

dataset = load_dataset("trl-lib/ultrafeedback_binarized", split="train")

trainer = RewardTrainer(
    "Qwen/Qwen3-4B",
    train_dataset=dataset,
    peft_config=LoraConfig(modules_to_save=["score"])  # important to include the score head when base model is not a sequence classification model
)

trainer.train()
```

You can also continue training your [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel). For that, first load a `PeftModel` outside [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) and pass it directly to the trainer without the `peft_config` argument being passed.

```python
from datasets import load_dataset
from trl import RewardTrainer
from peft import AutoPeftModelForCausalLM

model = AutoPeftModelForCausalLM.from_pretrained("trl-lib/Qwen3-4B-Reward-LoRA", is_trainable=True)
dataset = load_dataset("trl-lib/Capybara", split="train")

trainer = RewardTrainer(
    model=model,
    train_dataset=dataset,
)

trainer.train()
```

> [!TIP]
> When training adapters, you typically use a higher learning rate (≈1e‑3) since only new parameters are being learned.
>
> ```python
> RewardConfig(learning_rate=1e-3, ...)
> ```

## Tool Calling with Reward Modeling

The [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) fully supports fine-tuning models with _tool calling_ capabilities. In this case, each dataset example should include:

* The conversation messages, including any tool calls (`tool_calls`) and tool responses (`tool` role messages)
* The list of available tools in the `tools` column, typically provided as JSON schemas

For details on the expected dataset structure, see the [Dataset Format — Tool Calling](dataset_formats#tool-calling) section.

## RewardTrainer[[trl.RewardTrainer]]

#### trl.RewardTrainer[[trl.RewardTrainer]]

```python
trl.RewardTrainer(model: str | PreTrainedModel | PeftModel, args: trl.trainer.reward_config.RewardConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | datasets.dataset_dict.DatasetDict | datasets.dataset_dict.IterableDatasetDict | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), optimizer_cls_and_kwargs: tuple[type[torch.optim.Optimizer], dict[str, typing.Any]] | None = None, preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, quantization_config: BitsAndBytesConfig | None = None, peft_config: PeftConfig | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/reward_trainer.py#L227)

**Parameters:**

model (`str` or [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) or [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) : Model to be trained. Can be either:  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using `AutoModelForSequenceClassification.from_pretrained` with the keyword arguments in `args.model_init_kwargs`. If `dtype` is not specified in `args.model_init_kwargs`, it defaults to `float32`. This differs from [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained), where (since Transformers v5) the dtype is inferred from the model config. - A sequence classification [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) object. - A sequence classification [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel) object.

args ([RewardConfig](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardConfig), *optional*) : Configuration for this trainer. If `None`, a default configuration is used.

data_collator (`DataCollator`, *optional*) : Function to use to form a batch from a list of elements of the processed `train_dataset` or `eval_dataset`. Will default to `DataCollatorForPreference`.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset)) : Dataset to use for training. This trainer supports [preference](#preference) type (both implicit and explicit prompt). The format of the samples can be either:  - [Standard](dataset_formats#standard): Each sample contains plain text. - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role and content).  The trainer also supports processed datasets (tokenized) as long as they contain `chosen_ids` and `rejected_ids` fields.  When `train_dataset` is an [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset) (e.g. a streaming dataset), `max_steps` must be set in the training arguments, since its length cannot be inferred and the total number of training steps is required to bound the training loop and configure the learning rate scheduler.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset), [DatasetDict](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.DatasetDict), [IterableDatasetDict](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDatasetDict) or `dict[str, Dataset | IterableDataset]`) : Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) : Tokenizer used to process the data. If `None`, the tokenizer is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained). A padding token, `processing_class.pad_token`, must be set. If the processing class has not set a padding token, `processing_class.eos_token` will be used as the default.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function that will be used to compute metrics at evaluation. Must take a [EvalPrediction](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.EvalPrediction) and return a dictionary string to metric values. When passing [RewardConfig](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardConfig) with `batch_eval_metrics` set to `True`, your `compute_metrics` function must take a boolean `compute_result` argument. This will be triggered after the last eval batch to signal that the function needs to calculate and return the global summary statistics rather than accumulating the batch-level statistics.

callbacks (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) : List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed in [here](https://huggingface.co/docs/transformers/main_classes/callback).  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer.remove_callback) method.

optimizers (`tuple[torch.optim.Optimizer | None, torch.optim.lr_scheduler.LambdaLR | None]`, *optional*, defaults to `(None, None)`) : A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your model and a scheduler given by [get_linear_schedule_with_warmup](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup) controlled by `args`.

optimizer_cls_and_kwargs (`tuple[Type[torch.optim.Optimizer], Dict[str, Any]]`, *optional*) : A tuple containing the optimizer class and keyword arguments to use. Overrides `optim` and `optim_args` in `args`. Incompatible with the `optimizers` argument.  Unlike `optimizers`, this argument avoids the need to place model parameters on the correct devices before initializing the Trainer.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`, *optional*) : A function that preprocess the logits right before caching them at each evaluation step. Must take two tensors, the logits and the labels, and return the logits once processed as desired. The modifications made by this function will be reflected in the predictions received by `compute_metrics`.  Note that the labels (second parameter) will be `None` if the dataset does not have them.

quantization_config ([BitsAndBytesConfig](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/quantization#transformers.BitsAndBytesConfig), *optional*) : Quantization configuration used when loading the model from a model identifier. Combine with `peft_config` for QLoRA training. Ignored if the model is already instantiated, or if `quantization_config` is also set in `args.model_init_kwargs`.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : PEFT configuration used to wrap the model. If `None`, the model is not wrapped. Note that if the loaded model is a causal LM, it's highly recommended to set `modules_to_save=["score"]` in the PEFT configuration to ensure that the reward head is properly trained.

Trainer for Outcome-supervised Reward Models (ORM).

This class is a wrapper around the [Trainer](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer) class and inherits all of its attributes and methods.

Example:

```python
>>> from trl import RewardTrainer
>>> from datasets import load_dataset

>>> dataset = load_dataset("trl-lib/ultrafeedback_binarized", split="train")

>>> trainer = RewardTrainer(
...     model="Qwen/Qwen2.5-0.5B-Instruct",
...     train_dataset=dataset,
... )
>>> trainer.train()
```

#### train[[trl.RewardTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L1350)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.RewardTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.RewardTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L4052)

**Parameters:**

commit_message (`str`, *optional*, defaults to `"End of training"`) : Message to commit while pushing.

blocking (`bool`, *optional*, defaults to `True`) : Whether the function should return only when the `git push` has finished.

token (`str`, *optional*, defaults to `None`) : Token with write permission to overwrite Trainer's original args.

revision (`str`, *optional*) : The git revision to commit from. Defaults to the head of the "main" branch.

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to `~Trainer.create_model_card`.

**Returns:**

The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.

## RewardConfig[[trl.RewardConfig]]

#### trl.RewardConfig[[trl.RewardConfig]]

```python
trl.RewardConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 0.0001, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, chat_template_path: str | None = None, disable_dropout: bool = True, dataset_num_proc: int | None = None, eos_token: str | None = None, max_length: int | None = 1024, pad_to_multiple_of: int | None = None, center_rewards_coefficient: float | None = None, activation_offloading: bool = False, pad_token: str | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/reward_config.py#L23)

**Parameters that control the model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model` argument of the [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer) is provided as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

chat_template_path (`str`, *optional*) : If specified, sets the model's chat template. This can either be the path to a tokenizer (local directory or Hugging Face Hub model) or a direct path to a Jinja template file. When using a Jinja file, you must ensure that any special tokens referenced in the template are added to the tokenizer and that the model's embedding layer is resized accordingly.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

**Parameters that control the data preprocessing:**

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

eos_token (`str`, *optional*) : Token used to indicate the end of a turn or sequence. If `None`, it defaults to `processing_class.eos_token`.

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the tokenized sequence. Samples are filtered out if either chosen or rejected sequence exceeds this value. If `None`, no filtering is applied.

pad_to_multiple_of (`int`, *optional*) : If set, the sequences will be padded to a multiple of this value.

**Parameters that control the training:**

center_rewards_coefficient (`float`, *optional*) : Coefficient to incentivize the reward model to output mean-zero rewards (proposed by https://huggingface.co/papers/2312.09244, Eq. 2). Recommended value: `0.01`.

activation_offloading (`bool`, *optional*, defaults to `False`) : Whether to offload the activations to the CPU.

**Deprecated parameters:**

pad_token :   Parameter `pad_token` is deprecated and will be removed in version v2.0.0. Set `tokenizer.pad_token` directly and pass it as `processing_class` to the trainer instead.  

Configuration class for the [RewardTrainer](/docs/trl/v1.12.0/en/reward_trainer#trl.RewardTrainer).

This class includes only the parameters that are specific to Reward training. For a full list of training
arguments, please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this
class may differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `1e-4` instead of `5e-5`.
