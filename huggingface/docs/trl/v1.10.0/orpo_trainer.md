# ORPO Trainer

[![model badge](https://img.shields.io/badge/All_models-ORPO-blue)](https://huggingface.co/models?other=orpo,trl) [![model badge](https://img.shields.io/badge/smol_course-Chapter_2-yellow)](https://github.com/huggingface/smol-course/tree/main/2_preference_alignment)

## Overview

Odds Ratio Preference Optimization (ORPO) was introduced in [ORPO: Monolithic Preference Optimization without Reference Model](https://huggingface.co/papers/2403.07691) by [Jiwoo Hong](https://huggingface.co/JW17), [Noah Lee](https://huggingface.co/nlee-208), and [James Thorne](https://huggingface.co/j6mes).

The abstract from the paper is the following:

> While recent preference alignment algorithms for language models have demonstrated promising results, supervised fine-tuning (SFT) remains imperative for achieving successful convergence. In this paper, we study the crucial role of SFT within the context of preference alignment, emphasizing that a minor penalty for the disfavored generation style is sufficient for preference-aligned SFT. Building on this foundation, we introduce a straightforward and innovative reference model-free monolithic odds ratio preference optimization algorithm, ORPO, eliminating the necessity for an additional preference alignment phase. We demonstrate, both empirically and theoretically, that the odds ratio is a sensible choice for contrasting favored and disfavored styles during SFT across the diverse sizes from 125M to 7B. Specifically, fine-tuning Phi-2 (2.7B), Llama-2 (7B), and Mistral (7B) with ORPO on the UltraFeedback alone surpasses the performance of state-of-the-art language models with more than 7B and 13B parameters: achieving up to 12.20% on AlpacaEval_{2.0} (Figure 1), 66.19% on IFEval (instruction-level loose, Table 6), and 7.32 in MT-Bench (Figure 12). We release code and model checkpoints for Mistral-ORPO-alpha (7B) and Mistral-ORPO-beta (7B).

It studies the crucial role of SFT within the context of preference alignment. Using preference data the method posits that a minor penalty for the disfavored generation together with a strong adaption signal to the chosen response via a simple log odds ratio term appended to the NLL loss is sufficient for preference-aligned SFT.

Thus ORPO is a reference model-free preference optimization algorithm eliminating the necessity for an additional preference alignment phase thus saving compute and memory.

The official code can be found in [xfactlab/orpo](https://github.com/xfactlab/orpo).

This post-training method was contributed by [Kashif Rasul](https://huggingface.co/kashif), [Lewis Tunstall](https://huggingface.co/lewtun) and [Alvaro Bartolome](https://huggingface.co/alvarobartt).

## Quick start

This example demonstrates how to train a model using the ORPO method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) as the base model. We use the preference data from the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback). You can view the data in the dataset here:

<iframe
  src="https://huggingface.co/datasets/trl-lib/ultrafeedback_binarized/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_orpo.py
from datasets import load_dataset
from trl.experimental.orpo import ORPOConfig, ORPOTrainer
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
train_dataset = load_dataset("trl-lib/ultrafeedback_binarized", split="train")

training_args = ORPOConfig(output_dir="Qwen2-0.5B-ORPO")
trainer = ORPOTrainer(model=model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_orpo.py
```

Distributed across 8 GPUs, the training takes approximately 30 minutes. You can verify the training progress by checking the reward graph. An increasing trend in the reward margin indicates that the model is improving and generating better responses over time.

![orpo qwen2 reward margin](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/orpo-qwen2-reward-margin.png)

To see how the [trained model](https://huggingface.co/trl-lib/Qwen2-0.5B-ORPO) performs, you can use the [Transformers Chat CLI](https://huggingface.co/docs/transformers/quicktour#chat-with-text-generation-models).

$ transformers chat trl-lib/Qwen2-0.5B-ORPO
&lt;quentin_gallouedec&gt;:
What is the best programming language?

&lt;trl-lib/Qwen2-0.5B-ORPO&gt;:
It's challenging to determine the best programming language as no one language is perfect, as the complexity of a task and the type of project are significant factors. Some popular languages include Java, Python, JavaScript, and
C++. If you have specific needs or requirements for a specific project, it's important to choose the language that best suits those needs.

Here are some other factors to consider when choosing a programming language for a project:

 • Language proficiency: A good programming language is more likely to be easy to understand and use, and will allow developers to collaborate on projects more efficiently.
 • Ease of use: There are tools and libraries available to make programming more accessible, so developers should choose a language that can help them get started easier.
 • Code readability: A clear and concise codebase should be easy to read and understand, especially when working with large projects.
 • Tool and framework support: There are numerous libraries available for Python, Java, and JavaScript, along with tools like IDEs and static code analysis tools.
 • Accessibility: Some languages and tools have features that make them more accessible to developers with disabilities, such as support for screen readers.
 • Version control: As your projects grow and complexity increases, version control tools can be beneficial for tracking changes.

## Expected dataset type

ORPO requires a [preference dataset](dataset_formats#preference). The [experimental.orpo.ORPOTrainer](/docs/trl/v1.10.0/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset format. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

Although the [experimental.orpo.ORPOTrainer](/docs/trl/v1.10.0/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer) supports both explicit and implicit prompts, we recommend using explicit prompts. If provided with an implicit prompt dataset, the trainer will automatically extract the prompt from the `"chosen"` and `"rejected"` columns. For more information, refer to the [preference style](dataset_formats#preference) section.

## Example script

We provide an example script to train a model using the ORPO method. The script is available in [`examples/scripts/orpo.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/orpo.py)

To test the ORPO script with the [Qwen2 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) on the [UltraFeedback dataset](https://huggingface.co/datasets/trl-lib/ultrafeedback_binarized), run the following command:

```bash
accelerate launch examples/scripts/orpo.py \
    --model_name_or_path Qwen/Qwen2-0.5B-Instruct \
    --dataset_name trl-lib/ultrafeedback_binarized \
    --num_train_epochs 1 \
    --output_dir Qwen2-0.5B-ORPO
```

## Usage tips

### For Mixture of Experts Models: Enabling the auxiliary loss

MOEs are the most efficient if the load is about equally distributed between experts.  
To ensure that we train MOEs similarly during preference-tuning, it is beneficial to add the auxiliary loss from the load balancer to the final loss.

This option is enabled by setting `output_router_logits=True` in the model config (e.g. [MixtralConfig](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/mixtral#transformers.MixtralConfig)).  
To scale how much the auxiliary loss contributes to the total loss, use the hyperparameter `router_aux_loss_coef=...` (default: `0.001`) in the model config.

## Logged metrics

While training and evaluating, we record the following metrics:

- `rewards/chosen`: the mean log probabilities of the policy model for the chosen responses scaled by beta
- `rewards/rejected`: the mean log probabilities of the policy model for the rejected responses scaled by beta
- `rewards/accuracies`: mean of how often the chosen rewards are > than the corresponding rejected rewards
- `rewards/margins`: the mean difference between the chosen and corresponding rejected rewards
- `log_odds_chosen`: the mean log odds ratio of the chosen responses over the rejected responses
- `log_odds_ratio`: the mean of the `log(sigmoid(log_odds_chosen))`
- `nll_loss`: the mean negative log likelihood loss from the SFT part of the loss over chosen responses

## ORPOTrainer[[trl.experimental.orpo.ORPOTrainer]]

#### trl.experimental.orpo.ORPOTrainer[[trl.experimental.orpo.ORPOTrainer]]

```python
trl.experimental.orpo.ORPOTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str, NoneType] = None, args: trl.experimental.orpo.orpo_config.ORPOConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, model_init: collections.abc.Callable[[], transformers.modeling_utils.PreTrainedModel] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, peft_config: PeftConfig | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalLoopOutput], dict] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/orpo/orpo_trainer.py#L95)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel)) : The model to train, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForSequenceClassification).

args ([experimental.orpo.ORPOConfig](/docs/trl/v1.10.0/en/orpo_trainer#trl.experimental.orpo.ORPOConfig)) : The ORPO config arguments to use for training.

data_collator (`DataCollator`) : The data collator to use for training. If None is specified, the default data collator (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the maximum length of the sequences in the batch, given a dataset of paired sequences.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for training.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for evaluation.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.15.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. If provided, will be used to automatically process the inputs for the model, and it will be saved along the model to make it easier to rerun an interrupted training or reuse the fine-tuned model.

model_init (`Callable[[], transformers.PreTrainedModel]`) : The model initializer to use for training. If None is specified, the default model initializer will be used.

callbacks (`list[transformers.TrainerCallback]`) : The callbacks to use for training.

optimizers (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) : The optimizer and scheduler to use for training.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) : The function to use to preprocess the logits before computing the metrics.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : The PEFT configuration to use for training. If you pass a PEFT configuration, the model will be wrapped in a PEFT model.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to metric values.

Initialize ORPOTrainer.

#### train[[trl.experimental.orpo.ORPOTrainer.train]]

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

#### save_model[[trl.experimental.orpo.ORPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.orpo.ORPOTrainer.push_to_hub]]

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

## ORPOConfig[[trl.experimental.orpo.ORPOConfig]]

#### trl.experimental.orpo.ORPOConfig[[trl.experimental.orpo.ORPOConfig]]

```python
trl.experimental.orpo.ORPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-06, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, max_length: int | None = 1024, max_completion_length: int | None = None, beta: float = 0.1, disable_dropout: bool = True, padding_value: int | None = None, generate_during_eval: bool = False, is_encoder_decoder: bool | None = None, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, dataset_num_proc: int | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/orpo/orpo_config.py#L22)

**Parameters:**

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the sequences (prompt + completion) in the batch. This argument is required if you want to use the default data collator.

max_completion_length (`int`, *optional*) : Maximum length of the completion. This argument is required if you want to use the default data collator and your model is an encoder-decoder.

beta (`float`, *optional*, defaults to `0.1`) : Parameter controlling the relative ratio loss weight in the ORPO loss. In the [paper](https://huggingface.co/papers/2403.07691), it is denoted by λ. In the [code](https://github.com/xfactlab/orpo), it is denoted by `alpha`.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

padding_value (`int`, *optional*) : Padding value to use. If `None`, the padding value of the tokenizer is used.

generate_during_eval (`bool`, *optional*, defaults to `False`) : If `True`, generates and logs completions from the model to W&B or Comet during evaluation.

is_encoder_decoder (`bool`, *optional*) : When using the `model_init` argument (callable) to instantiate the model instead of the `model` argument, you need to specify if the model returned by the callable is an encoder-decoder model.

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model from a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained).

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

Configuration class for the [experimental.orpo.ORPOTrainer](/docs/trl/v1.10.0/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer).

This class includes only the parameters that are specific to ORPO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.15.0/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `1e-6` instead of `5e-5`.
