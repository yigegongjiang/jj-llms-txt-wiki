# GaudiTrainer

The [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer#optimum.habana.GaudiTrainer) class provides an extended API for the feature-complete [Transformers Trainer](https://huggingface.co/docs/transformers/main_classes/trainer). It is used in all the [example scripts](/examples).

Before instantiating your [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer#optimum.habana.GaudiTrainer), create a [GaudiTrainingArguments](/docs/optimum.habana/main/en/package_reference/trainer#optimum.habana.GaudiTrainingArguments) object to access all the points of customization during training.

The [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer#optimum.habana.GaudiTrainer) class is optimized for 🤗 Transformers models running on Intel Gaudi.

Here is an example of how to customize [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer#optimum.habana.GaudiTrainer) to use a weighted loss (useful when you have an unbalanced training set):

```python
from torch import nn
from optimum.habana import GaudiTrainer

class CustomGaudiTrainer(GaudiTrainer):
    def compute_loss(self, model, inputs, return_outputs=False):
        labels = inputs.get("labels")
        # forward pass
        outputs = model(**inputs)
        logits = outputs.get("logits")
        # compute custom loss (suppose one has 3 labels with different weights)
        loss_fct = nn.CrossEntropyLoss(weight=torch.tensor([1.0, 2.0, 3.0]))
        loss = loss_fct(logits.view(-1, self.model.config.num_labels), labels.view(-1))
        return (loss, outputs) if return_outputs else loss
```

Another way to customize the training loop behavior for the PyTorch [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer#optimum.habana.GaudiTrainer) is to use [callbacks](https://huggingface.co/docs/transformers/main_classes/callback) that can inspect the training loop state (for progress reporting, logging on TensorBoard or other ML platforms...) and take decisions (like early stopping).

## GaudiTrainer[[optimum.habana.GaudiTrainer]]

#### optimum.habana.GaudiTrainer[[optimum.habana.GaudiTrainer]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L214)

GaudiTrainer is built on top of the tranformers' Trainer to enable
deployment on Habana's Gaudi.

autocast_smart_context_manageroptimum.habana.GaudiTrainer.autocast_smart_context_managerhttps://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L1690[{"name": "cache_enabled", "val": ": typing.Optional[bool] = True"}]

A helper wrapper that creates an appropriate context manager for `autocast` while feeding it the desired
arguments, depending on the situation.
Modified by Habana to enable using `autocast` on Gaudi devices.
#### evaluate[[optimum.habana.GaudiTrainer.evaluate]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L1887)

From https://github.com/huggingface/transformers/blob/v4.38.2/src/transformers/trainer.py#L3162 with the following modification
1. use throughput_warmup_steps in evaluation throughput calculation
#### evaluation_loop[[optimum.habana.GaudiTrainer.evaluation_loop]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L2013)

Prediction/evaluation loop, shared by `Trainer.evaluate()` and `Trainer.predict()`.
Works both with or without labels.
#### predict[[optimum.habana.GaudiTrainer.predict]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L1966)

From https://github.com/huggingface/transformers/blob/v4.45.2/src/transformers/trainer.py#L3904 with the following modification
1. comment out TPU related
2. use throughput_warmup_steps in evaluation throughput calculation
#### prediction_step[[optimum.habana.GaudiTrainer.prediction_step]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L2282)

Perform an evaluation step on `model` using `inputs`.
Subclass and override to inject custom behavior.

**Parameters:**

model (`torch.nn.Module`) : The model to evaluate.

inputs (`dict[str, Union[torch.Tensor, Any]]`) : The inputs and targets of the model. The dictionary will be unpacked before being fed to the model. Most models expect the targets under the argument `labels`. Check your model's documentation for all accepted arguments.

prediction_loss_only (`bool`) : Whether or not to return the loss only.

ignore_keys (`List[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions.

**Returns:**

`tuple[Optional[torch.Tensor], Optional[torch.Tensor], Optional[torch.Tensor]]`

A tuple with the loss,
logits and labels (each being optional).
#### save_model[[optimum.habana.GaudiTrainer.save_model]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L1790)

Will save the model, so you can reload it using `from_pretrained()`.
Will only save from the main process.
#### train[[optimum.habana.GaudiTrainer.train]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L517)

Main training entry point.

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`List[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments used to hide deprecated arguments
#### training_step[[optimum.habana.GaudiTrainer.training_step]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer.py#L1710)

Perform a training step on a batch of inputs.

Subclass and override to inject custom behavior.

**Parameters:**

model (`torch.nn.Module`) : The model to train.

inputs (`dict[str, Union[torch.Tensor, Any]]`) : The inputs and targets of the model.  The dictionary will be unpacked before being fed to the model. Most models expect the targets under the argument `labels`. Check your model's documentation for all accepted arguments.

**Returns:**

``torch.Tensor``

The tensor with training loss on this batch.

## GaudiSeq2SeqTrainer[[optimum.habana.GaudiSeq2SeqTrainer]]

#### optimum.habana.GaudiSeq2SeqTrainer[[optimum.habana.GaudiSeq2SeqTrainer]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer_seq2seq.py#L56)

evaluateoptimum.habana.GaudiSeq2SeqTrainer.evaluatehttps://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer_seq2seq.py#L142[{"name": "eval_dataset", "val": ": typing.Optional[torch.utils.data.dataset.Dataset] = None"}, {"name": "ignore_keys", "val": ": typing.Optional[list[str]] = None"}, {"name": "metric_key_prefix", "val": ": str = 'eval'"}, {"name": "**gen_kwargs", "val": ""}]- **eval_dataset** (`Dataset`, *optional*) --
  Pass a dataset if you wish to override `self.eval_dataset`. If it is an [Dataset](https://huggingface.co/docs/datasets/main/en/package_reference/main_classes#datasets.Dataset), columns
  not accepted by the `model.forward()` method are automatically removed. It must implement the `__len__`
  method.
- **ignore_keys** (`List[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions.
- **metric_key_prefix** (`str`, *optional*, defaults to `"eval"`) --
  An optional prefix to be used as the metrics key prefix. For example the metrics "bleu" will be named
  "eval_bleu" if the prefix is `"eval"` (default)
- **max_length** (`int`, *optional*) --
  The maximum target length to use when predicting with the generate method.
- **num_beams** (`int`, *optional*) --
  Number of beams for beam search that will be used when predicting with the generate method. 1 means no
  beam search.
- **gen_kwargs** --
  Additional `generate` specific kwargs.0A dictionary containing the evaluation loss and the potential metrics computed from the predictions. The
dictionary also contains the epoch number which comes from the training state.

Run evaluation and returns metrics.
The calling script will be responsible for providing a method to compute metrics, as they are task-dependent
(pass it to the init `compute_metrics` argument).
You can also subclass and override this method to inject custom behavior.

**Parameters:**

eval_dataset (`Dataset`, *optional*) : Pass a dataset if you wish to override `self.eval_dataset`. If it is an [Dataset](https://huggingface.co/docs/datasets/main/en/package_reference/main_classes#datasets.Dataset), columns not accepted by the `model.forward()` method are automatically removed. It must implement the `__len__` method.

ignore_keys (`List[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions.

metric_key_prefix (`str`, *optional*, defaults to `"eval"`) : An optional prefix to be used as the metrics key prefix. For example the metrics "bleu" will be named "eval_bleu" if the prefix is `"eval"` (default)

max_length (`int`, *optional*) : The maximum target length to use when predicting with the generate method.

num_beams (`int`, *optional*) : Number of beams for beam search that will be used when predicting with the generate method. 1 means no beam search.

gen_kwargs : Additional `generate` specific kwargs.

**Returns:**

A dictionary containing the evaluation loss and the potential metrics computed from the predictions. The
dictionary also contains the epoch number which comes from the training state.
#### predict[[optimum.habana.GaudiSeq2SeqTrainer.predict]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/trainer_seq2seq.py#L194)

Run prediction and returns predictions and potential metrics.
Depending on the dataset and your use case, your test dataset may contain labels. In that case, this method
will also return metrics, like in `evaluate()`.

If your predictions or labels have different sequence lengths (for instance because you're doing dynamic
padding in a token classification task) the predictions will be padded (on the right) to allow for
concatenation into one array. The padding index is -100.

Returns: *NamedTuple* A namedtuple with the following keys:
- predictions (`np.ndarray`): The predictions on `test_dataset`.
- label_ids (`np.ndarray`, *optional*): The labels (if the dataset contained some).
- metrics (`Dict[str, float]`, *optional*): The potential dictionary of metrics (if the dataset contained
  labels).

**Parameters:**

test_dataset (`Dataset`) : Dataset to run the predictions on. If it is a [Dataset](https://huggingface.co/docs/datasets/main/en/package_reference/main_classes#datasets.Dataset), columns not accepted by the `model.forward()` method are automatically removed. Has to implement the method `__len__`

ignore_keys (`List[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions.

metric_key_prefix (`str`, *optional*, defaults to `"eval"`) : An optional prefix to be used as the metrics key prefix. For example the metrics "bleu" will be named "eval_bleu" if the prefix is `"eval"` (default)

max_length (`int`, *optional*) : The maximum target length to use when predicting with the generate method.

num_beams (`int`, *optional*) : Number of beams for beam search that will be used when predicting with the generate method. 1 means no beam search.

gen_kwargs : Additional `generate` specific kwargs.

## GaudiTrainingArguments[[optimum.habana.GaudiTrainingArguments]]

#### optimum.habana.GaudiTrainingArguments[[optimum.habana.GaudiTrainingArguments]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/training_args.py#L86)

GaudiTrainingArguments is built on top of the Tranformers' [TrainingArguments](https://huggingface.co/docs/transformers/main_classes/trainer#transformers.TrainingArguments)
to enable deployment on Habana's Gaudi.

**Parameters:**

use_habana (`bool`, *optional*, defaults to `False`) : Whether to use Habana's HPU for running the model.

gaudi_config_name (`str`, *optional*) : Pretrained Gaudi config name or path.

use_lazy_mode (`bool`, *optional*, defaults to `True`) : Whether to use lazy mode for running the model.

use_hpu_graphs (`bool`, *optional*, defaults to `False`) : Deprecated, use `use_hpu_graphs_for_inference` instead. Whether to use HPU graphs for performing inference.

use_hpu_graphs_for_inference (`bool`, *optional*, defaults to `False`) : Whether to use HPU graphs for performing inference. It will speed up latency but may not be compatible with some operations.

use_hpu_graphs_for_training (`bool`, *optional*, defaults to `False`) : Whether to use HPU graphs for performing inference. It will speed up training but may not be compatible with some operations.

use_compiled_autograd (`bool`, *optional*, defaults to `False`) : Whether to use compiled autograd for training. Currently only for summarization models.

compile_from_sec_iteration (`bool`, *optional*, defaults to `False`) : Whether to torch.compile from the second training iteration.

compile_dynamic (`bool|None`, *optional*, defaults to `None`) : Set value of 'dynamic' parameter for torch.compile.

use_regional_compilation (`bool`, *optional*, defaults to `False`) : Whether to use regional compile with deepspeed

inline_inbuilt_nn_modules (`bool`, *optional*, defaults to `None`) : Set value of 'inline_inbuilt_nn_modules' parameter for torch._dynamo.config. Currently, disabling this parameter improves the performance of the ALBERT model.

cache_size_limit(`int`, *optional*, defaults to 'None') : Set value of 'cache_size_limit' parameter for torch._dynamo.config

allow_unspec_int_on_nn_module (`bool`, *optional*, defaults to `None`) : Set value of 'allow_unspec_int_on_nn_module' parameter for torch._dynamo.config.

disable_tensor_cache_hpu_graphs (`bool`, *optional*, defaults to `False`) : Whether to disable tensor cache when using hpu graphs. If True, tensors won't be cached in hpu graph and memory can be saved.

max_hpu_graphs (`int`, *optional*) : Maximum number of hpu graphs to be cached. Reduce to save device memory.

distribution_strategy (`str`, *optional*, defaults to `ddp`) : Determines how data parallel distributed training is achieved. May be: `ddp` or `fast_ddp`.

throughput_warmup_steps (`int`, *optional*, defaults to 0) : Number of steps to ignore for throughput calculation. For example, with `throughput_warmup_steps=N`, the first N steps will not be considered in the calculation of the throughput. This is especially useful in lazy mode where the first two or three iterations typically take longer.

adjust_throughput ('bool', *optional*, defaults to `False`) : Whether to remove the time taken for logging, evaluating and saving from throughput calculation.

pipelining_fwd_bwd (`bool`, *optional*, defaults to `False`) : Whether to add an additional `mark_step` between forward and backward for pipelining host backward building and HPU forward computing.

non_blocking_data_copy (`bool`, *optional*, defaults to `False`) : Whether to enable async data copy when preparing inputs.

profiling_warmup_steps (`int`, *optional*, defaults to 0) : Number of training steps to ignore for profiling.

profiling_steps (`int`, *optional*, defaults to 0) : Number of training steps to be captured when enabling profiling.

profiling_warmup_steps_eval (`int`, *optional*, defaults to 0) : Number of eval steps to ignore for profiling.

profiling_steps_eval (`int`, *optional*, defaults to 0) : Number of eval steps to be captured when enabling profiling.

## GaudiSeq2SeqTrainingArguments[[optimum.habana.GaudiSeq2SeqTrainingArguments]]

#### optimum.habana.GaudiSeq2SeqTrainingArguments[[optimum.habana.GaudiSeq2SeqTrainingArguments]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/training_args_seq2seq.py#L30)

GaudiSeq2SeqTrainingArguments is built on top of the Tranformers' [Seq2SeqTrainingArguments](https://huggingface.co/docs/transformers/main_classes/trainer#transformers.Seq2SeqTrainingArguments)
to enable deployment on Habana's Gaudi.

to_dictoptimum.habana.GaudiSeq2SeqTrainingArguments.to_dicthttps://github.com/huggingface/optimum-habana/blob/main/optimum/habana/transformers/training_args_seq2seq.py#L83[]

Serializes this instance while replace `Enum` by their values and `GaudiGenerationConfig` by dictionaries (for JSON
serialization support). It obfuscates the token values by removing their value.

**Parameters:**

predict_with_generate (`bool`, *optional*, defaults to `False`) : Whether to use generate to calculate generative metrics (ROUGE, BLEU).

generation_max_length (`int`, *optional*) : The `max_length` to use on each evaluation loop when `predict_with_generate=True`. Will default to the `max_length` value of the model configuration.

generation_num_beams (`int`, *optional*) : The `num_beams` to use on each evaluation loop when `predict_with_generate=True`. Will default to the `num_beams` value of the model configuration.

generation_config (`str` or `Path` or `transformers.generation.GenerationConfig`, *optional*) : Allows to load a `transformers.generation.GenerationConfig` from the `from_pretrained` method. This can be either:  - a string, the *model id* of a pretrained model configuration hosted inside a model repo on huggingface.co. - a path to a *directory* containing a configuration file saved using the [transformers.GenerationConfig.save_pretrained](https://huggingface.co/docs/transformers/main/en/main_classes/text_generation#transformers.GenerationConfig.save_pretrained) method, e.g., `./my_model_directory/`. - a `transformers.generation.GenerationConfig` object.
