# Online DPO Trainer

[![model badge](https://img.shields.io/badge/All_models-Online_DPO-blue)](https://huggingface.co/models?other=online-dpo,trl)

## Overview

Online DPO was proposed in [Direct Language Model Alignment from Online AI Feedback](https://huggingface.co/papers/2402.04792) by Shangmin Guo, Biao Zhang, Tianlin Liu, Tianqi Liu, Misha Khalman, Felipe Llinares, Alexandre Rame, Thomas Mesnard, Yao Zhao, Bilal Piot, Johan Ferret, and Mathieu Blondel.

The abstract from the paper is the following:

> Direct alignment from preferences (DAP) methods, such as DPO, have recently emerged as efficient alternatives to reinforcement learning from human feedback (RLHF), that do not require a separate reward model. However, the preference datasets used in DAP methods are usually collected ahead of training and never updated, thus the feedback is purely offline. Moreover, responses in these datasets are often sampled from a language model distinct from the one being aligned, and since the model evolves over training, the alignment phase is inevitably off-policy. In this study, we posit that online feedback is key and improves DAP methods. Our method, online AI feedback (OAIF), uses an LLM as annotator: on each training iteration, we sample two responses from the current model and prompt the LLM annotator to choose which one is preferred, thus providing online feedback. Despite its simplicity, we demonstrate via human evaluation in several tasks that OAIF outperforms both offline DAP and RLHF methods. We further show that the feedback leveraged in OAIF is easily controllable, via instruction prompts to the LLM annotator.

This post-training method was contributed by [Michael Noukhovitch](https://huggingface.co/mnoukhov), [Shengyi Costa Huang](https://huggingface.co/vwxyzjn), [Quentin Gallouédec](https://huggingface.co/qgallouedec), and [Edward Beeching](https://huggingface.co/edbeeching).

## Quick start

This example demonstrates how to train a model using the online DPO method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) as the base model and the [trl-lib/Qwen2-0.5B-Reward](https://huggingface.co/trl-lib/Qwen2-0.5B-Reward) reward model. We use the prompts from the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback). You can view the prompts in the dataset here:

<iframe
  src="https://huggingface.co/datasets/trl-lib/ultrafeedback-prompt/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_online_dpo.py
from datasets import load_dataset
from trl.experimental.online_dpo import OnlineDPOConfig, OnlineDPOTrainer
from transformers import AutoModelForCausalLM, AutoModelForSequenceClassification, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
reward_model = AutoModelForSequenceClassification.from_pretrained("trl-lib/Qwen2-0.5B-Reward", num_labels=1)
train_dataset = load_dataset("trl-lib/ultrafeedback-prompt", split="train")

training_args = OnlineDPOConfig(output_dir="Qwen2-0.5B-OnlineDPO")
trainer = OnlineDPOTrainer(
    model=model, reward_funcs=reward_model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset
)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_online_dpo.py
```

Distributed across 8 GPUs, the training takes approximately 1 hour. You can verify the training progress by checking the reward graph. An increasing trend in both the reward for rejected and chosen completions indicates that the model is improving and generating better responses over time.

![](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/online-dpo-qwen2.png)

To see how the [trained model](https://huggingface.co/trl-lib/Qwen2-0.5B-OnlineDPO) performs, you can use the [Transformers Chat CLI](https://huggingface.co/docs/transformers/quicktour#chat-with-text-generation-models).

$ transformers chat trl-lib/Qwen2-0.5B-OnlineDPO
&lt;quentin_gallouedec&gt;:
What is the best programming language?

&lt;trl-lib/Qwen2-0.5B-OnlineDPO&gt;:
The best programming language depends on your specific needs and priorities. Some people prefer imperative programming languages (like Haskell or Lisp), while others prefer functional programming languages (like Scala or Python). It's important to consider your work style, programming environment, and project requirements when choosing a programming language.

## Expected dataset type

Online DPO only requires a [prompt-only dataset](dataset_formats#prompt-only) (unlike offline DPO, that expects [preference dataset](dataset_formats#preference)). The [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Usage tips

### Encourage EOS token generation

When using a reward model, we may want the model to generate completions within a given length. During training, the model will generate completions up to the maximum length specified in the `max_new_tokens` argument of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig). If you want to penalize the model for not generating an EOS token before reaching the maximum length, you can use the `missing_eos_penalty` argument of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig):

```python
training_args = OnlineDPOConfig(..., max_new_tokens=128, missing_eos_penalty=1.0)
```

### Logging Completions

To better understand your model's behavior during training, you can log sample completions periodically using the [LogCompletionsCallback](/docs/trl/v1.12.0/en/callbacks#trl.LogCompletionsCallback).

```python
trainer = OnlineDPOTrainer(..., eval_dataset=eval_dataset)
completions_callback = LogCompletionsCallback(trainer, num_prompts=8)
trainer.add_callback(completions_callback)
```

This callback logs the model's generated completions directly to Weights & Biases.

![Logged Completions](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/wandb_completions.png)

## Logged metrics

While training and evaluating, we record the following metrics. Here is an example [tracked run at Weights and Biases](https://wandb.ai/huggingface/trl/runs/w4apmsi9)

* `objective/kl`: The mean Kullback-Leibler (KL) divergence between the current model and reference model.
* `objective/entropy`: The mean entropy of the model, indicating the randomness of the actions chosen by the model.
* `objective/non_score_reward`: The mean reward from non-score-related sources, basically `beta * kl.sum(1)`, where `beta` is the KL penalty coefficient and `kl` is the per-token KL divergence.
* `objective/rlhf_reward`: The mean RLHF reward, which is `scores - non_score_reward`. The `rlhf_reward` is the ultimate objective of online DPO training. If training works as intended, this metric should keep going up.
* `objective/scores`: The mean scores returned by the reward model.
* `objective/scores_margin`: The mean score margin (according to the external reward model) between the chosen and rejected completions.
* `rewards/chosen`: The mean reward (according to online DPO's implicit reward model)of the chosen completions.
* `rewards/rejected`: The mean reward (according to online DPO's implicit reward model) of the rejected completions.
* `rewards/accuracies`: The accuracies of the online DPO's implicit reward model.
* `rewards/margins`: The mean reward margin (according to online DPO's implicit reward model) between the chosen and rejected completions.
* `logps/chosen`: The mean log probabilities of the chosen completions.
* `logps/rejected`: The mean log probabilities of the rejected completions.
* `val/contain_eos_token`: The fraction of completions which contain an EOS token.
* `beta`: The parameter that controls the weight of the loss term representing the deviation from the reference model. Typically fixed, but can be made dynamic by passing a list to [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig).

## Benchmark experiments

To validate the online DPO implementation works, we ran experiments with the Pythia 1B, 2.8B, and 6.9B models on a single node of 8 x H100s. Here are the commands we used to run the experiments, with the online DPO example script as it existed at the time ([`examples/scripts/online_dpo.py`](https://github.com/huggingface/trl/blob/v1.10.0/examples/scripts/online_dpo.py), since removed) — to reproduce them, run from a v1.10.0 checkout (`git checkout v1.10.0`). We take the SFT / RM models directly from [The N+ Implementation Details of RLHF with PPO: A Case Study on TL;DR Summarization](https://huggingface.co/papers/2403.17031).

```shell
# 1B Online DPO experiment
accelerate launch --config_file examples/accelerate_configs/multi_gpu.yaml \
    examples/scripts/online_dpo.py \
    --model_name_or_path trl-lib/pythia-1b-deduped-tldr-sft  \
    --reward_model_path trl-lib/pythia-1b-deduped-tldr-rm \
    --dataset_name trl-lib/tldr \
    --learning_rate 5.0e-7 \
    --output_dir pythia-1b-deduped-tldr-online-dpo \
    --beta 0.1 \
    --per_device_train_batch_size 8 \
    --gradient_accumulation_steps 2 \
    --num_train_epochs 3 \
    --max_new_tokens 53 \
    --warmup_steps 0.1 \
    --missing_eos_penalty 1.0 \
    --save_steps 0.1 \
    --push_to_hub

# 2.8B Online DPO experiment
accelerate launch --config_file examples/accelerate_configs/deepspeed_zero2.yaml \
    examples/scripts/online_dpo.py \
    --model_name_or_path trl-lib/pythia-2.8b-deduped-tldr-sft  \
    --reward_model_path trl-lib/pythia-2.8b-deduped-tldr-rm \
    --dataset_name trl-lib/tldr \
    --learning_rate 5.0e-7 \
    --output_dir pythia-2.8b-deduped-tldr-online-dpo \
    --beta 0.1 \
    --per_device_train_batch_size 8 \
    --gradient_accumulation_steps 2 \
    --num_train_epochs 3 \
    --max_new_tokens 53 \
    --warmup_steps 0.1 \
    --missing_eos_penalty 1.0 \
    --save_steps 0.1 \
    --push_to_hub

# 6.9B Online DPO experiment
accelerate launch --config_file examples/accelerate_configs/deepspeed_zero2.yaml \
    examples/scripts/online_dpo.py \
    --model_name_or_path trl-lib/pythia-6.9b-deduped-tldr-sft  \
    --reward_model_path trl-lib/pythia-6.9b-deduped-tldr-rm \
    --dataset_name trl-lib/tldr \
    --learning_rate 5.0e-7 \
    --output_dir pythia-6.9b-deduped-tldr-online-dpo \
    --beta 0.1 \
    --per_device_train_batch_size 4 \
    --gradient_accumulation_steps 4 \
    --num_train_epochs 3 \
    --max_new_tokens 53 \
    --warmup_steps 0.1 \
    --missing_eos_penalty 1.0 \
    --save_steps 0.1 \
    --push_to_hub
```

Checkpoints and experiment tracking are available at:

* [🤗 Model checkpoints](https://huggingface.co/collections/trl-lib/online-dpo-66acd3fa38a331a9cd457b07)
* [🐝 Tracked experiment](https://wandb.ai/huggingface/trl/reports/Online-DPO-experiments-for-TL-DR-summarisation--Vmlldzo5MTczMDU0)

The online DPO checkpoint gets increasingly more win rate as we scale up the model sizes. This is a good sign that the online DPO implementation is working as intended.

## OnlineDPOTrainer[[trl.experimental.online_dpo.OnlineDPOTrainer]]

#### trl.experimental.online_dpo.OnlineDPOTrainer[[trl.experimental.online_dpo.OnlineDPOTrainer]]

```python
trl.experimental.online_dpo.OnlineDPOTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str], ref_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, NoneType] = None, reward_funcs: str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]] | list[str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]]] | None = None, args: trl.experimental.online_dpo.online_dpo_config.OnlineDPOConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: typing.Union[datasets.arrow_dataset.Dataset, torch.utils.data.IterableDataset, NoneType] = None, eval_dataset: typing.Union[datasets.arrow_dataset.Dataset, torch.utils.data.IterableDataset, dict[str, typing.Union[datasets.arrow_dataset.Dataset, torch.utils.data.IterableDataset]], NoneType] = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.processing_utils.ProcessorMixin | None = None, reward_processing_classes: transformers.tokenization_utils_base.PreTrainedTokenizerBase | list[transformers.tokenization_utils_base.PreTrainedTokenizerBase] | None = None, peft_config: PeftConfig | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/online_dpo/online_dpo_trainer.py#L103)

