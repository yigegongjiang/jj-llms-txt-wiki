# A2PO

[![model badge](https://img.shields.io/badge/All_models-A2PO-blue)](https://huggingface.co/models?other=a2po,trl)

TRL supports A\*-PO (Optimal Advantage Regression) as described in the paper [Accelerating RL for LLM Reasoning with Optimal Advantage Regression](https://huggingface.co/papers/2505.20686) by Kianté Brantley, Mingyu Chen, Zhaolin Gao, Jason D. Lee, Wen Sun, Wenhao Zhan, and Xuezhou Zhang.

The abstract from the paper is the following:

> Reinforcement learning (RL) has emerged as a powerful tool for fine-tuning large language models (LLMs) to improve complex reasoning abilities. However, state-of-the-art policy optimization methods often suffer from high computational overhead and memory consumption, primarily due to the need for multiple generations per prompt and the reliance on critic networks or advantage estimates of the current policy. In this paper, we propose A\*-PO, a novel two-stage policy optimization framework that directly approximates the optimal advantage function and enables efficient training of LLMs for reasoning tasks. In the first stage, we leverage offline sampling from a reference policy to estimate the optimal value function V\*, eliminating the need for costly online value estimation. In the second stage, we perform on-policy updates using a simple least-squares regression loss with only a single generation per prompt. Theoretically, we establish performance guarantees and prove that the KL-regularized RL objective can be optimized without requiring complex exploration strategies. Empirically, A\*-PO achieves competitive performance across a wide range of mathematical reasoning benchmarks, while reducing training time by up to 2× and peak memory usage by over 30% compared to PPO, GRPO, and REBEL.

## Usage

A\*-PO assumes a **binary, verifiable reward** (`r ∈ {0, 1}`) and runs in two stages:

1. **Offline value estimation.** Before training, `num_value_samples` completions are sampled from the reference policy for every prompt and scored with `reward_funcs`. The optimal value `V*(x) = β₁·log(mean_i exp(r(x, yᵢ)/β₁))` is estimated and cached per prompt.
2. **On-policy regression.** During training, a single completion is generated per prompt from the current policy. The loss is the squared error between the implicit reward `β₂·log(π(y|x)/π_ref(y|x))` and the optimal advantage `r(x, y) − V*(x)`.

```python
from trl.experimental.a2po import A2POConfig, A2POTrainer

# A*-PO assumes a binary, verifiable reward in {0, 1}.
def reward_correct(completions, ground_truth, **kwargs):
    return [float(completion.strip() == truth) for completion, truth in zip(completions, ground_truth)]

training_args = A2POConfig(
    output_dir="Qwen2.5-0.5B-A2PO",
    num_value_samples=8,  # Stage 1: samples per prompt from the reference policy to estimate V*
    beta1=0.5,  # Stage 1: KL temperature for the V* estimate
    beta2=1e-3,  # Stage 2: KL temperature for the regression target
)
trainer = A2POTrainer(
    model="Qwen/Qwen2.5-0.5B",
    reward_funcs=reward_correct,
    args=training_args,
    train_dataset=...,
)
trainer.train()
```

Because `V*` is estimated entirely from reference-policy samples, A\*-PO cannot exceed the reference policy's Pass@K. The official implementation can be found at [ZhaolinGao/A-PO](https://github.com/ZhaolinGao/A-PO).

## A2POTrainer[[trl.experimental.a2po.A2POTrainer]]

- **model** (`PreTrainedModel` or `str`) --
  Model to be trained, or a model identifier (string) passed to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained).
- **reward_funcs** (`Callable` or `list[Callable]`) --
  Reward function(s). Each takes `prompts` and `completions` (plus dataset columns as keyword arguments) and
  returns a list of float rewards. When multiple are provided, their weighted sum (see
  `A2POConfig.reward_weights`) is the scalar reward `r`, which A*-PO assumes to be binary (in `{0, 1}`).
- **args** (`A2POConfig`, *optional*) --
  Configuration for this trainer. If `None`, a default configuration is used.
- **train_dataset** (`Dataset`, *optional*) --
  Training dataset. Must contain a `"prompt"` column.
- **eval_dataset** (`Dataset`, *optional*) --
  Evaluation dataset.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) --
  Processing class used to process the data. If `None`, it is loaded from the model's name with
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).
- **callbacks** (`list[~transformers.TrainerCallback]`, *optional*) --
  List of callbacks to customize the training loop.
