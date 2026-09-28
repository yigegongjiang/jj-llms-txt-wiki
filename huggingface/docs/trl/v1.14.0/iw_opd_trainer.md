# IW-OPD

In the paper [On the Position Bias of On-Policy Distillation](https://huggingface.co/papers/2606.22600), the authors introduce Importance-Weighted On-Policy Distillation (IW-OPD). IW-OPD addresses position bias in on-policy distillation by reweighting sampled-token updates according to accumulated teacher-student prefix discrepancy. Early tokens keep larger weights, while later tokens after high drift are downweighted.

To use IW-OPD, you can use the [experimental.iw_opd.IWOPDTrainer](/docs/trl/v1.14.0/en/iw_opd_trainer#trl.experimental.iw_opd.IWOPDTrainer) class in `trl.experimental.iw_opd`.

> [!NOTE]
> IW-OPD is currently part of the `trl.experimental` namespace. APIs may change without notice while the feature is iterated on.

## Usage

```python
from trl.experimental.iw_opd import IWOPDConfig, IWOPDTrainer

training_args = IWOPDConfig(
    distillation_objective="iw_opd",
    iw_opd_gamma=0.5,
)
trainer = IWOPDTrainer(
    model="Qwen/Qwen3-0.6B",
    teacher_model="...",
    train_dataset=...,
    args=training_args,
)
trainer.train()
```

IW-OPD is an on-policy objective: `distillation_objective="iw_opd"` requires `lmbda=1.0` (the default). `iw_opd_gamma` is the importance-weight amplification from Algorithm 1 of the paper.

## IWOPDTrainer[[trl.experimental.iw_opd.IWOPDTrainer]]

#### trl.experimental.iw_opd.IWOPDTrainer[[trl.experimental.iw_opd.IWOPDTrainer]]

```python
trl.experimental.iw_opd.IWOPDTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str, NoneType] = None, teacher_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str] = None, args: trl.experimental.iw_opd.iw_opd_config.IWOPDConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, peft_config: typing.Optional[ForwardRef('PeftConfig')] = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.14.0/trl/experimental/iw_opd/iw_opd_trainer.py#L349)

Trainer for Importance-Weighted On-Policy Distillation (IW-OPD).

IW-OPD ([On the Position Bias of On-Policy Distillation](https://huggingface.co/papers/2606.22600)) reweights the
sampled-token distillation signal by accumulated teacher-student prefix discrepancy, downweighting later tokens
whose supervision has drifted out of distribution. Select it with *distillation_objective="iw_opd"*.

This trainer is a frozen snapshot of the pre-refactor `DistillationTrainer`, kept as the home for IW-OPD (a
teacher-guided policy-gradient method that does not fit the stable, full-vocabulary `DistillationTrainer`).
It is not actively maintained and will not track that trainer's later improvements.

Supports:
- Generalized JSD loss (forward KL, reverse KL, or interpolated JSD via *beta*)
- Sampled-token IW-OPD objective (*distillation_objective="iw_opd"*)
- On-policy / off-policy mixing via *lmbda* (buffered across gradient accumulation)
- Local teacher model or external teacher via vLLM server
- Student on-policy generation via vLLM or model.generate()

#### train[[trl.experimental.iw_opd.IWOPDTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.14.0/transformers/trainer.py#L1408)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.experimental.iw_opd.IWOPDTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.14.0/transformers/trainer.py#L3930)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.iw_opd.IWOPDTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.14.0/transformers/trainer.py#L4177)

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

## IWOPDConfig[[trl.experimental.iw_opd.IWOPDConfig]]

#### trl.experimental.iw_opd.IWOPDConfig[[trl.experimental.iw_opd.IWOPDConfig]]

```python
trl.experimental.iw_opd.IWOPDConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-06, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, max_length: int | None = 1024, temperature: float = 1.0, lmbda: float = 1.0, beta: float = 1.0, distillation_objective: str = 'iw_opd', iw_opd_gamma: float = 0.5, iw_opd_epsilon: float = 1e-08, reverse_kl_top_1_mode: str = 'sampled', max_completion_length: int = 512, max_prompt_length: int | None = None, disable_dropout: bool = True, teacher_model_name_or_path: str | None = None, teacher_model_revision: str | None = None, teacher_model_init_kwargs: dict[str, typing.Any] | str | None = None, use_teacher_server: bool = False, teacher_model_server_url: str | None = None, loss_top_k: int = 1, loss_add_tail: bool = True, num_generations: int = 1, generation_batch_size: int | None = None, top_p: float = 0.95, top_k: int = 0, use_vllm: bool = False, vllm_mode: str = 'colocate', vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8001, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_gpu_memory_utilization: float = 0.3, vllm_tensor_parallel_size: int = 1, vllm_max_model_length: int | None = None, vllm_model_impl: str = 'vllm', vllm_structured_outputs_regex: str | None = None, vllm_sync_frequency: int = 1, vllm_enable_sleep_mode: bool = False, log_completions: bool = False, log_completions_steps: int = 100, num_completions_to_print: int | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.14.0/trl/experimental/iw_opd/iw_opd_config.py#L23)

**Parameters that control the model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for `AutoModelForCausalLM.from_pretrained`, used when the `model` argument of the trainer is provided as a string. The `revision` value is also used when loading the processing class.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained), for both the student and teacher.

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum total sequence length (prompt + completion) for tokenization and truncation.

**Parameters that control the distillation:**

temperature (`float`, *optional*, defaults to `1.0`) : Temperature for sampling during generation and for computing the distillation loss. Higher values produce softer probability distributions.

lmbda (`float`, *optional*, defaults to `1.0`) : Probability of using on-policy (student-generated) data for each gradient accumulation slice. A value of `0.0` means fully off-policy (dataset completions only), `1.0` means fully on-policy.

beta (`float`, *optional*, defaults to `1.0`) : Interpolation coefficient for the Generalized Jensen-Shannon Divergence loss. When `0.0`, the loss is the forward KL divergence. When `1.0`, the loss is the reverse KL divergence. When `0.5`, it is the standard JSD.

distillation_objective (`str`, *optional*, defaults to `"iw_opd"`) : Objective to optimize. `"iw_opd"` uses the sampled-token Importance-Weighted On-Policy Distillation objective. `"jsd"` keeps the generalized JSD/KL objective.

iw_opd_gamma (`float`, *optional*, defaults to `0.5`) : Importance-weight amplification for `distillation_objective="iw_opd"`.

iw_opd_epsilon (`float`, *optional*, defaults to `1e-8`) : Stabilizer used when normalizing IW-OPD prefix weights.

reverse_kl_top_1_mode (`str`, *optional*, defaults to `"sampled"`) : Selection rule for the reverse-KL top-1 token when `beta > 0` and `loss_top_k == 1`. `"sampled"` uses the actual completion token in the batch. `"argmax"` uses the student's highest-probability token. This setting does not affect the forward-KL support, which always uses the teacher's top-1 token. Ignored when `beta == 0` or `loss_top_k != 1`.

max_completion_length (`int`, *optional*, defaults to `512`) : Maximum number of tokens to generate per completion during on-policy generation.

max_prompt_length (`int` or `None`, *optional*) : Maximum number of tokens for the prompt. If `None`, auto-computed as `max_length - max_completion_length`. Prompts are truncated according to the tokenizer's `truncation_side` setting.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the student model during training.

**Parameters that control the teacher model:**

teacher_model_name_or_path (`str` or `None`, *optional*) : Model name or path for the teacher model. Used when the teacher is loaded locally.

teacher_model_revision (`str` or `None`, *optional*) : Model revision of the teacher model (e.g., branch name, tag, or commit hash).

teacher_model_init_kwargs (`dict[str, Any]` or `None`, *optional*) : Keyword arguments passed to `AutoModelForCausalLM.from_pretrained` when instantiating the teacher model from a string.

use_teacher_server (`bool`, *optional*, defaults to `False`) : Whether to use an external vLLM teacher server instead of a local teacher model.

teacher_model_server_url (`str` or `None`, *optional*) : Base URL of a vLLM server hosting the teacher model (e.g., `"http://localhost:8000"`). When set, teacher logprobs are fetched from the server instead of running a local forward pass when `use_teacher_server=True`.

loss_top_k (`int`, *optional*, defaults to `1`) : Number of top tokens to use when computing the JSD/KL loss. Both student and teacher distributions are restricted to these K tokens and re-normalized before computing divergence. If 0, the full vocabulary is used. For local teachers, the general support rule is teacher top-k for forward KL, student top-k for reverse KL, and the union for mixed JSD. When `beta > 0` and `loss_top_k == 1`, the forward support still uses the teacher's top-1 token, while the reverse top-1 token is controlled by `reverse_kl_top_1_mode`. When `use_teacher_server=True`, the pure forward path (`beta=0`) requires this to be positive and uses the teacher's top-k logprobs for the forward term. When `beta > 0`, server-backed distillation requires `loss_top_k == 1` and only supports `"sampled"` reverse top-1 tokens.

loss_add_tail (`bool`, *optional*, defaults to `True`) : Whether to append a tail bucket that represents the remaining probability mass outside the selected top-k support when computing the loss.

**Parameters that control on-policy generation:**

num_generations (`int`, *optional*, defaults to `1`) : Number of completions to generate per prompt during on-policy generation.

generation_batch_size (`int` or `None`, *optional*) : Number of unique prompts per worker per optimizer step. If `None`, computed from `(per_device_train_batch_size * gradient_accumulation_steps) // num_generations`.

top_p (`float`, *optional*, defaults to `0.95`) : Top-p (nucleus) sampling parameter for on-policy generation.

top_k (`int`, *optional*, defaults to `0`) : Top-k sampling parameter for on-policy generation. `0` disables top-k filtering.

**Parameters that control vLLM for student generation:**

use_vllm (`bool`, *optional*, defaults to `False`) : Whether to use vLLM for generating on-policy completions from the student model.

vllm_mode (`str`, *optional*, defaults to `"colocate"`) : Mode for student vLLM integration. Either `"server"` or `"colocate"`.

vllm_server_base_url (`str` or `None`, *optional*) : Base URL for the student vLLM server. If provided, `vllm_server_host` and `vllm_server_port` are ignored.

vllm_server_host (`str`, *optional*, defaults to `"0.0.0.0"`) : Host of the student vLLM server.

vllm_server_port (`int`, *optional*, defaults to `8001`) : Port of the student vLLM server.

vllm_server_timeout (`float`, *optional*, defaults to `240.0`) : Timeout for connecting to the student vLLM server.

vllm_group_port (`int`, *optional*, defaults to `51216`) : Port for the vLLM weight-update group (NCCL communicator).

vllm_gpu_memory_utilization (`float`, *optional*, defaults to `0.3`) : GPU memory utilization for the colocated student vLLM engine.

vllm_tensor_parallel_size (`int`, *optional*, defaults to `1`) : Tensor parallel size for the colocated student vLLM engine.

vllm_max_model_length (`int` or `None`, *optional*) : Maximum model sequence length for the colocated vLLM engine.

vllm_model_impl (`str`, *optional*, defaults to `"vllm"`) : Model implementation backend for vLLM. Use `"vllm"` or `"transformers"`.

vllm_structured_outputs_regex (`str` or `None`, *optional*) : Regex pattern for vLLM structured outputs.

vllm_sync_frequency (`int`, *optional*, defaults to `1`) : Frequency (in training steps) to synchronize student model weights to the vLLM engine.

vllm_enable_sleep_mode (`bool`, *optional*, defaults to `False`) : Enable vLLM sleep mode to offload student weights during the optimizer step.

**Parameters that control logging:**

log_completions (`bool`, *optional*, defaults to `False`) : Whether to log a sample of (prompt, completion) pairs every `log_completions_steps` steps. If `rich` is installed, it prints the sample. If `wandb` and/or `trackio` logging is enabled, it logs it to `wandb` and/or `trackio`.

log_completions_steps (`int`, *optional*, defaults to `100`) : Number of steps between logging completions. Only used if `log_completions` is `True`.

num_completions_to_print (`int` or `None`, *optional*) : Number of completions to print. If `None`, all completions are logged.

Configuration class for the [experimental.iw_opd.IWOPDTrainer](/docs/trl/v1.14.0/en/iw_opd_trainer#trl.experimental.iw_opd.IWOPDTrainer).

Extends [TrainingArguments](https://huggingface.co/docs/transformers/v5.17.0/en/main_classes/trainer#transformers.TrainingArguments) with parameters specific to knowledge distillation. This config is
independent of [SFTConfig](/docs/trl/v1.14.0/en/sft_trainer#trl.SFTConfig) — all necessary fields are declared here.

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.17.0/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.
