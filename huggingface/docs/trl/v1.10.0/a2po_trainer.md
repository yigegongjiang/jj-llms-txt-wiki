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

#### trl.experimental.a2po.A2POTrainer[[trl.experimental.a2po.A2POTrainer]]

```python
trl.experimental.a2po.A2POTrainer(model: transformers.modeling_utils.PreTrainedModel | str, reward_funcs: collections.abc.Callable[..., list[float]] | list[collections.abc.Callable[..., list[float]]], args: trl.experimental.a2po.a2po_config.A2POConfig | None = None, train_dataset = None, eval_dataset = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | None = None, callbacks = None, optimizers = (None, None))
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/a2po/a2po_trainer.py#L48)

**Parameters:**

model (`PreTrainedModel` or `str`) : Model to be trained, or a model identifier (string) passed to [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained).

reward_funcs (`Callable` or `list[Callable]`) : Reward function(s). Each takes `prompts` and `completions` (plus dataset columns as keyword arguments) and returns a list of float rewards. When multiple are provided, their weighted sum (see `A2POConfig.reward_weights`) is the scalar reward `r`, which A*-PO assumes to be binary (in `{0, 1}`).

args (`A2POConfig`, *optional*) : Configuration for this trainer. If `None`, a default configuration is used.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), *optional*) : Training dataset. Must contain a `"prompt"` column.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), *optional*) : Evaluation dataset.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.15.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) : Processing class used to process the data. If `None`, it is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

callbacks (`list[~transformers.TrainerCallback]`, *optional*) : List of callbacks to customize the training loop.

optimizers (`tuple[~torch.optim.Optimizer, ~torch.optim.lr_scheduler.LambdaLR]`, *optional*, defaults to `(None, None)`) : Tuple containing the optimizer and the learning rate scheduler.

Trainer for the A*-PO (Optimal Advantage Regression) method, introduced in [Accelerating RL for LLM Reasoning with
Optimal Advantage Regression](https://huggingface.co/papers/2505.20686).

A*-PO runs in two stages:

1. **Offline value estimation.** Before training, `num_value_samples` completions are sampled from the reference
   policy for every training prompt and scored with `reward_funcs`. The optimal value is estimated as
   `V*(x) = beta1 * log(mean_i exp(r(x, y_i) / beta1))` and cached per prompt.
2. **On-policy regression.** During training, a single completion is generated per prompt from the current policy.
   The loss is the squared error between the implicit reward `beta2 * log(pi(y|x) / pi_ref(y|x))` and the optimal
   advantage estimate `r(x, y) - V*(x)`.

#### train[[trl.experimental.a2po.A2POTrainer.train]]

```python
train(*args, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/a2po/a2po_trainer.py#L359)

#### save_model[[trl.experimental.a2po.A2POTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.a2po.A2POTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L4041)

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

## A2POConfig[[trl.experimental.a2po.A2POConfig]]

#### trl.experimental.a2po.A2POConfig[[trl.experimental.a2po.A2POConfig]]

```python
trl.experimental.a2po.A2POConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-05, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict | None = None, trust_remote_code: bool = False, max_prompt_length: int | None = 512, max_completion_length: int | None = 256, temperature: float = 1.0, top_p: float = 1.0, top_k: int | None = None, num_value_samples: int = 8, beta1: float = 0.5, filter_all_incorrect: bool = True, beta2: float = 0.001, reward_weights: list[float] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/a2po/a2po_config.py#L21)

**Parameters that control the model and reference model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model` argument of the `A2POTrainer` is provided as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained).

**Parameters that control the data preprocessing:**

remove_unused_columns (`bool`, *optional*, defaults to `False`) : Whether to only keep the column `"prompt"` in the dataset. If you use a custom reward function that requires any column other than `"prompts"` and `"completions"`, you should keep this to `False`.

**Parameters that control generation:**

max_prompt_length (`int` or `None`, *optional*, defaults to `512`) : Maximum length of the prompt. If the prompt is longer than this, it is left-truncated.

max_completion_length (`int` or `None`, *optional*, defaults to `256`) : Maximum length of the generated completion.

temperature (`float`, *optional*, defaults to `1.0`) : Sampling temperature, used in both Stage 1 and Stage 2 generation.

top_p (`float`, *optional*, defaults to `1.0`) : Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to `1.0` to consider all tokens.

top_k (`int` or `None`, *optional*) : Number of highest-probability vocabulary tokens to keep. If `None`, top-k filtering is disabled.

**Parameters that control Stage 1 (offline optimal value estimation):**

num_value_samples (`int`, *optional*, defaults to `8`) : Number of samples drawn from the reference policy per prompt to estimate `V*`.

beta1 (`float`, *optional*, defaults to `0.5`) : KL temperature used to estimate `V*` in Stage 1.

filter_all_incorrect (`bool`, *optional*, defaults to `True`) : Whether to drop prompts for which all reference samples are incorrect.

**Parameters that control Stage 2 (on-policy regression):**

beta2 (`float`, *optional*, defaults to `1e-3`) : KL temperature used in the Stage 2 regression target.

reward_weights (`list[float]`, *optional*) : Weights for each reward function. Must match the number of reward functions. If `None`, all rewards are weighted equally with weight `1.0`.

Configuration class for the `A2POTrainer`.

This class includes only the parameters that are specific to A2PO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments).
