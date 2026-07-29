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

XPO requires a [prompt-only dataset](dataset_formats#prompt-only). The [experimental.xpo.XPOTrainer](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset format. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Usage tips

### Encourage EOS token generation

When using a reward model, we may want the model to generate completions within a given length. During training, the model will generate completions up to the maximum length specified in the `max_new_tokens` argument of [experimental.xpo.XPOConfig](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOConfig). If you want to penalize the model for not generating an EOS token before reaching the maximum length, you can use the `missing_eos_penalty` argument of [experimental.xpo.XPOConfig](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOConfig):

```python
training_args = XPOConfig(..., max_new_tokens=128, missing_eos_penalty=1.0)
```

> [!WARNING]
> Make sure that the SFT model and reward model use the _same_ chat template and the same tokenizer. Otherwise, you may find the model completions are scored incorrectly during training.

### Logging Completions

To better understand your model's behavior during training, you can log sample completions periodically using the [LogCompletionsCallback](/docs/trl/v1.9.2/en/callbacks#trl.LogCompletionsCallback).

```python
trainer = XPOTrainer(..., eval_dataset=eval_dataset)
completions_callback = LogCompletionsCallback(trainer, num_prompts=8)
trainer.add_callback(completions_callback)
```

This callback logs the model's generated completions directly to Weights & Biases.

![Logged Completions](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/wandb_completions.png)

## Example script

We provide an example script to train a model using the XPO method. The script is available in [`examples/scripts/xpo.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/xpo.py)

To test the XPO script with the [Qwen2.5 0.5B model](https://huggingface.co/trl-lib/Qwen/Qwen2.5-0.5B-Instruct) on the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback), run the following command:

```bash
python examples/scripts/xpo.py \
    --model_name_or_path Qwen/Qwen2.5-0.5B-Instruct \
    --reward_model_path trl-lib/Qwen2-0.5B-Reward \
    --dataset_name trl-lib/ultrafeedback-prompt \
    --learning_rate 5.0e-7 \
    --output_dir Qwen2.5-0.5B-XPO \
    --warmup_steps 0.1 \
    --push_to_hub
```

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
* `alpha`: The weight of the XPO loss term. Typically fixed, but can be made dynamic by passing a list to [experimental.xpo.XPOConfig](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOConfig).
* `beta`: The parameter that controls the weight of the loss term representing the deviation from the reference model. Typically fixed, but can be made dynamic by passing a list to [experimental.xpo.XPOConfig](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOConfig).

## XPOTrainer[[trl.experimental.xpo.XPOTrainer]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to train, preferably an `AutoModelForCausalLM`.
- **ref_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Hugging Face transformer model with a casual language modelling head. Used for implicit reward computation
  and loss. If no reference model is provided, the trainer will create a reference model with the same
  architecture as the model to be optimized.
- **reward_funcs** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The reward model to score completions with, preferably an
  [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification).
- **args** ([experimental.xpo.XPOConfig](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOConfig)) --
  The XPO config arguments to use for training.
- **data_collator** (`DataCollator`) --
  The data collator to use for training. If None is specified, the default data collator
  (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the
  maximum length of the sequences in the batch, given a dataset of paired sequences.
- **train_dataset** (`Dataset`) --
  The dataset to use for training.
- **eval_dataset** (`Dataset`) --
  The dataset to use for evaluation.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
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
  The peft config to use for training.
- **compute_metrics** (`Callable[[EvalPrediction], dict]`, *optional*) --
  The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to
  metric values.
- **callbacks** (`list[transformers.TrainerCallback]`) --
  The callbacks to use for training.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) --
  The optimizer and scheduler to use for training.
- **preprocess_logits_for_metrics** (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) --
  The function to use to preprocess the logits before computing the metrics.

Trainer for Exploratory Preference Optimization (XPO).

It is implemented as a subclass of [experimental.online_dpo.OnlineDPOTrainer](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOTrainer).

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

## XPOConfig[[trl.experimental.xpo.XPOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "reward_model_path", "val": ": str | None = None"}, {"name": "max_new_tokens", "val": ": int = 64"}, {"name": "max_length", "val": ": int = 512"}, {"name": "temperature", "val": ": float = 0.9"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "generation_kwargs", "val": ": dict | None = None"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "missing_eos_penalty", "val": ": float | None = None"}, {"name": "beta", "val": ": list = "}, {"name": "loss_type", "val": ": str = 'sigmoid'"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_structured_outputs_regex", "val": ": str | None = None"}, {"name": "vllm_gpu_memory_utilization", "val": ": float | None = 0.55"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "reward_weights", "val": ": list[float] | None = None"}, {"name": "alpha", "val": ": list = "}]}>
- **alpha** (`float` or `list[float]`, *optional*, defaults to `1e-5`) --
  Weight of the XPO loss term. If a list of floats is provided then the alpha is selected for each new epoch
  and the last alpha is used for the rest of the epochs.

Configuration class for the [experimental.xpo.XPOTrainer](/docs/trl/v1.9.2/en/xpo_trainer#trl.experimental.xpo.XPOTrainer).

Subclass of [experimental.online_dpo.OnlineDPOConfig](/docs/trl/v1.9.2/en/online_dpo_trainer#trl.experimental.online_dpo.OnlineDPOConfig) we can use all its arguments and add the following:
