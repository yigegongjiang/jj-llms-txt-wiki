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

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "max_prompt_length", "val": ": int | None = 512"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "generation_batch_size", "val": ": int | None = None"}, {"name": "steps_per_generation", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "generation_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "chat_template_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "vllm_gpu_memory_utilization", "val": ": float = 0.3"}, {"name": "vllm_max_model_length", "val": ": int | None = None"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "filter_empty", "val": ": bool = True"}, {"name": "num_iterations", "val": ": int = 1"}, {"name": "shuffle_dataset", "val": ": bool = True"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}]}>
Parameters that control generation and rollout reuse

- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments used when the `model` argument is passed as a string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).
- **max_prompt_length** (`int` or `None`, *optional*, defaults to `512`) --
  Maximum prompt length. Longer prompts are truncated from the left.
- **max_completion_length** (`int` or `None`, *optional*, defaults to `256`) --
  Maximum generated completion length.
- **generation_batch_size** (`int` or `None`, *optional*) --
  Global batch size used for generation. Mutually exclusive with `steps_per_generation`.
- **steps_per_generation** (`int` or `None`, *optional*) --
  Number of optimizer steps that reuse one generated batch. Mutually exclusive with `generation_batch_size`.

Parameters that control sampling

- **temperature** (`float`, *optional*, defaults to `1.0`) --
  Sampling temperature (T_train in the paper).
- **top_k** (`int`, *optional*, defaults to `0`) --
  Top-k sampling parameter. `0` disables top-k filtering.
- **top_p** (`float`, *optional*, defaults to `1.0`) --
  Top-p (nucleus) sampling parameter.
- **min_p** (`float` or `None`, *optional*) --
  Minimum token probability for sampling.
- **repetition_penalty** (`float`, *optional*, defaults to `1.0`) --
  Repetition penalty used during generation.
- **generation_kwargs** (`dict[str, Any]` or `None`, *optional*) --
  Extra generation kwargs passed to `GenerationConfig`.

Parameters that control vLLM generation

- **use_vllm** (`bool`, *optional*, defaults to `False`) --
  Whether to use vLLM for generation instead of the training model.
- **vllm_mode** (`str`, *optional*, defaults to `"colocate"`) --
  vLLM mode: `"colocate"` (shared GPU) or `"server"` (separate vLLM server).
- **vllm_model_impl** (`str`, *optional*, defaults to `"vllm"`) --
  Model implementation for vLLM: `"vllm"`, `"transformers"`, or `"auto"`.
- **vllm_server_base_url** (`str` or `None`, *optional*) --
  Base URL for the vLLM server. If provided, `vllm_server_host` and `vllm_server_port` are ignored.
- **vllm_server_host** (`str`, *optional*, defaults to `"0.0.0.0"`) --
  Host of the vLLM server (server mode only).
- **vllm_server_port** (`int`, *optional*, defaults to `8000`) --
  Port of the vLLM server (server mode only).
- **vllm_server_timeout** (`float`, *optional*, defaults to `240.0`) --
  Timeout in seconds to wait for the vLLM server.
- **vllm_group_port** (`int`, *optional*, defaults to `51216`) --
  Port for the weight update group (server mode only).
- **vllm_tensor_parallel_size** (`int`, *optional*, defaults to `1`) --
  Tensor parallel size for colocated vLLM.
- **vllm_gpu_memory_utilization** (`float`, *optional*, defaults to `0.3`) --
  GPU memory utilization ratio for colocated vLLM.
- **vllm_max_model_length** (`int` or `None`, *optional*) --
  Model context length for vLLM. Inferred from model config if not set.
- **vllm_enable_sleep_mode** (`bool`, *optional*, defaults to `False`) --
  Whether to enable sleep mode for colocated vLLM engine.

Parameters that control training behavior

- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model during training.
- **filter_empty** (`bool`, *optional*, defaults to `True`) --
  Whether to filter out empty or single-line stub completions from the generated data.
- **num_iterations** (`int`, *optional*, defaults to `1`) --
  Number of optimization iterations per generated batch.
- **shuffle_dataset** (`bool`, *optional*, defaults to `True`) --
  Whether to shuffle the training dataset.
- **ds3_gather_for_generation** (`bool`, *optional*, defaults to `True`) --
  Whether to gather ZeRO-3 weights for generation.
- **cache_implementation** (`str` or `None`, *optional*) --
  Cache implementation used by transformers generation.
- **chat_template_kwargs** (`dict[str, Any]` or `None`, *optional*) --
  Extra kwargs forwarded to chat template application.

Configuration class for `SSDTrainer`.

Implements Simple Self-Distillation (SSD) from [*Embarrassingly Simple Self-Distillation Improves Code
Generation*](https://huggingface.co/papers/2604.01193). SSD samples completions from the model at a training-time
temperature and truncation configuration, then fine-tunes on those raw, unverified samples with standard
cross-entropy loss.

The `temperature`, `top_k`, and `top_p` parameters control the training-time sampling configuration (T_train,
rho_train in the paper). The evaluation-time configuration (T_eval, rho_eval) is set independently at inference
time.

## SSDTrainer[[trl.experimental.ssd.SSDTrainer]]

Trainer for SSD-style on-policy self-distillation with cross-entropy loss.

SSD generates completions from the model at a specified training-time temperature and truncation configuration,
then fine-tunes on those raw, unverified samples using standard cross-entropy loss. The dataset only requires a
`prompt` column.

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
