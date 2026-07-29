# SDPO

Self-Distillation Policy Optimization (SDPO) was introduced in [Reinforcement Learning via Self-Distillation](https://huggingface.co/papers/2601.20802) by [Jonas Hübotter](https://huggingface.co/jonhue), Frederike Lübeck, Lejs Behric, [Anton Baumann](https://huggingface.co/antonbaumann), Marco Bagatella, Daniel Marta, Ido Hakimi, Idan Shenfeld, Thomas Kleine Buening, Carlos Guestrin, and Andreas Krause.

> Large language models are increasingly post-trained with reinforcement learning in verifiable domains such as code and math. Yet, current methods for reinforcement learning with verifiable rewards (RLVR) learn only from a scalar outcome reward per attempt, creating a severe credit-assignment bottleneck. Many verifiable environments actually provide rich textual feedback, such as runtime errors or judge evaluations, that explain why an attempt failed. We formalize this setting as reinforcement learning with rich feedback and introduce Self-Distillation Policy Optimization (SDPO), which converts tokenized feedback into a dense learning signal without any external teacher or explicit reward model. SDPO treats the current model conditioned on feedback as a self-teacher and distills its feedback-informed next-token predictions back into the policy. In this way, SDPO leverages the model's ability to retrospectively identify its own mistakes in-context. Across scientific reasoning, tool use, and competitive programming on LiveCodeBench v6, SDPO improves sample efficiency and final accuracy over strong RLVR baselines. Notably, SDPO also outperforms baselines in standard RLVR environments that only return scalar feedback by using successful rollouts as implicit feedback for failed attempts. Finally, applying SDPO to individual questions at test time accelerates discovery on difficult binary-reward tasks, achieving the same discovery probability as best-of-k sampling or multi-turn conversations with 3x fewer attempts.

## How it works

SDPO targets reinforcement learning with verifiable rewards (RLVR), where each attempt yields only a sparse scalar reward. It turns that into a dense, token-level signal: for each prompt the policy samples `num_generations` completions scored by `reward_funcs`, a successful rollout (plus optional `privileged_context` feedback) becomes a teacher reprompt, and the teacher's feedback-informed distribution over a completion is distilled back into the policy. Teacher and student are the same network, so no external teacher or reward model is needed beyond the verifier.

## Loss modes and the teacher

`distillation_weight` controls how the two signals combine as a convex combination: the loss is `(1 - distillation_weight) * policy_loss + distillation_weight * distillation_loss`. `1.0` (the default) trains purely on the self-distillation loss, `0.0` falls back to the standard GRPO-style policy gradient, and intermediate values blend both. The distillation objective itself is set by `distillation_mode` — `"sampled_token"` (the default) uses a token-level reverse KL and requires `distillation_alpha=1.0`, while `"full_logits"` and `"topk_logits"` distill over the full or top-`distillation_topk` vocabulary. Setting `use_liger_kernel=True` swaps in a memory-efficient fused JSD loss (Liger) for the distillation term; it requires `distillation_weight=1.0`, `distillation_mode="full_logits"`, and is incompatible with `distillation_is_clip`.

`teacher_model_kind` chooses the teacher weights: `"ema"` (the default) tracks the student with an exponential moving average synced every `teacher_sync_steps` steps at rate `teacher_update_rate`, `"live"` reuses the current student directly, and `"base"` freezes the initial weights. Reprompting is governed by `use_successful_as_teacher`, `success_reward_threshold`, `dont_reprompt_on_self_success`, and the `reprompt_template` / `solution_template` / `feedback_template` strings. Generation runs through transformers by default, or vLLM (colocate or server mode) when `use_vllm=True`.

## Expected dataset columns

Each example must provide:

- `prompt`: the student-facing prompt
- `privileged_context`: optional privileged text, such as environment feedback, used when `include_environment_feedback=True`

## Usage

```python
from datasets import Dataset

from trl.experimental.sdpo import SDPOConfig, SDPOTrainer

dataset = Dataset.from_dict(
    {
        "prompt": [[{"role": "user", "content": "Solve 2+2."}]],
        "privileged_context": ["Your earlier answer used the wrong format."],
    }
)

training_args = SDPOConfig(
    output_dir="sdpo-model",
    distillation_mode="topk_logits",       # Explicitly select top-K logit distillation
    distillation_topk=100,                 # Required when using top-K logit distillation
    include_environment_feedback=True,     # Use dataset privileged_context for teacher reprompts
)

trainer = SDPOTrainer(
    model="Qwen/Qwen2.5-1.5B-Instruct",
    reward_funcs=reward_func,
    args=training_args,
    train_dataset=dataset,
)
trainer.train()
```

SDPO always requires a `prompt` column. To use environment feedback, also include a `privileged_context` column and set `include_environment_feedback=True`. SDPO will use successful rollouts and, when enabled, that text to build teacher reprompts for self-distillation.

## Serving the teacher from the vLLM server

With `teacher_model_kind="live"` the teacher is the current student, whose weights the vLLM **server** already holds (they are synced for generation each step). Set `use_teacher_server=True` to score the teacher log-probabilities on that same server instead of running a separate local teacher forward, removing the teacher from the training step entirely:

```python
training_args = SDPOConfig(
    output_dir="sdpo-model",
    use_vllm=True,
    vllm_mode="server",
    teacher_model_kind="live",
    use_teacher_server=True,
    distillation_weight=1.0,
    distillation_mode="sampled_token",
)
```

When using the teacher server:

- `use_vllm=True` and `vllm_mode="server"` are required
- `teacher_model_kind` must be `"live"` (the server holds the current student weights)
- `distillation_weight` must be `1.0` (pure distillation; a convex blend with the policy loss needs the full-vocabulary logits)
- `distillation_mode` must be `"sampled_token"` (reverse KL on the realized token) or `"topk_logits"`. The server returns the teacher's own top-k log-probs, so `topk_logits` distills over the teacher's top-k support (it cannot use the student's, unlike the local objective); with a `"live"` teacher the two supports nearly coincide. `full_logits` is unavailable.
- `use_liger_kernel` is not supported

## Callbacks

The trainer emits a small set of callback hooks that are useful for debugging, observability, and tests. These hooks are intended as practical integration points for experimental self-distillation workflows.

Shared self-distillation hooks:

- `on_self_distillation_batch_prepared`: fired when a self-distillation batch is ready. The payload includes `prompt_ids`, `completion_ids`, and `old_per_token_logps` when importance-sampling clipping inputs are available.
- `on_generation_batch_built`: fired when a new buffered generation batch is created. The payload includes `generate_every` and `steps_per_generation`.

SDPO-specific hook:

- `on_teacher_context_built`: fired after SDPO constructs the teacher-conditioned inputs. The payload includes `teacher_input_ids`, `teacher_attention_mask`, `completion_mask`, and `self_distillation_mask`.

## Example script

Use [`examples/scripts/sdpo.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/sdpo.py) to launch SDPO training from the command line. The script supports verifiable math rewards, environment feedback via `--feedback_column`, and PEFT/LoRA via the standard `ModelConfig` flags.

```bash
python examples/scripts/sdpo.py \
    --model_name_or_path Qwen/Qwen2.5-Math-1.5B-Instruct \
    --dataset_name openai/gsm8k \
    --dataset_config main \
    --output_dir outputs/sdpo-qwen35-2b-gsm8k \
    --learning_rate 5e-5 \
    --dtype bfloat16 \
    --bf16 true \
    --max_completion_length 128 \
    --use_peft \
    --lora_target_modules q_proj k_proj v_proj o_proj gate_proj up_proj down_proj \
    --per_device_train_batch_size 1 \
    --gradient_accumulation_steps 2 \
    --num_generations 8 \
    --generation_batch_size 32 \
    --distillation_alpha 1.0 \
    --distillation_mode sampled_token \
    --distillation_weight 0.5 \
    --report_to none \
    --eval_strategy steps \
    --eval_steps 1000 \
    --save_strategy no
```

## SDPOConfig[[trl.experimental.sdpo.SDPOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = False"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "max_prompt_length", "val": ": int | None = 512"}, {"name": "num_generations", "val": ": int = 8"}, {"name": "num_generations_eval", "val": ": int | None = None"}, {"name": "max_completion_length", "val": ": int | None = 256"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}, {"name": "shuffle_dataset", "val": ": bool = True"}, {"name": "generation_batch_size", "val": ": int | None = None"}, {"name": "steps_per_generation", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 1.0"}, {"name": "top_p", "val": ": float = 1.0"}, {"name": "top_k", "val": ": int = 0"}, {"name": "min_p", "val": ": float | None = None"}, {"name": "generation_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "chat_template_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "repetition_penalty", "val": ": float = 1.0"}, {"name": "cache_implementation", "val": ": str | None = None"}, {"name": "use_vllm", "val": ": bool = False"}, {"name": "use_teacher_server", "val": ": bool = False"}, {"name": "vllm_mode", "val": ": str = 'colocate'"}, {"name": "vllm_model_impl", "val": ": str = 'vllm'"}, {"name": "vllm_enable_sleep_mode", "val": ": bool = False"}, {"name": "vllm_server_base_url", "val": ": str | None = None"}, {"name": "vllm_server_host", "val": ": str = '0.0.0.0'"}, {"name": "vllm_server_port", "val": ": int = 8000"}, {"name": "vllm_group_port", "val": ": int = 51216"}, {"name": "vllm_server_timeout", "val": ": float = 240.0"}, {"name": "vllm_tensor_parallel_size", "val": ": int = 1"}, {"name": "vllm_gpu_memory_utilization", "val": ": float = 0.3"}, {"name": "vllm_max_model_length", "val": ": int | None = None"}, {"name": "num_iterations", "val": ": int = 1"}, {"name": "loss_type", "val": ": str = 'dapo'"}, {"name": "mask_truncated_completions", "val": ": bool = False"}, {"name": "dont_reprompt_on_self_success", "val": ": bool = True"}, {"name": "beta", "val": ": float = 0.0"}, {"name": "epsilon", "val": ": float = 0.2"}, {"name": "epsilon_high", "val": ": float | None = None"}, {"name": "importance_sampling_level", "val": ": str = 'token'"}, {"name": "reward_weights", "val": ": list[float] | None = None"}, {"name": "scale_rewards", "val": ": str | bool = 'group'"}, {"name": "distillation_alpha", "val": ": float = 1.0"}, {"name": "distillation_mode", "val": ": typing.Literal['sampled_token', 'full_logits', 'topk_logits'] = 'sampled_token'"}, {"name": "distillation_topk", "val": ": int | None = None"}, {"name": "distillation_is_clip", "val": ": float | None = 2.0"}, {"name": "distillation_add_tail", "val": ": bool = False"}, {"name": "distillation_weight", "val": ": float = 1.0"}, {"name": "teacher_model_kind", "val": ": str = 'ema'"}, {"name": "teacher_update_rate", "val": ": float = 0.05"}, {"name": "teacher_sync_steps", "val": ": int = 1"}, {"name": "max_reprompt_len", "val": ": int = 10240"}, {"name": "use_successful_as_teacher", "val": ": bool = True"}, {"name": "success_reward_threshold", "val": ": float = 1.0"}, {"name": "reprompt_template", "val": ": str = '{prompt}{solution}{feedback}\\n\\nCorrectly solve the original question.\\n'"}, {"name": "solution_template", "val": ": str = '\\nCorrect solution:\\n\\n{successful_previous_attempt}\\n\\n'"}, {"name": "feedback_template", "val": ": str = '\\nThe following is feedback from your unsuccessful earlier attempt:\\n\\n{feedback_raw}\\n\\n'"}, {"name": "include_environment_feedback", "val": ": bool = False"}, {"name": "environment_feedback_only_without_solution", "val": ": bool = False"}, {"name": "remove_thinking_from_demonstration", "val": ": bool = False"}, {"name": "diagnostics_warning_interval", "val": ": int = 10"}, {"name": "diagnostics_flat_tolerance", "val": ": float = 1e-08"}]}>
Parameters that control the online policy objective

- **beta** (`float`, *optional*, defaults to `0.0`) --
  KL coefficient. If `0.0` (default), the reference model is not loaded, reducing memory usage and improving
  training speed. [DeepSeek-R1 incentivizes reasoning in LLMs through reinforcement
  learning](https://huggingface.co/papers/2501.12948) use a value of `0.001`.
- **epsilon** (`float`, *optional*, defaults to `0.2`) --
  Epsilon value for clipping.
- **epsilon_high** (`float`, *optional*) --
  Upper-bound epsilon value for clipping. If not specified, it defaults to the same value as the lower-bound
  specified in argument `epsilon`. Paper DAPO recommends `0.28`.
- **importance_sampling_level** (`str`, *optional*, defaults to `"token"`) --
  Controls whether importance sampling ratios are computed at the `'token'` or `'sequence'` level. `'token'`
  keeps the raw per-token log-probability ratios (one weight per token). `'sequence'` averages the
  log-probability ratios across valid tokens to produce a single ratio per sequence. The GSPO paper shows
  that sequence-level sampling often yields more stable training and better alignment with sequence-level
  rewards.
- **reward_weights** (`list[float]`, *optional*) --
  Weights for each reward function. Must match the number of reward functions. If `None`, all rewards are
  weighted equally with weight `1.0`.
- **scale_rewards** (`str` or `bool`, *optional*, defaults to `"group"`) --
  Specifies the scaling strategy for rewards. Supported values are: `True` or `'group'` (default): rewards
  are scaled by the standard deviation within each group, ensuring unit variance within a group. `'batch'`:
  rewards are scaled by the standard deviation across the entire batch, as recommended in the PPO Lite paper.
  `False` or `'none'`: no scaling is applied. The Dr. GRPO paper recommends not scaling rewards, as scaling
  by the standard deviation introduces a question-level difficulty bias.

Parameters that control the SDPO loss

- **distillation_weight** (`float`, *optional*, defaults to `1.0`) --
  Convex combination weight between the policy and self-distillation objectives. The loss is `(1 -
  distillation_weight) * policy_loss + distillation_weight * distillation_loss`. Must be in `[0, 1]`. `1.0`
  (default) trains purely on self-distillation, `0.0` falls back to the standard GRPO-style policy gradient,
  and intermediate values blend both.
- **distillation_alpha** (`float`, *optional*, defaults to `1.0`) --
  Divergence interpolation coefficient. Sampled-token SDPO requires the official reverse-KL setting
  `distillation_alpha=1.0`.
- **distillation_mode** (`Literal["sampled_token", "full_logits", "topk_logits"]`, *optional*, defaults to `"sampled_token"`) --
  Distillation objective mode. `sampled_token` is the default SDPO mode and requires
  `distillation_alpha=1.0`.
- **distillation_topk** (`int`, *optional*) --
  Top-k approximation for logit-level SDPO. Must be set when `distillation_mode=topk_logits` and left unset
  otherwise.
- **distillation_is_clip** (`float`, *optional*, defaults to `2.0`) --
  Clipping coefficient for importance sampling in self-distillation. `None` disables clipping.
- **distillation_add_tail** (`bool`, *optional*, defaults to `False`) --
  Whether to add a tail bucket for non-top-k probability mass.

Parameters that control the teacher

- **teacher_model_kind** (`str`, *optional*, defaults to `"ema"`) --
  Semantic teacher choice. `base` uses the initial student, `live` uses the current student, and `ema` uses
  an exponentially averaged teacher.
- **teacher_update_rate** (`float`, *optional*, defaults to `0.05`) --
  Teacher update rate used for EMA teacher synchronization.
- **teacher_sync_steps** (`int`, *optional*, defaults to `1`) --
  How often to synchronize the EMA teacher model.

Parameters that control reprompting

- **use_successful_as_teacher** (`bool`, *optional*, defaults to `True`) --
  Use successful rollouts as implicit feedback for self-distillation.
- **success_reward_threshold** (`float`, *optional*, defaults to `1.0`) --
  Minimum reward for a rollout to be considered a successful demonstration.
- **dont_reprompt_on_self_success** (`bool`, *optional*, defaults to `True`) --
  Skip reprompting when model generates correct response.
- **max_reprompt_len** (`int`, *optional*, defaults to `10240`) --
  Maximum length for reprompting in self-distillation.
- **reprompt_template** (`str`, *optional*, defaults to `"{prompt}{solution}{feedback}\n\nCorrectly solve the original question.\n"`) --
  Template for reprompting the teacher with a successful demonstration.
- **solution_template** (`str`, *optional*, defaults to `"\nCorrect solution --
  \n\n{successful_previous_attempt}\n\n"`): Template for formatting the successful demonstration text.
- **feedback_template** (`str`, *optional*, defaults to `"\nThe following is feedback from your unsuccessful earlier attempt --
  \n\n{feedback_raw}\n\n"`): Template for formatting environment feedback for reprompting.
- **include_environment_feedback** (`bool`, *optional*, defaults to `False`) --
  Whether to include environment feedback in teacher reprompts when available.
- **environment_feedback_only_without_solution** (`bool`, *optional*, defaults to `False`) --
  Whether to use feedback only when no successful solution is available.
- **remove_thinking_from_demonstration** (`bool`, *optional*, defaults to `False`) --
  Whether to remove ... blocks from the demonstration text.

Parameters that control diagnostics

- **diagnostics_warning_interval** (`int`, *optional*, defaults to `10`) --
  Emit repeated trainer diagnostics every N consecutive degenerate steps. Set to 0 to disable.
- **diagnostics_flat_tolerance** (`float`, *optional*, defaults to `1e-8`) --
  Tolerance used to decide whether reward variance or reprompt activity is effectively zero.

Parameters that control the model

- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments for `transformers.AutoModelForCausalLM.from_pretrained`, used when the `model` argument
  of the `SDPOTrainer` is provided as a string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).
  Also applied to reward-model and reward-tokenizer loads.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model. This is useful for training with a reference model, as it prevents
  the model from generating different logprobs for the same input.

Parameters that control data preprocessing

- **remove_unused_columns** (`bool`, *optional*, defaults to `False`) --
  Whether to only keep the column 'prompt' in the dataset. If you use a custom reward function that requires
  any column other than 'prompts' and 'completions', you should keep this to `False`.
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
  supported for `teacher_model_kind='live'` with `use_vllm=True`, `vllm_mode='server'`,
  `distillation_weight=1.0` (pure distillation), and `distillation_mode` in {'sampled_token', 'topk_logits'}
  (the server returns the teacher's top-k logprobs, not the full vocabulary; `topk_logits` distills over the
  teacher's own top-k support).
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

- **loss_type** (`str`, *optional*, defaults to `"dapo"`) --
  Specifies the loss formulation to use. Supported values are 'grpo', 'bnpo', 'dr_grpo', and 'dapo'. 'grpo':
  Aggregates token-level losses by normalizing over sequence length. Not recommended due to length bias—this
  approach tends to prefer shorter completions with positive advantages and longer ones with negative
  advantages. 'dapo' (default): Aggregates token-level losses by normalizing with the number of active tokens
  in the global accumulated batch. This method was introduced in the DAPO paper to eliminate length bias.
  'dr_grpo': Aggregates token-level losses by normalizing with a global constant. This method was introduced
  in the Dr. GRPO paper to eliminate length bias. The value of the constant corresponds to
  `max_completion_length`. 'bnpo': Aggregates token-level losses by normalizing with the number of active
  tokens in the local batch. Note that normalization is performed over the local batch only, so results may
  slightly vary depending on the local batch size, despite a constant effective batch size. When using
  `per_device_train_batch_size==1`, the loss is equivalent to the GRPO loss.
- **num_iterations** (`int`, *optional*, defaults to `1`) --
  Number of iterations per batch (denoted as μ in the algorithm).
- **generation_batch_size** (`int`, *optional*) --
  Batch size to use for generation. If `None`, it defaults to the effective training batch size:
  `per_device_train_batch_size * num_processes * steps_per_generation`.
- **steps_per_generation** (`int`, *optional*) --
  Number of steps per generation. If `None`, it defaults to `gradient_accumulation_steps`.
- **mask_truncated_completions** (`bool`, *optional*, defaults to `False`) --
  When enabled, truncated completions are excluded from the loss calculation, preventing them from being
  incorrectly penalized and introducing noise during training. According to the DAPO paper, this is a good
  practice for training stability.

Configuration class for the `SDPOTrainer`.

## SDPOTrainer[[trl.experimental.sdpo.SDPOTrainer]]

Trainer for Self-Distillation Policy Optimization (SDPO).

SDPO augments on-policy optimization with self-distillation from the model's own high-reward trajectories. It
converts tokenized feedback into a dense learning signal without any external teacher or explicit reward model.
SDPO treats the current model conditioned on feedback as a self-teacher and distills its feedback-informed
next-token predictions back into the policy.

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
