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

Use [`examples/sdft_privileged_context/sdft_privileged_context.py`](https://github.com/huggingface/trl/blob/main/examples/sdft_privileged_context/sdft_privileged_context.py) to launch SDFT training from the command line. The script supports any causal LM from the Hub, custom local datasets via `--dataset_path`, and PEFT/LoRA via the standard `ModelConfig` flags.

```bash
python examples/sdft_privileged_context/sdft_privileged_context.py \
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

#### trl.experimental.sdft.SDFTConfig[[trl.experimental.sdft.SDFTConfig]]

```python
trl.experimental.sdft.SDFTConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-05, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | None = None, trust_remote_code: bool = False, disable_dropout: bool = True, max_prompt_length: int | None = 512, num_generations: int = 8, num_generations_eval: int | None = None, max_completion_length: int | None = 256, ds3_gather_for_generation: bool = True, shuffle_dataset: bool = True, generation_batch_size: int | None = None, steps_per_generation: int | None = None, temperature: float = 1.0, top_p: float = 1.0, top_k: int = 0, min_p: float | None = None, generation_kwargs: dict[str, typing.Any] | None = None, chat_template_kwargs: dict[str, typing.Any] | None = None, repetition_penalty: float = 1.0, cache_implementation: str | None = None, use_vllm: bool = False, use_teacher_server: bool = False, vllm_mode: str = 'colocate', vllm_model_impl: str = 'vllm', vllm_enable_sleep_mode: bool = False, vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_group_port: int = 51216, vllm_server_timeout: float = 240.0, vllm_tensor_parallel_size: int = 1, vllm_gpu_memory_utilization: float = 0.3, vllm_max_model_length: int | None = None, num_iterations: int = 1, teacher_model_kind: str = 'base', teacher_update_rate: float = 0.05, teacher_sync_steps: int = 1, distillation_alpha: float = 0.5, distillation_mode: typing.Literal['sampled_token', 'full_logits', 'topk_logits'] = 'topk_logits', distillation_topk: int | None = 100, distillation_is_clip: float | None = 2.0, distillation_add_tail: bool = False, generate_from_teacher: bool = False, teacher_prompt_template: str = '{prompt}\n\n{privileged_context}', num_loss_tokens_to_skip: int = 0)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/sdft/sdft_config.py#L24)

**Parameters that control the SDFT loss:**

distillation_alpha (`float`, *optional*, defaults to `0.5`) : Divergence interpolation coefficient for SDFT top-k logit distillation.

distillation_mode (`Literal["sampled_token", "full_logits", "topk_logits"]`, *optional*, defaults to `"topk_logits"`) : Distillation objective mode. SDFT defaults to top-k logit distillation.

distillation_topk (`int`, *optional*, defaults to `100`) : Number of top tokens used by the default SDFT top-k logit objective.

distillation_is_clip (`float`, *optional*, defaults to `2.0`) : Clipping coefficient for importance sampling in self-distillation. `None` disables clipping.

distillation_add_tail (`bool`, *optional*, defaults to `False`) : Whether to add a tail bucket for non-top-k probability mass.

num_loss_tokens_to_skip (`int`, *optional*, defaults to `0`) : Number of initial completion tokens to exclude from the distillation loss.

**Parameters that control the teacher:**

teacher_model_kind (`str`, *optional*, defaults to `"base"`) : Semantic teacher choice for SDFT. `base` uses the initial student, `live` uses the current student, and `ema` uses an exponentially averaged teacher.

teacher_update_rate (`float`, *optional*, defaults to `0.05`) : EMA update rate used when `teacher_model_kind="ema"`. A value of `1.0` reduces the update to a hard overwrite, periodically resyncing the teacher to the current student weights.

teacher_sync_steps (`int`, *optional*, defaults to `1`) : Number of optimizer steps between teacher updates.

**Parameters that control teacher-conditioned generation:**

generate_from_teacher (`bool`, *optional*, defaults to `False`) : Whether on-policy generation should use the teacher-conditioned prompt instead of the student prompt.

