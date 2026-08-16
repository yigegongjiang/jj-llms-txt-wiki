# CPO Trainer

[![model badge](https://img.shields.io/badge/All_models-CPO-blue)](https://huggingface.co/models?other=cpo,trl)

## Overview

Contrastive Preference Optimization (CPO) as introduced in the paper [Contrastive Preference Optimization: Pushing the Boundaries of LLM Performance in Machine Translation](https://huggingface.co/papers/2401.08417) by [Haoran Xu](https://huggingface.co/haoranxu), [Amr Sharaf](https://huggingface.co/amrsharaf), [Yunmo Chen](https://huggingface.co/yunmochen), Weiting Tan, Lingfeng Shen, Benjamin Van Durme, [Kenton Murray](https://huggingface.co/Kenton), and [Young Jin Kim](https://huggingface.co/ykim362). At a high level, CPO trains models to avoid generating adequate, but not perfect, translations in Machine Translation (MT) tasks. However, CPO is a general approximation of the DPO loss and can be applied to other domains, such as chat.

CPO aims to mitigate two fundamental shortcomings of SFT. First, SFT’s methodology of minimizing the discrepancy between predicted outputs and gold-standard references inherently caps model performance at the quality level of the training data. Secondly, SFT lacks a mechanism to prevent the model from rejecting mistakes in translations. The CPO objective is derived from the DPO objective.

## Quick start

This example demonstrates how to train a model using the CPO method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) as the base model. We use the preference data from the [UltraFeedback dataset](https://huggingface.co/datasets/openbmb/UltraFeedback). You can view the data in the dataset here:

<iframe
  src="https://huggingface.co/datasets/trl-lib/ultrafeedback_binarized/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_cpo.py
from datasets import load_dataset
from trl.experimental.cpo import CPOConfig, CPOTrainer
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
train_dataset = load_dataset("trl-lib/ultrafeedback_binarized", split="train")

training_args = CPOConfig(output_dir="Qwen2-0.5B-CPO")
trainer = CPOTrainer(model=model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_cpo.py
```

## Expected dataset type

CPO requires a [preference dataset](dataset_formats#preference). The [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer) supports both [conversational](dataset_formats#conversational) and [standard](dataset_formats#standard) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

## Example script

We provide an example script to train a model using the CPO method. The script is available in [`examples/scripts/cpo.py`](https://github.com/huggingface/trl/blob/main/examples/scripts/cpo.py)

To test the CPO script with the [Qwen2 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) on the [UltraFeedback dataset](https://huggingface.co/datasets/trl-lib/ultrafeedback_binarized), run the following command:

```bash
accelerate launch examples/scripts/cpo.py \
    --model_name_or_path Qwen/Qwen2-0.5B-Instruct \
    --dataset_name trl-lib/ultrafeedback_binarized \
    --num_train_epochs 1 \
    --output_dir Qwen2-0.5B-CPO
```

## Logged metrics

While training and evaluating, we record the following metrics:

* `rewards/chosen`: the mean log probabilities of the policy model for the chosen responses scaled by beta
* `rewards/rejected`: the mean log probabilities of the policy model for the rejected responses scaled by beta
* `rewards/accuracies`: mean of how often the chosen rewards are > than the corresponding rejected rewards
* `rewards/margins`: the mean difference between the chosen and corresponding rejected rewards
* `nll_loss`: the mean negative log likelihood loss of the policy model for the chosen responses

## CPO variants

### Simple Preference Optimization (SimPO)

[Simple Preference Optimization](https://huggingface.co/papers/2405.14734) (SimPO) by [Yu Meng](https://huggingface.co/yumeng5), [Mengzhou Xia](https://huggingface.co/mengzhouxia), and [Danqi Chen](https://huggingface.co/cdq10131) proposes a simpler and more effective preference optimization algorithm than DPO without using a reference model. The key designs in SimPO are (1) using length-normalized log likelihood as the implicit reward, and (2) incorporating a target reward margin in the Bradley-Terry ranking objective. The official code can be found at [princeton-nlp/SimPO](https://github.com/princeton-nlp/SimPO).

The abstract from the paper is the following:

> Direct Preference Optimization (DPO) is a widely used offline preference optimization algorithm that reparameterizes reward functions in reinforcement learning from human feedback (RLHF) to enhance simplicity and training stability. In this work, we propose SimPO, a simpler yet more effective approach. The effectiveness of SimPO is attributed to a key design: using the average log probability of a sequence as the implicit reward. This reward formulation better aligns with model generation and eliminates the need for a reference model, making it more compute and memory efficient. Additionally, we introduce a target reward margin to the Bradley-Terry objective to encourage a larger margin between the winning and losing responses, further enhancing the algorithm's performance. We compare SimPO to DPO and its latest variants across various state-of-the-art training setups, including both base and instruction-tuned models like Mistral and Llama3. We evaluated on extensive instruction-following benchmarks, including AlpacaEval 2, MT-Bench, and the recent challenging Arena-Hard benchmark. Our results demonstrate that SimPO consistently and significantly outperforms existing approaches without substantially increasing response length. Specifically, SimPO outperforms DPO by up to 6.4 points on AlpacaEval 2 and by up to 7.5 points on Arena-Hard. Our top-performing model, built on Llama3-8B-Instruct, achieves a remarkable 44.7 length-controlled win rate on AlpacaEval 2 -- surpassing Claude 3 Opus on the leaderboard, and a 33.8 win rate on Arena-Hard -- making it the strongest 8B open-source model.

The SimPO loss is integrated in the [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer), as it's an alternative loss that adds a reward margin, allows for length normalization, and does not use BC regularization. To use this loss, just turn on `loss_type="simpo"` and `cpo_alpha=0.0` in the [experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig) and set the `simpo_gamma` to a recommended value.

### CPO-SimPO

We also offer the combined use of CPO and SimPO, which enables more stable training and improved performance. Learn more details at [CPO-SimPO GitHub](https://github.com/fe1ixxu/CPO_SIMPO). To use this method, simply enable SimPO by setting `loss_type="simpo"` and a non-zero `cpo_alpha` in the [experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig).

### AlphaPO

The [AlphaPO -- Reward shape matters for LLM alignment](https://huggingface.co/papers/2501.03884) (AlphaPO) method by Aman Gupta, Shao Tang, Qingquan Song, Sirou Zhu, [Jiwoo Hong](https://huggingface.co/JW17), Ankan Saha, Viral Gupta, Noah Lee, Eunki Kim, Jason Zhu, Natesh Pillai, and S. Sathiya Keerthi is also implemented in the [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer). AlphaPO is an alternative method that applies a transformation to the reward function shape in the context of SimPO loss. The abstract from the paper is the following:

> Reinforcement Learning with Human Feedback (RLHF) and its variants have made huge strides toward the effective alignment of large language models (LLMs) to follow instructions and reflect human values. More recently, Direct Alignment Algorithms (DAAs) have emerged in which the reward modeling stage of RLHF is skipped by characterizing the reward directly as a function of the policy being learned. Some popular examples of DAAs include Direct Preference Optimization (DPO) and Simple Preference Optimization (SimPO). These methods often suffer from likelihood displacement, a phenomenon by which the probabilities of preferred responses are often reduced undesirably. In this paper, we argue that, for DAAs the reward (function) shape matters. We introduce AlphaPO, a new DAA method that leverages an α-parameter to help change the shape of the reward function beyond the standard log reward. AlphaPO helps maintain fine-grained control over likelihood displacement and overoptimization. Compared to SimPO, one of the best performing DAAs, AlphaPO leads to about 7% to 10% relative improvement in alignment performance for the instruct versions of Mistral-7B and Llama3-8B while achieving 15% to 50% relative improvement over DPO on the same models. The analysis and results presented highlight the importance of the reward shape and how one can systematically change it to affect training dynamics, as well as improve alignment performance.

To use this loss as described in the paper, we can set the `loss_type="alphapo"` which automatically sets `loss_type="simpo"` and `cpo_alpha=0.0`, together with `alpha` and `simpo_gamma` to recommended values in the [experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig). Alternatively, you can manually set `loss_type="simpo"`, `cpo_alpha=0.0`, together with `alpha` and `simpo_gamma` to recommended values. Other variants of this method are also possible, such as setting `loss_type="ipo"` and `alpha` to any non-zero value.

## Loss functions

The CPO algorithm supports several loss functions. The loss function can be set using the `loss_type` parameter in the [experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig). The following loss functions are supported:

| `loss_type=` | Description |
| --- | --- |
| `"sigmoid"` (default) | Given the preference data, we can fit a binary classifier according to the Bradley-Terry model, and in fact, the [DPO](https://huggingface.co/papers/2305.18290) authors propose the sigmoid loss on the normalized likelihood via the `logsigmoid` to fit a logistic regression. |
| `"hinge"` | The [RSO](https://huggingface.co/papers/2309.06657) authors propose to use a hinge loss on the normalized likelihood from the [SLiC](https://huggingface.co/papers/2305.10425) paper. In this case, the `beta` is the reciprocal of the margin. |
| `"ipo"` | The [IPO](https://huggingface.co/papers/2310.12036) authors provide a deeper theoretical understanding of the DPO algorithms and identify an issue with overfitting and propose an alternative loss. In this case, the `beta` is the reciprocal of the gap between the log-likelihood ratios of the chosen vs the rejected completion pair, and thus the smaller the `beta`, the larger this gap is. As per the paper, the loss is averaged over log-likelihoods of the completion (unlike DPO, which is summed only). |
| `"simpo"` | The [SimPO](https://huggingface.co/papers/2405.14734) method is also implemented in the [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer). SimPO is an alternative loss that adds a reward margin, allows for length normalization, and does not use BC regularization. To use this loss, simply set `loss_type="simpo"` and `cpo_alpha=0.0` in the [experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig) and `simpo_gamma` to a recommended value. |
| `"alphapo"` | The [AlphaPO](https://huggingface.co/papers/2501.03884) method is also implemented in the [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer). This is syntactic sugar that automatically sets `loss_type="simpo"` and `cpo_alpha=0.0`. AlphaPO applies a transformation to the reward function shape in the context of SimPO loss when the `alpha` parameter is non-zero. |

### For Mixture of Experts Models: Enabling the auxiliary loss

MOEs are the most efficient if the load is about equally distributed between experts.  
To ensure that we train MOEs similarly during preference-tuning, it is beneficial to add the auxiliary loss from the load balancer to the final loss.

This option is enabled by setting `output_router_logits=True` in the model config (e.g., [MixtralConfig](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/mixtral#transformers.MixtralConfig)).  
To scale how much the auxiliary loss contributes to the total loss, use the hyperparameter `router_aux_loss_coef=...` (default: `0.001`) in the model config.

## CPOTrainer[[trl.experimental.cpo.CPOTrainer]]

#### trl.experimental.cpo.CPOTrainer[[trl.experimental.cpo.CPOTrainer]]

```python
trl.experimental.cpo.CPOTrainer(model: typing.Union[transformers.modeling_utils.PreTrainedModel, torch.nn.Module, str, NoneType] = None, args: trl.experimental.cpo.cpo_config.CPOConfig | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | dict[str, datasets.arrow_dataset.Dataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.image_processing_utils.BaseImageProcessor | transformers.feature_extraction_utils.FeatureExtractionMixin | transformers.processing_utils.ProcessorMixin | None = None, model_init: collections.abc.Callable[[], transformers.modeling_utils.PreTrainedModel] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, peft_config: PeftConfig | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalLoopOutput], dict] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/cpo/cpo_trainer.py#L83)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/model#transformers.PreTrainedModel)) : The model to train, preferably an [AutoModelForSequenceClassification](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForSequenceClassification).

args ([experimental.cpo.CPOConfig](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOConfig)) : The CPO config arguments to use for training.

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

Initialize CPOTrainer.

#### train[[trl.experimental.cpo.CPOTrainer.train]]

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

#### save_model[[trl.experimental.cpo.CPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.experimental.cpo.CPOTrainer.push_to_hub]]

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

## CPOConfig[[trl.experimental.cpo.CPOConfig]]

#### trl.experimental.cpo.CPOConfig[[trl.experimental.cpo.CPOConfig]]

```python
trl.experimental.cpo.CPOConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 1e-06, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, max_length: int | None = 1024, max_completion_length: int | None = None, beta: float = 0.1, label_smoothing: float = 0.0, loss_type: str = 'sigmoid', disable_dropout: bool = True, cpo_alpha: float = 1.0, simpo_gamma: float = 0.5, alpha: float = 0.0, generate_during_eval: bool = False, is_encoder_decoder: bool | None = None, model_init_kwargs: dict[str, typing.Any] | str | None = None, trust_remote_code: bool = False, dataset_num_proc: int | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/cpo/cpo_config.py#L22)

**Parameters:**

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the sequences (prompt + completion) in the batch. This argument is required if you want to use the default data collator.

max_completion_length (`int`, *optional*) : Maximum length of the completion. This argument is required if you want to use the default data collator and your model is an encoder-decoder.

beta (`float`, *optional*, defaults to `0.1`) : Parameter controlling the deviation from the reference model. Higher β means less deviation from the reference model. For the IPO loss (`loss_type="ipo"`), β is the regularization parameter denoted by τ in the [paper](https://huggingface.co/papers/2310.12036).

label_smoothing (`float`, *optional*, defaults to `0.0`) : Label smoothing factor. This argument is required if you want to use the default data collator.

loss_type (`str`, *optional*, defaults to `"sigmoid"`) : Type of loss to use. Possible values are:  - `"sigmoid"`: sigmoid loss from the original [DPO](https://huggingface.co/papers/2305.18290) paper. - `"hinge"`: hinge loss on the normalized likelihood from the [SLiC](https://huggingface.co/papers/2305.10425) paper. - `"ipo"`: IPO loss from the [IPO](https://huggingface.co/papers/2310.12036) paper. - `"simpo"`: SimPO loss from the [SimPO](https://huggingface.co/papers/2405.14734) paper. - `"alphapo"`: AlphaPO loss from the [AlphaPO](https://huggingface.co/papers/2501.03884) paper. This automatically sets `loss_type="simpo"` and `cpo_alpha=0.0`. 

disable_dropout (`bool`, *optional*, defaults to `True`) : Whether to disable dropout in the model.

cpo_alpha (`float`, *optional*, defaults to `1.0`) : Weight of the BC regularizer in CPO training.

simpo_gamma (`float`, *optional*, defaults to `0.5`) : Target reward margin for the SimPO loss, used only when the `loss_type="simpo"`.

alpha (`float`, *optional*, defaults to `0.0`) : Alpha parameter that controls reward function shape across all loss types. When alpha=0 (default), uses standard log probability rewards. When `alpha != 0`, applies AlphaPO transformation: `r = (1 - p^(-alpha)) / alpha` from the [AlphaPO paper](https://huggingface.co/papers/2501.03884). This parameter works with all loss types.

generate_during_eval (`bool`, *optional*, defaults to `False`) : If `True`, generates and logs completions from the model to W&B or Comet during evaluation.

is_encoder_decoder (`bool`, *optional*) : When using the `model_init` argument (callable) to instantiate the model instead of the `model` argument, you need to specify if the model returned by the callable is an encoder-decoder model.

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments to pass to `AutoModelForCausalLM.from_pretrained` when instantiating the model from a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.15.0/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained).

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

Configuration class for the [experimental.cpo.CPOTrainer](/docs/trl/v1.10.0/en/cpo_trainer#trl.experimental.cpo.CPOTrainer).

This class includes only the parameters that are specific to CPO training. For a full list of training arguments,
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