**Parameters:**

model (`str | nn.Module | PreTrainedModel`) : Model to be trained. Can be either:  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) with the keyword arguments in `args.model_init_kwargs`. - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported.

ref_model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) or `torch.nn.Module` or `None`) : The reference model to use for training. If None is specified, the reference model will be created from the model.

reward_funcs (`RewardFunc | list[RewardFunc]`) : Reward functions to be used for computing the rewards. To compute the rewards, we call all the reward functions with the prompts and completions and sum the rewards. Can be either:  - A single reward function: Can be a string (path to model), a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel), or a custom callable function. - A list of reward functions: Must all be of compatible types.

args ([experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig)) : The online DPO config arguments to use for training.

data_collator (`DataCollator`) : The data collator to use for training. If None is specified, the default data collator (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the maximum length of the sequences in the batch, given a dataset of paired sequences.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset)) : The dataset to use for training.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset) or `dict[str, Dataset | IterableDataset]`) : The dataset to use for evaluation.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. If provided, will be used to automatically process the inputs for the model, and it will be saved along the model to make it easier to rerun an interrupted training or reuse the fine-tuned model.

reward_processing_classes ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or `list[PreTrainedTokenizerBase]`, *optional*) : Processing classes corresponding to the reward functions specified in `reward_funcs`. Can be either:  - A single processing class: Used when `reward_funcs` contains only one reward function. - A list of processing classes: Must match the order and length of the reward functions in `reward_funcs`.  If set to `None`, the tokenizer for each model-based reward function is automatically loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : PEFT configuration used to wrap the model. If `None`, the model is not wrapped.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to metric values.