teacher_prompt_template (`str`, *optional*, defaults to `"{prompt}\n\n{privileged_context}"`) : Template used to combine the student prompt and privileged context into the teacher prompt.

**Parameters that control the model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for `transformers.AutoModelForCausalLM.from_pretrained`, used when the `model` argument of the `SDFTTrainer` is provided as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained), for both the student and teacher.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the student and teacher models.

**Parameters that control data preprocessing:**

remove_unused_columns (`bool`, *optional*, defaults to `False`) : Whether to only keep the columns required by the trainer in the dataset. Keep this to `False` if you provide extra columns (such as `privileged_context`) that the trainer needs.

max_prompt_length (`int`, *optional*, defaults to `512`) : Maximum prompt length. Longer prompts are truncated from the left.

shuffle_dataset (`bool`, *optional*, defaults to `True`) : Whether to shuffle the training dataset.

**Parameters that control generation:**

num_generations (`int`, *optional*, defaults to `8`) : Number of generations to sample. The effective batch size (num_processes * per_device_batch_size * gradient_accumulation_steps) must be evenly divisible by this value.

num_generations_eval (`int`, *optional*) : Number of generations to sample during evaluation. This allows using fewer generations during evaluation to save computation. If `None`, uses the value of `num_generations`.

max_completion_length (`int`, *optional*, defaults to `256`) : Maximum length of the generated completion.

temperature (`float`, *optional*, defaults to `1.0`) : Temperature for sampling. The higher the temperature, the more random the completions.

top_p (`float`, *optional*, defaults to `1.0`) : Float that controls the cumulative probability of the top tokens to consider. Must be in (0, 1]. Set to 1.0 to consider all tokens.

top_k (`int`, *optional*, defaults to `0`) : Number of highest probability vocabulary tokens to keep for top-k-filtering. If `0`, top-k-filtering is disabled and all tokens are considered.

min_p (`float`, *optional*) : Minimum token probability, which will be scaled by the probability of the most likely token. It must be a value between 0.0 and 1.0. Typical values are in the 0.01-0.2 range.

repetition_penalty (`float`, *optional*, defaults to `1.0`) : Float that penalizes new tokens based on whether they appear in the prompt and the generated text so far. Values > 1.0 encourage the model to use new tokens, while values < 1.0 encourage the model to repeat tokens.

cache_implementation (`str`, *optional*) : Implementation of the cache method for faster generation when use_vllm is set to False.

generation_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to `GenerationConfig` (if using transformers) or `SamplingParams` (if using vLLM) when sampling completions. This can be used to further customize the generation behavior, such as setting `suppress_tokens`, `num_beams`, etc. If it contains keys that conflict with the other generation parameters (like `min_p`, `top_p`, etc.), they will override them.

chat_template_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the `apply_chat_template` function when generating completions.

ds3_gather_for_generation (`bool`, *optional*, defaults to `True`) : This setting applies to DeepSpeed ZeRO-3. If enabled, the policy model weights are gathered for generation, improving generation speed. However, disabling this option allows training models that exceed the VRAM capacity of a single GPU, albeit at the cost of slower generation. Disabling this option is not compatible with vLLM generation.

**Parameters that control generation acceleration powered by vLLM:**

use_vllm (`bool`, *optional*, defaults to `False`) : Whether to use vLLM for generating completions. If set to `True`, the trainer will use vLLM for generation instead of the default model.generate(). Requires `vllm` to be installed.

use_teacher_server (`bool`, *optional*, defaults to `False`) : Compute teacher logprobs from the running vLLM generation server instead of a local teacher forward. Only supported for `teacher_model_kind='live'` with `use_vllm=True` and `vllm_mode='server'`, and `distillation_mode` in {'sampled_token', 'topk_logits'} (the server returns the teacher's top-k logprobs, not the full vocabulary; `topk_logits` distills over the teacher's own top-k support).

