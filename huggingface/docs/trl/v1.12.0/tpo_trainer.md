# TPO Trainer

[![All_models-TPO-blue](https://img.shields.io/badge/All_models-TPO-blue)](https://huggingface.co/models?other=tpo,trl)

## Overview

Triple Preference Optimization (TPO) was introduced in the paper [Triple Preference Optimization: Achieving Better Alignment using a Single Step Optimization](https://huggingface.co/papers/2405.16681) by Amir Saeidi, Shivanshu Verma, Aswin RRV, and Chitta Baral. TPO enhances the instruction-following and reasoning capabilities of large language models in a single training step, starting from a pre-trained or instruction-tuned model.

The abstract from the paper is the following:

> Reinforcement Learning with Human Feedback (RLHF) enhances the alignment of Large Language Models (LLMs). However, its limitations have led to the development of Direct Preference Optimization (DPO), an RL-free approach designed to overcome these shortcomings. While studies have shown that DPO improves instruction-following capabilities, it negatively impacts the reasoning ability of LLMs. Additionally, DPO is highly sensitive to judgment noise in preference datasets and the size of the training set. Although several modifications to DPO have been proposed, they still fail to fully resolve these issues. To address these limitations, we propose Triple Preference Optimization (TPO), a new preference learning method designed to enhance both reasoning and instruction-following abilities through one-step optimization. We compare TPO against DPO and its recent variants using state-of-the-art training setups, including both base and instructiontuned models such as Mistral and Llama 3. Our evaluation covers a comprehensive range of chat-based and reasoning benchmarks. The results demonstrate that TPO achieves significant improvements over existing methods without substantially increasing response length across different dataset sizes. Specifically, TPO outperforms DPO and SimPO by up to 7.0% and 7.3% points on Arena-Hard, 12.2% and 13.3% points on MixEval-Hard, 10.4% and 10.1% points on MMLU-Pro, and 19.0% and 19.2% points on GSM8K, respectively. Furthermore, TPO achieves these improvements while requiring less data than DPO.

This post-training method was contributed by [Kashif Rasul](https://huggingface.co/kashif).

## Quick start

This example demonstrates how to train a model using the TPO method. We use the [Qwen 3 0.6B model](https://huggingface.co/Qwen/Qwen3-0.6B) as the base model. TPO requires a *triple-preference* dataset (`prompt`, `chosen`, `rejected`, `reference`) — see [Expected dataset type](#expected-dataset-type-and-format) below.

Below is the script to train the model:

```python
# train_tpo.py
from datasets import load_dataset
from trl.experimental.tpo import TPOConfig, TPOTrainer
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen3-0.6B")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen3-0.6B")
train_dataset = load_dataset("tpo-alignment/triple-preference-ultrafeedback-40K", split="train")

training_args = TPOConfig(output_dir="Qwen3-0.6B-TPO")
trainer = TPOTrainer(model=model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_tpo.py
```

## Expected dataset type and format

TPO requires a *triple-preference* dataset: each example must contain a `prompt`, a `chosen` (preferred) completion, a `rejected` (dispreferred) completion **and** a `reference` (gold) completion. The [experimental.tpo.TPOTrainer](/docs/trl/v1.12.0/en/tpo_trainer#trl.experimental.tpo.TPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

```python
# Standard format
triple_preference_example = {
    "prompt": "The sky is",
    "reference": " a beautiful shade of blue.",  # gold response (used for the NLL term)
    "chosen": " blue.",
    "rejected": " green.",
}

# Conversational format
triple_preference_example = {
    "prompt": [{"role": "user", "content": "What color is the sky?"}],
    "reference": [{"role": "assistant", "content": "It is a beautiful shade of blue."}],
    "chosen": [{"role": "assistant", "content": "It is blue."}],
    "rejected": [{"role": "assistant", "content": "It is green."}],
}
```

The reference response is typically the highest-quality completion available for the prompt; in the original TPO paper it is taken from the response with the highest score in [UltraFeedback](https://huggingface.co/datasets/openbmb/UltraFeedback), with the second-highest used as the chosen completion and the lowest as the rejected completion.

## Example script

We provide an example script to train a model using the TPO method. The script is available at [`examples/tpo_ultrafeedback/tpo.py`](https://github.com/huggingface/trl/blob/main/examples/tpo_ultrafeedback/tpo.py).

To test the TPO script with the [Qwen 3 0.6B model](https://huggingface.co/Qwen/Qwen3-0.6B) on a triple-preference dataset, run the following command:

```bash
accelerate launch examples/tpo_ultrafeedback/tpo.py \
    --model_name_or_path Qwen/Qwen3-0.6B \
    --dataset_name tpo-alignment/triple-preference-ultrafeedback-40K \
    --beta 0.01 \
    --tpo_alpha 1.0 \
    --learning_rate 5e-7 \
    --num_train_epochs 1 \
    --output_dir Qwen3-0.6B-TPO
```

## Looking deeper into the TPO method

Triple Preference Optimization (TPO) extends preference-based alignment from pairs to *triples* `(y_gold, y_chosen, y_rejected)`. The model is jointly optimized with two objectives in a single step:

1. A **contrastive loss** between the chosen and rejected completions, similar in spirit to DPO/SimPO but computed directly from the policy log-probabilities (no separate reference policy is required).
2. A **supervised negative log-likelihood (NLL) loss** on the gold (`reference`) completion, weighted by `tpo_alpha`. This term replaces the standalone SFT stage typically required before DPO.

The total TPO loss is:

$$
\mathcal{L}_{\mathrm{TPO}}(\theta) = \mathcal{L}_{\mathrm{contrast}}(\theta) + \alpha \cdot \mathcal{L}_{\mathrm{NLL}}(\theta; y_{\text{gold}})
$$

where  \\( \alpha \\) is `tpo_alpha` and  \\( \mathcal{L}_{\mathrm{contrast}} \\) is selected via `loss_type`.

### Loss types

| `loss_type=` | Description |
| --- | --- |
| `"sigmoid"` (default) | Sigmoid loss on the (sum) log-probability difference between the chosen and rejected completions, as in the original [TPO](https://huggingface.co/papers/2405.16681) paper. |
| `"hinge"` | Hinge loss on the normalized likelihood from the [SLiC](https://huggingface.co/papers/2305.10425) paper. In this case, `beta` is the reciprocal of the margin. |
| `"ipo"` | IPO loss from the [IPO](https://huggingface.co/papers/2310.12036) paper, computed on length-normalized log-probabilities. |
| `"tpo-l"` | Length-normalized TPO variant: uses average per-token log-probabilities and adds a target reward margin `tpo_l_gamma` to the Bradley-Terry objective, in the spirit of [SimPO](https://huggingface.co/papers/2405.14734). |

Setting `tpo_alpha=0.0` disables the NLL term entirely (the reference response is then unused, and the corresponding cross-entropy is skipped to save compute).

## Logged metrics

While training and evaluating, we record the following metrics:

* `loss`: The total TPO loss (contrastive + `tpo_alpha` × NLL) averaged over the current logging interval.
* `entropy`: The average entropy of the model's predicted token distribution over completion tokens.
* `mean_token_accuracy`: The proportion of completion tokens for which the model's top-1 prediction matches the chosen completion.
* `num_tokens`: The total number of tokens processed so far.
* `logits/chosen`: The average logit values assigned by the model to the tokens in the chosen completion.
* `logits/rejected`: The average logit values assigned by the model to the tokens in the rejected completion.
* `logps/chosen`: The average log-probability assigned by the model to the chosen completion.
* `logps/rejected`: The average log-probability assigned by the model to the rejected completion.
* `rewards/chosen`: The average implicit reward computed for the chosen completion, defined as  \\( \beta \log \pi_{\theta}(y^{+}\!\mid x) \\).
* `rewards/rejected`: The average implicit reward computed for the rejected completion, defined as  \\( \beta \log \pi_{\theta}(y^{-}\!\mid x) \\).
* `rewards/margins`: The average implicit reward margin between the chosen and rejected completions.
* `rewards/accuracies`: The proportion of examples where the implicit reward for the chosen completion is higher than that for the rejected completion.

## TPOTrainer[[trl.experimental.tpo.TPOTrainer]]

#### trl.experimental.tpo.TPOTrainer[[trl.experimental.tpo.TPOTrainer]]

```python
trl.experimental.tpo.TPOTrainer(model: str | PreTrainedModel | PeftModel, args: trl.experimental.tpo.tpo_config.TPOConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), peft_config: PeftConfig | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/tpo/tpo_trainer.py#L218)

**Parameters:**

model (`str` or [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) or [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) : Model to be trained. Can be either:  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using `<ModelArchitecture>.from_pretrained` (where `<ModelArchitecture>` is derived from the model config) with the keyword arguments in `args.model_init_kwargs`. - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported. - A [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel) object. Only causal language models are supported.

args ([experimental.tpo.TPOConfig](/docs/trl/v1.12.0/en/tpo_trainer#trl.experimental.tpo.TPOConfig), *optional*) : Configuration for this trainer. If `None`, a default configuration is used.

data_collator (`DataCollator`, *optional*) : Function to use to form a batch from a list of elements of the processed `train_dataset` or `eval_dataset`. Will default to `DataCollatorForTriplePreference`. Custom collators must truncate sequences before padding; the trainer does not apply post-collation truncation.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset)) : Dataset to use for training. TPO requires a *triple-preference* dataset: each sample must contain a `"chosen"`, a `"rejected"` and a `"reference"` (gold) completion. The format of the samples can be either:  - [Standard](dataset_formats#standard): Each sample contains plain text. - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role and content).

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset) or `dict[str, Dataset | IterableDataset]`) : Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) : Processing class used to process the data. If `None`, the processing class is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained). A padding token, `tokenizer.pad_token`, must be set. If the processing class has not set a padding token, `tokenizer.eos_token` will be used as the default.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function that will be used to compute metrics at evaluation. Must take a [EvalPrediction](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.EvalPrediction) and return a dictionary string to metric values.

callbacks (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) : List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed in [here](https://huggingface.co/docs/transformers/main_classes/callback).  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer.remove_callback) method.

optimizers (`tuple[torch.optim.Optimizer | None, torch.optim.lr_scheduler.LambdaLR | None]`, *optional*, defaults to `(None, None)`) : A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your model and a scheduler given by [get_linear_schedule_with_warmup](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup) controlled by `args`.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : PEFT configuration used to wrap the model. If `None`, the model is not wrapped.

Trainer for Triple Preference Optimization (TPO) method. This algorithm was initially proposed in the paper [Triple
Preference Optimization: Achieving Better Alignment using a Single Step
Optimization](https://huggingface.co/papers/2405.16681). This class is a wrapper around the
[Trainer](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer) class and inherits all of its attributes and methods.

#### train[[trl.experimental.tpo.TPOTrainer.train]]

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

#### save_model[[trl.experimental.tpo.TPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.tpo.TPOTrainer.push_to_hub]]

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

## TPOConfig[[trl.experimental.tpo.TPOConfig]]

#### trl.experimental.tpo.TPOConfig[[trl.experimental.tpo.TPOConfig]]

```python
trl.experimental.tpo.TPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-07, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, disable_dropout: bool = True, dataset_num_proc: int | None = None, max_length: int | None = 1024, truncation_mode: str = 'keep_start', pad_to_multiple_of: int | None = None, loss_type: str = 'sigmoid', beta: float = 0.01, label_smoothing: float = 0.0, tpo_alpha: float = 1.0, tpo_l_gamma: float = 0.5)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/tpo/tpo_config.py#L22)

**Parameters that control the model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model` argument of the [experimental.tpo.TPOTrainer](/docs/trl/v1.12.0/en/tpo_trainer#trl.experimental.tpo.TPOTrainer) is provided as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

**Parameters that control the data preprocessing:**

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the tokenized sequence. Sequences longer than `max_length` are truncated from the left or right depending on the `truncation_mode`. If `None`, no truncation is applied.

truncation_mode (`str`, *optional*, defaults to `"keep_start"`) : Truncation mode to use when the sequence exceeds `max_length`. Possible values are `"keep_start"` and `"keep_end"`.

pad_to_multiple_of (`int`, *optional*) : If set, the sequences will be padded to a multiple of this value.

**Parameters that control the training:**

loss_type (`str`, *optional*, defaults to `"sigmoid"`) : Type of loss to use. Possible values are:  - `"sigmoid"`: sigmoid loss from the original [TPO](https://huggingface.co/papers/2405.16681) paper. - `"hinge"`: hinge loss on the normalized likelihood from the [SLiC](https://huggingface.co/papers/2305.10425) paper. - `"ipo"`: IPO loss from the [IPO](https://huggingface.co/papers/2310.12036) paper. - `"tpo-l"`: length-normalized TPO variant from the [TPO](https://huggingface.co/papers/2405.16681) paper, which adds a target reward margin `tpo_l_gamma` to the Bradley-Terry objective. 

beta (`float`, *optional*, defaults to `0.01`) : Parameter controlling the temperature of the TPO loss. For the IPO loss (`loss_type="ipo"`), β is the regularization parameter denoted by τ in the [paper](https://huggingface.co/papers/2310.12036).

label_smoothing (`float`, *optional*, defaults to `0.0`) : Label smoothing factor.

tpo_alpha (`float`, *optional*, defaults to `1.0`) : Weight of the supervised negative log-likelihood term computed on the gold (`reference`) response in TPO training. Setting `tpo_alpha=0.0` disables the NLL term and skips the corresponding forward pass.

tpo_l_gamma (`float`, *optional*, defaults to `0.5`) : Target reward margin γ for the TPO-L loss, used only when `loss_type="tpo-l"`.

Configuration class for the [experimental.tpo.TPOTrainer](/docs/trl/v1.12.0/en/tpo_trainer#trl.experimental.tpo.TPOTrainer).

This class includes only the parameters that are specific to TPO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `5e-7` instead of `5e-5`.
