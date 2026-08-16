# GMPO

In the paper [Geometric-Mean Policy Optimization](https://huggingface.co/papers/2507.20673), the authors propose a GRPO variant that maximizes the *geometric* mean of the token-level importance ratios instead of the arithmetic mean. Because the geometric mean is far less sensitive to outlier ratios, the policy update is more stable and tolerates a much wider clipping range. Clipping is applied per token, in log space, and one-sided per the advantage sign (the standard PPO trust region) — crucially, *before* the geometric mean is taken.

To use GMPO, you can use the `GMPOTrainer` class in `trl.experimental.gmpo`.

## Usage

```python
from trl.experimental.gmpo import GMPOConfig, GMPOTrainer

training_args = GMPOConfig(
    epsilon=0.4,  # log-space clip range -> ratios clipped to (exp(-0.4), exp(0.4)); paper, Sec. 4
    beta=0.0,
)
trainer = GMPOTrainer(
    model="Qwen/Qwen3-0.6B",
    reward_funcs=...,
    train_dataset=...,
    args=training_args,
)
trainer.train()
```

In GMPO, clipping is applied to the per-token *log*-importance ratios (i.e. in log space) before the geometric mean is taken, so `epsilon` and `epsilon_high` are expressed in log space: the effective ratio clipping range is `(exp(-epsilon), exp(epsilon_high))`. The paper recommends a markedly wider range than GRPO/DAPO, `(exp(-0.4), exp(0.4))`, to encourage exploration.

## GMPOTrainer[[trl.experimental.gmpo.GMPOTrainer]]

#### trl.experimental.gmpo.GMPOTrainer[[trl.experimental.gmpo.GMPOTrainer]]

```python
trl.experimental.gmpo.GMPOTrainer(model, reward_funcs, args = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/gmpo/gmpo_trainer.py#L22)

Trainer for Geometric-Mean Policy Optimization (GMPO).

GMPO (https://huggingface.co/papers/2507.20673) is a GRPO variant that maximizes the *geometric* mean of the
token-level importance ratios instead of the arithmetic mean. Because the geometric mean is far less sensitive to
outlier ratios, the policy update is more stable and a much wider clipping range can be used.

The only change w.r.t. [GRPOTrainer](/docs/trl/v1.10.0/en/gspo_token#trl.GRPOTrainer) is `_compute_loss`. Everything else (generation, reward computation, weight
syncing, metric logging) is inherited unchanged

#### train[[trl.experimental.gmpo.GMPOTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L1347)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.experimental.gmpo.GMPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.gmpo.GMPOTrainer.push_to_hub]]

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

## GMPOConfig[[trl.experimental.gmpo.GMPOConfig]]

#### trl.experimental.gmpo.GMPOConfig[[trl.experimental.gmpo.GMPOConfig]]

```python
trl.experimental.gmpo.GMPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-06, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool | None = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, router_aux_loss_coef: float = 0.001, disable_dropout: bool = False, cast_lm_head_to_fp32: bool = False, num_generations: int | None = 8, num_generations_eval: int | None = None, max_completion_length: int | None = 512, ds3_gather_for_generation: bool = True, shuffle_dataset: bool | None = True, pad_to_multiple_of: int | None = None, generation_batch_size: int | None = None, steps_per_generation: int | None = None, temperature: float = 1.0, top_p: float = 1.0, top_k: int = 0, min_p: float | None = None, generation_kwargs: dict | None = None, chat_template_kwargs: dict | None = None, repetition_penalty: float = 1.0, cache_implementation: str | None = None, use_vllm: bool = False, vllm_mode: str = 'colocate', vllm_model_impl: str = 'vllm', vllm_enable_sleep_mode: bool = False, vllm_structured_outputs_regex: str | None = None, vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_gpu_memory_utilization: float = 0.3, vllm_max_model_length: int | None = None, vllm_tensor_parallel_size: int = 1, beta: float = 0.0, num_iterations: int = 1, epsilon: float = 0.4, delta: float | None = None, epsilon_high: float | None = None, sapo_temperature_neg: float = 1.05, sapo_temperature_pos: float = 1.0, vespo_k_pos: float = 2.0, vespo_lambda_pos: float = 3.0, vespo_k_neg: float = 3.0, vespo_lambda_neg: float = 2.0, importance_sampling_level: str = 'token', reward_weights: list[float] | None = None, multi_objective_aggregation: str = 'sum_then_normalize', scale_rewards: str = 'group', loss_type: str = 'dapo', mask_truncated_completions: bool = False, sync_ref_model: bool = False, ref_model_mixup_alpha: float = 0.6, ref_model_sync_steps: int = 512, top_entropy_quantile: float = 1.0, entropy_coef: float = 0.0, use_adaptive_entropy: bool = False, entropy_coef_min: float = 0.0, entropy_coef_max: float = 1.0, entropy_coef_delta: float = 0.005, entropy_target: float = 0.2, max_tool_calling_iterations: int | None = None, vllm_importance_sampling_correction: bool = True, vllm_importance_sampling_mode: str = 'sequence_mask', vllm_importance_sampling_clip_max: float | None = 3.0, vllm_importance_sampling_clip_min: float | None = None, off_policy_mask_threshold: float | None = None, use_bias_correction_kl: bool = True, log_completions: bool = False, log_multimodal: bool = True, num_completions_to_print: int | None = None, log_unique_prompts: bool = False, log_completions_hub_repo: str | None = None, use_transformers_continuous_batching: bool = False, transformers_continuous_batching_config: dict | None = None, use_transformers_paged: bool = False, vllm_importance_sampling_cap: float | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/gmpo/gmpo_config.py#L21)

**Parameters:**

epsilon (`float`, *optional*, defaults to `0.4`) : Lower-bound clipping value, expressed in log space. The lower bound of the per-token importance ratio is `exp(-epsilon)`.

epsilon_high (`float`, *optional*) : Upper-bound clipping value, expressed in log space. If `None`, it defaults to the value of `epsilon`. The upper bound of the per-token importance ratio is `exp(epsilon_high)`.

Configuration class for the `GMPOTrainer`.

`GMPOConfig` inherits every parameter from [GRPOConfig](/docs/trl/v1.10.0/en/grpo_trainer#trl.GRPOConfig); it only changes the meaning and default of the
clipping range. In GMPO, clipping is applied to the per-token *log*-importance ratios (i.e. in log space) before
the geometric mean is taken, so `epsilon` and `epsilon_high` are expressed in log space: the effective ratio
clipping range is `(exp(-epsilon), exp(epsilon_high))`. The [GMPO paper](https://huggingface.co/papers/2507.20673)
recommends a markedly wider range than GRPO/DAPO, `(exp(-0.4), exp(0.4))`, to encourage exploration.
