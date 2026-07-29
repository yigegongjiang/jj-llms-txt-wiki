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

ORPO requires a [preference dataset](dataset_formats#preference). The [experimental.orpo.ORPOTrainer](/docs/trl/v1.9.2/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset format. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

Although the [experimental.orpo.ORPOTrainer](/docs/trl/v1.9.2/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer) supports both explicit and implicit prompts, we recommend using explicit prompts. If provided with an implicit prompt dataset, the trainer will automatically extract the prompt from the `"chosen"` and `"rejected"` columns. For more information, refer to the [preference style](dataset_formats#preference) section.

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

This option is enabled by setting `output_router_logits=True` in the model config (e.g. [MixtralConfig](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/mixtral#transformers.MixtralConfig)).  
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

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to train, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForSequenceClassification).
- **args** ([experimental.orpo.ORPOConfig](/docs/trl/v1.9.2/en/orpo_trainer#trl.experimental.orpo.ORPOConfig)) --
  The ORPO config arguments to use for training.
- **data_collator** (`DataCollator`) --
  The data collator to use for training. If None is specified, the default data collator
  (`experimental.utils.DPODataCollatorWithPadding`) will be used which will pad the sequences to the
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

Initialize ORPOTrainer.

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

## ORPOConfig[[trl.experimental.orpo.ORPOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "max_length", "val": ": int | None = 1024"}, {"name": "max_completion_length", "val": ": int | None = None"}, {"name": "beta", "val": ": float = 0.1"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "padding_value", "val": ": int | None = None"}, {"name": "generate_during_eval", "val": ": bool = False"}, {"name": "is_encoder_decoder", "val": ": bool | None = None"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "dataset_num_proc", "val": ": int | None = None"}]}>
- **max_length** (`int` or `None`, *optional*, defaults to `1024`) --
  Maximum length of the sequences (prompt + completion) in the batch. This argument is required if you want
  to use the default data collator.
- **max_completion_length** (`int`, *optional*) --
  Maximum length of the completion. This argument is required if you want to use the default data collator
  and your model is an encoder-decoder.
- **beta** (`float`, *optional*, defaults to `0.1`) --
  Parameter controlling the relative ratio loss weight in the ORPO loss. In the
  [paper](https://huggingface.co/papers/2403.07691), it is denoted by λ. In the
  [code](https://github.com/xfactlab/orpo), it is denoted by `alpha`.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model.
- **padding_value** (`int`, *optional*) --
  Padding value to use. If `None`, the padding value of the tokenizer is used.
- **generate_during_eval** (`bool`, *optional*, defaults to `False`) --
  If `True`, generates and logs completions from the model to W&B or Comet during evaluation.
- **is_encoder_decoder** (`bool`, *optional*) --
  When using the `model_init` argument (callable) to instantiate the model instead of the `model` argument,
  you need to specify if the model returned by the callable is an encoder-decoder model.
- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model from a
  string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained).
- **dataset_num_proc** (`int`, *optional*) --
  Number of processes to use for processing the dataset.

Configuration class for the [experimental.orpo.ORPOTrainer](/docs/trl/v1.9.2/en/orpo_trainer#trl.experimental.orpo.ORPOTrainer).

This class includes only the parameters that are specific to ORPO training. For a full list of training arguments,
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
> - `learning_rate`: Defaults to `1e-6` instead of `5e-5`.
