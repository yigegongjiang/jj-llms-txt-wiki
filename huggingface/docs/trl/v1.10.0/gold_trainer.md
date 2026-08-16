# General Online Logit Distillation (GOLD) Trainer

[![All_models-GOLD-blue](https://img.shields.io/badge/All_models-GOLD-blue)](https://huggingface.co/models?other=sft,gold)

## Overview

General Online Logit Distillation (GOLD) is an extension of Universal Logit Distillation (ULD) that supports
student/teacher pairs with different tokenizers. It aligns the textual spans produced by both tokenizers and merges the
associated logits so no completion tokens are dropped. This enables cross-tokenizer knowledge distillation, including
mixed model families (for example, LLaMA students with Qwen teachers).

Key capabilities:

1. **Cross-tokenizer alignment** – GOLD incrementally decodes the student and teacher tokens, groups passages with the same visible text, and merges probabilities inside each group. This guarantees loss terms are computed over the full completion even when token boundaries differ.
2. **Hybrid ULD loss** – when `uld_use_hybrid_loss` is enabled, GOLD compares exact vocabulary matches directly and falls back to the original sorted-probability ULD loss for unmatched tokens. This improves stability for students whose vocabularies only partially overlap with the teacher.
3. **Seamless integration with GKD** – GOLD inherits the on-policy vs. off-policy scheduling from the [experimental.gkd.GKDTrainer](/docs/trl/v1.10.0/en/gkd_trainer#trl.experimental.gkd.GKDTrainer), so you can combine sequence-level KD, generalized JSD, and cross-tokenizer distillation in a single training run.

> [!NOTE]
> GOLD is currently part of the `trl.experimental` namespace. APIs may change without notice while the feature is iterated on.

## Usage tips

The `GOLDTrainer` subclasses [SFTTrainer](/docs/trl/v1.10.0/en/sft_trainer#trl.SFTTrainer) and accepts the same datasets as other TRL trainers (lists of ChatML style
messages). Important configuration flags on `GOLDConfig` include:

* `use_uld_loss` – toggles Universal Logit Distillation. Set this to `True` for cross-tokenizer setups.
* `teacher_tokenizer_name_or_path` – required when `use_uld_loss=True`; GOLD uses the teacher tokenizer to align tokens.
* `uld_use_hybrid_loss`, `uld_hybrid_matched_weight`, `uld_hybrid_unmatched_weight` – enables and weights the hybrid
  matched/unmatched loss.
* `beta`, `lmbda`, `seq_kd` – inherited from [experimental.gkd.GKDConfig](/docs/trl/v1.10.0/en/gkd_trainer#trl.experimental.gkd.GKDConfig), controlling the generalized JSD interpolation and on-policy
  sampling ratio.
* `num_generations`, `generation_batch_size` – control buffered rollout generation across gradient accumulation windows.
  `generation_batch_size` is the number of unique prompts per worker per optimizer step.

A minimal end-to-end example:

```python
from datasets import load_dataset
from trl.experimental.gold import GOLDConfig, GOLDTrainer

train_dataset = load_dataset(
    "HuggingFaceTB/OpenR1-Math-220k-default-verified",
    "all",
    split="train[:1024]",
)

trainer = GOLDTrainer(
    model="meta-llama/Llama-3.2-1B-Instruct",
    teacher_model="Qwen/Qwen2.5-0.5B-Instruct",
    args=GOLDConfig(output_dir="gold-model", use_uld_loss=True, teacher_tokenizer_name_or_path="Qwen/Qwen2.5-0.5B-Instruct"),
    train_dataset=train_dataset,
)
trainer.train()
```

For quick-start workflows you can rely on string identifiers as shown above—the trainer will load the model and tokenizer for you. Explicitly instantiating `AutoModelForCausalLM`, `AutoTokenizer`, or populating `GOLDConfig` is recommended only for advanced use cases where you need fine-grained control over initialization.

A more explicit setup might look like this when you need to customise model loading, tokenizer settings, or training arguments:

```python
from datasets import load_dataset
from trl.experimental.gold import GOLDConfig, GOLDTrainer
from transformers import AutoModelForCausalLM, AutoTokenizer

student_name = "meta-llama/Llama-3.2-1B-Instruct"
teacher_name = "Qwen/Qwen2.5-0.5B-Instruct"

tokenizer = AutoTokenizer.from_pretrained(student_name)
if tokenizer.pad_token is None:
    tokenizer.pad_token = tokenizer.eos_token

model = AutoModelForCausalLM.from_pretrained(student_name)
teacher_model = AutoModelForCausalLM.from_pretrained(teacher_name)

train_dataset = load_dataset(
    "HuggingFaceTB/Countdown-Task-GOLD",
    "verified_Qwen2.5-0.5B-Instruct",
    split="train",
)

training_args = GOLDConfig(
    output_dir="gold-model",
    per_device_train_batch_size=1,
    teacher_model_name_or_path=teacher_name,
    teacher_tokenizer_name_or_path=teacher_name,
    use_uld_loss=True,
    uld_use_hybrid_loss=True,
)

trainer = GOLDTrainer(
    model=model,
    teacher_model=teacher_model,
    args=training_args,
    processing_class=tokenizer,
    train_dataset=train_dataset,
)
trainer.train()
```

> [!NOTE]
> GOLD buffers one full optimizer-window generation batch (`per_device_train_batch_size * gradient_accumulation_steps`)
> and reuses it across accumulation steps. If the final batch is undersized, GOLD warns and drops that last batch
> (`Dropping last batch due to unexpected batch size`). Set `dataloader_drop_last=True` to avoid this warning.

### Expected dataset type

GOLD requires a [conversational](dataset_formats#conversational) [language modeling](dataset_formats#language-modeling) dataset, e.g.:

```python
{"messages": [{"role": "user", "content": "What color is the sky?"},
              {"role": "assistant", "content": "It is blue."}]}
```

`GOLDTrainer` keeps the raw messages so the ChatML collator can construct prompts and completions with the correct
boundaries.

## How Token Merging Works

When student and teacher use different tokenizers, the same text may be split differently:

- **Student**: `"Hugging Face"` → 1 token
- **Teacher**: `"Hugging"`, `" Face"` → 2 tokens

GOLD aligns these sequences and merges the teacher's multi-token probabilities into a single distribution that can be compared with the student's single-token distribution.

### Probability Merging

For a teacher sequence of tokens `[token₀, token₁, ..., tokenₖ]` that maps to a single student token, GOLD computes:

```
P_merged(y) = P(y | context) × P(token₁ | token₀, context) × ... × P(tokenₖ | ..., context)
```

where:
- `P(y | context)` is the marginal probability distribution over all vocabulary tokens at the first position
- `P(tokenᵢ | ..., context)` are **scalar** conditional probabilities of the actual tokens that were generated

**Key insight**: Only the conditional probabilities of the **actual continuation tokens** are extracted as scalars. The full marginal distribution at the first position is then scaled by multiplying these scalar probabilities.

This ensures:
1. **Correct joint probability** for the actual generated sequence (by the chain rule)
2. **Reasonable approximation** for counterfactual tokens (scaled by the same continuation likelihood)
3. **Unnormalized distributions** that preserve the correct relative probabilities for ULD loss computation

### Example

Given:
```
P(x₀):         ["HF": 0.6,  "is": 0.3,  "cool": 0.1]
P(x₁ | "HF"):  ["HF": 0.05, "is": 0.9,  "cool": 0.05]
```

If tokens 0 and 1 are merged, and the actual sequence was `["HF", "is"]`:
```
P_merged("HF")   = 0.6 × 0.9 = 0.54  ✓ (correct joint probability)
P_merged("is")   = 0.3 × 0.9 = 0.27
P_merged("cool") = 0.1 × 0.9 = 0.09
```

The merged distribution is unnormalized (sums to 0.81), but this is intentional and correct for ULD loss computation, which uses sorting and L1 distance.

## Example script

Use [`examples/scripts/gold.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/gold.py) to launch GOLD training from the command line. The script supports full training and LoRA via the standard `ModelConfig` flags.

```bash
python examples/scripts/gold.py \
    --model_name_or_path meta-llama/Llama-3.2-1B-Instruct \
    --teacher_model_name_or_path Qwen/Qwen2-1.5B-Instruct \
    --dataset_name trl-lib/chatbot_arena_completions \
    --learning_rate 2e-5 \
    --per_device_train_batch_size 4 \
    --gradient_accumulation_steps 8 \
    --output_dir gold-model \
    --num_train_epochs 1 \
    --push_to_hub
```

## Training Vision Language Models

`GOLDTrainer` supports VLM-to-VLM distillation. Both student and teacher must be vision-language models. To train a VLM, provide a dataset with either an `image` column (single image per sample) or an `images` column (list of images per sample). For more information on the expected dataset structure, see the [Dataset Format — Vision datasets](dataset_formats#vision-datasets) section.

When the student and teacher share the same architecture and tokenizer (e.g. Qwen3-VL-8B to Qwen3-VL-2B), the standard generalized JSD loss applies directly. When they have different `model_type` (e.g. Qwen3-VL to LFM2.5-VL), set `use_uld_loss=True` to enable cross-tokenizer alignment via Universal Logit Distillation. Images are processed separately through each model's processor.

```python
from datasets import load_dataset
import torch
from transformers import AutoModelForImageTextToText, AutoProcessor
from trl.experimental.gold import GOLDConfig, GOLDTrainer

student_name = "Qwen/Qwen3-VL-2B-Instruct"
teacher_name = "Qwen/Qwen3-VL-8B-Instruct"

processor = AutoProcessor.from_pretrained(student_name, padding_side="left")
student_model = AutoModelForImageTextToText.from_pretrained(student_name, dtype=torch.bfloat16)
teacher_model = AutoModelForImageTextToText.from_pretrained(teacher_name, dtype=torch.bfloat16)

train_dataset = load_dataset("trl-lib/llava-instruct-mix", split="train")

trainer = GOLDTrainer(
    model=student_model,
    teacher_model=teacher_model,
    args=GOLDConfig(
        output_dir="gold-vlm-model",
        max_length=None,
        teacher_model_name_or_path=teacher_name,
        use_uld_loss=False,
    ),
    train_dataset=train_dataset,
    processing_class=processor,
)
trainer.train()
```

For cross-family distillation, set `use_uld_loss=True` and `teacher_tokenizer_name_or_path` to the teacher model name.

Use [`trl/experimental/gold/gold_vlm.py`](https://github.com/huggingface/trl/blob/main/trl/experimental/gold/gold_vlm.py) to launch GOLD VLM training from the command line:

```bash
# Same-family distillation (JSD loss, vLLM enabled)
accelerate launch trl/experimental/gold/gold_vlm.py \
    --student_model_name Qwen/Qwen3-VL-2B-Instruct \
    --teacher_model_name Qwen/Qwen3-VL-8B-Instruct

# Cross-family distillation (ULD loss, local generation)
accelerate launch trl/experimental/gold/gold_vlm.py \
    --student_model_name LiquidAI/LFM2.5-VL-1.6B \
    --teacher_model_name Qwen/Qwen3-VL-8B-Instruct \
    --use_uld_loss \
    --no-use_vllm
```

> [!TIP]
> For VLMs, `truncation_mode='keep_end'` is not supported because image tokens reside in the prompt portion of the sequence and may be silently dropped. Use `truncation_mode='keep_start'` (the default) or set `max_length=None` in the `GOLDConfig`. This allows the model to process the full sequence length without truncating image tokens.
>
> ```python
> GOLDConfig(max_length=None, ...)
> ```
>
> Only use `max_length` when you've verified that truncation won't remove image tokens for the entire dataset.

> [!NOTE]
> Cross-architecture VLM distillation requires `use_uld_loss=True`. The trainer will raise an error if you attempt cross-architecture distillation without ULD loss.

## GOLDTrainer[[trl.experimental.gold.GOLDTrainer]]

#### trl.experimental.gold.GOLDTrainer[[trl.experimental.gold.GOLDTrainer]]

```python
trl.experimental.gold.GOLDTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str, NoneType] = None, teacher_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str] = None, args: trl.experimental.gold.gold_config.GOLDConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, peft_config: typing.Optional[ForwardRef('PeftConfig')] = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/gold/gold_trainer.py#L768)

#### train[[trl.experimental.gold.GOLDTrainer.train]]

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

#### generate_on_policy_outputs[[trl.experimental.gold.GOLDTrainer.generate_on_policy_outputs]]

```python
generate_on_policy_outputs(model, inputs, generation_config)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/gold/gold_trainer.py#L2633)

#### save_model[[trl.experimental.gold.GOLDTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.gold.GOLDTrainer.push_to_hub]]

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

## GOLDConfig[[trl.experimental.gold.GOLDConfig]]

#### trl.experimental.gold.GOLDConfig[[trl.experimental.gold.GOLDConfig]]

```python
trl.experimental.gold.GOLDConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-07, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, router_aux_loss_coef: float = 0.001, trust_remote_code: bool = False, chat_template_path: str | None = None, dataset_text_field: str = 'text', dataset_kwargs: dict[str, typing.Any] | None = None, dataset_num_proc: int | None = None, eos_token: str | None = None, max_length: int | None = 1024, truncation_mode: str = 'keep_start', shuffle_dataset: bool = False, packing: bool = False, packing_strategy: str = 'bfd', padding_free: bool = False, pad_to_multiple_of: int | None = None, eval_packing: bool | None = None, completion_only_loss: bool | None = None, assistant_only_loss: bool = False, loss_type: str | None = None, activation_offloading: bool = False, pad_token: str | None = None, temperature: float = 0.9, top_p: float = 0.95, top_k: int = 0, lmbda: float = 0.5, beta: float = 0.5, max_completion_length: int = 128, teacher_model_name_or_path: str | None = None, teacher_model_revision: str | None = None, teacher_model_init_kwargs: dict[str, typing.Any] | str | None = None, teacher_tokenizer_name_or_path: str | None = None, disable_dropout: bool = True, seq_kd: bool = False, num_generations: int = 1, generation_batch_size: int | None = None, use_uld_loss: bool = False, uld_token_merge_strategy: str = 'observed', use_extended_uld: bool = True, uld_use_hybrid_loss: bool = False, uld_hybrid_matched_weight: float | None = None, uld_hybrid_unmatched_weight: float | None = None, uld_crossentropy_weight: float = 0.0, uld_distillation_weight: float = 1.0, uld_student_temperature: float = 1.0, uld_teacher_temperature: float = 1.0, uld_skip_student_eos: bool = True, uld_skip_teacher_eos: bool = True, use_vllm: bool = False, vllm_mode: str = 'colocate', vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8001, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_gpu_memory_utilization: float = 0.9, vllm_tensor_parallel_size: int = 1, vllm_max_model_length: int | None = None, vllm_model_impl: str = 'vllm', vllm_structured_outputs_regex: str | None = None, vllm_sync_frequency: int = 1, vllm_enable_sleep_mode: bool = False, log_completions: bool = False, log_completions_steps: int = 100, num_completions_to_print: int | None = None, wandb_log_unique_prompts: bool = True, callbacks: list = <factory>)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/gold/gold_config.py#L23)

**Parameters that control generation and the training loop:**

temperature (`float`, *optional*, defaults to `0.9`) : Temperature for sampling. The higher the temperature, the more random the completions.

top_p (`float`, *optional*, defaults to `0.95`) : If set to float < 1, only the smallest set of most probable tokens with probabilities that add up to `top_p` or higher are kept for generation.

top_k (`int`, *optional*, defaults to `0`) : Number of highest probability vocabulary tokens to keep for top-k-filtering. If `0`, top-k-filtering is disabled and all tokens are considered.

lmbda (`float`, *optional*, defaults to `0.5`) : Lambda parameter that controls the student data fraction (i.e., the proportion of on-policy student-generated outputs).

beta (`float`, *optional*, defaults to `0.5`) : Interpolation coefficient between `0.0` and `1.0` of the Generalized Jensen-Shannon Divergence loss. When beta is `0.0`, the loss is the KL divergence. When beta is `1.0`, the loss is the Inverse KL Divergence.

max_completion_length (`int`, *optional*, defaults to `128`) : Maximum number of tokens to generate per completion.

teacher_model_name_or_path (`str`, *optional*) : Model name or path of the teacher model. If `None`, the teacher model will be the same as the model being trained.

teacher_model_revision (`str` or `None`, *optional*, defaults to `None`) : Model revision of the teacher model (e.g., branch name, tag, or commit hash). If `None`, the default revision is used.

teacher_model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the teacher model from a string.

teacher_tokenizer_name_or_path (`str`, *optional*) : Tokenizer name or path for the teacher model. If None when using ULD loss, will use the same tokenizer as the student model (not recommended for cross-tokenizer distillation).

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

seq_kd (`bool`, *optional*, defaults to `False`) : Seq_kd parameter that controls whether to perform Sequence-Level KD (can be viewed as supervised FT on teacher-generated output).

num_generations (`int`, *optional*, defaults to `1`) : Number of generations per prompt. Each prompt is repeated this many times in the generation batch.

generation_batch_size (`int` or `None`, *optional*, defaults to `None`) : Number of unique prompts per worker per optimizer step. If `None`, it is computed from `(per_device_train_batch_size * gradient_accumulation_steps) // num_generations`.

**Parameters that control the ULD loss:**

use_uld_loss (`bool`, *optional*, defaults to `False`) : Whether to use Universal Logit Distillation (ULD) loss instead of Generalized Jensen-Shannon Divergence loss.

use_extended_uld (`bool`, *optional*, defaults to `True`) : Whether to enable extended ULD alignment that uses tokenizers to align and merge token probabilities across student and teacher tokenizations. When `True`, the trainer will compute token mappings and merge probabilities for split tokens; when `False`, ULD will use simple positional truncation like in the original ULD paper.

uld_token_merge_strategy (`str`, *optional*, defaults to `"observed"`) : Strategy used to align answer logits and merge token probabilities in the ULD loss. With `"observed"`, the answer logits are sliced at the answer positions and split tokens (when `use_extended_uld=True`) are merged by multiplying the marginal distribution at the first position by the scalar conditional probabilities of the actual later tokens. With `"bayesian"`, the answer-logit slice is shifted one position earlier so that `probs[k]` predicts `token_ids[k]` (chain rule), and split tokens are merged using the last position's full distribution, conditioned on the actual prefix tokens, multiplied by the scalar probabilities of the earlier tokens. The logit-alignment shift applies whether or not `use_extended_uld` is enabled.

uld_use_hybrid_loss (`bool`, *optional*, defaults to `False`) : Whether to use a hybrid loss that combines ULD loss and JSD loss. When `True`, the final loss is a combination of JSD for known token mappings and ULD for unknown token mappings.

uld_hybrid_matched_weight (`float` or `None`, *optional*) : Weight for the matched token loss component when using hybrid ULD + JSD loss. This weight scales the JSD loss computed over tokens that have a direct mapping between student and teacher tokenizations. If `None`, uses adaptive weighting based on vocabulary overlap. Must be set together with `uld_hybrid_unmatched_weight` (both `None` or both `float`).

uld_hybrid_unmatched_weight (`float` or `None`, *optional*) : Weight for the unmatched token loss component when using hybrid ULD + JSD loss. This weight scales the ULD loss computed over tokens that do not have a direct mapping between student and teacher tokenizations. If `None`, uses adaptive weighting based on vocabulary overlap. Must be set together with `uld_hybrid_matched_weight` (both `None` or both `float`).

uld_crossentropy_weight (`float`, *optional*, defaults to `0.0`) : Weight for the cross-entropy loss component in ULD loss. If 0, only ULD distillation loss is used.

uld_distillation_weight (`float`, *optional*, defaults to `1.0`) : Weight for the distillation loss component in ULD loss.

uld_student_temperature (`float`, *optional*, defaults to `1.0`) : Temperature for student logits in ULD loss computation.

uld_teacher_temperature (`float`, *optional*, defaults to `1.0`) : Temperature for teacher logits in ULD loss computation.

uld_skip_student_eos (`bool`, *optional*, defaults to `True`) : Whether to skip EOS token for student in ULD loss computation.

uld_skip_teacher_eos (`bool`, *optional*, defaults to `True`) : Whether to skip EOS token for teacher in ULD loss computation.

**Parameters that control vLLM integration:**

use_vllm (`bool`, *optional*, defaults to `False`) : Whether to use vLLM for generating completions from the student model. Requires `vllm` to be installed.

vllm_mode (`str`, *optional*, defaults to `"colocate"`) : Mode for student vLLM integration. Either `"server"` (connect to a running TRL vLLM server) or `"colocate"` (run vLLM in the same process).

vllm_server_host (`str`, *optional*, defaults to `"0.0.0.0"`) : Host of the vLLM server for the student model (if `vllm_mode="server"`).

vllm_server_port (`int`, *optional*, defaults to `8001`) : Port of the vLLM server for the student model (if `vllm_mode="server"`).

vllm_server_timeout (`float`, *optional*, defaults to `240.0`) : Timeout for connecting to the student vLLM server (if `vllm_mode="server"`).

vllm_gpu_memory_utilization (`float`, *optional*, defaults to `0.9`) : GPU memory utilization for the colocated student vLLM engine (if `vllm_mode="colocate"`). It is recommended to set this to a low value if the student and teacher models share the same GPU.

vllm_tensor_parallel_size (`int`, *optional*, defaults to `1`) : Tensor parallel size for the colocated student vLLM engine (if `vllm_mode="colocate"`).

vllm_structured_outputs_regex (`str`, *optional*) : Regex for vLLM structured outputs for the student model.

vllm_server_base_url (`str`, *optional*) : Base URL for the vLLM server (e.g., `"http://localhost:8001"`). If provided, `vllm_server_host` and `vllm_server_port` are ignored.

vllm_group_port (`int`, *optional*, defaults to `51216`) : Port for the vLLM weight-update group (NCCL communicator). Unless the port is occupied, there is no need to change it.

vllm_max_model_length (`int`, *optional*) : Maximum model sequence length for the colocated vLLM engine when `vllm_mode="colocate"`. Defaults to the model's maximum context length.

vllm_model_impl (`str`, *optional*, defaults to `"vllm"`) : Model implementation backend to use in vLLM. Use `"vllm"` (default) or `"transformers"`.

vllm_sync_frequency (`int`, *optional*, defaults to `1`) : Frequency (in training steps) to synchronize student model weights to vLLM engine. Set to 1 to sync after every step.

vllm_enable_sleep_mode (`bool`, *optional*, defaults to `False`) : Enable vLLM sleep mode to offload student weights/cache during the optimizer step. Keeps GPU memory usage low, but waking the engine adds host–device transfer latency.

**Parameters that control logging:**

log_completions (`bool`, *optional*, defaults to `False`) : Whether to log a sample of (prompt, completion) pairs every `logging_steps` steps. If `rich` is installed, it prints the sample. If `wandb` logging is enabled, it logs it to `wandb`.

log_completions_steps (`int`, *optional*, defaults to `100`) : Number of steps between logging (prompt, completion) pairs. Only used if `log_completions` is set to `True`.

num_completions_to_print (`int` or `None`, *optional*) : Number of completions to print with `rich`. If `None`, all completions are logged.

wandb_log_unique_prompts (`bool`, *optional*, defaults to `True`) : Whether to log the unique prompts to wandb. This will create a new run for each unique prompt.

callbacks (`list[str]`, *optional*, defaults to `[]`) : The callbacks to run during training.

Configuration class for `GOLDTrainer`.

This class includes only the parameters that are specific to GOLD training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments) and [SFTConfig](/docs/trl/v1.10.0/en/sft_trainer#trl.SFTConfig) documentation.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments):
> - `learning_rate`: Defaults to `1e-7` instead of `5e-5`.
