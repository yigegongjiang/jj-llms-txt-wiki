# BCO Trainer

[![model badge](https://img.shields.io/badge/All_models-BCO-blue)](https://huggingface.co/models?other=bco,trl)

TRL supports the Binary Classifier Optimization (BCO).
The [BCO](https://huggingface.co/papers/2404.04656) authors train a binary classifier whose logit serves as a reward so that the classifier maps {prompt, chosen completion} pairs to 1 and {prompt, rejected completion} pairs to 0.

## Expected dataset type

The [experimental.bco.BCOTrainer](/docs/trl/v1.12.0/en/bco_trainer#trl.experimental.bco.BCOTrainer) requires an [unpaired preference dataset](dataset_formats#unpaired-preference).
The [experimental.bco.BCOTrainer](/docs/trl/v1.12.0/en/bco_trainer#trl.experimental.bco.BCOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Expected model format

The BCO trainer expects a model of `AutoModelForCausalLM`, compared to PPO that expects `AutoModelForCausalLMWithValueHead` for the value function.

## Using the `BCOTrainer`

At a high level we need to initialize the `BCOTrainer` with a `model` we wish to train and a reference `ref_model` which we will use to calculate the implicit rewards of the preferred and rejected response.

The `beta` refers to the hyperparameter of the implicit reward, and the dataset contains the 3 entries listed above. Note that the `model` and `ref_model` need to have the same architecture (ie decoder only or encoder-decoder).

```python
from datasets import load_dataset
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl.experimental.bco import BCOConfig, BCOTrainer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2.5-0.5B-Instruct")
model_ref = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2.5-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2.5-0.5B-Instruct")
train_dataset = load_dataset("trl-lib/ultrafeedback-gpt-3.5-turbo-helpfulness", split="train")

training_args = BCOConfig(
    beta=0.1,
)

bco_trainer = BCOTrainer(
    model,
    model_ref,
    args=training_args,
    train_dataset=train_dataset,
    processing_class=tokenizer,
)
```

After this one can then call:

```python
bco_trainer.train()
```

## Underlying Distribution matching (UDM)

In practical scenarios, the thumbs-up and thumbs-down datasets are likely to have divergent underlying distributions of prompts.
Consider an LLM deployed for user feedback: if the model excels in writing tasks but underperforms in coding, the thumbs-up dataset will be dominated by writing-related prompts, while the thumbs-down dataset will contain mostly coding-related prompts.  
If the prompts in your desired and undesired datasets differ a lot, it is useful to enable UDM.  

Choose an embedding model and tokenizer:

```python
embedding_model = AutoModel.from_pretrained(your_model_id)
embedding_tokenizer = AutoTokenizer.from_pretrained(your_model_id)

# customize this function depending on your embedding model
def embed_prompt(input_ids, attention_mask, model):
    outputs = model(input_ids=input_ids, attention_mask=attention_mask)
    return outputs.last_hidden_state.mean(dim=1)

embedding_model = Accelerator().prepare_model(self.embedding_model)
embedding_func = partial(embed_prompt, model=embedding_model)
```

Set `prompt_sample_size` to define how many prompts are selected to train the UDM classifier and start the training with the provided embedding function:

```python
training_args = BCOConfig(
    beta=0.1,
    prompt_sample_size=512,
)

bco_trainer = BCOTrainer(
    model,
    model_ref,
    args=training_args,
    train_dataset=train_dataset,
    processing_class=tokenizer,
    embedding_func=embedding_func,
    embedding_tokenizer=self.embedding_tokenizer,
)

bco_trainer.train()
```

### For Mixture of Experts Models: Enabling the auxiliary loss

MOEs are the most efficient if the load is about equally distributed between experts.  
To ensure that we train MOEs similarly during preference-tuning, it is beneficial to add the auxiliary loss from the load balancer to the final loss.  

This option is enabled by setting `output_router_logits=True` in the model config (e.g. MixtralConfig).  
To scale how much the auxiliary loss contributes to the total loss, use the hyperparameter `router_aux_loss_coef=...` (default: 0.001).

## BCOTrainer[[trl.experimental.bco.BCOTrainer]]

#### trl.experimental.bco.BCOTrainer[[trl.experimental.bco.BCOTrainer]]

```python
trl.experimental.bco.BCOTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str] = None, ref_model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str, NoneType] = None, args: BCOConfig = None, train_dataset: datasets.arrow_dataset.Dataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, model_init: collections.abc.Callable[[], transformers.modeling_utils.PreTrainedModel] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, peft_config: PeftConfig | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalLoopOutput], dict] | None = None, model_adapter_name: str | None = None, ref_adapter_name: str | None = None, embedding_func: collections.abc.Callable | None = None, embedding_tokenizer: transformers.tokenization_utils_base.PreTrainedTokenizerBase | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/bco/bco_trainer.py#L358)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to train, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification).

ref_model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel)) : Hugging Face transformer model with a casual language modelling head. Used for implicit reward computation and loss. If no reference model is provided, the trainer will create a reference model with the same architecture as the model to be optimized.

args ([experimental.bco.BCOConfig](/docs/trl/v1.12.0/en/bco_trainer#trl.experimental.bco.BCOConfig)) : The arguments to use for training.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for training.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset)) : The dataset to use for evaluation.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. If provided, will be used to automatically process the inputs for the model, and it will be saved along the model to make it easier to rerun an interrupted training or reuse the fine-tuned model.

data_collator (`DataCollator`, *optional*) : The data collator to use for training. If None is specified, the default data collator (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the maximum length of the sequences in the batch, given a dataset of paired sequences.

model_init (`Callable[[], transformers.PreTrainedModel]`) : The model initializer to use for training. If None is specified, the default model initializer will be used.

callbacks (`list[transformers.TrainerCallback]`) : The callbacks to use for training.

optimizers (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) : The optimizer and scheduler to use for training.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) : The function to use to preprocess the logits before computing the metrics.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : The PEFT configuration to use for training. If you pass a PEFT configuration, the model will be wrapped in a PEFT model.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to metric values.

model_adapter_name (`str`, defaults to `None`) : Name of the train target PEFT adapter, when using LoRA with multiple adapters.

ref_adapter_name (`str`, defaults to `None`) : Name of the reference PEFT adapter, when using LoRA with multiple adapters.

embedding_func (`Callable`, *optional*) : Function to compute prompt embeddings, used to train the underlying distribution matching (UDM) classifier when the desirable and undesirable datasets have divergent prompt distributions. Requires the scikit-learn and joblib libraries.

embedding_tokenizer ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) : Tokenizer used to prepare prompts for `embedding_func`.

Initialize BCOTrainer from [BCO](https://huggingface.co/papers/2404.04656) paper.

#### train[[trl.experimental.bco.BCOTrainer.train]]

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

#### save_model[[trl.experimental.bco.BCOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.bco.BCOTrainer.push_to_hub]]

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

## BCOConfig[[trl.experimental.bco.BCOConfig]]

#### trl.experimental.bco.BCOConfig[[trl.experimental.bco.BCOConfig]]

```python
trl.experimental.bco.BCOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 5e-07, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, max_length: int | None = 1024, max_completion_length: int | None = None, beta: float = 0.1, disable_dropout: bool = True, generate_during_eval: bool = False, is_encoder_decoder: bool | None = None, precompute_ref_log_probs: bool = False, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, dataset_num_proc: int | None = None, prompt_sample_size: int = 1024, min_density_ratio: float = 0.5, max_density_ratio: float = 10.0)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/bco/bco_config.py#L22)

**Parameters:**

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the sequences (prompt + completion) in the batch. This argument is required if you want to use the default data collator.

max_completion_length (`int`, *optional*) : Maximum length of the completion. This argument is required if you want to use the default data collator and your model is an encoder-decoder.

beta (`float`, *optional*, defaults to `0.1`) : Parameter controlling the deviation from the reference model. Higher β means less deviation from the reference model.

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model and reference model.

generate_during_eval (`bool`, *optional*, defaults to `False`) : If `True`, generates and logs completions from both the model and the reference model to W&B or Comet during evaluation.

is_encoder_decoder (`bool`, *optional*) : When using the `model_init` argument (callable) to instantiate the model instead of the `model` argument, you need to specify if the model returned by the callable is an encoder-decoder model.

precompute_ref_log_probs (`bool`, *optional*, defaults to `False`) : Whether to precompute reference model log probabilities for training and evaluation datasets. This is useful when training without the reference model to reduce the total GPU memory needed.

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model and reference model from strings.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) for both the model and reference model.

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

prompt_sample_size (`int`, *optional*, defaults to `1024`) : Number of prompts that are fed to density ratio classifier.

min_density_ratio (`float`, *optional*, defaults to `0.5`) : Minimum value of the density ratio. The estimated density ratio is clamped to this value.

max_density_ratio (`float`, *optional*, defaults to `10.0`) : Maximum value of the density ratio. The estimated density ratio is clamped to this value.

Configuration class for the [experimental.bco.BCOTrainer](/docs/trl/v1.12.0/en/bco_trainer#trl.experimental.bco.BCOTrainer).

This class includes only the parameters that are specific to BCO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `5e-7` instead of `5e-5`.
