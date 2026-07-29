# SDFT

Self-Distilled Fine-Tuning (SDFT) is described in the paper [Self-Distillation Enables Continual Learning](https://huggingface.co/papers/2601.19897) by Idan Shenfeld, Mehul Damani, Jonas Hübotter, and Pulkit Agrawal.

> Continual learning, enabling models to acquire new skills and knowledge without degrading existing capabilities, remains a fundamental challenge for foundation models. While on-policy reinforcement learning can reduce forgetting, it requires explicit reward functions that are often unavailable. Learning from expert demonstrations, the primary alternative, is dominated by supervised fine-tuning (SFT), which is inherently off-policy. We introduce Self-Distillation Fine-Tuning (SDFT), a simple method that enables on-policy learning directly from demonstrations. SDFT leverages in-context learning by using a demonstration-conditioned model as its own teacher, generating on-policy training signals that preserve prior capabilities while acquiring new skills. Across skill learning and knowledge acquisition tasks, SDFT consistently outperforms SFT, achieving higher new-task accuracy while substantially reducing catastrophic forgetting. In sequential learning experiments, SDFT enables a single model to accumulate multiple skills over time without performance regression, establishing on-policy distillation as a practical path to continual learning from demonstrations.

## How it works

Plain supervised fine-tuning trains on the demonstration text off-policy, which tends to overwrite prior capabilities. SDFT learns on-policy instead: the student generates from the plain `prompt`, a teacher — the same model shown the `prompt` plus the example's `privileged_context` — re-scores those tokens, and its demonstration-conditioned distribution is distilled back into the student. Teacher and student are one network differing only in what they see, creating a *self*-distillation loop.

## Choosing the teacher

`teacher_model_kind` selects which copy of the model acts as teacher. `"base"` (the default) freezes the initial weights as a fixed reference, matching the paper; `"live"` reuses the current student for a zero-lag self-teacher; `"ema"` maintains an exponential moving average, resynced every `teacher_sync_steps` steps at rate `teacher_update_rate`. Under PEFT, `"base"` is obtained by disabling the adapter during the teacher forward to recover the base weights, and `"ema"` with pure-LoRA training holds the moving average in a dedicated `"teacher"` adapter instead of a second model copy. `"ema"` with a non-pure-LoRA PEFT model (e.g. `modules_to_save` or `bias`) is not supported, since a separate EMA copy cannot be parameter-matched to the student.

By default the student generates from the plain prompt; set `generate_from_teacher=True` to sample from the demonstration-conditioned prompt instead, trading on-policy fidelity for higher-quality rollouts. The distillation objective is set by `distillation_mode` (`"topk_logits"` by default, with `"full_logits"` and `"sampled_token"` alternatives), `distillation_alpha`, and `distillation_topk`; `num_loss_tokens_to_skip` drops leading completion tokens from the loss. Setting `use_liger_kernel=True` swaps in a memory-efficient fused JSD loss (Liger) that avoids materializing the full-vocabulary logits; it requires `distillation_mode="full_logits"` and is incompatible with `distillation_is_clip`. Training is text-only; generation runs through transformers by default, or vLLM (colocate or server mode) when `use_vllm=True`.

## Usage

```python
from datasets import Dataset

from trl.experimental.sdft import SDFTConfig, SDFTTrainer

dataset = Dataset.from_dict(
    {
        "prompt": [[{"role": "user", "content": "Solve 2+2."}]],
        "privileged_context": ["Example answer: 4."],
    }
)

training_args = SDFTConfig(
    output_dir="sdft-model",
    distillation_alpha=0.5,
    distillation_mode="topk_logits",
    distillation_topk=5,
    max_completion_length=64,
)

trainer = SDFTTrainer(
    model="Qwen/Qwen2.5-1.5B-Instruct",
    args=training_args,
    train_dataset=dataset,
)
trainer.train()
```

To generate from the teacher-conditioned prompt instead of the student prompt, set `generate_from_teacher=True`.
To customize how the teacher prompt is built, set `teacher_prompt_template` on `SDFTConfig`.

## Serving the teacher from the vLLM server

With `teacher_model_kind="live"` the teacher is the current student, whose weights the vLLM **server** already holds (they are synced for generation each step). Set `use_teacher_server=True` to score the teacher log-probabilities on that same server instead of running a separate local teacher forward, removing the teacher from the training step entirely:

```python
training_args = SDFTConfig(
    output_dir="sdft-model",
    use_vllm=True,
    vllm_mode="server",
    teacher_model_kind="live",
    use_teacher_server=True,
    distillation_mode="sampled_token",
)
```

When using the teacher server:

- `use_vllm=True` and `vllm_mode="server"` are required
- `teacher_model_kind` must be `"live"` (the server holds the current student weights)
- `distillation_mode` must be `"sampled_token"` (reverse KL on the realized token) or `"topk_logits"`. The server returns the teacher's own top-k log-probs, so `topk_logits` distills over the teacher's top-k support (it cannot use the student's, unlike the local objective); with a `"live"` teacher the two supports nearly coincide. `full_logits` is unavailable.
- `use_liger_kernel` is not supported

## Expected dataset columns

Each example must provide:

- `prompt`: the student-facing prompt
- `privileged_context`: only the extra teacher-only information, such as a demonstration, hint, or privileged feedback

Both standard text prompts and conversational prompts are supported by the trainer prompt handling.

## Callbacks

The trainer emits a small set of callback hooks that are useful for debugging, observability, and tests. These hooks are intended as practical integration points for experimental self-distillation workflows.

Shared self-distillation hooks:

- `on_self_distillation_batch_prepared`: fired when a self-distillation batch is ready. The payload includes `prompt_ids`, `completion_ids`, and `old_per_token_logps` when importance-sampling clipping inputs are available.
- `on_generation_batch_built`: fired when a new buffered generation batch is created. The payload includes `generate_every` and `steps_per_generation`.

SDFT-specific hook:

- `on_generation_prompts_selected`: fired when SDFT chooses the prompt source for on-policy generation. The payload includes the selected `generation_prompts` and the corresponding `generation_prompt_text`.

## Example script

Use [`examples/scripts/sdft.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/sdft.py) to launch SDFT training from the command line. The script supports any causal LM from the Hub, custom local datasets via `--dataset_path`, and PEFT/LoRA via the standard `ModelConfig` flags.

```bash
python examples/scripts/sdft.py \
    --model_name_or_path Qwen/Qwen3.5-0.8B \
    --dataset_name your-org/your-dataset \
    --output_dir outputs/sdft-qwen3.5-0.8b \
    --per_device_train_batch_size 1 \
    --gradient_accumulation_steps 16 \
    --learning_rate 2e-5 \
    --max_prompt_length 1024 \
    --max_completion_length 512 \
    --generate_from_teacher \
    --teacher_model_kind ema \
    --teacher_sync_steps 1 \
    --teacher_update_rate 0.05 \
    --eval_strategy steps \
    --eval_steps 50 \
    --report_to wandb
```

The original implementation is available at [idanshen/Self-Distillation](https://github.com/idanshen/Self-Distillation).

## SDFTConfig[[trl.experimental.sdft.SDFTConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "max_prompt_length", "val": ": int | None = 512"}, {"name": "num_generations", "val": ": int = 8"}, {"name": "num_generations_eval", "val": ": int | None = None"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "shuffle_dataset", "val": ": bool = True"}, {"name": "generation_batch_size", "val": ": int | None = None"}, {"name": "steps_per_generation", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "generation_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "chat_template_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "use_teacher_server", "val": ": bool = False"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "vllm_gpu_memory_utilization", "val": ": float = 0.3"}, {"name": "vllm_max_model_length", "val": ": int | None = None"}, {"name": "num_iterations", "val": ": int = 1"}, {"name": "teacher_model_kind", "val": ": str = 'base'"}, {"name": "teacher_update_rate", "val": ": float = 0.05"}, {"name": "teacher_sync_steps", "val": ": int = 1"}, {"name": "distillation_alpha", "val": ": float = 0.5"}, {"name": "distillation_mode", "val": ": typing.Literal['sampled_token', 'full_logits', 'topk_logits'] = 'topk_logits'"}, {"name": "distillation_topk", "val": ": int | None = 100"}, {"name": "distillation_is_clip", "val": ": float | None = 2.0"}, {"name": "distillation_add_tail", "val": ": bool = False"}, {"name": "generate_from_teacher", "val": ": bool = False"}, {"name": "teacher_prompt_template", "val": ": str = '{prompt}\\n\\n{privileged_context}'"}, {"name": "num_loss_tokens_to_skip", "val": ": int = 0"}]}>
Parameters that control the SDFT loss

- **distillation_alpha** (`float`, *optional*, defaults to `0.5`) --
  Divergence interpolation coefficient for SDFT top-k logit distillation.
- **distillation_mode** (`Literal["sampled_token", "full_logits", "topk_logits"]`, *optional*, defaults to `"topk_logits"`) --
  Distillation objective mode. SDFT defaults to top-k logit distillation.
- **distillation_topk** (`int`, *optional*, defaults to `100`) --
  Number of top tokens used by the default SDFT top-k logit objective.
- **distillation_is_clip** (`float`, *optional*, defaults to `2.0`) --
  Clipping coefficient for importance sampling in self-distillation. `None` disables clipping.
- **distillation_add_tail** (`bool`, *optional*, defaults to `False`) --
  Whether to add a tail bucket for non-top-k probability mass.
- **num_loss_tokens_to_skip** (`int`, *optional*, defaults to `0`) --
  Number of initial completion tokens to exclude from the distillation loss.

Parameters that control the teacher

- **teacher_model_kind** (`str`, *optional*, defaults to `"base"`) --
  Semantic teacher choice for SDFT. `base` uses the initial student, `live` uses the current student, and
  `ema` uses an exponentially averaged teacher.
- **teacher_update_rate** (`float`, *optional*, defaults to `0.05`) --
  EMA update rate used when `teacher_model_kind="ema"`. A value of `1.0` reduces the update to a hard
  overwrite, periodically resyncing the teacher to the current student weights.
- **teacher_sync_steps** (`int`, *optional*, defaults to `1`) --
  Number of optimizer steps between teacher updates.

Parameters that control teacher-conditioned generation

- **generate_from_teacher** (`bool`, *optional*, defaults to `False`) --
  Whether on-policy generation should use the teacher-conditioned prompt instead of the student prompt.
- **teacher_prompt_template** (`str`, *optional*, defaults to `"{prompt}\n\n{privileged_context}"`) --
  Template used to combine the student prompt and privileged context into the teacher prompt.

Parameters that control the model

- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments for `transformers.AutoModelForCausalLM.from_pretrained`, used when the `model` argument
  of the `SDFTTrainer` is provided as a string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained),
  for both the student and teacher.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the student and teacher models.

Parameters that control data preprocessing

- **remove_unused_columns** (`bool`, *optional*, defaults to `False`) --
  Whether to only keep the columns required by the trainer in the dataset. Keep this to `False` if you
  provide extra columns (such as `privileged_context`) that the trainer needs.
- **max_prompt_length** (`int`, *optional*, defaults to `512`) --
  Maximum prompt length. Longer prompts are truncated from the left.
- **shuffle_dataset** (`bool`, *optional*, defaults to `True`) --
  Whether to shuffle the training dataset.

Parameters that control generation

- **num_generations** (`int`, *optional*, defaults to `8`) --
  Number of generations to sample. The effective batch size (num_processes * per_device_batch_size *
  gradient_accumulation_steps) must be evenly divisible by this value.
- **num_generations_eval** (`int`, *optional*) --
  Number of generations to sample during evaluation. This allows using fewer generations during evaluation to
  save computation. If `None`, uses the value of `num_generations`.
- **max_completion_length** (`int`, *optional*, defaults to `256`) --
  Maximum length of the generated completion.
- **temperature** (`float`, *optional*, defaults to `1.0`) --
  Temperature for sampling. The higher the temperature, the more random the completions.
- **top_p** (`float`, *optional*, defaults to `1.0`) --
  Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to 1.0
  to consider all tokens.
- **top_k** (`int`, *optional*, defaults to `0`) --
  Number of highest probability vocabulary tokens to keep for top-k-filtering. If `0`, top-k-filtering is
  disabled and all tokens are considered.
- **min_p** (`float`, *optional*) --
  Minimum token probability, which will be scaled by the probability of the most likely token. It must be a
  value between 0.0 and 1.0. Typical values are in the 0.01-0.2 range.
- **repetition_penalty** (`float`, *optional*, defaults to `1.0`) --
  Float that penalizes new tokens based on whether they appear in the prompt and the generated text so far.
  Values > 1.0 encourage the model to use new tokens, while values < 1.0 encourage the model to repeat
  tokens.
- **cache_implementation** (`str`, *optional*) --
  Implementation of the cache method for faster generation when use_vllm is set to False.
- **generation_kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments to pass to `GenerationConfig` (if using transformers) or `SamplingParams` (if
  using vLLM) when sampling completions. This can be used to further customize the generation behavior, such
  as setting `suppress_tokens`, `num_beams`, etc. If it contains keys that conflict with the other generation
  parameters (like `min_p`, `top_p`, etc.), they will override them.
- **chat_template_kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments to pass to the `apply_chat_template` function when generating completions.
- **ds3_gather_for_generation** (`bool`, *optional*, defaults to `True`) --
  This setting applies to DeepSpeed ZeRO-3. If enabled, the policy model weights are gathered for generation,
  improving generation speed. However, disabling this option allows training models that exceed the VRAM
  capacity of a single GPU, albeit at the cost of slower generation. Disabling this option is not compatible
  with vLLM generation.

Parameters that control generation acceleration powered by vLLM

- **use_vllm** (`bool`, *optional*, defaults to `False`) --
  Whether to use vLLM for generating completions. If set to `True`, the trainer will use vLLM for generation
  instead of the default model.generate(). Requires `vllm` to be installed.
- **use_teacher_server** (`bool`, *optional*, defaults to `False`) --
  Compute teacher logprobs from the running vLLM generation server instead of a local teacher forward. Only
  supported for `teacher_model_kind='live'` with `use_vllm=True` and `vllm_mode='server'`, and
  `distillation_mode` in {'sampled_token', 'topk_logits'} (the server returns the teacher's top-k logprobs,
  not the full vocabulary; `topk_logits` distills over the teacher's own top-k support).
- **vllm_mode** (`str`, *optional*, defaults to `"colocate"`) --
  Mode to use for vLLM integration when `use_vllm` is set to `True`. Must be one of `'server'` or
  `'colocate'`. `'server'`: The trainer will send generation requests to a separate vLLM server. Make sure a
  TRL vLLM server is running (start with `trl vllm-serve`). `'colocate'`: vLLM will run in the same process
  and share the training GPUs. This avoids the need for a separate server but may cause resource contention
  with training.
- **vllm_model_impl** (`str`, *optional*, defaults to `"vllm"`) --
  Model implementation to use for vLLM. Must be one of `transformers` or `vllm`. `transformers`: Use the
  `transformers` backend for model implementation. `vllm`: Use the `vllm` library for model implementation.
- **vllm_enable_sleep_mode** (`bool`, *optional*, defaults to `False`) --
  Enable vLLM sleep mode to offload weights/cache during the optimizer step. Keeps GPU memory usage low, but
  waking the engine adds host–device transfer latency.
- **vllm_server_base_url** (`str`, *optional*) --
  Base URL for the vLLM server (e.g., 'http://localhost:8000'). If provided, `vllm_server_host` and
  `vllm_server_port` are ignored.
- **vllm_server_host** (`str`, *optional*, defaults to `"0.0.0.0"`) --
  Host of the vLLM server to connect to. Ignored if vllm_server_base_url is provided.
- **vllm_server_port** (`int`, *optional*, defaults to `8000`) --
  Port of the vLLM server to connect to. Ignored if vllm_server_base_url is provided.
- **vllm_group_port** (`int`, *optional*, defaults to `51216`) --
  Port number for the weight update group. This is used to communicate with the vLLM server. Unless the port
  is occupied, there is no need to change it.
- **vllm_server_timeout** (`float`, *optional*, defaults to `240.0`) --
  Total timeout duration in seconds to wait for the vLLM server to be up. If the server is not up after the
  timeout, a `ConnectionError` is raised.
- **vllm_tensor_parallel_size** (`int`, *optional*, defaults to `1`) --
  Control the tensor parallel size for vLLM. This setting only applies when `vllm_mode` is set to
  `'colocate'`. If you are using `vllm_mode='server'`, this parameter must be passed separately when
  launching the vLLM server via the `--vllm_tensor_parallel_size` flag.
- **vllm_gpu_memory_utilization** (`float`, *optional*, defaults to `0.3`) --
  Control the GPU memory utilization for vLLM. This setting only applies when `vllm_mode` is set to
  `'colocate'`. If you are using `vllm_mode='server'`, this parameter must be passed separately when
  launching the vLLM server via the `--vllm_gpu_memory_utilization` flag.
- **vllm_max_model_length** (`int`, *optional*) --
  Context window for vLLM. Set it to at least the maximum prompt length in the dataset plus
  `max_completion_length`; if omitted, it is inferred from the model config.

Parameters that control the training

- **num_iterations** (`int`, *optional*, defaults to `1`) --
  Number of iterations per batch (denoted as μ in the algorithm).
- **generation_batch_size** (`int`, *optional*) --
  Batch size to use for generation. If `None`, it defaults to the effective training batch size:
  `per_device_train_batch_size * num_processes * steps_per_generation`.
- **steps_per_generation** (`int`, *optional*) --
  Number of steps per generation. If `None`, it defaults to `gradient_accumulation_steps`.

Configuration class for the `SDFTTrainer`.

## SDFTTrainer[[trl.experimental.sdft.SDFTTrainer]]

Trainer for SDFT-style on-policy self-distillation with explicit teacher prompts.

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
