# XPO Trainer

[![model badge](https://img.shields.io/badge/All_models-XPO-blue)](https://huggingface.co/models?other=xpo,trl)

## Overview

Exploratory Preference Optimization (XPO) was proposed in the paper [Exploratory Preference Optimization: Harnessing Implicit Q*-Approximation for Sample-Efficient RLHF](https://huggingface.co/papers/2405.21046) by Tengyang Xie, Dylan J. Foster, Akshay Krishnamurthy, [Corby Rosset](https://huggingface.co/corbyrosset), [Ahmed Awadallah](https://huggingface.co/AhmedAwadallah), and Alexander Rakhlin. It is a simple online preference tuning method based on the DPO loss together with a reward model (RM). XPO augments the DPO objective with an exploration bonus allowing the method to explore outside the support of the initial model and human feedback data.

The abstract from the paper is the following:

> Reinforcement learning from human feedback (RLHF) has emerged as a central tool for language model alignment. We consider online exploration in RLHF, which exploits interactive access to human or AI feedback by deliberately encouraging the model to produce diverse, maximally informative responses. By allowing RLHF to confidently stray from the pre-trained model, online exploration offers the possibility of novel, potentially super-human capabilities, but its full potential as a paradigm for language model training has yet to be realized, owing to computational and statistical bottlenecks in directly adapting existing reinforcement learning techniques. We propose a new algorithm for online exploration in RLHF, Exploratory Preference Optimization (XPO), which is simple and practical -- a one-line change to (online) Direct Preference Optimization (DPO; Rafailov et al., 2023) -- yet enjoys the strongest known provable guarantees and promising empirical performance. XPO augments the DPO objective with a novel and principled exploration bonus, empowering the algorithm to explore outside the support of the initial model and human feedback data. In theory, we show that XPO is provably sample-efficient and converges to a near-optimal language model policy under natural exploration conditions, irrespective of whether the initial model has good coverage. Our analysis, which builds on the observation that DPO implicitly performs a form of Q*-approximation (or, Bellman error minimization), combines previously disparate techniques from language modeling and theoretical reinforcement learning in a serendipitous fashion through the perspective of KL-regularized Markov decision processes. Empirically, we find that XPO is more sample-efficient than non-exploratory DPO variants in a preliminary evaluation.

This post-training method was contributed by [Kashif Rasul](https://huggingface.co/kashif),  [Quentin Gallouédec](https://huggingface.co/qgallouedec) and [Lewis Tunstall](https://huggingface.co/lewtun).

> [!NOTE]
> XPO is currently experimental. The API may change without notice while the feature is iterated on.

## Quick start

This example demonstrates how to train a model using the XPO method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) as the base model and the [trl-lib/Qwen2-0.5B-Reward](https://huggingface.co/trl-lib/Qwen2-0.5B-Reward) reward model. We use the prompts from the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback). You can view the prompts in the dataset here:
<iframe
  src="https://huggingface.co/datasets/trl-lib/ultrafeedback-prompt/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_xpo.py
from datasets import load_dataset
from trl.experimental.xpo import XPOConfig, XPOTrainer
from transformers import AutoModelForCausalLM, AutoModelForSequenceClassification, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
reward_model = AutoModelForSequenceClassification.from_pretrained("trl-lib/Qwen2-0.5B-Reward", num_labels=1)
train_dataset = load_dataset("trl-lib/ultrafeedback-prompt", split="train")

training_args = XPOConfig(output_dir="Qwen2-0.5B-XPO")
trainer = XPOTrainer(
    model=model, reward_funcs=reward_model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset
)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_xpo.py
```

Distributed across 8 GPUs, the training takes approximately 1 hour.

To see how the [trained model](https://huggingface.co/trl-lib/Qwen2-0.5B-XPO) performs, you can use the [Transformers Chat CLI](https://huggingface.co/docs/transformers/quicktour#chat-with-text-generation-models).

$ transformers chat trl-lib/Qwen2-0.5B-XPO
&lt;quentin_gallouedec&gt;:
What is the best programming language?

&lt;trl-lib/Qwen2-0.5B-XPO&gt;:
The best programming language depends on individual preferences and familiarity with coding concepts. Some popular languages include Python, Java, C++, and JavaScript.

## Expected dataset type

XPO requires a [prompt-only dataset](dataset_formats#prompt-only). The [experimental.xpo.XPOTrainer](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset format. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Usage tips

### Encourage EOS token generation

When using a reward model, we may want the model to generate completions within a given length. During training, the model will generate completions up to the maximum length specified in the `max_new_tokens` argument of [experimental.xpo.XPOConfig](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOConfig). If you want to penalize the model for not generating an EOS token before reaching the maximum length, you can use the `missing_eos_penalty` argument of [experimental.xpo.XPOConfig](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOConfig):

```python
training_args = XPOConfig(..., max_new_tokens=128, missing_eos_penalty=1.0)
```

> [!WARNING]
> Make sure that the SFT model and reward model use the _same_ chat template and the same tokenizer. Otherwise, you may find the model completions are scored incorrectly during training.

### Logging Completions

To better understand your model's behavior during training, you can log sample completions periodically using the [LogCompletionsCallback](/docs/trl/v1.12.0/en/callbacks#trl.LogCompletionsCallback).

```python
trainer = XPOTrainer(..., eval_dataset=eval_dataset)
completions_callback = LogCompletionsCallback(trainer, num_prompts=8)
trainer.add_callback(completions_callback)
```

This callback logs the model's generated completions directly to Weights & Biases.

![Logged Completions](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/wandb_completions.png)

## Logged metrics

While training and evaluating, we record the following metrics:

* `loss/xpo`: The mean xpo part of the full loss.
* `loss/dpo`: The mean dpo part of the full loss.
* `objective/kl`: The mean KL divergence between the model and reference data.
* `objective/entropy`: The mean entropy of the model and reference data.
* `objective/model_scores`: The mean scores (according to the reward model) of the model completions.
* `objective/ref_scores`: The mean scores (according to the reward model) of the reference completions.
* `objective/scores_margin`: The mean score margin (according to the external reward model) between the chosen and rejected completions.
* `rewards/chosen`: The mean reward (according to XPO's DPO implicit reward model) of the chosen completions.
* `rewards/rejected`: The mean reward (according to XPO's DPO implicit reward model) of the rejected completions.
* `rewards/accuracies`: The accuracies of the XPO's implicit reward model.
* `rewards/margins`: The mean reward margin (according to online DPO's implicit reward model) between the chosen and rejected completions.
* `logps/chosen`: The mean log probabilities of the chosen completions.
* `logps/rejected`: The mean log probabilities of the rejected completions.
* `val/model_contain_eos_token`: The amount of times the model's output contains the eos token.
* `val/ref_contain_eos_token`: The amount of times the reference's output contains the eos token.
* `alpha`: The weight of the XPO loss term. Typically fixed, but can be made dynamic by passing a list to [experimental.xpo.XPOConfig](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOConfig).
* `beta`: The parameter that controls the weight of the loss term representing the deviation from the reference model. Typically fixed, but can be made dynamic by passing a list to [experimental.xpo.XPOConfig](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOConfig).

## XPOTrainer[[trl.experimental.xpo.XPOTrainer]]

#### trl.experimental.xpo.XPOTrainer[[trl.experimental.xpo.XPOTrainer]]

```python
trl.experimental.xpo.XPOTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module] = None, ref_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module] = None, reward_funcs: typing.Optional[torch.nn.Module] = None, args: trl.experimental.xpo.xpo_config.XPOConfig | None = None, data_collator: collections.abc.Callable | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, reward_processing_classes: transformers.tokenization_utils_base.PreTrainedTokenizerBase | list[transformers.tokenization_utils_base.PreTrainedTokenizerBase] | None = None, peft_config: PeftConfig | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/xpo/xpo_trainer.py#L47)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to train, preferably an `AutoModelForCausalLM`.

ref_model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel)) : Hugging Face transformer model with a casual language modelling head. Used for implicit reward computation and loss. If no reference model is provided, the trainer will create a reference model with the same architecture as the model to be optimized.

reward_funcs ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel)) : The reward model to score completions with, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification).

args ([experimental.xpo.XPOConfig](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOConfig)) : The XPO config arguments to use for training.

data_collator (`DataCollator`) : The data collator to use for training. If None is specified, the default data collator (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the maximum length of the sequences in the batch, given a dataset of paired sequences.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for training.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for evaluation.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. If provided, will be used to automatically process the inputs for the model, and it will be saved along the model to make it easier to rerun an interrupted training or reuse the fine-tuned model.

reward_processing_classes ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or `list[PreTrainedTokenizerBase]`, *optional*) : Processing classes corresponding to the reward functions specified in `reward_funcs`. Can be either:  - A single processing class: Used when `reward_funcs` contains only one reward function. - A list of processing classes: Must match the order and length of the reward functions in `reward_funcs`.  If set to `None`, the tokenizer for each model-based reward function is automatically loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : The peft config to use for training.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to metric values.

callbacks (`list[transformers.TrainerCallback]`) : The callbacks to use for training.

optimizers (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) : The optimizer and scheduler to use for training.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) : The function to use to preprocess the logits before computing the metrics.

Trainer for Exploratory Preference Optimization (XPO).

It is implemented as a subclass of [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer).

#### train[[trl.experimental.xpo.XPOTrainer.train]]

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

#### save_model[[trl.experimental.xpo.XPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.xpo.XPOTrainer.push_to_hub]]

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

## XPOConfig[[trl.experimental.xpo.XPOConfig]]

#### trl.experimental.xpo.XPOConfig[[trl.experimental.xpo.XPOConfig]]

```python
trl.experimental.xpo.XPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-07, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, reward_model_path: str | None = None, max_new_tokens: int = 64, max_length: int = 512, temperature: float = 0.9, top_p: float = 1.0, top_k: int = 0, min_p: float | None = None, repetition_penalty: float = 1.0, generation_kwargs: dict | None = None, cache_implementation: str | None = None, missing_eos_penalty: float | None = None, beta: list = <factory>, loss_type: str = 'sigmoid', disable_dropout: bool = True, use_vllm: bool = False, vllm_model_impl: str = 'vllm', vllm_structured_outputs_regex: str | None = None, vllm_gpu_memory_utilization: float | None = 0.55, vllm_mode: str = 'colocate', vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_tensor_parallel_size: int = 1, vllm_enable_sleep_mode: bool = False, ds3_gather_for_generation: bool = True, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, reward_weights: list[float] | None = None, alpha: list = <factory>)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/xpo/xpo_config.py#L21)

**Parameters:**

alpha (`float` or `list[float]`, *optional*, defaults to `1e-5`) : Weight of the XPO loss term. If a list of floats is provided then the alpha is selected for each new epoch and the last alpha is used for the rest of the epochs.

Configuration class for the [experimental.xpo.XPOTrainer](/docs/trl/v1.12.0/en/xpo_trainer#trl.experimental.xpo.XPOTrainer).

Subclass of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.12.0/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig) we can use all its arguments and add the following:
