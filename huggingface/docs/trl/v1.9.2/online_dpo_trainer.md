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

Online DPO only requires a [prompt-only dataset](dataset_formats#prompt-only) (unlike offline DPO, that expects [preference dataset](dataset_formats#preference)). The [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Usage tips

### Encourage EOS token generation

When using a reward model, we may want the model to generate completions within a given length. During training, the model will generate completions up to the maximum length specified in the `max_new_tokens` argument of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig). If you want to penalize the model for not generating an EOS token before reaching the maximum length, you can use the `missing_eos_penalty` argument of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig):

```python
training_args = OnlineDPOConfig(..., max_new_tokens=128, missing_eos_penalty=1.0)
```

### Logging Completions

To better understand your model's behavior during training, you can log sample completions periodically using the [LogCompletionsCallback](/docs/trl/v1.9.2/en/callbacks#trl.LogCompletionsCallback).

```python
trainer = OnlineDPOTrainer(..., eval_dataset=eval_dataset)
completions_callback = LogCompletionsCallback(trainer, num_prompts=8)
trainer.add_callback(completions_callback)
```

This callback logs the model's generated completions directly to Weights & Biases.

![Logged Completions](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/wandb_completions.png)

## Example script

We provide an example script to train a model using the online DPO method. The script is available in [`examples/scripts/dpo_online.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/dpo_online.py)

To test the online DPO script with the [Qwen2.5 0.5B model](https://huggingface.co/trl-lib/Qwen/Qwen2.5-0.5B-Instruct) on the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback), run the following command:

```bash
python examples/scripts/dpo_online.py \
    --model_name_or_path Qwen/Qwen2.5-0.5B-Instruct \
    --reward_model_path trl-lib/Qwen2-0.5B-Reward \
    --dataset_name trl-lib/ultrafeedback-prompt \
    --learning_rate 5.0e-7 \
    --output_dir Qwen2.5-0.5B-Online-DPO \
    --warmup_steps 0.1 \
    --push_to_hub
```

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
* `beta`: The parameter that controls the weight of the loss term representing the deviation from the reference model. Typically fixed, but can be made dynamic by passing a list to [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig).

## Benchmark experiments

To validate the online DPO implementation works, we ran experiments with the Pythia 1B, 2.8B, and 6.9B models on a single node of 8 x H100s. Here are the commands we used to run the experiments. We take the SFT / RM models directly from [The N+ Implementation Details of RLHF with PPO: A Case Study on TL;DR Summarization](https://huggingface.co/papers/2403.17031).

```shell
# 1B Online DPO experiment
accelerate launch --config_file examples/accelerate_configs/multi_gpu.yaml \
    examples/scripts/dpo_online.py \
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
    examples/scripts/dpo_online.py \
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
    examples/scripts/dpo_online.py \
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

- **model** (`str | nn.Module | PreTrainedModel`) --
  Model to be trained. Can be either:

  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a
    path to a *directory* containing model weights saved using
    [save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded
    using [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) with the keyword arguments in
    `args.model_init_kwargs`.
  - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported.
- **ref_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) or `torch.nn.Module` or `None`) --
  The reference model to use for training. If None is specified, the reference model will be created from the
  model.
- **reward_funcs** (`RewardFunc | list[RewardFunc]`) --
  Reward functions to be used for computing the rewards. To compute the rewards, we call all the reward
  functions with the prompts and completions and sum the rewards. Can be either:

  - A single reward function: Can be a string (path to model), a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel), or a
    custom callable function.
  - A list of reward functions: Must all be of compatible types.
- **args** ([experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig)) --
  The online DPO config arguments to use for training.
- **data_collator** (`DataCollator`) --
  The data collator to use for training. If None is specified, the default data collator
  (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the
  maximum length of the sequences in the batch, given a dataset of paired sequences.
- **train_dataset** (`Dataset` or `IterableDataset`) --
  The dataset to use for training.
- **eval_dataset** (`Dataset`, `IterableDataset` or `dict[str, Dataset | IterableDataset]`) --
  The dataset to use for evaluation.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
  Processing class used to process the data. If provided, will be used to automatically process the inputs
  for the model, and it will be saved along the model to make it easier to rerun an interrupted training or
  reuse the fine-tuned model.
- **reward_processing_classes** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or `list[PreTrainedTokenizerBase]`, *optional*) --
  Processing classes corresponding to the reward functions specified in `reward_funcs`. Can be either:

  - A single processing class: Used when `reward_funcs` contains only one reward function.
  - A list of processing classes: Must match the order and length of the reward functions in `reward_funcs`.

  If set to `None`, the tokenizer for each model-based reward function is automatically loaded using
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).
- **peft_config** (`PeftConfig`, *optional*) --
  PEFT configuration used to wrap the model. If `None`, the model is not wrapped.
