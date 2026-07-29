# BCO Trainer

[![model badge](https://img.shields.io/badge/All_models-BCO-blue)](https://huggingface.co/models?other=bco,trl)

TRL supports the Binary Classifier Optimization (BCO).
The [BCO](https://huggingface.co/papers/2404.04656) authors train a binary classifier whose logit serves as a reward so that the classifier maps {prompt, chosen completion} pairs to 1 and {prompt, rejected completion} pairs to 0.
For a full example have a look at [`examples/scripts/bco.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/bco.py).

## Expected dataset type

The [experimental.bco.BCOTrainer](/docs/trl/v1.9.2/en/bco_trainer#trl.experimental.bco.BCOTrainer) requires an [unpaired preference dataset](dataset_formats#unpaired-preference).
The [experimental.bco.BCOTrainer](/docs/trl/v1.9.2/en/bco_trainer#trl.experimental.bco.BCOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Expected model format

The BCO trainer expects a model of `AutoModelForCausalLM`, compared to PPO that expects `AutoModelForCausalLMWithValueHead` for the value function.

## Using the `BCOTrainer`

For a detailed example have a look at the `examples/scripts/bco.py` script. At a high level we need to initialize the `BCOTrainer` with a `model` we wish to train and a reference `ref_model` which we will use to calculate the implicit rewards of the preferred and rejected response.

The `beta` refers to the hyperparameter of the implicit reward, and the dataset contains the 3 entries listed above. Note that the `model` and `ref_model` need to have the same architecture (ie decoder only or encoder-decoder).

```python
from trl.experimental.bco import BCOConfig, BCOTrainer

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

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to train, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification).
- **ref_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Hugging Face transformer model with a casual language modelling head. Used for implicit reward computation
  and loss. If no reference model is provided, the trainer will create a reference model with the same
  architecture as the model to be optimized.
- **args** ([experimental.bco.BCOConfig](/docs/trl/v1.9.2/en/bco_trainer#trl.experimental.bco.BCOConfig)) --
  The arguments to use for training.
- **train_dataset** (`Dataset`) --
  The dataset to use for training.
- **eval_dataset** (`Dataset`) --
  The dataset to use for evaluation.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
  Processing class used to process the data. If provided, will be used to automatically process the inputs
  for the model, and it will be saved along the model to make it easier to rerun an interrupted training or
  reuse the fine-tuned model.
- **data_collator** (`DataCollator`, *optional*) --
  The data collator to use for training. If None is specified, the default data collator
  (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the
  maximum length of the sequences in the batch, given a dataset of paired sequences.
- **model_init** (`Callable[[], transformers.PreTrainedModel]`) --
  The model initializer to use for training. If None is specified, the default model initializer will be
  used.
- **callbacks** (`list[transformers.TrainerCallback]`) --
  The callbacks to use for training.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) --
  The optimizer and scheduler to use for training.
- **preprocess_logits_for_metrics** (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) --
  The function to use to preprocess the logits before computing the metrics.
- **peft_config** (`PeftConfig`, *optional*) --
  The PEFT configuration to use for training. If you pass a PEFT configuration, the model will be wrapped in
  a PEFT model.
- **compute_metrics** (`Callable[[EvalPrediction], dict]`, *optional*) --
  The function to use to compute the metrics. Must take a `EvalPrediction` and return a dictionary string to
  metric values.
- **model_adapter_name** (`str`, defaults to `None`) --
  Name of the train target PEFT adapter, when using LoRA with multiple adapters.
- **ref_adapter_name** (`str`, defaults to `None`) --
  Name of the reference PEFT adapter, when using LoRA with multiple adapters.
- **embedding_func** (`Callable`, *optional*) --
  Function to compute prompt embeddings, used to train the underlying distribution matching (UDM) classifier
  when the desirable and undesirable datasets have divergent prompt distributions. Requires the scikit-learn
  and joblib libraries.
- **embedding_tokenizer** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), *optional*) --
  Tokenizer used to prepare prompts for `embedding_func`.

Initialize BCOTrainer from [BCO](https://huggingface.co/papers/2404.04656) paper.

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

## BCOConfig[[trl.experimental.bco.BCOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "max_length", "val": ": int | None = 1024"}, {"name": "max_completion_length", "val": ": int | None = None"}, {"name": "beta", "val": ": float = 0.1"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "generate_during_eval", "val": ": bool = False"}, {"name": "is_encoder_decoder", "val": ": bool | None = None"}, {"name": "precompute_ref_log_probs", "val": ": bool = False"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "dataset_num_proc", "val": ": int | None = None"}, {"name": "prompt_sample_size", "val": ": int = 1024"}, {"name": "min_density_ratio", "val": ": float = 0.5"}, {"name": "max_density_ratio", "val": ": float = 10.0"}]}>
- **max_length** (`int` or `None`, *optional*, defaults to `1024`) --
  Maximum length of the sequences (prompt + completion) in the batch. This argument is required if you want
  to use the default data collator.
- **max_completion_length** (`int`, *optional*) --
  Maximum length of the completion. This argument is required if you want to use the default data collator
  and your model is an encoder-decoder.
- **beta** (`float`, *optional*, defaults to `0.1`) --
  Parameter controlling the deviation from the reference model. Higher β means less deviation from the
  reference model.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model and reference model.
- **generate_during_eval** (`bool`, *optional*, defaults to `False`) --
  If `True`, generates and logs completions from both the model and the reference model to W&B or Comet
  during evaluation.
- **is_encoder_decoder** (`bool`, *optional*) --
  When using the `model_init` argument (callable) to instantiate the model instead of the `model` argument,
  you need to specify if the model returned by the callable is an encoder-decoder model.
- **precompute_ref_log_probs** (`bool`, *optional*, defaults to `False`) --
  Whether to precompute reference model log probabilities for training and evaluation datasets. This is
  useful when training without the reference model to reduce the total GPU memory needed.
- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model and
  reference model from strings.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) for both the model and reference model.
- **dataset_num_proc** (`int`, *optional*) --
  Number of processes to use for processing the dataset.
- **prompt_sample_size** (`int`, *optional*, defaults to `1024`) --
  Number of prompts that are fed to density ratio classifier.
- **min_density_ratio** (`float`, *optional*, defaults to `0.5`) --
  Minimum value of the density ratio. The estimated density ratio is clamped to this value.
- **max_density_ratio** (`float`, *optional*, defaults to `10.0`) --
  Maximum value of the density ratio. The estimated density ratio is clamped to this value.

Configuration class for the [experimental.bco.BCOTrainer](/docs/trl/v1.9.2/en/bco_trainer#trl.experimental.bco.BCOTrainer).

This class includes only the parameters that are specific to BCO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.14.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `5e-7` instead of `5e-5`.
