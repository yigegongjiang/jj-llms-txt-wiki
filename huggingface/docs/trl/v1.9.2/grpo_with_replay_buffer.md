# GRPO With Replay Buffer

This experimental trainer, trains a model with GRPO but replaces groups (and corresponding completions) that have 0 standard deviation with groups with high rewards and standard deviation that've been used to train a model in prior batches.

## Usage

```python
import torch
from trl.experimental.grpo_with_replay_buffer import GRPOWithReplayBufferConfig, GRPOWithReplayBufferTrainer
from datasets import load_dataset

dataset = load_dataset("trl-internal-testing/zen", "standard_prompt_only", split="train")

# Guarantee that some rewards have 0 std
def custom_reward_func(completions, **kwargs):
    if torch.rand(1).item() < 0.25:
        return [0] * len(completions)  # simulate some None rewards
    else:
        return torch.rand(len(completions)).tolist()

training_args = GRPOWithReplayBufferConfig(
    output_dir="./tmp",
    learning_rate=1e-4,
    per_device_train_batch_size=4,
    num_generations=4,
    max_completion_length=8,
    replay_buffer_size=8,
    report_to="none",
)

trainer = GRPOWithReplayBufferTrainer(
    model="trl-internal-testing/tiny-Qwen2ForCausalLM-2.5",
    reward_funcs=[custom_reward_func],
    args=training_args,
    train_dataset=dataset,
)

previous_trainable_params = {n: param.clone() for n, param in trainer.model.named_parameters()}

trainer.train()
```

## GRPOWithReplayBufferTrainer[[trl.experimental.grpo_with_replay_buffer.GRPOWithReplayBufferTrainer]]

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

## GRPOWithReplayBufferConfig[[trl.experimental.grpo_with_replay_buffer.GRPOWithReplayBufferConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool | None = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "router_aux_loss_coef", "val": ": float = 0.001"}, {"name": "disable_dropout", "val": ": bool = False"}, {"name": "cast_lm_head_to_fp32", "val": ": bool = False"}, {"name": "num_generations", "val": ": int | None = 8"}, {"name": "num_generations_eval", "val": ": int | None = None"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "shuffle_dataset", "val": ": bool | None = True"}, {"name": "pad_to_multiple_of", "val": ": int | None = None"}, {"name": "generation_batch_size", "val": ": int | None = None"}, {"name": "steps_per_generation", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "generation_kwargs", "val": ": dict | None = None"}, {"name": "chat_template_kwargs", "val": ": dict | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "vllm_structured_outputs_regex", "val": ": str | None = None"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_gpu_memory_utilization", "val": ": float = 0.3"}, {"name": "vllm_max_model_length", "val": ": int | None = None"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "beta", "val": ": float = 0.0"}, {"name": "num_iterations", "val": ": int = 1"}, {"name": "epsilon", "val": ": float = 0.2"}, {"name": "delta", "val": ": float | None = None"}, {"name": "epsilon_high", "val": ": float | None = None"}, {"name": "sapo_temperature_neg", "val": ": float = 1.05"}, {"name": "sapo_temperature_pos", "val": ": float = 1.0"}, {"name": "vespo_k_pos", "val": ": float = 2.0"}, {"name": "vespo_lambda_pos", "val": ": float = 3.0"}, {"name": "vespo_k_neg", "val": ": float = 3.0"}, {"name": "vespo_lambda_neg", "val": ": float = 2.0"}, {"name": "importance_sampling_level", "val": ": str = 'token'"}, {"name": "reward_weights", "val": ": list[float] | None = None"}, {"name": "multi_objective_aggregation", "val": ": str = 'sum_then_normalize'"}, {"name": "scale_rewards", "val": ": str = 'group'"}, {"name": "loss_type", "val": ": str = 'dapo'"}, {"name": "mask_truncated_completions", "val": ": bool = False"}, {"name": "sync_ref_model", "val": ": bool = False"}, {"name": "ref_model_mixup_alpha", "val": ": float = 0.6"}, {"name": "ref_model_sync_steps", "val": ": int = 512"}, {"name": "top_entropy_quantile", "val": ": float = 1.0"}, {"name": "entropy_coef", "val": ": float = 0.0"}, {"name": "use_adaptive_entropy", "val": ": bool = False"}, {"name": "entropy_coef_min", "val": ": float = 0.0"}, {"name": "entropy_coef_max", "val": ": float = 1.0"}, {"name": "entropy_coef_delta", "val": ": float = 0.005"}, {"name": "entropy_target", "val": ": float = 0.2"}, {"name": "max_tool_calling_iterations", "val": ": int | None = None"}, {"name": "vllm_importance_sampling_correction", "val": ": bool = True"}, {"name": "vllm_importance_sampling_mode", "val": ": str = 'sequence_mask'"}, {"name": "vllm_importance_sampling_clip_max", "val": ": float | None = 3.0"}, {"name": "vllm_importance_sampling_clip_min", "val": ": float | None = None"}, {"name": "off_policy_mask_threshold", "val": ": float | None = None"}, {"name": "use_bias_correction_kl", "val": ": bool = False"}, {"name": "log_completions", "val": ": bool = False"}, {"name": "log_multimodal", "val": ": bool = True"}, {"name": "num_completions_to_print", "val": ": int | None = None"}, {"name": "log_unique_prompts", "val": ": bool = False"}, {"name": "log_completions_hub_repo", "val": ": str | None = None"}, {"name": "use_transformers_continuous_batching", "val": ": bool = False"}, {"name": "transformers_continuous_batching_config", "val": ": dict | None = None"}, {"name": "use_transformers_paged", "val": ": bool = False"}, {"name": "vllm_importance_sampling_cap", "val": ": float | None = None"}, {"name": "replay_buffer_size", "val": ": int = 64"}]}>

New Parameters:
replay_buffer_size (`int`, *optional*, defaults to `64`):
A cache that stores the rollouts with the highest advantage scores and variance per group. If a new
group has 0 variance, it is replaced with a group sampled from the replay buffer.

## ReplayBuffer[[trl.experimental.grpo_with_replay_buffer.ReplayBuffer]]

A simple replay buffer to store and sample previously seen rollouts.
