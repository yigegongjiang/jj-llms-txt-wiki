# PRM Trainer

[![model badge](https://img.shields.io/badge/All_models-PRM-blue)](https://huggingface.co/models?other=prm,trl)

> [!WARNING]
> PRM Trainer is an experimental API which is subject to change at any time.

## Overview

Process-supervised Reward Models (PRM) were proposed in [Solving math word problems with process- and outcome-based feedback](https://huggingface.co/papers/2211.14275) by Jonathan Uesato, Nate Kushman, Ramana Kumar, Francis Song, Noah Siegel, Lisa Wang, Antonia Creswell, Geoffrey Irving, and Irina Higgins.

The abstract from the paper is the following:

> Recent work has shown that asking language models to generate reasoning steps improves performance on many reasoning tasks. When moving beyond prompting, this raises the question of how we should supervise such models: outcome-based approaches which supervise the final result, or process-based approaches which supervise the reasoning process itself? Differences between these approaches might naturally be expected not just in final-answer errors but also in reasoning errors, which can be difficult to detect and are problematic in many real-world domains such as education. We run the first comprehensive comparison between process- and outcome-based approaches trained on a natural language task, GSM8K. We find that pure outcome-based supervision produces similar final-answer error rates with less label supervision. However, for correct reasoning steps we find it necessary to use processbased supervision or supervision from learned reward models that emulate process-based feedback. In total, we improve the previous best results from 16.8% → 12.7% final-answer error and 14.0% → 3.4% reasoning error among final-answer-correct solutions.

This post-training method was contributed by [Gaetan Lopez](https://github.com/gaetanlop), [Lewis Tunstall](https://huggingface.co/lewtun), [Quentin Gallouédec](https://huggingface.co/qgallouedec) and [Agustín Piqueres](https://huggingface.co/plaguss).

## Quick start

This example demonstrates how to train a model using the PRM method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B) as the base model. We use the stepwise supervision data from the [Math Shepherd dataset](https://huggingface.co/datasets/trl-lib/math_shepherd). You can view the data in the dataset here:

<iframe
  src="https://huggingface.co/datasets/trl-lib/math_shepherd/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_prm.py
from datasets import load_dataset
from trl.experimental.prm import PRMConfig, PRMTrainer
from transformers import AutoModelForTokenClassification, AutoTokenizer

model = AutoModelForTokenClassification.from_pretrained("Qwen/Qwen2-0.5B", num_labels=2)
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B")
train_dataset = load_dataset("trl-lib/math_shepherd", split="train[:10%]")

training_args = PRMConfig(output_dir="Qwen2-0.5B-Reward-Math-Sheperd")
trainer = PRMTrainer(model=model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_prm.py
```

Distributed across 8 GPUs, the training takes approximately 1 hour.

To see how the [trained model](https://huggingface.co/trl-lib/Qwen2-0.5B-Reward-Math-Sheperd) performs, you can use the following script.

```python
from datasets import load_dataset
from transformers import pipeline

pipe = pipeline("token-classification", model="trl-lib/Qwen2-0.5B-Reward-Math-Sheperd")
dataset = load_dataset("trl-lib/math_shepherd")
example = {
    "prompt": "Musa is the class teacher of a class of 45 students. He wants to split them into three groups by age. If a third of the class is under 11 years, and two-fifths are above 11 but under 13, how many students will be in the third group (13 years and above)?",
    "completions": [
        "Step 1: A third of the class is under 11 years because 11 - 1/3 = <<11-1/3=7>>7.",
        "Step 2: Two-fifths of the class are above 11 but under 13 because 2/5 * 11 = <<2/5*11=8>>8.",
        "Step 3: There are 45 students, so the third group will have 45 - 7 - 8 = <<45-7-8=20>>20 students. The answer is: 20",
    ],
    "labels": [True, False, False],
}

separator = "\n"  # It's important to use the same separator as the one used during training

for idx in range(1, len(example["completions"]) + 1):
    steps = example["completions"][0:idx]
    text = separator.join((example["prompt"], *steps)) + separator  # Add a separator between the prompt and each steps
    pred_entity = pipe(text)[-1]["entity"]
    pred = {"LABEL_0": False, "LABEL_1": True}[pred_entity]
    label = example["labels"][idx - 1]
    print(f"Step {idx}\tPredicted: {pred} \tLabel: {label}")
```

```text
Step 1  Predicted: True         Label: True
Step 2  Predicted: False        Label: False
Step 3  Predicted: False        Label: False
```

It's a win!

## Expected dataset type

PRM requires a [stepwise supervision](dataset_formats#stepwise-supervision).
The dataset should contain the following columns: `prompt`, `completions` and `labels`, where `completions` contains a list of reasoning steps and `labels` a list of booleans or floats indicating the correctness of each step.

The [experimental.prm.PRMTrainer](/docs/trl/v1.9.2/en/prm_trainer#trl.experimental.prm.PRMTrainer) only supports [standard](dataset_formats#standard) dataset format.

## Example script

We provide an example script to train a model using the PRM method. The script is available in [`examples/scripts/prm.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/prm.py)

To use the PRM script with the [Qwen2 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B) on the [Math Shepherd dataset](https://huggingface.co/datasets/trl-lib/math_shepherd), run the following command:

```bash
accelerate launch examples/scripts/prm.py \
    --model_name_or_path Qwen/Qwen2-0.5B \
    --dataset_name trl-lib/math_shepherd \
    --num_train_epochs 1 \
    --output_dir Qwen2-0.5B-Reward-Math-Sheperd
```

## PRMTrainer[[trl.experimental.prm.PRMTrainer]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to train, preferably an `AutoModelForTokenClassification`.
- **args** ([experimental.prm.PRMConfig](/docs/trl/v1.9.2/en/prm_trainer#trl.experimental.prm.PRMConfig)) --
  The arguments to use for training.
- **data_collator** (`DataCollator`) --
  The data collator to use for training. If None is specified, the default data collator
  ([DataCollatorForTokenClassification](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/data_collator#transformers.DataCollatorForTokenClassification)) will be used which will pad the sequences to the
  maximum length of the sequences in the batch, given a dataset of paired sequences.
- **train_dataset** (`Dataset`) --
  The dataset to use for training.
- **eval_dataset** (`Dataset`) --
  The dataset to use for evaluation.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
  Processing class used to process the data. If provided, will be used to automatically process the inputs
  for the model, and it will be saved along the model to make it easier to rerun an interrupted training or
  reuse the fine-tuned model.
- **model_init** (`Callable[[], transformers.PreTrainedModel]`) --
  The model initializer to use for training. If None is specified, the default model initializer will be
  used.
- **compute_metrics** (`Callable[[transformers.EvalPrediction], dict]`, *optional* defaults to `compute_accuracy`) --
  The metrics to use for evaluation. If no metrics are specified, the default metric (`compute_accuracy`)
  will be used.
- **callbacks** (`list[transformers.TrainerCallback]`) --
  The callbacks to use for training.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`) --
  The optimizer and scheduler to use for training.
- **preprocess_logits_for_metrics** (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`) --
  The function to use to preprocess the logits before computing the metrics.
- **peft_config** (`PeftConfig`, *optional*) --
  The PEFT configuration to use for training. If you pass a PEFT configuration, the model will be wrapped in
  a PEFT model.

Initialize PRMTrainer.

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

## PRMConfig[[trl.experimental.prm.PRMConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "max_length", "val": ": int | None = 1024"}, {"name": "max_completion_length", "val": ": int | None = None"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "step_separator", "val": ": str = '\\n'"}, {"name": "train_on_last_step_only", "val": ": bool = False"}, {"name": "dataset_num_proc", "val": ": int | None = None"}]}>
- **max_length** (`int` or `None`, *optional*, defaults to `1024`) --
  Maximum length of the sequences (prompt + completion) used for truncation.
- **max_completion_length** (`int`, *optional*) --
  Maximum length of the completion used for truncation. The completion is the concatenation of the steps.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model.
- **step_separator** (`str`, *optional*, defaults to `"\n"`) --
  Separator used to separate each step of the reasoning process.
- **train_on_last_step_only** (`bool`, *optional*, defaults to `False`) --
  Whether to train only on the last step.
- **dataset_num_proc** (`int`, *optional*) --
  Number of processes to use for processing the dataset.

Configuration class for the [experimental.prm.PRMTrainer](/docs/trl/v1.9.2/en/prm_trainer#trl.experimental.prm.PRMTrainer).

This class includes only the parameters that are specific to PRM training. For a full list of training arguments,
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
> - `learning_rate`: Defaults to `1e-5` instead of `5e-5`.
