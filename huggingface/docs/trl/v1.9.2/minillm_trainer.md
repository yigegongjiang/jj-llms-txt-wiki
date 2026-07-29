# MiniLLM Trainer

[![All_models-MiniLLM-blue](https://img.shields.io/badge/All_models-MiniLLM-blue)](https://huggingface.co/models?other=minillm,trl)

## Overview

TRL supports the MiniLLM Trainer for distilling large language models into smaller ones using reverse KLD for better precision, quality, and performance, as described in the paper [Knowledge Distillation of Large Language Models](https://huggingface.co/papers/2306.08543) by [Yuxian Gu](https://huggingface.co/t1101675), [Li Dong](https://huggingface.co/unilm), [Furu Wei](https://huggingface.co/thegenerality), and Minlie Huang.
The abstract from the paper is the following:

> Knowledge Distillation (KD) is a promising technique for reducing the high computational demand of large language models (LLMs). However, previous KD methods are primarily applied to white-box classification models or training small models to imitate black-box model APIs like ChatGPT. How to effectively distill the knowledge from white-box generative LLMs is still under-explored, which becomes more and more important with the prosperity of LLMs. In this work, we propose MiniLLM that distills smaller language models from generative larger language models. We first replace the forward Kullback-Leibler divergence (KLD) objective in the standard KD approaches with reverse KLD, which is more suitable for KD on generative language models, to prevent the student model from overestimating the low-probability regions of the teacher distribution. Then, we derive an effective optimization approach to learn this objective. Extensive experiments in the instruction-following setting show that the MiniLLM models generate more precise responses with the higher overall quality, lower exposure bias, better calibration, and higher long-text generation performance. Our method is also scalable for different model families with 120M to 13B parameters. We will release our code and model checkpoints at https://aka.ms/MiniLLM.

This post-training method was contributed by [Yuxian Gu](https://huggingface.co/t1101675).

It is a generalized version of [Think Machine Lab's On-Policy Distillation](https://thinkingmachines.ai/blog/on-policy-distillation/), with the option to add distribution-level single-step distillation signals (like GKD when `beta=1`) and long-context reverse KLD signals.

$$
\begin{align}
L_{\text{MiniLLM}}&=\alpha_1\mathbb{E}_{x\sim \pi_{\theta}}\sum_{t'=t}^{|x|}\frac{\gamma^{t'-t}}{\sum_{t'}\gamma^{t'-t}}\left[\log \frac{\pi_{\theta}(x_{t'+1}|x_{1..t'})}{\pi_{\text{teacher}}(x_{t'+1}|x_{1..t'})}\right] \\
&+ \alpha_2\mathbb{E}_{x\sim \pi_{\theta}} \text{KL}\left[\pi_\theta(\cdot|x_{1..t})||\pi_{\text{teacher}}(\cdot | x_{1..t})\right].
\end{align}
$$

When  \\( \alpha_1=1 \\), \\( \alpha_2=0 \\), \\( \gamma=0 \\), which corresponds to

```python
from trl.experimental.minillm import MiniLLMConfig

training_args = MiniLLMConfig(
    rkl_advantage=True,
    single_step_decomposition=False,
    gamma=0.0
)
```

\\( L_{\text{MiniLLM}} \\) becomes the on-policy KD implemented in [Tinker](https://github.com/thinking-machines-lab/tinker-cookbook/blob/5d08be6d130596b7bedd02197861c41fa81ea436/tinker_cookbook/distillation/train_on_policy.py#L88):

$$
L_{\text{tinker}}=\mathbb{E}_{x\sim \pi_{\theta}}\left[\log \frac{\pi_{\theta}(x_{t'+1}|x_{1..t'})}{\pi_{\text{teacher}}(x_{t'+1}|x_{1..t'})}\right].
$$

When \\( \alpha_1=0 \\), \\( \alpha_2=1 \\), which corresponds to

```python
from trl.experimental.minillm import MiniLLMConfig

training_args = MiniLLMConfig(
    rkl_advantage=False,
    single_step_decomposition=True
)
```

\\( L_{\text{MiniLLM}} \\) becomes the reverse KLD version of the GKD loss as in [GKD Trainer](gkd_trainer):

$$
L_{\text{GKD-RKL}}=\mathbb{E}_{x\sim \pi_{\theta}} \text{KL}\left[\pi_\theta(\cdot|x_{1..t})||\pi_{\text{teacher}}(\cdot | x_{1..t})\right].
$$

## MiniLLMTrainer[[trl.experimental.minillm.MiniLLMTrainer]]

- **model** (`str | PreTrainedModel`) --
  Model to be trained. Can be either:

  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a
    path to a *directory* containing model weights saved using
    [save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded
    using [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) with the keyword arguments in
    `args.model_init_kwargs`.
  - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported.
- **teacher_model** (`PreTrainedModel | nn.Module | str`) --
  Teacher model used for knowledge distillation. Instantiated similarly to `model`.
- **reward_funcs** (`RewardFunc | list[RewardFunc]`, *optional*) --
  Reward functions to be used for computing the rewards. To compute the rewards, we call all the reward
  functions with the prompts and completions and sum the rewards. Can be either:

  - A single reward function, such as:
    - A string: The *model ID* of a pretrained model hosted inside a model repo on huggingface.co, or a
    path to a *directory* containing model weights saved using
    [save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded
    using [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification.from_pretrained) with `num_labels=1` and the
    keyword arguments in `args.model_init_kwargs`.
    - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) object: Only sequence classification models are supported.
    - A custom reward function: The function is provided with the prompts and the generated completions,
      plus any additional columns in the dataset. It should return a list of rewards. Custom reward
      functions can also return `None` when the reward is not applicable to those samples. This is useful
      for multi-task training where different reward functions apply to different types of samples. When a
      reward function returns `None` for a sample, that reward function is excluded from the reward
      calculation for that sample. For more details, see [Using a custom reward
      function](#using-a-custom-reward-function).

      The trainer's state is also passed to the reward function. The trainer's state is an instance of
      [TrainerState](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/callback#transformers.TrainerState) and can be accessed by accessing the `trainer_state` argument to the
      reward function's signature.
  - A list of reward functions, where each item can independently be any of the above types. Mixing different
  types within the list (e.g., a string model ID and a custom reward function) is allowed.
- **args** ([experimental.minillm.MiniLLMConfig](/docs/trl/v1.9.2/en/minillm_trainer#trl.experimental.minillm.MiniLLMConfig), *optional*) --
  Configuration for this trainer. If `None`, a default configuration is used.
- **train_dataset** (`Dataset` or `IterableDataset`) --
  Dataset to use for training. It must include a column `"prompt"`. Any additional columns in the dataset is
  ignored. The format of the samples can be either:

  - [Standard](dataset_formats#standard): Each sample contains plain text.
  - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role
    and content).
- **eval_dataset** (`Dataset`, `IterableDataset` or `dict[str, Dataset | IterableDataset]`) --
  Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
  Processing class used to process the data. The padding side must be set to "left". If `None`, the
  processing class is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained). A
  padding token, `tokenizer.pad_token`, must be set. If the processing class has not set a padding token,
  `tokenizer.eos_token` will be used as the default.
- **reward_processing_classes** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or `list[PreTrainedTokenizerBase]`, *optional*) --
  Processing classes corresponding to the reward functions specified in `reward_funcs`. Can be either:

  - A single processing class: Used when `reward_funcs` contains only one reward function.
  - A list of processing classes: Must match the order and length of the reward functions in `reward_funcs`.
  If set to `None`, or if an element of the list corresponding to a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) is
  `None`, the tokenizer for the model is automatically loaded using
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained). For elements in `reward_funcs` that are custom reward
  functions (not [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)), the corresponding entries in `reward_processing_classes`
  are ignored.
- **callbacks** (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) --
  List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed
  in [here](https://huggingface.co/docs/transformers/main_classes/callback).

  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.Trainer.remove_callback)
  method.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`, *optional*, defaults to `(None, None)`) --
  A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your
  model and a scheduler given by `get_linear_schedule_with_warmup` controlled by `args`.
- **peft_config** (`PeftConfig`, *optional*) --
  PEFT configuration used to wrap the model. If `None`, the model is not wrapped.
- **rollout_func** (`RolloutFunc`, *optional*) --
  Function to use for generating completions. It must take prompts, args, and processing_class as parameters
  and return a dict with `"prompt_ids"`, `"completion_ids"`, and `"logprobs"` fields. Any other fields that
  are forwarded to the reward functions. This feature is experimental and may change or be removed at any
  time without prior notice.

Trainer for the Knowledge Distillation of Language Models (MiniLLM) method. This algorithm was initially proposed
in the paper [Knowledge Distillation of Large Language Models](https://huggingface.co/papers/2306.08543).

Example:

```python
>>> from datasets import load_dataset
>>> from trl.experimental.minillm import MiniLLMTrainer

>>> dataset = load_dataset("trl-lib/tldr", split="train")

>>> trainer = MiniLLMTrainer(
...     model="Qwen/Qwen3-0.6B",
...     teacher_model="Qwen/Qwen3-1.7B",
...     train_dataset=dataset,
... )
>>> trainer.train()
```

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

## MiniLLMConfig[[trl.experimental.minillm.MiniLLMConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool | None = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "router_aux_loss_coef", "val": ": float = 0.001"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "cast_lm_head_to_fp32", "val": ": bool = False"}, {"name": "num_generations", "val": ": int | None = 8"}, {"name": "num_generations_eval", "val": ": int | None = None"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "shuffle_dataset", "val": ": bool | None = True"}, {"name": "pad_to_multiple_of", "val": ": int | None = None"}, {"name": "generation_batch_size", "val": ": int | None = None"}, {"name": "steps_per_generation", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "generation_kwargs", "val": ": dict | None = None"}, {"name": "chat_template_kwargs", "val": ": dict | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "vllm_structured_outputs_regex", "val": ": str | None = None"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_gpu_memory_utilization", "val": ": float = 0.3"}, {"name": "vllm_max_model_length", "val": ": int | None = None"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "beta", "val": ": float = 0.0"}, {"name": "num_iterations", "val": ": int = 1"}, {"name": "epsilon", "val": ": float = 0.2"}, {"name": "delta", "val": ": float | None = None"}, {"name": "epsilon_high", "val": ": float | None = None"}, {"name": "sapo_temperature_neg", "val": ": float = 1.05"}, {"name": "sapo_temperature_pos", "val": ": float = 1.0"}, {"name": "vespo_k_pos", "val": ": float = 2.0"}, {"name": "vespo_lambda_pos", "val": ": float = 3.0"}, {"name": "vespo_k_neg", "val": ": float = 3.0"}, {"name": "vespo_lambda_neg", "val": ": float = 2.0"}, {"name": "importance_sampling_level", "val": ": str = 'token'"}, {"name": "reward_weights", "val": ": list[float] | None = None"}, {"name": "multi_objective_aggregation", "val": ": str = 'sum_then_normalize'"}, {"name": "scale_rewards", "val": ": str = 'group'"}, {"name": "loss_type", "val": ": str = 'dapo'"}, {"name": "mask_truncated_completions", "val": ": bool = False"}, {"name": "sync_ref_model", "val": ": bool = False"}, {"name": "ref_model_mixup_alpha", "val": ": float = 0.6"}, {"name": "ref_model_sync_steps", "val": ": int = 512"}, {"name": "top_entropy_quantile", "val": ": float = 1.0"}, {"name": "entropy_coef", "val": ": float = 0.0"}, {"name": "use_adaptive_entropy", "val": ": bool = False"}, {"name": "entropy_coef_min", "val": ": float = 0.0"}, {"name": "entropy_coef_max", "val": ": float = 1.0"}, {"name": "entropy_coef_delta", "val": ": float = 0.005"}, {"name": "entropy_target", "val": ": float = 0.2"}, {"name": "max_tool_calling_iterations", "val": ": int | None = None"}, {"name": "vllm_importance_sampling_correction", "val": ": bool = True"}, {"name": "vllm_importance_sampling_mode", "val": ": str = 'sequence_mask'"}, {"name": "vllm_importance_sampling_clip_max", "val": ": float | None = 3.0"}, {"name": "vllm_importance_sampling_clip_min", "val": ": float | None = None"}, {"name": "off_policy_mask_threshold", "val": ": float | None = None"}, {"name": "use_bias_correction_kl", "val": ": bool = False"}, {"name": "log_completions", "val": ": bool = False"}, {"name": "log_multimodal", "val": ": bool = True"}, {"name": "num_completions_to_print", "val": ": int | None = None"}, {"name": "log_unique_prompts", "val": ": bool = False"}, {"name": "log_completions_hub_repo", "val": ": str | None = None"}, {"name": "use_transformers_continuous_batching", "val": ": bool = False"}, {"name": "transformers_continuous_batching_config", "val": ": dict | None = None"}, {"name": "use_transformers_paged", "val": ": bool = False"}, {"name": "vllm_importance_sampling_cap", "val": ": float | None = None"}, {"name": "teacher_model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "rkl_advantage", "val": ": bool = True"}, {"name": "single_step_decomposition", "val": ": bool = True"}, {"name": "kd_temperature", "val": ": float = 1.0"}, {"name": "gamma", "val": ": float = 0.0"}, {"name": "length_normalization", "val": ": bool = True"}]}>
- **teacher_model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the teacher model
  from a string.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model.
- **rkl_advantage** (`bool`, *optional*, defaults to `True`) --
  Whether to add the reverse KL advantage to the reward advantage.
- **single_step_decomposition** (`bool`, *optional*, defaults to `True`) --
  Whether to use single-step decomposition for the KL divergence computation.
- **kd_temperature** (`float`, *optional*, defaults to `1.0`) --
  Temperature for knowledge distillation. Higher temperatures produce softer probability distributions over
  classes.
- **gamma** (`float`, *optional*, defaults to `0.0`) --
  Discount factor for future rewards in reinforcement learning.
- **length_normalization** (`bool`, *optional*, defaults to `True`) --
  Whether to apply length normalization to the rewards.

Configuration class for `MiniLLMTrainer`.

This class includes only the parameters that are specific to MiniLLM training. For a full list of training
arguments, please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments) and [GRPOConfig](/docs/trl/v1.9.2/en/grpo_trainer#trl.GRPOConfig) documentation.