- **compute_metrics** (`Callable[[EvalPrediction], dict]`, *optional*) --
  The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to
  metric values.
- **callbacks** (`list[transformers.TrainerCallback]`) --
  The callbacks to use for training.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) --
  The optimizer and scheduler to use for training.
- **preprocess_logits_for_metrics** (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) --
  The function to use to preprocess the logits before computing the metrics.

Initialize OnlineDPOTrainer.

- **resume_from_checkpoint** (`str` or `bool`, *optional*) --
  If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a
  `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance
  of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.
- **trial** (`optuna.Trial` or `dict[str, Any]`, *optional*) --
  The trial run or the hyperparameter dictionary for hyperparameter search.
- **ignore_keys_for_eval** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions for evaluation during the training.`~trainer_utils.TrainOutput`Object containing the global step count, training loss, and metrics.

Main training entry point.

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

- **commit_message** (`str`, *optional*, defaults to `"End of training"`) --
  Message to commit while pushing.
- **blocking** (`bool`, *optional*, defaults to `True`) --
  Whether the function should return only when the `git push` has finished.
- **token** (`str`, *optional*, defaults to `None`) --
  Token with write permission to overwrite Trainer's original args.
- **revision** (`str`, *optional*) --
  The git revision to commit from. Defaults to the head of the "main" branch.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments passed along to `~Trainer.create_model_card`.The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.

## OnlineDPOConfig[[trl.experimental.online_dpo.OnlineDPOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "reward_model_path", "val": ": str | None = None"}, {"name": "max_new_tokens", "val": ": int = 64"}, {"name": "max_length", "val": ": int = 512"}, {"name": "temperature", "val": ": float = 0.9"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "generation_kwargs", "val": ": dict | None = None"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "missing_eos_penalty", "val": ": float | None = None"}, {"name": "beta", "val": ": list = "}, {"name": "loss_type", "val": ": str = 'sigmoid'"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_structured_outputs_regex", "val": ": str | None = None"}, {"name": "vllm_gpu_memory_utilization", "val": ": float | None = 0.55"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "reward_weights", "val": ": list[float] | None = None"}]}>
- **reward_model_path** (`str`, *optional*) --
  Path to the reward model.
- **max_new_tokens** (`int`, *optional*, defaults to `64`) --
  Maximum number of tokens to generate per completion.
- **max_length** (`int`, *optional*, defaults to `512`) --
  Maximum total length of the sequence (prompt + completion) used to compute log probabilities. If the
  sequence exceeds this limit, the leftmost tokens will be truncated to preserve as much of the completion as
  possible.
- **temperature** (`float`, *optional*, defaults to `0.9`) --
  Temperature for sampling. The higher the temperature, the more random the completions.
- **missing_eos_penalty** (`float`, *optional*) --
  Penalty applied to the score when the model fails to generate an EOS token. This is useful to encourage to
  generate completions shorter than the maximum length (`max_new_tokens`). The penalty must be a positive
  value.
- **beta** (`float` or `list[float]`, *optional*, defaults to `0.1`) --
  Parameter controlling the deviation from the reference model. Higher β means less deviation from the
  reference model. For the IPO loss (`loss_type="ipo"`), β is the regularization parameter denoted by τ in
  the [paper](https://huggingface.co/papers/2310.12036). If a list of floats is provided then the β is
  selected for each new epoch and the last β is used for the rest of the epochs.
- **loss_type** (`str`, *optional*, defaults to `"sigmoid"`) --
  Type of loss to use. Possible values are:

  - `"sigmoid"`: sigmoid loss from the original [DPO](https://huggingface.co/papers/2305.18290) paper.
  - `"ipo"`: IPO loss from the [IPO](https://huggingface.co/papers/2310.12036) paper.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model and reference model.

Parameters that control generation

- **top_p** (`float`, *optional*, defaults to `1.0`) --
  Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to
  `1.0` to consider all tokens.
- **top_k** (`int`, *optional*, defaults to `0`) --
  Number of highest probability vocabulary tokens to keep for top-k-filtering. If `0`, top-k-filtering is
  disabled and all tokens are considered.
- **min_p** (`float`, *optional*) --
  Minimum token probability, which will be scaled by the probability of the most likely token. It must be a
  value between `0.0` and `1.0`. Typical values are in the `0.01-0.2` range.
- **repetition_penalty** (`float`, *optional*, defaults to `1.0`) --
  Float that penalizes new tokens based on whether they appear in the prompt and the generated text so far.
  Values > `1.0` encourage the model to use new tokens, while values < `1.0` encourage the model to repeat
  tokens.
- **cache_implementation** (`str`, *optional*) --
  Implementation of the cache method for faster generation when `use_vllm` is set to `False`.
- **generation_kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments to pass to [GenerationConfig](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/text_generation#transformers.GenerationConfig) (if using transformers) or
  `SamplingParams` (if using vLLM) when sampling completions. This can be used to further customize the
  generation behavior, such as setting `suppress_tokens`, `num_beams`, etc. If it contains keys that conflict
  with the other generation parameters (like `min_p`, `top_p`, etc.), they will override them.

Parameters that control generation acceleration powered by vLLM

- **use_vllm** (`bool`, *optional*, defaults to `False`) --
  Whether to use vLLM for generating completions. If set to `True`, the trainer will use vLLM for generation
  instead of the default model.generate(). Requires `vllm` to be installed.
- **vllm_model_impl** (`str`, *optional*, defaults to `"vllm"`) --
  Model implementation to use for vLLM. Must be one of `"transformers"` or `"vllm"`. `"transformers"`: Use
  the `transformers` backend for model implementation. `"vllm"`: Use the `vllm` library for model
  implementation.
- **vllm_mode** (`str`, *optional*, defaults to `"colocate"`) --
  Mode to use for vLLM integration when `use_vllm` is set to `True`. Must be one of `"server"` or
  `"colocate"`.

  - `"server"`: The trainer will send generation requests to a separate vLLM server. Make sure a TRL vLLM
    server is running (start with `trl vllm-serve`).
  - `"colocate"`: vLLM will run in the same process and share the training GPUs. This avoids the need for a
    separate server but may cause resource contention with training.
- **vllm_structured_outputs_regex** (`str`, *optional*) --
  Regex for vLLM structured outputs. If `None` (default), structured outputs is disabled.

Parameters that control the vLLM server (only used when `vllm_mode` is `"server"`)

- **vllm_server_base_url** (`str`, *optional*) --
  Base URL for the vLLM server (e.g., `"http://localhost:8000"`). If provided, `vllm_server_host` and
  `vllm_server_port` are ignored.
- **vllm_server_host** (`str`, *optional*, defaults to `"0.0.0.0"`) --
  Host of the vLLM server to connect to. Ignored if `vllm_server_base_url` is provided.
- **vllm_server_port** (`int`, *optional*, defaults to `8000`) --
  Port of the vLLM server to connect to. Ignored if `vllm_server_base_url` is provided.
- **vllm_server_timeout** (`float`, *optional*, defaults to `240.0`) --
  Total timeout duration in seconds to wait for the vLLM server to be up. If the server is not up after the
  timeout, a `ConnectionError` is raised.
- **vllm_group_port** (`int`, *optional*, defaults to `51216`) --
  Port number for the weight update group. This is used to communicate with the vLLM server. Unless the port
  is occupied, there is no need to change it.

Parameters that control colocated vLLM execution (only used when `vllm_mode` is `"colocate"`)

- **vllm_gpu_memory_utilization** (`float`, *optional*, defaults to `0.55`) --
  Control the GPU memory utilization for vLLM. This setting only applies when `vllm_mode` is set to
  `"colocate"`. If you are using `vllm_mode="server"`, this parameter must be passed separately when
  launching the vLLM server via the `--vllm_gpu_memory_utilization` flag.
- **vllm_tensor_parallel_size** (`int`, *optional*, defaults to `1`) --
  Control the tensor parallel size for vLLM. This setting only applies when `vllm_mode` is set to
  `"colocate"`. If you are using `vllm_mode="server"`, this parameter must be passed separately when
  launching the vLLM server via the `--vllm_tensor_parallel_size` flag.
- **vllm_enable_sleep_mode** (`bool`, *optional*, defaults to `False`) --
  Enable vLLM sleep mode to offload weights/cache during the optimizer step. Keeps GPU memory usage low, but
  waking the engine adds host–device transfer latency.

Other parameters

- **ds3_gather_for_generation** (`bool`, *optional*, defaults to `True`) --
  This setting applies to DeepSpeed ZeRO-3. If enabled, the policy model weights are gathered for generation,
  improving generation speed. However, disabling this option allows training models that exceed the VRAM
  capacity of a single GPU, albeit at the cost of slower generation. Disabling this option is not compatible
  with vLLM generation.
- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model from a
  string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained). Also applied to reward-model and reward-tokenizer
  loads.
- **reward_weights** (`list[float]`, *optional*) --
  Weights for combining multiple reward functions. Must match the number of reward functions. If `None`, all
  reward functions are equally weighted.

Configuration class for the [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer).

This class includes only the parameters that are specific to Online DPO training. For a full list of training
arguments, please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this
class may differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.14.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `5e-7` instead of `5e-5`.
> - `remove_unused_columns`: Defaults to `False` instead of `True`.
