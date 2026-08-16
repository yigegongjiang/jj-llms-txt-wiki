# MiniLLM Trainer

[![All_models-MiniLLM-blue](https://img.shields.io/badge/All_models-MiniLLM-blue)](https://huggingface.co/models?other=minillm,trl)

## Overview

TRL supports the MiniLLM Trainer for distilling large language models into smaller ones using reverse KLD for better precision, quality, and performance, as described in the paper [Knowledge Distillation of Large Language Models](https://huggingface.co/papers/2306.08543) by [Yuxian Gu](https://huggingface.co/t1101675), [Li Dong](https://huggingface.co/unilm), [Furu Wei](https://huggingface.co/thegenerality), and Minlie Huang.
The abstract from the paper is the following:

> Knowledge Distillation (KD) is a promising technique for reducing the high computational demand of large language models (LLMs). However, previous KD methods are primarily applied to white-box classification models or training small models to imitate black-box model APIs like ChatGPT. How to effectively distill the knowledge from white-box generative LLMs is still under-explored, which becomes more and more important with the prosperity of LLMs. In this work, we propose MiniLLM that distills smaller language models from generative larger language models. We first replace the forward Kullback-Leibler divergence (KLD) objective in the standard KD approaches with reverse KLD, which is more suitable for KD on generative language models, to prevent the student model from overestimating the low-probability regions of the teacher distribution. Then, we derive an effective optimization approach to learn this objective. Extensive experiments in the instruction-following setting show that the MiniLLM models generate more precise responses with the higher overall quality, lower exposure bias, better calibration, and higher long-text generation performance. Our method is also scalable for different model families with 120M to 13B parameters. We will release our code and model checkpoints at https://aka.ms/MiniLLM.

This post-training method was contributed by [Yuxian Gu](https://huggingface.co/t1101675).

It is a generalized version of [Think Machine Lab's On-Policy Distillation](https://thinkingmachines.ai/blog/on-policy-distillation/), with the option to add distribution-level single-step distillation signals (like GKD when `beta=1`) and long-context reverse KLD signals.

$$
\begin{align}
L_{\text{MiniLLM}}&=\alpha_1\mathbb{E}_{x\sim \pi_{\theta}}\sum_{t'=t}^{|x|}\frac{\gamma^{t'-t}}{\sum_{t'}\gamma^{t'-t}}\left[\log \frac{\pi_{\theta}(x_{t'+1}|x_{1..t'})}{\pi_{\text{teacher}}(x_{t'+1}|x_{1..t'})}\right] \\
&+ \alpha_2\mathbb{E}_{x\sim \pi_{\theta}} \text{KL}\left[\pi_\theta(\cdot|x_{1..t})||\pi_{\text{teacher}}(\cdot | x_{1..t})\right].
\end{align}
$$

When  \\( \alpha_1=1 \\), \\( \alpha_2=0 \\), \\( \gamma=0 \\), which corresponds to

```python
from trl.experimental.minillm import MiniLLMConfig

training_args = MiniLLMConfig(
    rkl_advantage=True,
    single_step_decomposition=False,
    gamma=0.0
)
```

\\( L_{\text{MiniLLM}} \\) becomes the on-policy KD implemented in [Tinker](https://github.com/thinking-machines-lab/tinker-cookbook/blob/5d08be6d130596b7bedd02197861c41fa81ea436/tinker_cookbook/distillation/train_on_policy.py#L88):

$$
L_{\text{tinker}}=\mathbb{E}_{x\sim \pi_{\theta}}\left[\log \frac{\pi_{\theta}(x_{t'+1}|x_{1..t'})}{\pi_{\text{teacher}}(x_{t'+1}|x_{1..t'})}\right].
$$

When \\( \alpha_1=0 \\), \\( \alpha_2=1 \\), which corresponds to

```python
from trl.experimental.minillm import MiniLLMConfig

training_args = MiniLLMConfig(
    rkl_advantage=False,
    single_step_decomposition=True
)
```

\\( L_{\text{MiniLLM}} \\) becomes the reverse KLD version of the GKD loss as in [GKD Trainer](gkd_trainer):

$$
L_{\text{GKD-RKL}}=\mathbb{E}_{x\sim \pi_{\theta}} \text{KL}\left[\pi_\theta(\cdot|x_{1..t})||\pi_{\text{teacher}}(\cdot | x_{1..t})\right].
$$

## MiniLLMTrainer[[trl.experimental.minillm.MiniLLMTrainer]]

#### trl.experimental.minillm.MiniLLMTrainer[[trl.experimental.minillm.MiniLLMTrainer]]

```python
trl.experimental.minillm.MiniLLMTrainer(model: str | transformers.modeling_utils.PreTrainedModel, teacher_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str], reward_funcs: str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]] | list[str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]]] | None = None, args: trl.experimental.minillm.minillm_config.MiniLLMConfig | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.processing_utils.ProcessorMixin | None = None, reward_processing_classes: transformers.tokenization_utils_base.PreTrainedTokenizerBase | list[transformers.tokenization_utils_base.PreTrainedTokenizerBase] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), peft_config: PeftConfig | None = None, rollout_func: collections.abc.Callable[[list[str], 'GRPOTrainer'], dict[str, typing.Any]] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/minillm/minillm_trainer.py#L48)

**Parameters:**

model (`str | PreTrainedModel`) : Model to be trained. Can be either:  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) with the keyword arguments in `args.model_init_kwargs`. - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported.

teacher_model (`PreTrainedModel | nn.Module | str`) : Teacher model used for knowledge distillation. Instantiated similarly to `model`.

reward_funcs (`RewardFunc | list[RewardFunc]`, *optional*) : Reward functions to be used for computing the rewards. To compute the rewards, we call all the reward functions with the prompts and completions and sum the rewards. Can be either:  - A single reward function, such as: - A string: The *model ID* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForSequenceClassification.from_pretrained) with `num_labels=1` and the keyword arguments in `args.model_init_kwargs`. - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel) object: Only sequence classification models are supported. - A custom reward function: The function is provided with the prompts and the generated completions, plus any additional columns in the dataset. It should return a list of rewards. Custom reward functions can also return `None` when the reward is not applicable to those samples. This is useful for multi-task training where different reward functions apply to different types of samples. When a reward function returns `None` for a sample, that reward function is excluded from the reward calculation for that sample. For more details, see [Using a custom reward function](#using-a-custom-reward-function).  The trainer's state is also passed to the reward function. The trainer's state is an instance of [TrainerState](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/callback#transformers.TrainerState) and can be accessed by accessing the `trainer_state` argument to the reward function's signature. - A list of reward functions, where each item can independently be any of the above types. Mixing different types within the list (e.g., a string model ID and a custom reward function) is allowed.

args ([experimental.minillm.MiniLLMConfig](/docs/trl/v1.10.0/en/minillm_trainer#trl.experimental.minillm.MiniLLMConfig), *optional*) : Configuration for this trainer. If `None`, a default configuration is used.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset)) : Dataset to use for training. It must include a column `"prompt"`. Any additional columns in the dataset is ignored. The format of the samples can be either:  - [Standard](dataset_formats#standard): Each sample contains plain text. - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role and content).

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset) or `dict[str, Dataset | IterableDataset]`) : Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.15.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [ProcessorMixin](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. The padding side must be set to "left". If `None`, the processing class is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoProcessor.from_pretrained). A padding token, `tokenizer.pad_token`, must be set. If the processing class has not set a padding token, `tokenizer.eos_token` will be used as the default.

reward_processing_classes ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.15.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or `list[PreTrainedTokenizerBase]`, *optional*) : Processing classes corresponding to the reward functions specified in `reward_funcs`. Can be either:  - A single processing class: Used when `reward_funcs` contains only one reward function. - A list of processing classes: Must match the order and length of the reward functions in `reward_funcs`. If set to `None`, or if an element of the list corresponding to a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel) is `None`, the tokenizer for the model is automatically loaded using [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoTokenizer.from_pretrained). For elements in `reward_funcs` that are custom reward functions (not [PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel)), the corresponding entries in `reward_processing_classes` are ignored.

callbacks (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/callback#transformers.TrainerCallback), *optional*) : List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed in [here](https://huggingface.co/docs/transformers/main_classes/callback).  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.Trainer.remove_callback) method.

optimizers (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`, *optional*, defaults to `(None, None)`) : A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your model and a scheduler given by `get_linear_schedule_with_warmup` controlled by `args`.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : PEFT configuration used to wrap the model. If `None`, the model is not wrapped.

rollout_func (`RolloutFunc`, *optional*) : Function to use for generating completions. It must take prompts, args, and processing_class as parameters and return a dict with `"prompt_ids"`, `"completion_ids"`, and `"logprobs"` fields. Any other fields that are forwarded to the reward functions. This feature is experimental and may change or be removed at any time without prior notice.

Trainer for the Knowledge Distillation of Language Models (MiniLLM) method. This algorithm was initially proposed
in the paper [Knowledge Distillation of Large Language Models](https://huggingface.co/papers/2306.08543).

Example:

```python
>>> from datasets import load_dataset
>>> from trl.experimental.minillm import MiniLLMTrainer

>>> dataset = load_dataset("trl-lib/tldr", split="train")

>>> trainer = MiniLLMTrainer(
...     model="Qwen/Qwen3-0.6B",
...     teacher_model="Qwen/Qwen3-1.7B",
...     train_dataset=dataset,
... )
>>> trainer.train()
```

#### train[[trl.experimental.minillm.MiniLLMTrainer.train]]

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

#### save_model[[trl.experimental.minillm.MiniLLMTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.minillm.MiniLLMTrainer.push_to_hub]]

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

## MiniLLMConfig[[trl.experimental.minillm.MiniLLMConfig]]

#### trl.experimental.minillm.MiniLLMConfig[[trl.experimental.minillm.MiniLLMConfig]]

```python
trl.experimental.minillm.MiniLLMConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-06, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool | None = False, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, router_aux_loss_coef: float = 0.001, disable_dropout: bool = True, cast_lm_head_to_fp32: bool = False, num_generations: int | None = 8, num_generations_eval: int | None = None, max_completion_length: int | None = 512, ds3_gather_for_generation: bool = True, shuffle_dataset: bool | None = True, pad_to_multiple_of: int | None = None, generation_batch_size: int | None = None, steps_per_generation: int | None = None, temperature: float = 1.0, top_p: float = 1.0, top_k: int = 0, min_p: float | None = None, generation_kwargs: dict | None = None, chat_template_kwargs: dict | None = None, repetition_penalty: float = 1.0, cache_implementation: str | None = None, use_vllm: bool = False, vllm_mode: str = 'colocate', vllm_model_impl: str = 'vllm', vllm_enable_sleep_mode: bool = False, vllm_structured_outputs_regex: str | None = None, vllm_server_base_url: str | None = None, vllm_server_host: str = '0.0.0.0', vllm_server_port: int = 8000, vllm_server_timeout: float = 240.0, vllm_group_port: int = 51216, vllm_gpu_memory_utilization: float = 0.3, vllm_max_model_length: int | None = None, vllm_tensor_parallel_size: int = 1, beta: float = 0.0, num_iterations: int = 1, epsilon: float = 0.2, delta: float | None = None, epsilon_high: float | None = None, sapo_temperature_neg: float = 1.05, sapo_temperature_pos: float = 1.0, vespo_k_pos: float = 2.0, vespo_lambda_pos: float = 3.0, vespo_k_neg: float = 3.0, vespo_lambda_neg: float = 2.0, importance_sampling_level: str = 'token', reward_weights: list[float] | None = None, multi_objective_aggregation: str = 'sum_then_normalize', scale_rewards: str = 'group', loss_type: str = 'dapo', mask_truncated_completions: bool = False, sync_ref_model: bool = False, ref_model_mixup_alpha: float = 0.6, ref_model_sync_steps: int = 512, top_entropy_quantile: float = 1.0, entropy_coef: float = 0.0, use_adaptive_entropy: bool = False, entropy_coef_min: float = 0.0, entropy_coef_max: float = 1.0, entropy_coef_delta: float = 0.005, entropy_target: float = 0.2, max_tool_calling_iterations: int | None = None, vllm_importance_sampling_correction: bool = True, vllm_importance_sampling_mode: str = 'sequence_mask', vllm_importance_sampling_clip_max: float | None = 3.0, vllm_importance_sampling_clip_min: float | None = None, off_policy_mask_threshold: float | None = None, use_bias_correction_kl: bool = True, log_completions: bool = False, log_multimodal: bool = True, num_completions_to_print: int | None = None, log_unique_prompts: bool = False, log_completions_hub_repo: str | None = None, use_transformers_continuous_batching: bool = False, transformers_continuous_batching_config: dict | None = None, use_transformers_paged: bool = False, vllm_importance_sampling_cap: float | None = None, teacher_model_init_kwargs: dict[str, typing.Any] | str | None = None, rkl_advantage: bool = True, single_step_decomposition: bool = True, kd_temperature: float = 1.0, gamma: float = 0.0, length_normalization: bool = True)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/minillm/minillm_config.py#L23)

**Parameters:**

teacher_model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the teacher model from a string.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

rkl_advantage (`bool`, *optional*, defaults to `True`) : Whether to add the reverse KL advantage to the reward advantage.

single_step_decomposition (`bool`, *optional*, defaults to `True`) : Whether to use single-step decomposition for the KL divergence computation.

kd_temperature (`float`, *optional*, defaults to `1.0`) : Temperature for knowledge distillation. Higher temperatures produce softer probability distributions over classes.

gamma (`float`, *optional*, defaults to `0.0`) : Discount factor for future rewards in reinforcement learning.

length_normalization (`bool`, *optional*, defaults to `True`) : Whether to apply length normalization to the rewards.

Configuration class for `MiniLLMTrainer`.

This class includes only the parameters that are specific to MiniLLM training. For a full list of training
arguments, please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments) and [GRPOConfig](/docs/trl/v1.10.0/en/grpo_trainer#trl.GRPOConfig) documentation.