- **optimizers** (`tuple[~torch.optim.Optimizer, ~torch.optim.lr_scheduler.LambdaLR]`, *optional*, defaults to `(None, None)`) --
  Tuple containing the optimizer and the learning rate scheduler.

Trainer for the A*-PO (Optimal Advantage Regression) method, introduced in [Accelerating RL for LLM Reasoning with
Optimal Advantage Regression](https://huggingface.co/papers/2505.20686).

A*-PO runs in two stages:

1. **Offline value estimation.** Before training, `num_value_samples` completions are sampled from the reference
   policy for every training prompt and scored with `reward_funcs`. The optimal value is estimated as
   `V*(x) = beta1 * log(mean_i exp(r(x, y_i) / beta1))` and cached per prompt.
2. **On-policy regression.** During training, a single completion is generated per prompt from the current policy.
   The loss is the squared error between the implicit reward `beta2 * log(pi(y|x) / pi_ref(y|x))` and the optimal
   advantage estimate `r(x, y) - V*(x)`.

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

## A2POConfig[[trl.experimental.a2po.A2POConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "max_prompt_length", "val": ": int | None = 512"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int | None = None"}, {"name": "num_value_samples", "val": ": int = 8"}, {"name": "beta1", "val": ": float = 0.5"}, {"name": "filter_all_incorrect", "val": ": bool = True"}, {"name": "beta2", "val": ": float = 0.001"}, {"name": "reward_weights", "val": ": list[float] | None = None"}]}>
Parameters that control the model and reference model

- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model`
  argument of the `A2POTrainer` is provided as a string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

Parameters that control the data preprocessing

- **remove_unused_columns** (`bool`, *optional*, defaults to `False`) --
  Whether to only keep the column `"prompt"` in the dataset. If you use a custom reward function that
  requires any column other than `"prompts"` and `"completions"`, you should keep this to `False`.

Parameters that control generation

- **max_prompt_length** (`int` or `None`, *optional*, defaults to `512`) --
  Maximum length of the prompt. If the prompt is longer than this, it is left-truncated.
- **max_completion_length** (`int` or `None`, *optional*, defaults to `256`) --
  Maximum length of the generated completion.
- **temperature** (`float`, *optional*, defaults to `1.0`) --
  Sampling temperature, used in both Stage 1 and Stage 2 generation.
- **top_p** (`float`, *optional*, defaults to `1.0`) --
  Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to
  `1.0` to consider all tokens.
- **top_k** (`int` or `None`, *optional*) --
  Number of highest-probability vocabulary tokens to keep. If `None`, top-k filtering is disabled.

Parameters that control Stage 1 (offline optimal value estimation)

- **num_value_samples** (`int`, *optional*, defaults to `8`) --
  Number of samples drawn from the reference policy per prompt to estimate `V*`.
- **beta1** (`float`, *optional*, defaults to `0.5`) --
  KL temperature used to estimate `V*` in Stage 1.
- **filter_all_incorrect** (`bool`, *optional*, defaults to `True`) --
  Whether to drop prompts for which all reference samples are incorrect.

Parameters that control Stage 2 (on-policy regression)

- **beta2** (`float`, *optional*, defaults to `1e-3`) --
  KL temperature used in the Stage 2 regression target.
- **reward_weights** (`list[float]`, *optional*) --
  Weights for each reward function. Must match the number of reward functions. If `None`, all rewards are
  weighted equally with weight `1.0`.

Configuration class for the `A2POTrainer`.

This class includes only the parameters that are specific to A2PO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments).