vllm_mode (`str`, *optional*, defaults to `"colocate"`) : Mode to use for vLLM integration when `use_vllm` is set to `True`. Must be one of `'server'` or `'colocate'`. `'server'`: The trainer will send generation requests to a separate vLLM server. Make sure a vLLM server is running (start with `vllm serve`). `'colocate'`: vLLM will run in the same process and share the training GPUs. This avoids the need for a separate server but may cause resource contention with training.

vllm_model_impl (`str`, *optional*, defaults to `"vllm"`) : Model implementation to use for vLLM. Must be one of `transformers` or `vllm`. `transformers`: Use the `transformers` backend for model implementation. `vllm`: Use the `vllm` library for model implementation.

vllm_enable_sleep_mode (`bool`, *optional*, defaults to `False`) : Enable vLLM sleep mode to offload weights/cache during the optimizer step. Keeps GPU memory usage low, but waking the engine adds host–device transfer latency.

vllm_server_base_url (`str`, *optional*) : Base URL for the vLLM server (e.g., 'http://localhost:8000'). If provided, `vllm_server_host` and `vllm_server_port` are ignored.

vllm_server_host (`str`, *optional*, defaults to `"0.0.0.0"`) : Host of the vLLM server to connect to. Ignored if vllm_server_base_url is provided.

vllm_server_port (`int`, *optional*, defaults to `8000`) : Port of the vLLM server to connect to. Ignored if vllm_server_base_url is provided.

vllm_group_port (`int`, *optional*, defaults to `51216`) : Port number for the weight update group. This is used to communicate with the vLLM server. Unless the port is occupied, there is no need to change it.

vllm_server_timeout (`float`, *optional*, defaults to `240.0`) : Total timeout duration in seconds to wait for the vLLM server to be up. If the server is not up after the timeout, a `ConnectionError` is raised.

vllm_tensor_parallel_size (`int`, *optional*, defaults to `1`) : Control the tensor parallel size for vLLM. This setting only applies when `vllm_mode` is set to `'colocate'`. If you are using `vllm_mode='server'`, this parameter must be passed separately when launching the vLLM server via the `--vllm_tensor_parallel_size` flag.

vllm_gpu_memory_utilization (`float`, *optional*, defaults to `0.3`) : Control the GPU memory utilization for vLLM. This setting only applies when `vllm_mode` is set to `'colocate'`. If you are using `vllm_mode='server'`, this parameter must be passed separately when launching the vLLM server via the `--vllm_gpu_memory_utilization` flag.

vllm_max_model_length (`int`, *optional*) : Context window for vLLM. Set it to at least the maximum prompt length in the dataset plus `max_completion_length`; if omitted, it is inferred from the model config.

**Parameters that control the training:**

num_iterations (`int`, *optional*, defaults to `1`) : Number of iterations per batch (denoted as μ in the algorithm).

generation_batch_size (`int`, *optional*) : Batch size to use for generation. If `None`, it defaults to the effective training batch size: `per_device_train_batch_size * num_processes * steps_per_generation`.

steps_per_generation (`int`, *optional*) : Number of steps per generation. If `None`, it defaults to `gradient_accumulation_steps`.

Configuration class for the `SDFTTrainer`.

## SDFTTrainer[[trl.experimental.sdft.SDFTTrainer]]

#### trl.experimental.sdft.SDFTTrainer[[trl.experimental.sdft.SDFTTrainer]]

```python
trl.experimental.sdft.SDFTTrainer(model: typing.Union[str, transformers.modeling_utils.PreTrainedModel, torch.nn.Module], args: trl.experimental.sdft.sdft_config.SDFTConfig | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.processing_utils.ProcessorMixin | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), peft_config: peft.config.PeftConfig | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/sdft/sdft_trainer.py#L211)

Trainer for SDFT-style on-policy self-distillation with explicit teacher prompts.

#### train[[trl.experimental.sdft.SDFTTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L1350)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.experimental.sdft.SDFTTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.sdft.SDFTTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L4052)

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