callbacks (`list[transformers.TrainerCallback]`) : The callbacks to use for training.

optimizers (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) : The optimizer and scheduler to use for training.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) : The function to use to preprocess the logits before computing the metrics.

Initialize OnlineDPOTrainer.

#### train[[trl.experimental.online_dpo.OnlineDPOTrainer.train]]

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

#### save_model[[trl.experimental.online_dpo.OnlineDPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.online_dpo.OnlineDPOTrainer.push_to_hub]]

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

## OnlineDPOConfig[[trl.experimental.online_dpo.OnlineDPOConfig]]

#### trl.experimental.online_dpo.OnlineDPOConfig[[trl.experimental.online_dpo.OnlineDPOConfig]]

```python
trl.experimental.online_dpo.OnlineDPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-07, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, reward_model_path: str | None = None, max_new_tokens: int = 64, max_length: int = 512, temperature: float = 0.9, top_p: float = 1.0, top_k: int = 0, min_p: float | None = None, repetition_penalty: float = 1.0, generation_kwargs: dict | None = None, cache_implementation: str | None = None, missing_eos_penalty: float | None = None, beta: list = <factory>, loss_type: str = 'sigmoid', disable_dropout: bool = True, use_vllm: bool = False, vllm_model_impl: str = 'vllm', vllm_structured_outputs_regex: str | None = None, vllm_gpu_memory_utilization: float | None = 0.55, vllm_mode: str = 'colocate', vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_tensor_parallel_size: int = 1, vllm_enable_sleep_mode: bool = False, ds3_gather_for_generation: bool = True, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, reward_weights: list[float] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/online_dpo/online_dpo_config.py#L23)

**Parameters:**

reward_model_path (`str`, *optional*) : Path to the reward model.

max_new_tokens (`int`, *optional*, defaults to `64`) : Maximum number of tokens to generate per completion.

max_length (`int`, *optional*, defaults to `512`) : Maximum total length of the sequence (prompt + completion) used to compute log probabilities. If the sequence exceeds this limit, the leftmost tokens will be truncated to preserve as much of the completion as possible.

temperature (`float`, *optional*, defaults to `0.9`) : Temperature for sampling. The higher the temperature, the more random the completions.

missing_eos_penalty (`float`, *optional*) : Penalty applied to the score when the model fails to generate an EOS token. This is useful to encourage to generate completions shorter than the maximum length (`max_new_tokens`). The penalty must be a positive value.

beta (`float` or `list[float]`, *optional*, defaults to `0.1`) : Parameter controlling the deviation from the reference model. Higher β means less deviation from the reference model. For the IPO loss (`loss_type="ipo"`), β is the regularization parameter denoted by τ in the [paper](https://huggingface.co/papers/2310.12036). If a list of floats is provided then the β is selected for each new epoch and the last β is used for the rest of the epochs.

loss_type (`str`, *optional*, defaults to `"sigmoid"`) : Type of loss to use. Possible values are:  - `"sigmoid"`: sigmoid loss from the original [DPO](https://huggingface.co/papers/2305.18290) paper. - `"ipo"`: IPO loss from the [IPO](https://huggingface.co/papers/2310.12036) paper.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model and reference model.

**Parameters that control generation:**

top_p (`float`, *optional*, defaults to `1.0`) : Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to `1.0` to consider all tokens.

top_k (`int`, *optional*, defaults to `0`) : Number of highest probability vocabulary tokens to keep for top-k-filtering. If `0`, top-k-filtering is disabled and all tokens are considered.

min_p (`float`, *optional*) : Minimum token probability, which will be scaled by the probability of the most likely token. It must be a value between `0.0` and `1.0`. Typical values are in the `0.01-0.2` range.

repetition_penalty (`float`, *optional*, defaults to `1.0`) : Float that penalizes new tokens based on whether they appear in the prompt and the generated text so far. Values > `1.0` encourage the model to use new tokens, while values < `1.0` encourage the model to repeat tokens.

cache_implementation (`str`, *optional*) : Implementation of the cache method for faster generation when `use_vllm` is set to `False`.

generation_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to [GenerationConfig](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/text_generation#transformers.GenerationConfig) (if using transformers) or `SamplingParams` (if using vLLM) when sampling completions. This can be used to further customize the generation behavior, such as setting `suppress_tokens`, `num_beams`, etc. If it contains keys that conflict with the other generation parameters (like `min_p`, `top_p`, etc.), they will override them.

**Parameters that control generation acceleration powered by vLLM:**

use_vllm (`bool`, *optional*, defaults to `False`) : Whether to use vLLM for generating completions. If set to `True`, the trainer will use vLLM for generation instead of the default model.generate(). Requires `vllm` to be installed.

vllm_model_impl (`str`, *optional*, defaults to `"vllm"`) : Model implementation to use for vLLM. Must be one of `"transformers"` or `"vllm"`. `"transformers"`: Use the `transformers` backend for model implementation. `"vllm"`: Use the `vllm` library for model implementation.

vllm_mode (`str`, *optional*, defaults to `"colocate"`) : Mode to use for vLLM integration when `use_vllm` is set to `True`. Must be one of `"server"` or `"colocate"`.  - `"server"`: The trainer will send generation requests to a separate vLLM server. Make sure a vLLM server is running (start with `vllm serve`). - `"colocate"`: vLLM will run in the same process and share the training GPUs. This avoids the need for a separate server but may cause resource contention with training.

vllm_structured_outputs_regex (`str`, *optional*) : Regex for vLLM structured outputs. If `None` (default), structured outputs is disabled.

**Parameters that control the vLLM server (only used when `vllm_mode` is `"server"`):**

vllm_server_base_url (`str`, *optional*) : Base URL for the vLLM server (e.g., `"http://localhost:8000"`). If provided, `vllm_server_host` and `vllm_server_port` are ignored.

vllm_server_host (`str`, *optional*, defaults to `"0.0.0.0"`) : Host of the vLLM server to connect to. Ignored if `vllm_server_base_url` is provided.

vllm_server_port (`int`, *optional*, defaults to `8000`) : Port of the vLLM server to connect to. Ignored if `vllm_server_base_url` is provided.

vllm_server_timeout (`float`, *optional*, defaults to `240.0`) : Total timeout duration in seconds to wait for the vLLM server to be up. If the server is not up after the timeout, a `ConnectionError` is raised.

vllm_group_port (`int`, *optional*, defaults to `51216`) : Port number for the weight update group. This is used to communicate with the vLLM server. Unless the port is occupied, there is no need to change it.

**Parameters that control colocated vLLM execution (only used when `vllm_mode` is `"colocate"`):**

vllm_gpu_memory_utilization (`float`, *optional*, defaults to `0.55`) : Control the GPU memory utilization for vLLM. This setting only applies when `vllm_mode` is set to `"colocate"`. If you are using `vllm_mode="server"`, this parameter must be passed separately when launching the vLLM server via the `--vllm_gpu_memory_utilization` flag.

vllm_tensor_parallel_size (`int`, *optional*, defaults to `1`) : Control the tensor parallel size for vLLM. This setting only applies when `vllm_mode` is set to `"colocate"`. If you are using `vllm_mode="server"`, this parameter must be passed separately when launching the vLLM server via the `--vllm_tensor_parallel_size` flag.

vllm_enable_sleep_mode (`bool`, *optional*, defaults to `False`) : Enable vLLM sleep mode to offload weights/cache during the optimizer step. Keeps GPU memory usage low, but waking the engine adds host–device transfer latency.

**Other parameters:**

ds3_gather_for_generation (`bool`, *optional*, defaults to `True`) : This setting applies to DeepSpeed ZeRO-3. If enabled, the policy model weights are gathered for generation, improving generation speed. However, disabling this option allows training models that exceed the VRAM capacity of a single GPU, albeit at the cost of slower generation. Disabling this option is not compatible with vLLM generation.

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model from a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained). Also applied to reward-model and reward-tokenizer loads.

reward_weights (`list[float]`, *optional*) : Weights for combining multiple reward functions. Must match the number of reward functions. If `None`, all reward functions are equally weighted.

Configuration class for the [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer).

This class includes only the parameters that are specific to Online DPO training. For a full list of training
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
> - `learning_rate`: Defaults to `5e-7` instead of `5e-5`.
> - `remove_unused_columns`: Defaults to `False` instead of `True`.
