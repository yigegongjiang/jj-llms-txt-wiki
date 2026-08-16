# SSD

Simple Self-Distillation (SSD) is described in [Embarrassingly Simple Self-Distillation Improves Code Generation](https://huggingface.co/papers/2604.01193).

SSD samples completions from the model at a training-time temperature and truncation configuration, then fine-tunes on those raw, unverified samples with standard cross-entropy loss. It requires no reward model, verifier, teacher model, or reinforcement learning — only a set of problem prompts and the model itself.

In the current TRL implementation:

- the model generates completions at a specified training-time temperature (`temperature`) and truncation (`top_k`, `top_p`)
- the dataset only requires a `prompt` column
- training uses standard cross-entropy loss on the generated completions
- empty or single-line stub completions are filtered by default (`filter_empty=True`)
- the evaluation-time temperature and truncation are set independently at inference time
- vLLM can be used for faster generation via `use_vllm=True` (see [vLLM integration](vllm_integration))

## Usage

```python
from datasets import Dataset

from trl.experimental.ssd import SSDConfig, SSDTrainer

dataset = Dataset.from_dict(
    {
        "prompt": [
            [{"role": "user", "content": "Write a function to add two numbers."}],
            [{"role": "user", "content": "Write a function to check if a number is prime."}],
        ],
    }
)

training_args = SSDConfig(
    output_dir="ssd-model",
    temperature=0.6,           # T_train from the paper
    top_k=20,                  # training-time top-k truncation
    top_p=0.95,                # training-time top-p truncation
    max_completion_length=65536,
    learning_rate=5e-6,
)

trainer = SSDTrainer(
    model="Qwen/Qwen3-4B-Instruct",
    args=training_args,
    train_dataset=dataset,
)
trainer.train()
```

## Expected dataset columns

Each example must provide:

- `prompt`: the problem prompt (string or conversational format)

No `privileged_context`, reward functions, or teacher model are needed.

## Key hyperparameters

The paper identifies the following key hyperparameters:

- **`temperature`**: training-time sampling temperature (T_train). Higher values create more diverse samples but may include more noise. The paper uses T_train=0.6 with truncation.
- **`top_k`** and **`top_p`**: training-time truncation parameters (rho_train). These suppress low-probability distractor tails during data synthesis.
- **T_eval**: the evaluation-time decoding temperature is set independently at inference time. The paper shows that T_train and T_eval compose through an effective temperature T_eff = T_train * T_eval, with a broad optimal band.

## Example script                                                                                                            
 
Use [`examples/scripts/ssd.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/ssd.py) to launch SSD training from the command line. The script supports any causal LM from the Hub, custom local datasets via `--dataset_path`, and PEFT/LoRA via the standard `ModelConfig` flags.

```bash
python examples/scripts/ssd.py \
    --model_name_or_path Qwen/Qwen3-4B-Instruct-2507 \
    --dataset_name microsoft/rStar-Coder \
    --dataset_config seed_sft \
    --prompt_column question \
    --output_dir outputs/ssd-qwen3-4b \
    --per_device_train_batch_size 1 \
    --gradient_accumulation_steps 32 \
    --learning_rate 5e-6 \
    --lr_scheduler_type cosine \
    --max_prompt_length 1024 \
    --max_completion_length 65536 \
    --temperature 1.6 \
    --top_k 20 \
    --top_p 0.8 \
    --num_train_epochs 1 \
    --bf16 \
    --report_to trackio
```

## Evaluation on LiveCodeBench

Use [`examples/scripts/ssd_eval.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/ssd_eval.py) to evaluate a base model or an SSD-trained checkpoint on LiveCodeBench v6. The script uses vLLM for generation and LiveCodeBench's official `codegen_metrics` for sandboxed `pass@k` scoring; default decoding parameters match Table 3 of the paper.

```bash
python examples/scripts/ssd_eval.py \
    --model_name_or_path <path-or-repo> \
    --temperature 1.1 --top_k 20 --top_p 0.8 \
    --n 1 \
    --output_file outputs/lcb_v6.json
```

## SSDConfig[[trl.experimental.ssd.SSDConfig]]

#### trl.experimental.ssd.SSDConfig[[trl.experimental.ssd.SSDConfig]]

```python
trl.experimental.ssd.SSDConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-05, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | None = None, trust_remote_code: bool = False, max_prompt_length: int | None = 512, max_completion_length: int | None = 256, generation_batch_size: int | None = None, steps_per_generation: int | None = None, temperature: float = 1.0, top_k: int = 0, top_p: float = 1.0, min_p: float | None = None, repetition_penalty: float = 1.0, generation_kwargs: dict[str, typing.Any] | None = None, cache_implementation: str | None = None, chat_template_kwargs: dict[str, typing.Any] | None = None, use_vllm: bool = False, vllm_mode: str = 'colocate', vllm_model_impl: str = 'vllm', vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_tensor_parallel_size: int = 1, vllm_gpu_memory_utilization: float = 0.3, vllm_max_model_length: int | None = None, vllm_enable_sleep_mode: bool = False, disable_dropout: bool = True, filter_empty: bool = True, num_iterations: int = 1, shuffle_dataset: bool = True, ds3_gather_for_generation: bool = True)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/ssd/ssd_config.py#L24)

**Parameters that control generation and rollout reuse:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments used when the `model` argument is passed as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).

max_prompt_length (`int` or `None`, *optional*, defaults to `512`) : Maximum prompt length. Longer prompts are truncated from the left.

max_completion_length (`int` or `None`, *optional*, defaults to `256`) : Maximum generated completion length.

generation_batch_size (`int` or `None`, *optional*) : Global batch size used for generation. Mutually exclusive with `steps_per_generation`.

steps_per_generation (`int` or `None`, *optional*) : Number of optimizer steps that reuse one generated batch. Mutually exclusive with `generation_batch_size`.

**Parameters that control sampling:**

temperature (`float`, *optional*, defaults to `1.0`) : Sampling temperature (T_train in the paper).

top_k (`int`, *optional*, defaults to `0`) : Top-k sampling parameter. `0` disables top-k filtering.

top_p (`float`, *optional*, defaults to `1.0`) : Top-p (nucleus) sampling parameter.

min_p (`float` or `None`, *optional*) : Minimum token probability for sampling.

repetition_penalty (`float`, *optional*, defaults to `1.0`) : Repetition penalty used during generation.

generation_kwargs (`dict[str, Any]` or `None`, *optional*) : Extra generation kwargs passed to `GenerationConfig`.

**Parameters that control vLLM generation:**

use_vllm (`bool`, *optional*, defaults to `False`) : Whether to use vLLM for generation instead of the training model.

vllm_mode (`str`, *optional*, defaults to `"colocate"`) : vLLM mode: `"colocate"` (shared GPU) or `"server"` (separate vLLM server).

vllm_model_impl (`str`, *optional*, defaults to `"vllm"`) : Model implementation for vLLM: `"vllm"`, `"transformers"`, or `"auto"`.

vllm_server_base_url (`str` or `None`, *optional*) : Base URL for the vLLM server. If provided, `vllm_server_host` and `vllm_server_port` are ignored.

vllm_server_host (`str`, *optional*, defaults to `"0.0.0.0"`) : Host of the vLLM server (server mode only).

vllm_server_port (`int`, *optional*, defaults to `8000`) : Port of the vLLM server (server mode only).

vllm_server_timeout (`float`, *optional*, defaults to `240.0`) : Timeout in seconds to wait for the vLLM server.

vllm_group_port (`int`, *optional*, defaults to `51216`) : Port for the weight update group (server mode only).

vllm_tensor_parallel_size (`int`, *optional*, defaults to `1`) : Tensor parallel size for colocated vLLM.

vllm_gpu_memory_utilization (`float`, *optional*, defaults to `0.3`) : GPU memory utilization ratio for colocated vLLM.

vllm_max_model_length (`int` or `None`, *optional*) : Model context length for vLLM. Inferred from model config if not set.

vllm_enable_sleep_mode (`bool`, *optional*, defaults to `False`) : Whether to enable sleep mode for colocated vLLM engine.

**Parameters that control training behavior:**

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model during training.

filter_empty (`bool`, *optional*, defaults to `True`) : Whether to filter out empty or single-line stub completions from the generated data.

num_iterations (`int`, *optional*, defaults to `1`) : Number of optimization iterations per generated batch.

shuffle_dataset (`bool`, *optional*, defaults to `True`) : Whether to shuffle the training dataset.

ds3_gather_for_generation (`bool`, *optional*, defaults to `True`) : Whether to gather ZeRO-3 weights for generation.

cache_implementation (`str` or `None`, *optional*) : Cache implementation used by transformers generation.

chat_template_kwargs (`dict[str, Any]` or `None`, *optional*) : Extra kwargs forwarded to chat template application.

Configuration class for `SSDTrainer`.

Implements Simple Self-Distillation (SSD) from [*Embarrassingly Simple Self-Distillation Improves Code
Generation*](https://huggingface.co/papers/2604.01193). SSD samples completions from the model at a training-time
temperature and truncation configuration, then fine-tunes on those raw, unverified samples with standard
cross-entropy loss.

The `temperature`, `top_k`, and `top_p` parameters control the training-time sampling configuration (T_train,
rho_train in the paper). The evaluation-time configuration (T_eval, rho_eval) is set independently at inference
time.

## SSDTrainer[[trl.experimental.ssd.SSDTrainer]]

#### trl.experimental.ssd.SSDTrainer[[trl.experimental.ssd.SSDTrainer]]

```python
trl.experimental.ssd.SSDTrainer(model: str | PreTrainedModel | nn.Module, args: SSDConfig | None = None, train_dataset: Dataset | IterableDataset | None = None, eval_dataset: Dataset | IterableDataset | dict[str, Dataset | IterableDataset] | None = None, processing_class: PreTrainedTokenizerBase | ProcessorMixin | None = None, callbacks: list[TrainerCallback] | None = None, optimizers: tuple[torch.optim.Optimizer | None, torch.optim.lr_scheduler.LambdaLR | None] = (None, None), peft_config: PeftConfig | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/ssd/ssd_trainer.py#L74)

Trainer for SSD-style on-policy self-distillation with cross-entropy loss.

SSD generates completions from the model at a specified training-time temperature and truncation configuration,
then fine-tunes on those raw, unverified samples using standard cross-entropy loss. The dataset only requires a
`prompt` column.

#### train[[trl.experimental.ssd.SSDTrainer.train]]

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

#### save_model[[trl.experimental.ssd.SSDTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.ssd.SSDTrainer.push_to_hub]]

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
