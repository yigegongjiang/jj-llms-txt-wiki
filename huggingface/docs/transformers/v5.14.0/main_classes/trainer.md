# Trainer

The [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) class provides an API for feature-complete training in PyTorch, and it supports distributed training on multiple GPUs/TPUs, mixed precision for [NVIDIA GPUs](https://nvidia.github.io/apex/), [AMD GPUs](https://rocm.docs.amd.com/en/latest/rocm.html), and [`torch.amp`](https://pytorch.org/docs/stable/amp.html) for PyTorch. [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) goes hand-in-hand with the [TrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.TrainingArguments) class, which offers a wide range of options to customize how a model is trained. Together, these two classes provide a complete training API.

[Seq2SeqTrainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Seq2SeqTrainer) and [Seq2SeqTrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Seq2SeqTrainingArguments) inherit from the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) and [TrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.TrainingArguments) classes and they're adapted for training models for sequence-to-sequence tasks such as summarization or translation.

The [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) class is optimized for 🤗 Transformers models and can have surprising behaviors
when used with other models. When using it with your own model, make sure:

- your model always return tuples or subclasses of [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput)
- your model can compute the loss if a `labels` argument is provided and that loss is returned as the first
  element of the tuple (if your model returns tuples)
- your model can accept multiple label arguments (use `label_names` in [TrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.TrainingArguments) to indicate their name to the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer)) but none of them should be named `"label"`

## Trainer[[api-reference]][[transformers.Trainer]]

)>, )>], )>] | None = None"}]}>
- **model** ([PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel) or `torch.nn.Module`, *optional*) --
  The model to train, evaluate or use for predictions. If not provided, a `model_init` must be passed.

  

  [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) is optimized to work with the [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel) provided by the library. You can still use
  your own models defined as `torch.nn.Module` as long as they work the same way as the 🤗 Transformers
  models.

  

- **args** ([TrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.TrainingArguments), *optional*) --
  The arguments to tweak for training. Will default to a basic instance of [TrainingArguments](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.TrainingArguments) with the
  `output_dir` set to a directory named *tmp_trainer* in the current directory if not provided.
- **data_collator** (`DataCollator`, *optional*) --
  The function to use to form a batch from a list of elements of `train_dataset` or `eval_dataset`. Will
  default to [default_data_collator()](/docs/transformers/v5.14.0/en/main_classes/data_collator#transformers.default_data_collator) if no `processing_class` is provided, an instance of
  [DataCollatorWithPadding](/docs/transformers/v5.14.0/en/main_classes/data_collator#transformers.DataCollatorWithPadding) otherwise if the processing_class is a feature extractor or tokenizer.
- **train_dataset** (`torch.utils.data.Dataset` | `torch.utils.data.IterableDataset` | `datasets.Dataset`, *optional*) --
  The dataset to use for training. If it is a `Dataset`, columns not accepted by the
  `model.forward()` method are automatically removed.

  Note that if it's a `torch.utils.data.IterableDataset` with some randomization and you are training in a
  distributed fashion, your iterable dataset should either use a internal attribute `generator` that is a
  `torch.Generator` for the randomization that must be identical on all processes (and the Trainer will
  manually set the seed of this `generator` at each epoch) or have a `set_epoch()` method that internally
  sets the seed of the RNGs used.

  If the dataset does not implement `__len__` (e.g. a streaming `IterableDataset`), `max_steps` must be set,
  since the total number of training steps cannot be inferred.
- **eval_dataset** (`torch.utils.data.Dataset` | dict[str, `torch.utils.data.Dataset`] | `datasets.Dataset`, *optional*) --
  The dataset to use for evaluation. If it is a `Dataset`, columns not accepted by the
  `model.forward()` method are automatically removed. If it is a dictionary, it will evaluate on each
  dataset prepending the dictionary key to the metric name.
- **processing_class** (`PreTrainedTokenizerBase` or `BaseImageProcessor` or `FeatureExtractionMixin` or `ProcessorMixin`, *optional*) --
  Processing class used to process the data. If provided, will be used to automatically process the inputs
  for the model, and it will be saved along the model to make it easier to rerun an interrupted training or
  reuse the fine-tuned model.
- **model_init** (`Callable[[], PreTrainedModel]`, *optional*) --
  A function that instantiates the model to be used. If provided, each call to [train()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.train) will start
  from a new instance of the model as given by this function.

  The function may have zero argument, or a single one containing the optuna/Ray Tune trial object, to
  be able to choose different architectures according to hyperparameters (such as layer count, sizes of
  inner layers, dropout probabilities etc).
- **compute_loss_func** (`Callable`, *optional*) --
  A function that accepts the raw model outputs, labels, and the number of items in the entire accumulated
  batch (batch_size * gradient_accumulation_steps) and returns the loss. For example, see the default [loss function](https://github.com/huggingface/transformers/blob/052e652d6d53c2b26ffde87e039b723949a53493/src/transformers/trainer.py#L3618) used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer).
- **compute_metrics** (`Callable[[EvalPrediction], Dict]`, *optional*) --
  The function that will be used to compute metrics at evaluation. Must take a [EvalPrediction](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.EvalPrediction) and return
  a dictionary string to metric values. *Note* When passing TrainingArgs with `batch_eval_metrics` set to
  `True`, your compute_metrics function must take a boolean `compute_result` argument. This will be triggered
  after the last eval batch to signal that the function needs to calculate and return the global summary
  statistics rather than accumulating the batch-level statistics
- **callbacks** (List of [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback), *optional*) --
  A list of callbacks to customize the training loop. Will add those to the list of default callbacks
  detailed in [here](callback).

  If you want to remove one of the default callbacks used, use the [Trainer.remove_callback()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.remove_callback) method.
- **optimizers** (`tuple[torch.optim.Optimizer, torch.optim.lr_scheduler.LambdaLR]`, *optional*, defaults to `(None, None)`) --
  A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your
  model and a scheduler given by [get_linear_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup) controlled by `args`.
- **optimizer_cls_and_kwargs** (`tuple[Type[torch.optim.Optimizer], dict[str, Any]]`, *optional*) --
  A tuple containing the optimizer class and keyword arguments to use.
  Overrides `optim` and `optim_args` in `args`. Incompatible with the `optimizers` argument.

  Unlike `optimizers`, this argument avoids the need to place model parameters on the correct devices before initializing the Trainer.
- **preprocess_logits_for_metrics** (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`, *optional*) --
  A function that preprocess the logits right before caching them at each evaluation step. Must take two
  tensors, the logits and the labels, and return the logits once processed as desired. The modifications made
  by this function will be reflected in the predictions received by `compute_metrics`.

  Note that the labels (second parameter) will be `None` if the dataset does not have them.

Trainer is a simple but feature-complete training and eval loop for PyTorch, optimized for 🤗 Transformers.

Important attributes:

- **model** -- Always points to the core model. If using a transformers model, it will be a [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel)
  subclass.
- **model_wrapped** -- Always points to the most external model in case one or more other modules wrap the
  original model. This is the model that should be used for the forward pass. For example, under `DeepSpeed`,
  the inner model is wrapped in `DeepSpeed` and then again in `torch.nn.DistributedDataParallel`. If the inner
  model hasn't been wrapped, then `self.model_wrapped` is the same as `self.model`.
- **is_model_parallel** -- Whether or not a model has been switched to a model parallel mode (different from
  data parallelism, this means some of the model layers are split on different GPUs).
- **place_model_on_device** -- Whether or not to automatically place the model on the device. Defaults to
  `True` unless model parallel, DeepSpeed, FSDP, full fp16/bf16 eval, or SageMaker MP is active. Can be
  overridden by subclassing `TrainingArguments` and overriding the `place_model_on_device` property.
- **is_in_train** -- Whether or not a model is currently running `train` (e.g. when `evaluate` is called while
  in `train`)

- **callback** (`type` or [`~transformers.TrainerCallback]`) --
  A [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback) class or an instance of a [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback). In the
  first case, will instantiate a member of that class.

Add a callback to the current list of [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback).

A helper wrapper that creates an appropriate context manager for `autocast` while feeding it the desired
arguments, depending on the situation. We rely on accelerate for autocast, hence we do nothing here.

Invoke `model_init` to get a fresh model instance, optionally conditioned on a hyperparameter trial.

- **model** (`nn.Module`) --
  The model to compute the loss for.
- **inputs** (`dict[str, torch.Tensor | Any]`) --
  The input data for the model.
- **return_outputs** (`bool`, *optional*, defaults to `False`) --
  Whether to return the model outputs along with the loss.
- **num_items_in_batch** (Optional[torch.Tensor], *optional*) --
  The number of items in the batch. If not passed, the loss is computed
  using the default batch size reduction logic.The loss of the model along with its output if return_outputs was set to True

How the loss is computed by Trainer. By default, all models return the loss in the first element.

Subclass and override for custom behavior. If you are not using `num_items_in_batch` when computing your loss,
make sure to overwrite `self.model_accepts_loss_kwargs` to `False`. Otherwise, the loss calculation might be slightly inaccurate when performing gradient accumulation.

A helper wrapper to group together context managers.

Create the accelerator and perform post-creation setup (FSDP, DeepSpeed, etc.).

- **language** (`str`, *optional*) --
  The language of the model (if applicable)
- **license** (`str`, *optional*) --
  The license of the model. Will default to the license of the pretrained model used, if the original
  model given to the `Trainer` comes from a repo on the Hub.
- **tags** (`str` or `list[str]`, *optional*) --
  Some tags to be included in the metadata of the model card.
- **model_name** (`str`, *optional*) --
  The name of the model.
- **finetuned_from** (`str`, *optional*) --
  The name of the model used to fine-tune this one (if applicable). Will default to the name of the repo
  of the original model given to the `Trainer` (if it comes from the Hub).
- **tasks** (`str` or `list[str]`, *optional*) --
  One or several task identifiers, to be included in the metadata of the model card.
- **dataset_tags** (`str` or `list[str]`, *optional*) --
  One or several dataset tags, to be included in the metadata of the model card.
- **dataset** (`str` or `list[str]`, *optional*) --
  One or several dataset identifiers, to be included in the metadata of the model card.
- **dataset_args** (`str` or `list[str]`, *optional*) --
  One or several dataset arguments, to be included in the metadata of the model card.

Creates a draft of a model card using the information available to the `Trainer`.

`torch.optim.Optimizer`The optimizer instance.

Setup the optimizer.

We provide a reasonable default that works well. If you want to use something else, you can pass a tuple in the
Trainer's init through `optimizers`, or subclass and override this method in a subclass.

Setup the optimizer and the learning rate scheduler.

We provide a reasonable default that works well. If you want to use something else, you can pass a tuple in the
Trainer's init through `optimizers`, or subclass and override this method (or `create_optimizer` and/or
`create_scheduler`) in a subclass.

- **num_training_steps** (int) -- The number of training steps to do.`torch.optim.lr_scheduler.LRScheduler`The learning rate scheduler instance.

Setup the scheduler. The optimizer of the trainer must have been set up either before this method is called or
passed as an argument.

- **eval_dataset** (`Dataset` | dict[str, `Dataset`], *optional*) --
  Pass a dataset if you wish to override `self.eval_dataset`. If it is a `Dataset`, columns
  not accepted by the `model.forward()` method are automatically removed. If it is a dictionary, it will
  evaluate on each dataset, prepending the dictionary key to the metric name.

  

  If you pass a dictionary with names of datasets as keys and datasets as values, evaluate will run
  separate evaluations on each dataset. This can be useful to monitor how training affects other
  datasets or simply to get a more fine-grained evaluation.
  When used with `load_best_model_at_end`, make sure `metric_for_best_model` references exactly one
  of the datasets. If you, for example, pass in `{"data1": data1, "data2": data2}` for two datasets
  `data1` and `data2`, you could specify `metric_for_best_model="eval_data1_loss"` for using the
  loss on `data1` and `metric_for_best_model="eval_data2_loss"` for the loss on `data2`.

  

- **ignore_keys** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions.
- **metric_key_prefix** (`str`, *optional*, defaults to `"eval"`) --
  An optional prefix to be used as the metrics key prefix. For example the metrics "bleu" will be named
  "eval_bleu" if the prefix is "eval" (default)A dictionary containing the evaluation loss and the potential metrics computed from the predictions. The
dictionary also contains the epoch number which comes from the training state.

Run evaluation and returns metrics.

The calling script will be responsible for providing a method to compute metrics, as they are task-dependent
(pass it to the init `compute_metrics` argument).

You can also subclass and override this method to inject custom behavior.

Prediction/evaluation loop, shared by `Trainer.evaluate()` and `Trainer.predict()`.

Works both with or without labels.

- **inputs** (`dict[str, torch.Tensor | Any]`) --
  The inputs and targets of the model.`int`The number of floating-point operations.

For models that inherit from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel), uses that method to compute the number of floating point
operations for every backward + forward pass. If using another model, either implement such a method in the
model or subclass and override this method.

Collects a specified number of batches from the epoch iterator and optionally counts the number of items in the batches to properly scale the loss.

Get the context parallel size

Get all parameter names that weight decay will be applied to.

This function filters out parameters in two ways:
1. By layer type (instances of layers specified in ALL_LAYERNORM_LAYERS)
2. By parameter name patterns (containing 'bias', or variation of 'norm')

- **eval_dataset** (`str` or `torch.utils.data.Dataset`, *optional*) --
  If a `str`, will use `self.eval_dataset[eval_dataset]` as the evaluation dataset. If a `Dataset`, will override `self.eval_dataset`. If it is a `Dataset`, columns not accepted by the `model.forward()` method are automatically removed.

Returns the evaluation `~torch.utils.data.DataLoader`.

Subclass and override this method if you want to inject some custom behavior.

Returns the learning rate of each parameter from self.optimizer.

Get the number of trainable parameters.

- **args** (`transformers.training_args.TrainingArguments`) --
  The training arguments for the training session.
- **model** (`PreTrainedModel`, *optional*) --
  The model being trained. Required for some optimizers (GaLore, Apollo, LOMO).A tuple containing the optimizer class and a dictionary of optimizer keyword arguments.

Returns the optimizer class and optimizer parameters based on the training arguments.

- **param** (`str` or `torch.nn.parameter.Parameter`, *optional*) --
  The parameter for which optimizer group needs to be returned.

Returns optimizer group for a parameter if given, else returns all optimizer groups for params.

Get the sequence parallel size

- **test_dataset** (`torch.utils.data.Dataset`, *optional*) --
  The test dataset to use. If it is a `Dataset`, columns not accepted by the
  `model.forward()` method are automatically removed.

Returns the test `~torch.utils.data.DataLoader`.

Subclass and override this method if you want to inject some custom behavior.

Calculates total batch size (micro_batch * grad_accum * dp_world_size).

Accounts for all parallelism dimensions: TP, CP, and SP.

Formula: dp_world_size = world_size // (tp_size * cp_size * sp_size)

Where:
- TP (Tensor Parallelism): Model layers split across GPUs
- CP (Context Parallelism): Sequences split using Ring Attention (FSDP2)
- SP (Sequence Parallelism): Sequences split using ALST/Ulysses (DeepSpeed)

All dimensions are separate and multiplicative: world_size = dp_size * tp_size * cp_size * sp_size

Get the tensor parallel size from either the model or DeepSpeed config.

Returns the training `~torch.utils.data.DataLoader`.

Will use no sampler if `train_dataset` does not implement `__len__`, a random sampler (adapted to distributed
training if necessary) otherwise.

Subclass and override this method if you want to inject some custom behavior.

- **hp_space** (`Callable[["optuna.Trial"], dict[str, float]]`, *optional*) --
  A function that defines the hyperparameter search space. Will default to
  `default_hp_space_optuna()` or `default_hp_space_ray()`
  depending on your backend.
- **compute_objective** (`Callable[[dict[str, float]], float]`, *optional*) --
  A function computing the objective to minimize or maximize from the metrics returned by the `evaluate`
  method. Will default to `default_compute_objective()`.
- **n_trials** (`int`, *optional*, defaults to 100) --
  The number of trial runs to test.
- **direction** (`str` or `list[str]`, *optional*, defaults to `"minimize"`) --
  If it's single objective optimization, direction is `str`, can be `"minimize"` or `"maximize"`, you
  should pick `"minimize"` when optimizing the validation loss, `"maximize"` when optimizing one or
  several metrics. If it's multi objectives optimization, direction is `list[str]`, can be List of
  `"minimize"` and `"maximize"`, you should pick `"minimize"` when optimizing the validation loss,
  `"maximize"` when optimizing one or several metrics.
- **backend** (`str` or `~training_utils.HPSearchBackend`, *optional*) --
  The backend to use for hyperparameter search. Will default to optuna or Ray Tune, depending
  on which one is installed. If all are installed, will default to optuna.
- **hp_name** (`Callable[["optuna.Trial"], str]]`, *optional*) --
  A function that defines the trial/run name. Will default to None.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments for each backend:

  - `optuna`: parameters from
    [optuna.study.create_study](https://optuna.readthedocs.io/en/stable/reference/generated/optuna.study.create_study.html)
    and also the parameters `timeout`, `n_jobs` and `gc_after_trial` from
    [optuna.study.Study.optimize](https://optuna.readthedocs.io/en/stable/reference/generated/optuna.study.Study.html#optuna.study.Study.optimize)
  - `ray`: parameters from [tune.run](https://docs.ray.io/en/latest/tune/api_docs/execution.html#tune-run).
    If `resources_per_trial` is not set in the `kwargs`, it defaults to 1 CPU core and 1 GPU (if available).
    If `progress_reporter` is not set in the `kwargs`,
    [ray.tune.CLIReporter](https://docs.ray.io/en/latest/tune/api/doc/ray.tune.CLIReporter.html) is used.[`trainer_utils.BestRun` or `list[trainer_utils.BestRun]`]All the information about the best run or best
runs for multi-objective optimization. Experiment summary can be found in `run_summary` attribute for Ray
backend.

Launch a hyperparameter search using `optuna` or `Ray Tune`. The optimized quantity is determined
by `compute_objective`, which defaults to a function returning the evaluation loss when no metric is provided,
the sum of all metrics otherwise.

To use this method, you need to have provided a `model_init` when initializing your [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer): we need to
reinitialize the model at each new run. This is incompatible with the `optimizers` argument, so you need to
subclass [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) and override the method [create_optimizer_and_scheduler()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.create_optimizer_and_scheduler) for custom
optimizer/scheduler.

Initializes a git repo in `self.args.hub_model_id`.

Whether or not this process is the local (e.g., on one machine if training in a distributed fashion on several
machines) main process.

Whether or not this process is the global main process (when training in a distributed fashion on several
machines, this is only going to be `True` for one process).

- **logs** (`dict[str, float]`) --
  The values to log.
- **start_time** (`Optional[float]`) --
  The start of training.

Log `logs` on the various objects watching training.

Subclass and override this method to inject custom behavior.

- **split** (`str`) --
  Mode/split name: one of `train`, `eval`, `test`
- **metrics** (`dict[str, float]`) --
  The metrics returned from train/evaluate/predictmetrics: metrics dict

Log metrics in a specially formatted way.

Under distributed environment this is done only for a process with rank 0.

Notes on memory reports:

In order to get memory usage report you need to install `psutil`. You can do that with `pip install psutil`.

Now when this method is run, you will see a report that will include:

```
init_mem_cpu_alloc_delta   =     1301MB
init_mem_cpu_peaked_delta  =      154MB
init_mem_gpu_alloc_delta   =      230MB
init_mem_gpu_peaked_delta  =        0MB
train_mem_cpu_alloc_delta  =     1345MB
train_mem_cpu_peaked_delta =        0MB
train_mem_gpu_alloc_delta  =      693MB
train_mem_gpu_peaked_delta =        7MB
```

**Understanding the reports:**

- the first segment, e.g., `train__`, tells you which stage the metrics are for. Reports starting with `init_`
  will be added to the first stage that gets run. So that if only evaluation is run, the memory usage for the
  `__init__` will be reported along with the `eval_` metrics.
- the third segment, is either `cpu` or `gpu`, tells you whether it's the general RAM or the gpu0 memory
  metric.
- `*_alloc_delta` - is the difference in the used/allocated memory counter between the end and the start of the
  stage - it can be negative if a function released more memory than it allocated.
- `*_peaked_delta` - is any extra memory that was consumed and then freed - relative to the current allocated
  memory counter - it is never negative. When you look at the metrics of any stage you add up `alloc_delta` +
  `peaked_delta` and you know how much memory was needed to complete that stage.

The reporting happens only for process of rank 0 and gpu 0 (if there is a gpu). Typically this is enough since the
main process does the bulk of work, but it could be not quite so if model parallel is used and then other GPUs may
use a different amount of gpu memory. This is also not the same under DataParallel where gpu0 may require much more
memory than the rest since it stores the gradient and optimizer states for all participating GPUs. Perhaps in the
future these reports will evolve to measure those too.

The CPU RAM metric measures RSS (Resident Set Size) includes both the memory which is unique to the process and the
memory shared with other processes. It is important to note that it does not include swapped out memory, so the
reports could be imprecise.

The CPU peak memory is measured using a sampling thread. Due to python's GIL it may miss some of the peak memory if
that thread didn't get a chance to run when the highest memory was used. Therefore this report can be less than
reality. Using `tracemalloc` would have reported the exact peak memory, but it doesn't report memory allocations
outside of python. So if some C++ CUDA extension allocated its own memory it won't be reported. And therefore it
was dropped in favor of the memory sampling approach, which reads the current process memory usage.

The GPU allocated and peak memory reporting is done with `torch.cuda.memory_allocated()` and
`torch.cuda.max_memory_allocated()`. This metric reports only "deltas" for pytorch-specific allocations, as
`torch.cuda` memory management system doesn't track any memory allocated outside of pytorch. For example, the very
first cuda call typically loads CUDA kernels, which may take from 0.5 to 2GB of GPU memory.

Note that this tracker doesn't account for memory allocations outside of [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer)'s `__init__`, `train`,
`evaluate` and `predict` calls.

Because `evaluation` calls may happen during `train`, we can't handle nested invocations because
`torch.cuda.max_memory_allocated` is a single counter, so if it gets reset by a nested eval call, `train`'s tracker
will report incorrect info. If this [pytorch issue](https://github.com/pytorch/pytorch/issues/16266) gets resolved
it will be possible to change this class to be re-entrant. Until then we will only track the outer level of
`train`, `evaluate` and `predict` methods. Which means that if `eval` is called during `train`, it's the latter
that will account for its memory usage and that of the former.

This also means that if any other tool that is used along the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) calls
`torch.cuda.reset_peak_memory_stats`, the gpu peak memory stats could be invalid. And the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) will disrupt
the normal behavior of any such tools that rely on calling `torch.cuda.reset_peak_memory_stats` themselves.

For best performance you may want to consider turning the memory profiling off for production runs.

- **metrics** (`dict[str, float]`) --
  The metrics returned from train/evaluate/predictmetrics (`dict[str, float]`)The reformatted metrics

Reformat Trainer metrics values to a human-readable format.

Helper to get number of samples in a `~torch.utils.data.DataLoader` by accessing its dataset. When
dataloader.dataset does not exist or has no length, estimates as best it can

- **callback** (`type` or [`~transformers.TrainerCallback]`) --
  A [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback) class or an instance of a [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback). In the
  first case, will pop the first member of that class found in the list of callbacks.[TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback)The callback removed, if found.

Remove a callback from the current list of [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback) and returns it.

If the callback is not found, returns `None` (and no error is raised).

- **test_dataset** (`Dataset`) --
  Dataset to run the predictions on. If it is an `datasets.Dataset`, columns not accepted by the
  `model.forward()` method are automatically removed.
- **ignore_keys** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions.
- **metric_key_prefix** (`str`, *optional*, defaults to `"test"`) --
  An optional prefix to be used as the metrics key prefix. For example the metrics "bleu" will be named
  "test_bleu" if the prefix is "test" (default)

Run prediction and returns predictions and potential metrics.

Depending on the dataset and your use case, your test dataset may contain labels. In that case, this method
will also return metrics, like in `evaluate()`.

If your predictions or labels have different sequence length (for instance because you're doing dynamic padding
in a token classification task) the predictions will be padded (on the right) to allow for concatenation into
one array. The padding index is -100.

Returns: *NamedTuple* A namedtuple with the following keys:

- predictions (`np.ndarray`): The predictions on `test_dataset`.
- label_ids (`np.ndarray`, *optional*): The labels (if the dataset contained some).
- metrics (`dict[str, float]`, *optional*): The potential dictionary of metrics (if the dataset contained
  labels).

- **model** (`nn.Module`) --
  The model to evaluate.
- **inputs** (`dict[str, torch.Tensor | Any]`) --
  The inputs and targets of the model.

  The dictionary will be unpacked before being fed to the model. Most models expect the targets under the
  argument `labels`. Check your model's documentation for all accepted arguments.
- **prediction_loss_only** (`bool`) --
  Whether or not to return the loss only.
- **ignore_keys** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions.tuple[Optional[torch.Tensor], Optional[torch.Tensor], Optional[torch.Tensor]]A tuple with the loss,
logits and labels (each being optional).

Perform an evaluation step on `model` using `inputs`.

Subclass and override to inject custom behavior.

- **commit_message** (`str`, *optional*, defaults to `"End of training"`) --
  Message to commit while pushing.
- **blocking** (`bool`, *optional*, defaults to `True`) --
  Whether the function should return only when the `git push` has finished.
- **token** (`str`, *optional*, defaults to `None`) --
  Token with write permission to overwrite Trainer's original args.
- **revision** (`str`, *optional*) --
  The git revision to commit from. Defaults to the head of the "main" branch.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments passed along to [create_model_card()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.create_model_card).The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.

- **callback** (`type` or [`~transformers.TrainerCallback]`) --
  A [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback) class or an instance of a [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback). In the
  first case, will remove the first member of that class found in the list of callbacks.

Remove a callback from the current list of [TrainerCallback](/docs/transformers/v5.14.0/en/main_classes/callback#transformers.TrainerCallback).

- **split** (`str`) --
  Mode/split name: one of `train`, `eval`, `test`, `all`
- **metrics** (`dict[str, float]`) --
  The metrics returned from train/evaluate/predict
- **combined** (`bool`, *optional*, defaults to `True`) --
  Creates combined metrics by updating `all_results.json` with metrics of this call

Save metrics into a json file for that split, e.g. `train_results.json`.

Under distributed environment this is done only for a process with rank 0.

To understand the metrics please read the docstring of [log_metrics()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.log_metrics). The only difference is that raw
unformatted numbers are saved in the current method.

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

Saves the Trainer state, since Trainer.save_model saves only the tokenizer with the model.

Under distributed environment this is done only for a process with rank 0.

Calculates and returns the following values:
- `num_train_epochs`
- `num_update_steps_per_epoch`
- `num_examples`
- `num_train_samples`
- `total_train_batch_size`
- `steps_in_epoch` (total batches per epoch)
- `max_steps`

Store the number of floating-point operations that went into the model.

- **resume_from_checkpoint** (`str` or `bool`, *optional*) --
  If a `str`, local path to a saved checkpoint as saved by a previous instance of [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer). If a
  `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance
  of [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer). If present, training will resume from the model/optimizer/scheduler states loaded here.
- **trial** (`optuna.Trial` or `dict[str, Any]`, *optional*) --
  The trial run or the hyperparameter dictionary for hyperparameter search.
- **ignore_keys_for_eval** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions for evaluation during the training.`TrainOutput`Object containing the global step count, training loss, and metrics.

Main training entry point.

- **model** (`nn.Module`) --
  The model to train.
- **inputs** (`dict[str, torch.Tensor | Any]`) --
  The inputs and targets of the model.

  The dictionary will be unpacked before being fed to the model. Most models expect the targets under the
  argument `labels`. Check your model's documentation for all accepted arguments.`torch.Tensor`The tensor with training loss on this batch.

Perform a training step on a batch of inputs.

Subclass and override to inject custom behavior.

## Seq2SeqTrainer[[transformers.Seq2SeqTrainer]]

)>, )>], )>] | None = None"}]}>

- **eval_dataset** (`Dataset`, *optional*) --
  Pass a dataset if you wish to override `self.eval_dataset`. If it is an `Dataset`, columns
  not accepted by the `model.forward()` method are automatically removed. It must implement the `__len__`
  method.
- **ignore_keys** (`list[str]`, *optional*) --
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
  Additional `generate` specific kwargs.A dictionary containing the evaluation loss and the potential metrics computed from the predictions. The
dictionary also contains the epoch number which comes from the training state.

Run evaluation and returns metrics.

The calling script will be responsible for providing a method to compute metrics, as they are task-dependent
(pass it to the init `compute_metrics` argument).

You can also subclass and override this method to inject custom behavior.

- **test_dataset** (`Dataset`) --
  Dataset to run the predictions on. If it is a `Dataset`, columns not accepted by the
  `model.forward()` method are automatically removed. Has to implement the method `__len__`
- **ignore_keys** (`list[str]`, *optional*) --
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
  Additional `generate` specific kwargs.

Run prediction and returns predictions and potential metrics.

Depending on the dataset and your use case, your test dataset may contain labels. In that case, this method
will also return metrics, like in `evaluate()`.

If your predictions or labels have different sequence lengths (for instance because you're doing dynamic
padding in a token classification task) the predictions will be padded (on the right) to allow for
concatenation into one array. The padding index is -100.

Returns: *NamedTuple* A namedtuple with the following keys:

- predictions (`np.ndarray`): The predictions on `test_dataset`.
- label_ids (`np.ndarray`, *optional*): The labels (if the dataset contained some).
- metrics (`dict[str, float]`, *optional*): The potential dictionary of metrics (if the dataset contained
  labels).

## TrainingArguments[[transformers.TrainingArguments]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": typing.Optional[typing.Any] = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}]}>
- **output_dir** (`str`, *optional*, defaults to `"trainer_output"`) --
  The output directory where the model predictions and checkpoints will be written.

Training Duration and Batch Size

- **per_device_train_batch_size** (`int`, *optional*, defaults to 8) --
  The batch size *per device*. The **global batch size** is computed as:
  `per_device_train_batch_size * number_of_devices` in multi-GPU or distributed setups.
- **num_train_epochs(`float`,** *optional*, defaults to 3.0) --
  Total number of training epochs to perform (if not an integer, will perform the decimal part percents of
  the last epoch before stopping training).
- **max_steps** (`int`, *optional*, defaults to -1) --
  Overrides `num_train_epochs`. If set to a positive number, the total number of training steps to perform.
  For a finite dataset, training is reiterated through the dataset (if all data is exhausted) until
  `max_steps` is reached. It must be set to a positive value when the training dataset does not implement
  `__len__` (e.g. a streaming dataset), since the total number of steps cannot then be inferred and is
  required to bound the training loop and configure the learning rate scheduler.

Learning Rate & Scheduler

- **learning_rate** (`float`, *optional*, defaults to 5e-5) --
  The initial learning rate for the optimizer. This is typically the peak learning rate when using a scheduler with warmup.
- **lr_scheduler_type** (`str` or [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType), *optional*, defaults to `"linear"`) --
  The learning rate scheduler type to use. See [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType) for all possible values. Common choices:
  - "linear" = [get_linear_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup)
  - "cosine" = [get_cosine_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_cosine_schedule_with_warmup)
  - "constant" =  [get_constant_schedule()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_constant_schedule)
  - "constant_with_warmup" = [get_constant_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_constant_schedule_with_warmup)
- **lr_scheduler_kwargs** (`dict` or `str`, *optional*, defaults to `None`) --
  The extra arguments for the lr_scheduler. See the documentation of each scheduler for possible values.
- **warmup_steps** (`int` or `float`, *optional*, defaults to 0) --
  Number of steps for a linear warmup from 0 to `learning_rate`. Warmup helps stabilize training in the initial phase. Can be:
  - An integer: exact number of warmup steps
  - A float in range [0, 1): interpreted as ratio of total training steps

Optimizer

- **optim** (`str` or `training_args.OptimizerNames`, *optional*, defaults to `"adamw_torch"` (for torch>=2.8 `"adamw_torch_fused"`)) --
  The optimizer to use. Common options:
  - `"adamw_torch"`: PyTorch's AdamW (recommended default)
  - `"adamw_torch_fused"`: Fused AdamW kernel
  - `"adamw_hf"`: HuggingFace's AdamW implementation
  - `"sgd"`: Stochastic Gradient Descent with momentum
  - `"adafactor"`: Memory-efficient optimizer for large models
  - `"adamw_8bit"`: 8-bit AdamW (requires bitsandbytes)
  See `OptimizerNames` for the complete list.
- **optim_args** (`str`, *optional*) --
  Optional arguments that are supplied to optimizers such as AnyPrecisionAdamW, AdEMAMix, and GaLore.
- **weight_decay** (`float`, *optional*, defaults to 0) --
  Weight decay coefficient applied by the optimizer (not the loss function). Adds L2
  regularization to prevent overfitting by penalizing large weights. Automatically
  excluded from bias and LayerNorm parameters. Typical values: 0.01 (standard), 0.1
  (stronger regularization), 0.0 (no regularization).
- **adam_beta1** (`float`, *optional*, defaults to 0.9) --
  The exponential decay rate for the first moment estimates (momentum) in Adam-based
  optimizers. Controls how much history of gradients to retain.
- **adam_beta2** (`float`, *optional*, defaults to 0.999) --
  The exponential decay rate for the second moment estimates (variance) in Adam-based
  optimizers. Controls adaptive learning rate scaling.
- **adam_epsilon** (`float`, *optional*, defaults to 1e-8) --
  Epsilon value for numerical stability in Adam-based optimizers. Prevents division by
  zero in the denominator of the update rule.
- **optim_target_modules** (`Union[str, list[str]]`, *optional*) --
  The target modules to optimize, i.e. the module names that you would like to train.
  Currently used for the [GaLore algorithm](https://huggingface.co/papers/2403.03507) and [APOLLO algorithm](https://huggingface.co/papers/2412.05270).
  See [GaLore implementation](https://github.com/jiaweizzhao/GaLore) and [APOLLO implementation](https://github.com/zhuhanqing/APOLLO) for more details.
  You need to make sure to pass a valid GaLore or APOLLO optimizer, e.g., one of: "apollo_adamw", "galore_adamw", "galore_adamw_8bit", "galore_adafactor" and make sure that the target modules are `nn.Linear` modules only.

Regularization & Training Stability

- **gradient_accumulation_steps** (`int`, *optional*, defaults to 1) --
  Number of update steps to accumulate gradients before performing a backward/update pass.
  Simulates larger batch sizes without additional memory. Effective batch size =
  `per_device_train_batch_size × num_devices × gradient_accumulation_steps`.
  > [!TIP]
  > When using gradient accumulation, one "step" is counted as one step with a backward pass. Therefore, logging, evaluation, and saving will occur every `gradient_accumulation_steps × xxx_step` training examples.
- **average_tokens_across_devices** (`bool`, *optional*, defaults to `True`) --
  Whether or not to average tokens across devices. If enabled, will use all_reduce to synchronize
  num_tokens_in_batch for precise loss calculation. Reference:
  https://github.com/huggingface/transformers/issues/34242
- **max_grad_norm** (`float`, *optional*, defaults to 1.0) --
  Maximum gradient norm for gradient clipping. Applied after backward pass, before
  optimizer step. Prevents gradient explosion by scaling down gradients when their global
  norm exceeds this threshold. Set to 0 to disable clipping. Typical values:
  1.0 (standard), 0.5 (more conservative), 5.0 (less aggressive).
- **label_smoothing_factor** (`float`, *optional*, defaults to 0.0) --
  Label smoothing factor to prevent overconfidence. Replaces hard 0/1 targets with soft
  targets: 0 becomes `ε/num_labels` and 1 becomes `1 - ε + ε/num_labels`, where
  ε = `label_smoothing_factor`. Zero means no smoothing. Typical range: 0.0 to 0.1.

Mixed Precision Training

- **bf16** (`bool`, *optional*, defaults to `False`) --
  Enable bfloat16 (BF16) mixed precision training
  Generally preferred over FP16 due to better numerical stability and no loss scaling required.
- **fp16** (`bool`, *optional*, defaults to `False`) --
  Enable float16 (FP16) mixed precision training.
  Consider using BF16 instead if your hardware supports it.
- **bf16_full_eval** (`bool`, *optional*, defaults to `False`) --
  Use full BF16 precision for evaluation (not just mixed precision). Faster and saves
  memory but may affect metric values slightly. Only applies during evaluation.
- **fp16_full_eval** (`bool`, *optional*, defaults to `False`) --
  Use full FP16 precision for evaluation (not just mixed precision). Faster and saves
  memory but may affect metric values slightly. Only applies during evaluation.
- **tf32** (`bool`, *optional*) --
  Enable TensorFloat-32 (TF32) mode on Ampere and newer GPUs. TF32 uses 19-bit precision
  for matrix multiplications (instead of FP32's 23-bit), providing up to 8x speedup with
  negligible accuracy loss. Default depends on PyTorch version. See
  [TF32 docs](https://huggingface.co/docs/transformers/perf_train_gpu_one#tf32).

Gradient Checkpointing

- **gradient_checkpointing** (`bool`, *optional*, defaults to `False`) --
  Enable gradient checkpointing to trade compute for memory. Reduces memory usage by
  clearing activations during forward pass and recomputing them during backward pass.
  Enables training larger models or batch sizes at the cost of ~20% slower training.
- **gradient_checkpointing_kwargs** (`dict`, *optional*, defaults to `None`) --
  Keyword arguments passed to `gradient_checkpointing_enable()`.

Compilation

- **torch_compile** (`bool`, *optional*, defaults to `False`) --
  Compile the model using PyTorch 2.0's `torch.compile()` for faster training. Can provide
  20-50% speedup with no code changes. Uses default compilation settings unless
  `torch_compile_backend` or `torch_compile_mode` are specified.
- **torch_compile_backend** (`str`, *optional*) --
  Backend for `torch.compile()`. If set, automatically enables `torch_compile`. Options
  include `"inductor"` (default), `"aot_eager"`, `"cudagraphs"`. Backends vary by PyTorch
  version - see PyTorch docs for available options.
- **torch_compile_mode** (`str`, *optional*) --
  Compilation mode for `torch.compile()`. If set, automatically enables `torch_compile`.
  Options: `"default"`, `"reduce-overhead"` (minimize Python overhead), `"max-autotune"`
  (aggressive optimization, slower compile time).

Kernels

- **use_liger_kernel** (`bool`, *optional*, defaults to `False`) --
  Enable [Liger Kernel](https://github.com/linkedin/Liger-Kernel) optimizations. Increases
  multi-GPU throughput by ~20% and reduces memory usage by ~60%. Works with Flash Attention,
  FSDP, and DeepSpeed. Currently supports Llama, Mistral, Mixtral, and Gemma models.
- **liger_kernel_config** (`Optional[dict]`, *optional*) --
  Configuration for Liger Kernel. Passed as kwargs to `_apply_liger_kernel_to_instance()`.
  Options typically include: `"rope"`, `"swiglu"`, `"cross_entropy"`,
  `"fused_linear_cross_entropy"`, `"rms_norm"`. If `None`, uses default configuration.

Additional Optimizations

- **use_cache** (`bool`, *optional*, defaults to `False`) --
  Whether or not to enable cache for the model. For training, this is usually not needed apart from some PEFT methods that uses `past_key_values`.
- **neftune_noise_alpha** (`Optional[float]`) --
  If not `None`, this will activate NEFTune noise embeddings. This can drastically improve model performance
  for instruction fine-tuning. Check out the [original paper](https://huggingface.co/papers/2310.05914) and the
  [original code](https://github.com/neelsjain/NEFTune). Support transformers `PreTrainedModel` and also
  `PeftModel` from peft. The original paper used values in the range [5.0, 15.0].
- **torch_empty_cache_steps** (`int`, *optional*) --
  Number of steps to wait before calling `torch.<device>.empty_cache()`. If left unset or set to None, cache will not be emptied.
  This can help avoid CUDA out-of-memory errors by lowering peak VRAM usage at a cost of about [10% slower performance](https://github.com/huggingface/transformers/issues/31372).
- **auto_find_batch_size** (`bool`, *optional*, defaults to `False`) --
  Whether to find a batch size that will fit into memory automatically through exponential decay, avoiding
  CUDA Out-of-Memory errors.

Logging & Monitoring Training

- **logging_strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"steps"`) --
  The logging strategy to adopt during training. Possible values are:
  - `"no"`: No logging is done during training.
  - `"epoch"`: Logging is done at the end of each epoch.
  - `"steps"`: Logging is done every `logging_steps`.
- **logging_steps** (`int` or `float`, *optional*, defaults to 500) --
  Number of update steps between two logs if `logging_strategy="steps"`. Should be an integer or a float in
  range `[0,1)`. If smaller than 1, will be interpreted as ratio of total training steps.
- **logging_first_step** (`bool`, *optional*, defaults to `False`) --
  Whether to log the first `global_step` or not.
- **log_on_each_node** (`bool`, *optional*, defaults to `True`) --
  In multinode distributed training, whether to log using `log_level` once per node, or only on the main
  node.
- **logging_nan_inf_filter** (`bool`, *optional*, defaults to `True`) --
  Filter out NaN and Inf losses when logging. If `True`, replaces NaN/Inf losses with the
  average of recent valid losses. Does not affect gradient computation, only logging.
- **include_num_input_tokens_seen** (`Optional[Union[str, bool]]`, *optional*, defaults to "no") --
  Whether to track the number of input tokens seen. Must be one of ["all", "non_padding", "no"] or a boolean value which map to "all" or "no".
  May be slower in distributed training as gather operations must be called.

Logging

- **log_level** (`str`, *optional*, defaults to `passive`) --
  Logging level for the main process. Options: `"debug"`, `"info"`, `"warning"`, `"error"`,
  `"critical"`, or `"passive"` (doesn't change the current Transformers logging level,
  which defaults to `"warning"`)
- **log_level_replica** (`str`, *optional*, defaults to `"warning"`) --
  Logging level for replica processes in distributed training. Same options as `log_level`.
- **disable_tqdm** (`bool`, *optional*) --
  Disable tqdm progress bars. Defaults to `True` if `log_level` is warning or lower, `False` otherwise.

Experiment Tracking Integration

- **report_to** (`str` or `list[str]`, *optional*, defaults to `"none"`) --
  The list of integrations to report the results and logs to. Supported platforms are `"azure_ml"`,
  `"clearml"`, `"codecarbon"`, `"comet_ml"`, `"dagshub"`, `"dvclive"`, `"flyte"`, `"mlflow"`, `"swanlab"`,
  `"tensorboard"`, `"trackio"` and `"wandb"`. Use `"all"` to report to all integrations installed, `"none"`
  for no integrations.
- **run_name** (`str`, *optional*) --
  A descriptor for the run. Typically used for [trackio](https://github.com/gradio-app/trackio),
  [wandb](https://www.wandb.com/), [mlflow](https://www.mlflow.org/), [comet](https://www.comet.com/site) and
  [swanlab](https://swanlab.cn) logging.
- **project** (`str`, *optional*, defaults to `"huggingface"`) --
  The name of the project to use for logging. Currently, only used by Trackio.
- **trackio_space_id** (`str` or `None`, *optional*, defaults to `None`) --
  The Hugging Face Space ID to use for live Trackio logging with a Gradio-based Space. Should be a full
  Space name like `'username/reponame'` or `'orgname/reponame'`, or just `'reponame'` (the Space is created in
  the currently logged-in user's namespace). If `None`, metrics are logged only to a **local** directory (no
  Space on the Hub). That Gradio Space has **read and write** access to the Trackio **Bucket**, which is what you want while
  training is **in progress**—for example when **resuming** a partial run or **aggregating logs** from multiple
  machines. The Space will be **public** unless you set `hub_private_repo=True` or your organization's default is to
  create private Spaces.
- **trackio_bucket_id** (`str` or `None`, *optional*, defaults to `None`) --
  Optional Hugging Face Bucket id for Trackio. If unset, Trackio derives one. Used together with a Gradio Space
  (`trackio_space_id`) and when deploying a static Space (`trackio_static_space_id` is not `False`).
- **trackio_static_space_id** (`str`, `False`, or `None`, *optional*, defaults to `None`) --
  The Hugging Face Space ID to use for static Space created after training is complete. Should be a full
  Space name like `'username/reponame'` or `'orgname/reponame'`, or just `'reponame'` (the Space is created in
  the currently logged-in user's namespace). If False, no static Space will be created. If None, and model is pushed to the Hub,
  a static Space will be created with a default name and this will be linked from the model card. The Space will be public
  unless you set `hub_private_repo=True` or your organization's default is to create private Spaces.

Evaluation

- **eval_strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"no"`) --
  When to run evaluation. Options:
  - `"no"`: No evaluation during training
  - `"steps"`: Evaluate every `eval_steps`
  - `"epoch"`: Evaluate at the end of each epoch
- **eval_steps** (`int` or `float`, *optional*) --
  Number of update steps between two evaluations if `eval_strategy="steps"`. Will default to the same
  value as `logging_steps` if not set. Should be an integer or a float in range `[0,1)`. If smaller than 1,
  will be interpreted as ratio of total training steps.
- **eval_delay** (`float`, *optional*) --
  Number of epochs or steps to wait for before the first evaluation can be performed, depending on the
  eval_strategy.
- **per_device_eval_batch_size** (`int`, *optional*, defaults to 8) --
  The batch size per device accelerator core/CPU for evaluation.
- **prediction_loss_only** (`bool`, *optional*, defaults to `False`) --
  When performing evaluation and generating predictions, only returns the loss.
- **eval_on_start** (`bool`, *optional*, defaults to `False`) --
  Whether to perform a evaluation step (sanity check) before the training to ensure the validation steps works correctly.
- **eval_do_concat_batches** (`bool`, *optional*, defaults to `True`) --
  Whether to recursively concat inputs/losses/labels/predictions across batches. If `False`,
  will instead store them as lists, with each batch kept separate.
- **eval_use_gather_object** (`bool`, *optional*, defaults to `False`) --
  Whether to run recursively gather object in a nested list/tuple/dictionary of objects from all devices. This should only be enabled if users are not just returning tensors, and this is actively discouraged by PyTorch.
  This is useful when the labels structure is non standard, like in computer vision tasks.
- **eval_accumulation_steps** (`int`, *optional*) --
  Number of predictions steps to accumulate the output tensors for, before moving the results to the CPU. If
  left unset, the whole predictions are accumulated on the device accelerator before being moved to the CPU (faster but
  requires more memory).

Metrics Computation

- **include_for_metrics** (`list[str]`, *optional*, defaults to `[]`) --
  Include additional data in the `compute_metrics` function if needed for metrics computation.
  Possible options to add to `include_for_metrics` list:
  - `"inputs"`: Input data passed to the model, intended for calculating input dependent metrics.
  - `"loss"`: Loss values computed during evaluation, intended for calculating loss dependent metrics.
- **batch_eval_metrics** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, evaluation will call compute_metrics at the end of each batch to accumulate statistics
  rather than saving all eval logits in memory. When set to `True`, you must pass a compute_metrics function
  that takes a boolean argument `compute_result`, which when passed `True`, will trigger the final global
  summary statistics from the batch-level summary statistics you've accumulated over the evaluation set.

Checkpointing & Saving

- **save_only_model** (`bool`, *optional*, defaults to `False`) --
  Save only model weights, not optimizer/scheduler/RNG state. Significantly reduces
  checkpoint size but prevents resuming training from the checkpoint. Use when you only
  need the trained model for inference, not continued training.
  You can only load the model using `from_pretrained` with this option set to `True`.
- **save_strategy** (`str` or `SaveStrategy`, *optional*, defaults to `"steps"`) --
  The checkpoint save strategy to adopt during training. Possible values are:
  - `"no"`: No save is done during training.
  - `"epoch"`: Save is done at the end of each epoch.
  - `"steps"`: Save is done every `save_steps`.
  - `"best"`: Save is done whenever a new `best_metric` is achieved.
- **save_steps** (`int` or `float`, *optional*, defaults to 500) --
  Number of updates steps before two checkpoint saves if `save_strategy="steps"`. Should be an integer or a
  float in range `[0,1)`. If smaller than 1, will be interpreted as ratio of total training steps.
- **save_on_each_node** (`bool`, *optional*, defaults to `False`) --
  When doing multi-node distributed training, whether to save models and checkpoints on each node, or only on
  the main one.
  This should not be activated when the different nodes use the same storage as the files will be saved with
  the same names for each node.
- **save_total_limit** (`int`, *optional*) --
  Maximum number of checkpoints to keep. Deletes older checkpoints in `output_dir`. When
  `load_best_model_at_end=True`, the best checkpoint is always retained plus the most
  recent ones. For example, `save_total_limit=5` keeps the 4 most recent plus the best
- **enable_jit_checkpoint** (`bool`, *optional*, defaults to `False`) --
  Enable Just-In-Time checkpointing on SIGTERM signal for graceful termination on
  preemptible workloads. **Important**: Configure your orchestrator's graceful shutdown
  period to allow sufficient time. For Kubernetes, set `terminationGracePeriodSeconds`
  (default 30s is usually insufficient). For Slurm, use `--signal=TERM@<seconds>`.
  Required grace period ≥ longest iteration time + checkpoint save time.

Hugging Face Hub Integration

- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push the model to the Hub every time the model is saved. If this is activated,
  `output_dir` will begin a git directory synced with the repo (determined by `hub_model_id`) and the content
  will be pushed each time a save is triggered (depending on your `save_strategy`). Calling
  [save_model()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.save_model) will also trigger a push.
- **hub_token** (`str`, *optional*) --
  The token to use to push the model to the Hub. Will default to the token in the cache folder obtained with
  `hf auth login`.
- **hub_private_repo** (`bool`, *optional*) --
  Whether to make the repo private. If `None` (default), the repo will be public unless the organization's
  default is private. This value is ignored if the repo already exists. If reporting to Trackio with
  deployment to Hugging Face Spaces enabled, the same logic determines whether the Space is private.
- **hub_model_id** (`str`, *optional*) --
  The name of the repository to keep in sync with the local *output_dir*. It can be a simple model ID in
  which case the model will be pushed in your namespace. Otherwise it should be the whole repository name,
  for instance `"user_name/model"`, which allows you to push to an organization you are a member of with
  `"organization_name/model"`. Will default to `user_name/output_dir_name` with *output_dir_name* being the
  name of `output_dir`.
- **hub_strategy** (`str` or `HubStrategy`, *optional*, defaults to `"every_save"`) --
  Defines what and when to push to Hub. Options:
  - `"end"`: Push only at the end of training
  - `"every_save"`: Push on each save (async to not block training)
  - `"checkpoint"`: Like `"every_save"` plus push latest checkpoint to `"last-checkpoint"` subfolder for easy resuming
  - `"all_checkpoints"`: Push all checkpoints as they appear
- **hub_always_push** (`bool`, *optional*, defaults to `False`) --
  Unless this is `True`, the `Trainer` will skip pushing a checkpoint when the previous push is not finished.
- **hub_revision** (`str`, *optional*) --
  The revision to use when pushing to the Hub. Can be a branch name, a tag, or a commit hash.

Best Model Tracking

- **load_best_model_at_end** (`bool`, *optional*, defaults to `False`) --
  Load the best checkpoint at the end of training. Requires `eval_strategy` to be set.
  When enabled, the best checkpoint is always saved (see `save_total_limit`).
  
  When `True`, `save_strategy` must match `eval_strategy` (unless `save_strategy` is `"best"`), and if using `"steps"`,
  `save_steps` must be a multiple of `eval_steps`.
  
- **metric_for_best_model** (`str`, *optional*) --
  Metric to use for comparing models when `load_best_model_at_end=True`. Must be a metric
  name returned by evaluation, with or without the `"eval_"` prefix. Defaults to `"loss"`.
  If you set this, `greater_is_better` will default to `True` unless the name ends with
  `"loss"`. Examples: `"accuracy"`, `"f1"`, `"eval_bleu"`.
- **greater_is_better** (`bool`, *optional*) --
  Whether higher metric values are better. Defaults based on `metric_for_best_model`:
  `True` if the metric name doesn't end in `"loss"`, `False` otherwise.

Resuming Training

- **ignore_data_skip** (`bool`, *optional*, defaults to `False`) --
  When resuming training, skip fast-forwarding through the dataset to reach the previous
  state. If `True`, training starts from the beginning of the dataset (faster resume but
  results won't match interrupted training). If `False`, skips seen data (slower resume
  but exact continuation).
- **restore_callback_states_from_checkpoint** (`bool`, *optional*, defaults to `False`) --
  Restore callback states from checkpoint when resuming. If `True`, will override callbacks
  passed to Trainer if they exist in the checkpoint.

Reproducibility

- **full_determinism** (`bool`, *optional*, defaults to `False`) --
  If `True`, [enable_full_determinism()](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.enable_full_determinism) is called instead of [set_seed()](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.set_seed) to ensure reproducible results in
  distributed training. Important: this will negatively impact the performance, so only use it for debugging.
- **seed** (`int`, *optional*, defaults to 42) --
  Random seed that will be set at the beginning of training. To ensure reproducibility across runs, use the
  `~Trainer.model_init` function to instantiate the model if it has some randomly initialized parameters.
- **data_seed** (`int`, *optional*) --
  Random seed to be used with data samplers. If not set, random generators for data sampling will use the
  same seed as `seed`. This can be used to ensure reproducibility of data sampling, independent of the model
  seed.

Hardware Configuration

- **use_cpu** (`bool`, *optional*, defaults to `False`) --
  Whether or not to use cpu. If set to False, we will use the available torch device/backend.

Accelerate Configuration

- **accelerator_config** (`str`, `dict`, or `AcceleratorConfig`, *optional*) --
  Configuration for the internal Accelerate integration. Can be:
  - Path to JSON config file: `"accelerator_config.json"`
  - Dictionary with config options
  - `AcceleratorConfig` instance
  Key options:
  - `split_batches` (`bool`, defaults to `False`): Whether to split batches across devices.
    If `True`, actual batch size is the same on all devices (total must be divisible by
    num_processes). If `False`, each device gets the specified batch size.
  - `dispatch_batches` (`bool`): If `True`, only main process iterates through dataloader
    and dispatches batches to devices. Defaults to `True` for `IterableDataset`, `False`
    otherwise.
  - `even_batches` (`bool`, defaults to `True`): Duplicate samples from dataset start to
    ensure all workers get equal batch sizes.
  - `use_seedable_sampler` (`bool`, defaults to `True`): Use fully seedable random sampler
    for reproducibility.
  - `use_configured_state` (`bool`, defaults to `False`): Use pre-initialized
    `AcceleratorState`/`PartialState` instead of creating new one. May cause issues with
    hyperparameter tuning.

- **parallelism_config** (`ParallelismConfig`, *optional*) --
  Parallelism configuration for the training run. Requires Accelerate `1.10.1`

Dataloader

- **dataloader_drop_last** (`bool`, *optional*, defaults to `False`) --
  Whether to drop the last incomplete batch (if the length of the dataset is not divisible by the batch size)
  or not.
- **dataloader_num_workers** (`int`, *optional*, defaults to 0) --
  Number of subprocesses to use for data loading (PyTorch only). 0 means that the data will be loaded in the
  main process.
- **dataloader_pin_memory** (`bool`, *optional*, defaults to `True`) --
  Whether you want to pin memory in data loaders or not. Will default to `True`.
- **dataloader_persistent_workers** (`bool`, *optional*, defaults to `False`) --
  If True, the data loader will not shut down the worker processes after a dataset has been consumed once.
  This allows to maintain the workers Dataset instances alive. Can potentially speed up training, but will
  increase RAM usage. Will default to `False`.
- **dataloader_prefetch_factor** (`int`, *optional*) --
  Number of batches loaded in advance by each worker.
  2 means there will be a total of 2 * num_workers batches prefetched across all workers.
- **remove_unused_columns** (`bool`, *optional*, defaults to `True`) --
  Whether or not to automatically remove the columns unused by the model forward method.
- **label_names** (`list[str]`, *optional*) --
  The list of keys in your dictionary of inputs that correspond to the labels.
  Will eventually default to the list of argument names accepted by the model that contain the word "label",
  except if the model used is one of the `XxxForQuestionAnswering` in which case it will also include the
  `["start_positions", "end_positions"]` keys.
  You should only specify `label_names` if you're using custom label names or if your model's `forward` consumes multiple label tensors (e.g., extractive QA).
- **train_sampling_strategy** (`str`, *optional*, defaults to `"random"`) --
  The sampler to use for the training dataloader. Possible values are:

  - `"random"`: Uses `RandomSampler` (default).
  - `"sequential"`: Uses `SequentialSampler`.
  - `"group_by_length"`: Uses `LengthGroupedSampler` to group samples of roughly the same length
    together (to minimize padding and be more efficient).

  Note: When using an `IterableDataset`, this argument is ignored.
- **length_column_name** (`str`, *optional*, defaults to `"length"`) --
  Column name for precomputed lengths. If the column exists, grouping by length will use these values rather
  than computing them on train startup. Ignored unless `train_sampling_strategy` is `"group_by_length"` and the dataset
  is an instance of `Dataset`.

DDP (DistributedDataParallel)

- **ddp_find_unused_parameters** (`bool`, *optional*) --
  When using distributed training, the value of the flag `find_unused_parameters` passed to
  `DistributedDataParallel`. Will default to `False` if gradient checkpointing is used, `True` otherwise.
- **ddp_bucket_cap_mb** (`int`, *optional*) --
  When using distributed training, the value of the flag `bucket_cap_mb` passed to `DistributedDataParallel`.
- **ddp_broadcast_buffers** (`bool`, *optional*) --
  When using distributed training, the value of the flag `broadcast_buffers` passed to
  `DistributedDataParallel`. Will default to `False` if gradient checkpointing is used, `True` otherwise.
- **ddp_static_graph** (`bool`, *optional*) --
  When using distributed training, the value of the flag `static_graph` passed to
  `DistributedDataParallel`.
- **ddp_backend** (`str`, *optional*) --
  The backend to use for distributed training. Must be one of `"nccl"`, `"mpi"`, `"xccl"`, `"gloo"`, `"hccl"`.
- **ddp_timeout** (`int`, *optional*, defaults to 1800) --
  The timeout for `torch.distributed.init_process_group` calls, used to avoid GPU socket timeouts when
  performing slow operations in distributed runnings. Please refer to the [PyTorch documentation](https://pytorch.org/docs/stable/distributed.html#torch.distributed.init_process_group) for more
  information.

FSDP (Fully Sharded Data Parallel)

- **fsdp** (`bool`, *optional*, defaults to `None`) --
  Enable PyTorch Fully Sharded Data Parallel (FSDP) for distributed training. Pass `True` to enable FSDP.
- **fsdp_config** (`str` or `dict`, *optional*) --
  Configuration settings for when `fsdp` is enabled. Pass a path to a JSON config file, such
  as `fsdp_config.json`, or an already-loaded dict.

  Supported keys:
  - version (`int`, *optional*, defaults to `2`):
    The version of FSDP to use (`2` for FSDP2, `1` for the legacy FSDP1).
  - reshard_after_forward (`bool`, *optional*, defaults to `True`):
    Whether to reshard parameters after the forward pass. Set to `False` to keep parameters
    gathered between the forward and backward passes, avoids the re-all-gather, and use higher peak memory.
  - cpu_offload (`bool`, *optional*, defaults to `False`):
    Offload parameters and gradients to CPU when not in use to save GPU memory.
  - activation_checkpointing (`bool`, *optional*, defaults to `False`):
    Set to `True` to reduce memory by recomputing activations during the backward pass. Prefer
    `activation_checkpointing` over `gradient_checkpointing` when using FSDP. `gradient_checkpointing`
    introduces a redundant all-gather in the backward pass.
  - cpu_ram_efficient_loading (`bool`, *optional*, defaults to `False`):
    Set to `True` to load the pretrained checkpoint on the first process only. Other processes start
    with empty weights and receive the weights by broadcast.
  - state_dict_type (`str`, *optional*, defaults to `"FULL_STATE_DICT"`):
    Checkpoint format: `"FULL_STATE_DICT"` (single HF-compatible file) or
    `"SHARDED_STATE_DICT"` (one file per rank, faster for large models).
  - auto_wrap_policy (`str`, *optional*, defaults to `"TRANSFORMER_BASED_WRAP"`):
    Auto-wrap policy to use. Choose `"TRANSFORMER_BASED_WRAP"`, `"SIZE_BASED_WRAP"`, or `"NO_WRAP"`.
  - transformer_layer_cls_to_wrap (`list[str]`, *optional*):
    Transformer layer class names (case-sensitive) to wrap, e.g. `LlamaDecoderLayer`. Usually
    unnecessary: the wrap policy falls back to the model's `_no_split_modules`, which covers
    most transformers models.
  - min_num_params (`int`, *optional*, defaults to `0`):
    Minimum number of parameters per module for size-based auto-wrapping (used with
    `auto_wrap_policy="SIZE_BASED_WRAP"`).
  - xla (`bool`, *optional*, defaults to `False`):
    Whether to use PyTorch/XLA Fully Sharded Data Parallel Training. Experimental.
  - xla_fsdp_settings (`dict`, *optional*):
    Dictionary of XLA FSDP wrapping parameters. For a complete list of options, see the
    [XLA FSDP source](https://github.com/pytorch/xla/blob/master/torch_xla/distributed/fsdp/xla_fully_sharded_data_parallel.py).
  - xla_fsdp_grad_ckpt (`bool`, *optional*, defaults to `False`):
    Set to `True` to use gradient checkpointing over each nested XLA FSDP wrapped layer. Requires
    `xla=True` and an auto-wrapping policy (`min_num_params` or `transformer_layer_cls_to_wrap`).

DeepSpeed

- **deepspeed** (`str` or `dict`, *optional*) --
  Enable [DeepSpeed](https://github.com/deepspeedai/DeepSpeed) integration. Value is either:
  - Path to DeepSpeed JSON config file: `"ds_config.json"`
  - Loaded config as dictionary
  > [!TIP]
  > If using ZeRO initialization, instantiate your model *after* initializing
  `TrainingArguments`, otherwise ZeRO won't be applied.

Debugging & Profiling (Experimental)

- **debug** (`str` or list of `DebugOption`, *optional*, defaults to `""`) --
  Enable one or more debug features. This is an experimental feature.
  Possible options are:
  - "underflow_overflow": detects overflow in model's input/outputs and reports the last frames that led to
    the event
  - "tpu_metrics_debug": print debug metrics on TPU
- **skip_memory_metrics** (`bool`, *optional*, defaults to `True`) --
  Whether to skip adding of memory profiler reports to metrics. This is skipped by default because it slows
  down the training and evaluation speed.

External Script Flags (not used by Trainer)

- **do_train** (`bool`, *optional*, defaults to `False`) --
  Whether to run training or not. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used
  by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **do_eval** (`bool`, *optional*) --
  Whether to run evaluation on the validation set or not. Will be set to `True` if `eval_strategy` is
  different from `"no"`. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used by your
  training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **do_predict** (`bool`, *optional*, defaults to `False`) --
  Whether to run predictions on the test set or not. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's
  intended to be used by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **resume_from_checkpoint** (`str`, *optional*) --
  The path to a folder with a valid checkpoint for your model. This argument is not directly used by
  [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.

Configuration class for controlling all aspects of model training with the Trainer.
TrainingArguments centralizes all hyperparameters, optimization settings, logging preferences, and infrastructure choices needed for training.

[HfArgumentParser](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.HfArgumentParser) can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

Returns the log level to be used depending on whether this process is the main process of node 0, main process
of node non-0, or a non-main process.

For the main process the log level defaults to the logging level set (`logging.WARNING` if you didn't do
anything) unless overridden by `log_level` argument.

For the replica processes the log level defaults to `logging.WARNING` unless overridden by `log_level_replica`
argument.

The choice between the main and replica process settings is made according to the return value of `should_log`.

Get number of steps used for a linear warmup.

- **local** (`bool`, *optional*, defaults to `True`) --
  if `True` first means process of rank 0 of each node if `False` first means process of rank 0 of node
  rank 0 In multi-node environment with a shared filesystem you most likely will want to use
  `local=False` so that only the main process of the first node will do the processing. If however, the
  filesystem is not shared, then the main process of each node will need to do the processing, which is
  the default behavior.
- **desc** (`str`, *optional*, defaults to `"work"`) --
  a work description to be used in debug logs

A context manager for torch distributed environment where on needs to do something on the main process, while
blocking replicas, and when it's finished releasing the replicas.

One such use is for `datasets`'s `map` feature which to be efficient should be run once on the main process,
which upon completion saves a cached version of results and which then automatically gets loaded by the
replicas.

- **drop_last** (`bool`, *optional*, defaults to `False`) --
  Whether to drop the last incomplete batch (if the length of the dataset is not divisible by the batch
  size) or not.
- **num_workers** (`int`, *optional*, defaults to 0) --
  Number of subprocesses to use for data loading (PyTorch only). 0 means that the data will be loaded in
  the main process.
- **pin_memory** (`bool`, *optional*, defaults to `True`) --
  Whether you want to pin memory in data loaders or not. Will default to `True`.
- **persistent_workers** (`bool`, *optional*, defaults to `False`) --
  If True, the data loader will not shut down the worker processes after a dataset has been consumed
  once. This allows to maintain the workers Dataset instances alive. Can potentially speed up training,
  but will increase RAM usage. Will default to `False`.
- **prefetch_factor** (`int`, *optional*) --
  Number of batches loaded in advance by each worker.
  2 means there will be a total of 2 * num_workers batches prefetched across all workers.
- **auto_find_batch_size** (`bool`, *optional*, defaults to `False`) --
  Whether to find a batch size that will fit into memory automatically through exponential decay,
  avoiding CUDA Out-of-Memory errors. Requires accelerate to be installed (`pip install accelerate`)
- **ignore_data_skip** (`bool`, *optional*, defaults to `False`) --
  When resuming training, whether or not to skip the epochs and batches to get the data loading at the
  same stage as in the previous training. If set to `True`, the training will begin faster (as that
  skipping step can take a long time) but will not yield the same results as the interrupted training
  would have.
- **sampler_seed** (`int`, *optional*) --
  Random seed to be used with data samplers. If not set, random generators for data sampling will use the
  same seed as `self.seed`. This can be used to ensure reproducibility of data sampling, independent of
  the model seed.

A method that regroups all arguments linked to the dataloaders creation.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_dataloader(train_batch_size=16, eval_batch_size=64)
>>> args.per_device_train_batch_size
16
```

- **strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"no"`) --
  The evaluation strategy to adopt during training. Possible values are:

  - `"no"`: No evaluation is done during training.
  - `"steps"`: Evaluation is done (and logged) every `steps`.
  - `"epoch"`: Evaluation is done at the end of each epoch.

  Setting a `strategy` different from `"no"` will set `self.do_eval` to `True`.
- **steps** (`int`, *optional*, defaults to 500) --
  Number of update steps between two evaluations if `strategy="steps"`.
- **batch_size** (`int` *optional*, defaults to 8) --
  The batch size per device (GPU/TPU core/CPU...) used for evaluation.
- **accumulation_steps** (`int`, *optional*) --
  Number of predictions steps to accumulate the output tensors for, before moving the results to the CPU.
  If left unset, the whole predictions are accumulated on GPU/TPU before being moved to the CPU (faster
  but requires more memory).
- **delay** (`float`, *optional*) --
  Number of epochs or steps to wait for before the first evaluation can be performed, depending on the
  eval_strategy.
- **loss_only** (`bool`, *optional*, defaults to `False`) --
  Ignores all outputs except the loss.

A method that regroups all arguments linked to evaluation.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_evaluate(strategy="steps", steps=100)
>>> args.eval_steps
100
```

- **strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"steps"`) --
  The logging strategy to adopt during training. Possible values are:

  - `"no"`: No logging is done during training.
  - `"epoch"`: Logging is done at the end of each epoch.
  - `"steps"`: Logging is done every `logging_steps`.

- **steps** (`int`, *optional*, defaults to 500) --
  Number of update steps between two logs if `strategy="steps"`.
- **level** (`str`, *optional*, defaults to `"passive"`) --
  Logger log level to use on the main process. Possible choices are the log levels as strings: `"debug"`,
  `"info"`, `"warning"`, `"error"` and `"critical"`, plus a `"passive"` level which doesn't set anything
  and lets the application set the level.
- **report_to** (`str` or `list[str]`, *optional*, defaults to `"none"`) --
  The list of integrations to report the results and logs to. Supported platforms are `"azure_ml"`,
  `"clearml"`, `"codecarbon"`, `"comet_ml"`, `"dagshub"`, `"dvclive"`, `"flyte"`, `"mlflow"`,
  `"swanlab"`, `"tensorboard"`, `"trackio"` and `"wandb"`. Use `"all"` to report to all integrations
  installed, `"none"` for no integrations.
- **first_step** (`bool`, *optional*, defaults to `False`) --
  Whether to log and evaluate the first `global_step` or not.
- **nan_inf_filter** (`bool`, *optional*, defaults to `True`) --
  Whether to filter `nan` and `inf` losses for logging. If set to `True` the loss of every step that is
  `nan` or `inf` is filtered and the average loss of the current logging window is taken instead.

  

  `nan_inf_filter` only influences the logging of loss values, it does not change the behavior the
  gradient is computed or applied to the model.

  

- **on_each_node** (`bool`, *optional*, defaults to `True`) --
  In multinode distributed training, whether to log using `log_level` once per node, or only on the main
  node.
- **replica_level** (`str`, *optional*, defaults to `"passive"`) --
  Logger log level to use on replicas. Same choices as `log_level`

A method that regroups all arguments linked to logging.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_logging(strategy="steps", steps=100)
>>> args.logging_steps
100
```

- **name** (`str` or [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType), *optional*, defaults to `"linear"`) --
  The scheduler type to use. See the documentation of [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType) for all possible values.
- **num_epochs(`float`,** *optional*, defaults to 3.0) --
  Total number of training epochs to perform (if not an integer, will perform the decimal part percents
  of the last epoch before stopping training).
- **max_steps** (`int`, *optional*, defaults to -1) --
  If set to a positive number, the total number of training steps to perform. Overrides `num_train_epochs`.
  For a finite dataset, training is reiterated through the dataset (if all data is exhausted) until
  `max_steps` is reached. It must be set to a positive value when the training dataset does not
  implement `__len__` (e.g. a streaming dataset), since the total number of steps cannot then be
  inferred and is required to bound the training loop and configure the learning rate scheduler.
- **warmup_steps** (`float`, *optional*, defaults to 0) --
  Number of steps used for a linear warmup from 0 to `learning_rate`.  Should be an integer or a float in range `[0,1)`.
  If smaller than 1, will be interpreted as ratio of steps used for a linear warmup from 0 to `learning_rate`.

A method that regroups all arguments linked to the learning rate scheduler and its hyperparameters.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_lr_scheduler(name="cosine", warmup_steps=0.05)
>>> args.warmup_steps
0.05
```

- **name** (`str` or `training_args.OptimizerNames`, *optional*, defaults to `"adamw_torch"`) --
  The optimizer to use: `"adamw_torch"`, `"adamw_torch_fused"`,
  `"adamw_anyprecision"` or `"adafactor"`.
- **learning_rate** (`float`, *optional*, defaults to 5e-5) --
  The initial learning rate.
- **weight_decay** (`float`, *optional*, defaults to 0) --
  The weight decay to apply (if not zero) to all layers except all bias and LayerNorm weights.
- **beta1** (`float`, *optional*, defaults to 0.9) --
  The beta1 hyperparameter for the adam optimizer or its variants.
- **beta2** (`float`, *optional*, defaults to 0.999) --
  The beta2 hyperparameter for the adam optimizer or its variants.
- **epsilon** (`float`, *optional*, defaults to 1e-8) --
  The epsilon hyperparameter for the adam optimizer or its variants.
- **args** (`str`, *optional*) --
  Optional arguments that are supplied to AnyPrecisionAdamW (only useful when
  `optim="adamw_anyprecision"`).

A method that regroups all arguments linked to the optimizer and its hyperparameters.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_optimizer(name="adamw_torch", beta1=0.8)
>>> args.optim
'adamw_torch'
```

- **model_id** (`str`) --
  The name of the repository to keep in sync with the local *output_dir*. It can be a simple model ID in
  which case the model will be pushed in your namespace. Otherwise it should be the whole repository
  name, for instance `"user_name/model"`, which allows you to push to an organization you are a member of
  with `"organization_name/model"`.
- **strategy** (`str` or `HubStrategy`, *optional*, defaults to `"every_save"`) --
  Defines the scope of what is pushed to the Hub and when. Possible values are:

  - `"end"`: push the model, its configuration, the processing_class e.g. tokenizer (if passed along to the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer)) and a
    draft of a model card when the [save_model()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.save_model) method is called.
  - `"every_save"`: push the model, its configuration, the processing_class e.g. tokenizer (if passed along to the [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer))
    and a draft of a model card each time there is a model save. The pushes are asynchronous to not block
    training, and in case the save are very frequent, a new push is only attempted if the previous one is
    finished. A last push is made with the final model at the end of training.
  - `"checkpoint"`: like `"every_save"` but the latest checkpoint is also pushed in a subfolder named
    last-checkpoint, allowing you to resume training easily with
    `trainer.train(resume_from_checkpoint="last-checkpoint")`.
  - `"all_checkpoints"`: like `"checkpoint"` but all checkpoints are pushed like they appear in the
    output folder (so you will get one checkpoint folder per folder in your final repository)

- **token** (`str`, *optional*) --
  The token to use to push the model to the Hub. Will default to the token in the cache folder obtained
  with `hf auth login`.
- **private_repo** (`bool`, *optional*, defaults to `False`) --
  Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.
- **always_push** (`bool`, *optional*, defaults to `False`) --
  Unless this is `True`, the `Trainer` will skip pushing a checkpoint when the previous push is not
  finished.
- **revision** (`str`, *optional*) --
  The revision to use when pushing to the Hub. Can be a branch name, a tag, or a commit hash.

A method that regroups all arguments linked to synchronizing checkpoints with the Hub.

Calling this method will set `self.push_to_hub` to `True`, which means the `output_dir` will begin a git
directory synced with the repo (determined by `model_id`) and the content will be pushed each time a save is
triggered (depending on your `self.save_strategy`). Calling [save_model()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.save_model) will also trigger a push.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_push_to_hub("me/awesome-model")
>>> args.hub_model_id
'me/awesome-model'
```

- **strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"steps"`) --
  The checkpoint save strategy to adopt during training. Possible values are:

  - `"no"`: No save is done during training.
  - `"epoch"`: Save is done at the end of each epoch.
  - `"steps"`: Save is done every `save_steps`.

- **steps** (`int`, *optional*, defaults to 500) --
  Number of updates steps before two checkpoint saves if `strategy="steps"`.
- **total_limit** (`int`, *optional*) --
  If a value is passed, will limit the total amount of checkpoints. Deletes the older checkpoints in
  `output_dir`.
- **on_each_node** (`bool`, *optional*, defaults to `False`) --
  When doing multi-node distributed training, whether to save models and checkpoints on each node, or
  only on the main one.

  This should not be activated when the different nodes use the same storage as the files will be saved
  with the same names for each node.

A method that regroups all arguments linked to checkpoint saving.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_save(strategy="steps", steps=100)
>>> args.save_steps
100
```

- **batch_size** (`int` *optional*, defaults to 8) --
  The batch size per device (GPU/TPU core/CPU...) used for testing.
- **loss_only** (`bool`, *optional*, defaults to `False`) --
  Ignores all outputs except the loss.

A method that regroups all basic arguments linked to testing on a held-out dataset.

Calling this method will automatically set `self.do_predict` to `True`.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_testing(batch_size=32)
>>> args.per_device_eval_batch_size
32
```

- **learning_rate** (`float`, *optional*, defaults to 5e-5) --
  The initial learning rate for the optimizer.
- **batch_size** (`int` *optional*, defaults to 8) --
  The batch size per device (GPU/TPU core/CPU...) used for training.
- **weight_decay** (`float`, *optional*, defaults to 0) --
  The weight decay to apply (if not zero) to all layers except all bias and LayerNorm weights in the
  optimizer.
- **num_train_epochs(`float`,** *optional*, defaults to 3.0) --
  Total number of training epochs to perform (if not an integer, will perform the decimal part percents
  of the last epoch before stopping training).
- **max_steps** (`int`, *optional*, defaults to -1) --
  If set to a positive number, the total number of training steps to perform. Overrides `num_train_epochs`.
  For a finite dataset, training is reiterated through the dataset (if all data is exhausted) until
  `max_steps` is reached. It must be set to a positive value when the training dataset does not
  implement `__len__` (e.g. a streaming dataset), since the total number of steps cannot then be
  inferred and is required to bound the training loop and configure the learning rate scheduler.
- **gradient_accumulation_steps** (`int`, *optional*, defaults to 1) --
  Number of updates steps to accumulate the gradients for, before performing a backward/update pass.

  

  When using gradient accumulation, one step is counted as one step with backward pass. Therefore,
  logging, evaluation, save will be conducted every `gradient_accumulation_steps * xxx_step` training
  examples.

  

- **seed** (`int`, *optional*, defaults to 42) --
  Random seed that will be set at the beginning of training. To ensure reproducibility across runs, use
  the `~Trainer.model_init` function to instantiate the model if it has some randomly initialized
  parameters.
- **gradient_checkpointing** (`bool`, *optional*, defaults to `False`) --
  If True, use gradient checkpointing to save memory at the expense of slower backward pass.

A method that regroups all basic arguments linked to the training.

Calling this method will automatically set `self.do_train` to `True`.

Example:

```py
>>> from transformers import TrainingArguments

>>> args = TrainingArguments("working_dir")
>>> args = args.set_training(learning_rate=1e-4, batch_size=32)
>>> args.learning_rate
1e-4
```

Serializes this instance while replace `Enum` by their values (for JSON serialization support). It obfuscates
the token values by removing their value.

Serializes this instance to a JSON string.

Sanitized serialization to use with TensorBoard's hparams

## Seq2SeqTrainingArguments[[transformers.Seq2SeqTrainingArguments]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": typing.Optional[typing.Any] = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "sortish_sampler", "val": ": bool = False"}, {"name": "predict_with_generate", "val": ": bool = False"}, {"name": "generation_max_length", "val": ": int | None = None"}, {"name": "generation_num_beams", "val": ": int | None = None"}, {"name": "generation_config", "val": ": str | pathlib.Path | transformers.generation.configuration_utils.GenerationConfig | None = None"}]}>
- **output_dir** (`str`, *optional*, defaults to `"trainer_output"`) --
  The output directory where the model predictions and checkpoints will be written.

Training Duration and Batch Size

- **per_device_train_batch_size** (`int`, *optional*, defaults to 8) --
  The batch size *per device*. The **global batch size** is computed as:
  `per_device_train_batch_size * number_of_devices` in multi-GPU or distributed setups.
- **num_train_epochs(`float`,** *optional*, defaults to 3.0) --
  Total number of training epochs to perform (if not an integer, will perform the decimal part percents of
  the last epoch before stopping training).
- **max_steps** (`int`, *optional*, defaults to -1) --
  Overrides `num_train_epochs`. If set to a positive number, the total number of training steps to perform.
  For a finite dataset, training is reiterated through the dataset (if all data is exhausted) until
  `max_steps` is reached. It must be set to a positive value when the training dataset does not implement
  `__len__` (e.g. a streaming dataset), since the total number of steps cannot then be inferred and is
  required to bound the training loop and configure the learning rate scheduler.

Learning Rate & Scheduler

- **learning_rate** (`float`, *optional*, defaults to 5e-5) --
  The initial learning rate for the optimizer. This is typically the peak learning rate when using a scheduler with warmup.
- **lr_scheduler_type** (`str` or [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType), *optional*, defaults to `"linear"`) --
  The learning rate scheduler type to use. See [SchedulerType](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.SchedulerType) for all possible values. Common choices:
  - "linear" = [get_linear_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup)
  - "cosine" = [get_cosine_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_cosine_schedule_with_warmup)
  - "constant" =  [get_constant_schedule()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_constant_schedule)
  - "constant_with_warmup" = [get_constant_schedule_with_warmup()](/docs/transformers/v5.14.0/en/main_classes/optimizer_schedules#transformers.get_constant_schedule_with_warmup)
- **lr_scheduler_kwargs** (`dict` or `str`, *optional*, defaults to `None`) --
  The extra arguments for the lr_scheduler. See the documentation of each scheduler for possible values.
- **warmup_steps** (`int` or `float`, *optional*, defaults to 0) --
  Number of steps for a linear warmup from 0 to `learning_rate`. Warmup helps stabilize training in the initial phase. Can be:
  - An integer: exact number of warmup steps
  - A float in range [0, 1): interpreted as ratio of total training steps

Optimizer

- **optim** (`str` or `training_args.OptimizerNames`, *optional*, defaults to `"adamw_torch"` (for torch>=2.8 `"adamw_torch_fused"`)) --
  The optimizer to use. Common options:
  - `"adamw_torch"`: PyTorch's AdamW (recommended default)
  - `"adamw_torch_fused"`: Fused AdamW kernel
  - `"adamw_hf"`: HuggingFace's AdamW implementation
  - `"sgd"`: Stochastic Gradient Descent with momentum
  - `"adafactor"`: Memory-efficient optimizer for large models
  - `"adamw_8bit"`: 8-bit AdamW (requires bitsandbytes)
  See `OptimizerNames` for the complete list.
- **optim_args** (`str`, *optional*) --
  Optional arguments that are supplied to optimizers such as AnyPrecisionAdamW, AdEMAMix, and GaLore.
- **weight_decay** (`float`, *optional*, defaults to 0) --
  Weight decay coefficient applied by the optimizer (not the loss function). Adds L2
  regularization to prevent overfitting by penalizing large weights. Automatically
  excluded from bias and LayerNorm parameters. Typical values: 0.01 (standard), 0.1
  (stronger regularization), 0.0 (no regularization).
- **adam_beta1** (`float`, *optional*, defaults to 0.9) --
  The exponential decay rate for the first moment estimates (momentum) in Adam-based
  optimizers. Controls how much history of gradients to retain.
- **adam_beta2** (`float`, *optional*, defaults to 0.999) --
  The exponential decay rate for the second moment estimates (variance) in Adam-based
  optimizers. Controls adaptive learning rate scaling.
- **adam_epsilon** (`float`, *optional*, defaults to 1e-8) --
  Epsilon value for numerical stability in Adam-based optimizers. Prevents division by
  zero in the denominator of the update rule.
- **optim_target_modules** (`Union[str, list[str]]`, *optional*) --
  The target modules to optimize, i.e. the module names that you would like to train.
  Currently used for the [GaLore algorithm](https://huggingface.co/papers/2403.03507) and [APOLLO algorithm](https://huggingface.co/papers/2412.05270).
  See [GaLore implementation](https://github.com/jiaweizzhao/GaLore) and [APOLLO implementation](https://github.com/zhuhanqing/APOLLO) for more details.
  You need to make sure to pass a valid GaLore or APOLLO optimizer, e.g., one of: "apollo_adamw", "galore_adamw", "galore_adamw_8bit", "galore_adafactor" and make sure that the target modules are `nn.Linear` modules only.

Regularization & Training Stability

- **gradient_accumulation_steps** (`int`, *optional*, defaults to 1) --
  Number of update steps to accumulate gradients before performing a backward/update pass.
  Simulates larger batch sizes without additional memory. Effective batch size =
  `per_device_train_batch_size × num_devices × gradient_accumulation_steps`.
  > [!TIP]
  > When using gradient accumulation, one "step" is counted as one step with a backward pass. Therefore, logging, evaluation, and saving will occur every `gradient_accumulation_steps × xxx_step` training examples.
- **average_tokens_across_devices** (`bool`, *optional*, defaults to `True`) --
  Whether or not to average tokens across devices. If enabled, will use all_reduce to synchronize
  num_tokens_in_batch for precise loss calculation. Reference:
  https://github.com/huggingface/transformers/issues/34242
- **max_grad_norm** (`float`, *optional*, defaults to 1.0) --
  Maximum gradient norm for gradient clipping. Applied after backward pass, before
  optimizer step. Prevents gradient explosion by scaling down gradients when their global
  norm exceeds this threshold. Set to 0 to disable clipping. Typical values:
  1.0 (standard), 0.5 (more conservative), 5.0 (less aggressive).
- **label_smoothing_factor** (`float`, *optional*, defaults to 0.0) --
  Label smoothing factor to prevent overconfidence. Replaces hard 0/1 targets with soft
  targets: 0 becomes `ε/num_labels` and 1 becomes `1 - ε + ε/num_labels`, where
  ε = `label_smoothing_factor`. Zero means no smoothing. Typical range: 0.0 to 0.1.

Mixed Precision Training

- **bf16** (`bool`, *optional*, defaults to `False`) --
  Enable bfloat16 (BF16) mixed precision training
  Generally preferred over FP16 due to better numerical stability and no loss scaling required.
- **fp16** (`bool`, *optional*, defaults to `False`) --
  Enable float16 (FP16) mixed precision training.
  Consider using BF16 instead if your hardware supports it.
- **bf16_full_eval** (`bool`, *optional*, defaults to `False`) --
  Use full BF16 precision for evaluation (not just mixed precision). Faster and saves
  memory but may affect metric values slightly. Only applies during evaluation.
- **fp16_full_eval** (`bool`, *optional*, defaults to `False`) --
  Use full FP16 precision for evaluation (not just mixed precision). Faster and saves
  memory but may affect metric values slightly. Only applies during evaluation.
- **tf32** (`bool`, *optional*) --
  Enable TensorFloat-32 (TF32) mode on Ampere and newer GPUs. TF32 uses 19-bit precision
  for matrix multiplications (instead of FP32's 23-bit), providing up to 8x speedup with
  negligible accuracy loss. Default depends on PyTorch version. See
  [TF32 docs](https://huggingface.co/docs/transformers/perf_train_gpu_one#tf32).

Gradient Checkpointing

- **gradient_checkpointing** (`bool`, *optional*, defaults to `False`) --
  Enable gradient checkpointing to trade compute for memory. Reduces memory usage by
  clearing activations during forward pass and recomputing them during backward pass.
  Enables training larger models or batch sizes at the cost of ~20% slower training.
- **gradient_checkpointing_kwargs** (`dict`, *optional*, defaults to `None`) --
  Keyword arguments passed to `gradient_checkpointing_enable()`.

Compilation

- **torch_compile** (`bool`, *optional*, defaults to `False`) --
  Compile the model using PyTorch 2.0's `torch.compile()` for faster training. Can provide
  20-50% speedup with no code changes. Uses default compilation settings unless
  `torch_compile_backend` or `torch_compile_mode` are specified.
- **torch_compile_backend** (`str`, *optional*) --
  Backend for `torch.compile()`. If set, automatically enables `torch_compile`. Options
  include `"inductor"` (default), `"aot_eager"`, `"cudagraphs"`. Backends vary by PyTorch
  version - see PyTorch docs for available options.
- **torch_compile_mode** (`str`, *optional*) --
  Compilation mode for `torch.compile()`. If set, automatically enables `torch_compile`.
  Options: `"default"`, `"reduce-overhead"` (minimize Python overhead), `"max-autotune"`
  (aggressive optimization, slower compile time).

Kernels

- **use_liger_kernel** (`bool`, *optional*, defaults to `False`) --
  Enable [Liger Kernel](https://github.com/linkedin/Liger-Kernel) optimizations. Increases
  multi-GPU throughput by ~20% and reduces memory usage by ~60%. Works with Flash Attention,
  FSDP, and DeepSpeed. Currently supports Llama, Mistral, Mixtral, and Gemma models.
- **liger_kernel_config** (`Optional[dict]`, *optional*) --
  Configuration for Liger Kernel. Passed as kwargs to `_apply_liger_kernel_to_instance()`.
  Options typically include: `"rope"`, `"swiglu"`, `"cross_entropy"`,
  `"fused_linear_cross_entropy"`, `"rms_norm"`. If `None`, uses default configuration.

Additional Optimizations

- **use_cache** (`bool`, *optional*, defaults to `False`) --
  Whether or not to enable cache for the model. For training, this is usually not needed apart from some PEFT methods that uses `past_key_values`.
- **neftune_noise_alpha** (`Optional[float]`) --
  If not `None`, this will activate NEFTune noise embeddings. This can drastically improve model performance
  for instruction fine-tuning. Check out the [original paper](https://huggingface.co/papers/2310.05914) and the
  [original code](https://github.com/neelsjain/NEFTune). Support transformers `PreTrainedModel` and also
  `PeftModel` from peft. The original paper used values in the range [5.0, 15.0].
- **torch_empty_cache_steps** (`int`, *optional*) --
  Number of steps to wait before calling `torch.<device>.empty_cache()`. If left unset or set to None, cache will not be emptied.
  This can help avoid CUDA out-of-memory errors by lowering peak VRAM usage at a cost of about [10% slower performance](https://github.com/huggingface/transformers/issues/31372).
- **auto_find_batch_size** (`bool`, *optional*, defaults to `False`) --
  Whether to find a batch size that will fit into memory automatically through exponential decay, avoiding
  CUDA Out-of-Memory errors.

Logging & Monitoring Training

- **logging_strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"steps"`) --
  The logging strategy to adopt during training. Possible values are:
  - `"no"`: No logging is done during training.
  - `"epoch"`: Logging is done at the end of each epoch.
  - `"steps"`: Logging is done every `logging_steps`.
- **logging_steps** (`int` or `float`, *optional*, defaults to 500) --
  Number of update steps between two logs if `logging_strategy="steps"`. Should be an integer or a float in
  range `[0,1)`. If smaller than 1, will be interpreted as ratio of total training steps.
- **logging_first_step** (`bool`, *optional*, defaults to `False`) --
  Whether to log the first `global_step` or not.
- **log_on_each_node** (`bool`, *optional*, defaults to `True`) --
  In multinode distributed training, whether to log using `log_level` once per node, or only on the main
  node.
- **logging_nan_inf_filter** (`bool`, *optional*, defaults to `True`) --
  Filter out NaN and Inf losses when logging. If `True`, replaces NaN/Inf losses with the
  average of recent valid losses. Does not affect gradient computation, only logging.
- **include_num_input_tokens_seen** (`Optional[Union[str, bool]]`, *optional*, defaults to "no") --
  Whether to track the number of input tokens seen. Must be one of ["all", "non_padding", "no"] or a boolean value which map to "all" or "no".
  May be slower in distributed training as gather operations must be called.

Logging

- **log_level** (`str`, *optional*, defaults to `passive`) --
  Logging level for the main process. Options: `"debug"`, `"info"`, `"warning"`, `"error"`,
  `"critical"`, or `"passive"` (doesn't change the current Transformers logging level,
  which defaults to `"warning"`)
- **log_level_replica** (`str`, *optional*, defaults to `"warning"`) --
  Logging level for replica processes in distributed training. Same options as `log_level`.
- **disable_tqdm** (`bool`, *optional*) --
  Disable tqdm progress bars. Defaults to `True` if `log_level` is warning or lower, `False` otherwise.

Experiment Tracking Integration

- **report_to** (`str` or `list[str]`, *optional*, defaults to `"none"`) --
  The list of integrations to report the results and logs to. Supported platforms are `"azure_ml"`,
  `"clearml"`, `"codecarbon"`, `"comet_ml"`, `"dagshub"`, `"dvclive"`, `"flyte"`, `"mlflow"`, `"swanlab"`,
  `"tensorboard"`, `"trackio"` and `"wandb"`. Use `"all"` to report to all integrations installed, `"none"`
  for no integrations.
- **run_name** (`str`, *optional*) --
  A descriptor for the run. Typically used for [trackio](https://github.com/gradio-app/trackio),
  [wandb](https://www.wandb.com/), [mlflow](https://www.mlflow.org/), [comet](https://www.comet.com/site) and
  [swanlab](https://swanlab.cn) logging.
- **project** (`str`, *optional*, defaults to `"huggingface"`) --
  The name of the project to use for logging. Currently, only used by Trackio.
- **trackio_space_id** (`str` or `None`, *optional*, defaults to `None`) --
  The Hugging Face Space ID to use for live Trackio logging with a Gradio-based Space. Should be a full
  Space name like `'username/reponame'` or `'orgname/reponame'`, or just `'reponame'` (the Space is created in
  the currently logged-in user's namespace). If `None`, metrics are logged only to a **local** directory (no
  Space on the Hub). That Gradio Space has **read and write** access to the Trackio **Bucket**, which is what you want while
  training is **in progress**—for example when **resuming** a partial run or **aggregating logs** from multiple
  machines. The Space will be **public** unless you set `hub_private_repo=True` or your organization's default is to
  create private Spaces.
- **trackio_bucket_id** (`str` or `None`, *optional*, defaults to `None`) --
  Optional Hugging Face Bucket id for Trackio. If unset, Trackio derives one. Used together with a Gradio Space
  (`trackio_space_id`) and when deploying a static Space (`trackio_static_space_id` is not `False`).
- **trackio_static_space_id** (`str`, `False`, or `None`, *optional*, defaults to `None`) --
  The Hugging Face Space ID to use for static Space created after training is complete. Should be a full
  Space name like `'username/reponame'` or `'orgname/reponame'`, or just `'reponame'` (the Space is created in
  the currently logged-in user's namespace). If False, no static Space will be created. If None, and model is pushed to the Hub,
  a static Space will be created with a default name and this will be linked from the model card. The Space will be public
  unless you set `hub_private_repo=True` or your organization's default is to create private Spaces.

Evaluation

- **eval_strategy** (`str` or [IntervalStrategy](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.IntervalStrategy), *optional*, defaults to `"no"`) --
  When to run evaluation. Options:
  - `"no"`: No evaluation during training
  - `"steps"`: Evaluate every `eval_steps`
  - `"epoch"`: Evaluate at the end of each epoch
- **eval_steps** (`int` or `float`, *optional*) --
  Number of update steps between two evaluations if `eval_strategy="steps"`. Will default to the same
  value as `logging_steps` if not set. Should be an integer or a float in range `[0,1)`. If smaller than 1,
  will be interpreted as ratio of total training steps.
- **eval_delay** (`float`, *optional*) --
  Number of epochs or steps to wait for before the first evaluation can be performed, depending on the
  eval_strategy.
- **per_device_eval_batch_size** (`int`, *optional*, defaults to 8) --
  The batch size per device accelerator core/CPU for evaluation.
- **prediction_loss_only** (`bool`, *optional*, defaults to `False`) --
  When performing evaluation and generating predictions, only returns the loss.
- **eval_on_start** (`bool`, *optional*, defaults to `False`) --
  Whether to perform a evaluation step (sanity check) before the training to ensure the validation steps works correctly.
- **eval_do_concat_batches** (`bool`, *optional*, defaults to `True`) --
  Whether to recursively concat inputs/losses/labels/predictions across batches. If `False`,
  will instead store them as lists, with each batch kept separate.
- **eval_use_gather_object** (`bool`, *optional*, defaults to `False`) --
  Whether to run recursively gather object in a nested list/tuple/dictionary of objects from all devices. This should only be enabled if users are not just returning tensors, and this is actively discouraged by PyTorch.
  This is useful when the labels structure is non standard, like in computer vision tasks.
- **eval_accumulation_steps** (`int`, *optional*) --
  Number of predictions steps to accumulate the output tensors for, before moving the results to the CPU. If
  left unset, the whole predictions are accumulated on the device accelerator before being moved to the CPU (faster but
  requires more memory).

Metrics Computation

- **include_for_metrics** (`list[str]`, *optional*, defaults to `[]`) --
  Include additional data in the `compute_metrics` function if needed for metrics computation.
  Possible options to add to `include_for_metrics` list:
  - `"inputs"`: Input data passed to the model, intended for calculating input dependent metrics.
  - `"loss"`: Loss values computed during evaluation, intended for calculating loss dependent metrics.
- **batch_eval_metrics** (`bool`, *optional*, defaults to `False`) --
  If set to `True`, evaluation will call compute_metrics at the end of each batch to accumulate statistics
  rather than saving all eval logits in memory. When set to `True`, you must pass a compute_metrics function
  that takes a boolean argument `compute_result`, which when passed `True`, will trigger the final global
  summary statistics from the batch-level summary statistics you've accumulated over the evaluation set.

Checkpointing & Saving

- **save_only_model** (`bool`, *optional*, defaults to `False`) --
  Save only model weights, not optimizer/scheduler/RNG state. Significantly reduces
  checkpoint size but prevents resuming training from the checkpoint. Use when you only
  need the trained model for inference, not continued training.
  You can only load the model using `from_pretrained` with this option set to `True`.
- **save_strategy** (`str` or `SaveStrategy`, *optional*, defaults to `"steps"`) --
  The checkpoint save strategy to adopt during training. Possible values are:
  - `"no"`: No save is done during training.
  - `"epoch"`: Save is done at the end of each epoch.
  - `"steps"`: Save is done every `save_steps`.
  - `"best"`: Save is done whenever a new `best_metric` is achieved.
- **save_steps** (`int` or `float`, *optional*, defaults to 500) --
  Number of updates steps before two checkpoint saves if `save_strategy="steps"`. Should be an integer or a
  float in range `[0,1)`. If smaller than 1, will be interpreted as ratio of total training steps.
- **save_on_each_node** (`bool`, *optional*, defaults to `False`) --
  When doing multi-node distributed training, whether to save models and checkpoints on each node, or only on
  the main one.
  This should not be activated when the different nodes use the same storage as the files will be saved with
  the same names for each node.
- **save_total_limit** (`int`, *optional*) --
  Maximum number of checkpoints to keep. Deletes older checkpoints in `output_dir`. When
  `load_best_model_at_end=True`, the best checkpoint is always retained plus the most
  recent ones. For example, `save_total_limit=5` keeps the 4 most recent plus the best
- **enable_jit_checkpoint** (`bool`, *optional*, defaults to `False`) --
  Enable Just-In-Time checkpointing on SIGTERM signal for graceful termination on
  preemptible workloads. **Important**: Configure your orchestrator's graceful shutdown
  period to allow sufficient time. For Kubernetes, set `terminationGracePeriodSeconds`
  (default 30s is usually insufficient). For Slurm, use `--signal=TERM@<seconds>`.
  Required grace period ≥ longest iteration time + checkpoint save time.

Hugging Face Hub Integration

- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push the model to the Hub every time the model is saved. If this is activated,
  `output_dir` will begin a git directory synced with the repo (determined by `hub_model_id`) and the content
  will be pushed each time a save is triggered (depending on your `save_strategy`). Calling
  [save_model()](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer.save_model) will also trigger a push.
- **hub_token** (`str`, *optional*) --
  The token to use to push the model to the Hub. Will default to the token in the cache folder obtained with
  `hf auth login`.
- **hub_private_repo** (`bool`, *optional*) --
  Whether to make the repo private. If `None` (default), the repo will be public unless the organization's
  default is private. This value is ignored if the repo already exists. If reporting to Trackio with
  deployment to Hugging Face Spaces enabled, the same logic determines whether the Space is private.
- **hub_model_id** (`str`, *optional*) --
  The name of the repository to keep in sync with the local *output_dir*. It can be a simple model ID in
  which case the model will be pushed in your namespace. Otherwise it should be the whole repository name,
  for instance `"user_name/model"`, which allows you to push to an organization you are a member of with
  `"organization_name/model"`. Will default to `user_name/output_dir_name` with *output_dir_name* being the
  name of `output_dir`.
- **hub_strategy** (`str` or `HubStrategy`, *optional*, defaults to `"every_save"`) --
  Defines what and when to push to Hub. Options:
  - `"end"`: Push only at the end of training
  - `"every_save"`: Push on each save (async to not block training)
  - `"checkpoint"`: Like `"every_save"` plus push latest checkpoint to `"last-checkpoint"` subfolder for easy resuming
  - `"all_checkpoints"`: Push all checkpoints as they appear
- **hub_always_push** (`bool`, *optional*, defaults to `False`) --
  Unless this is `True`, the `Trainer` will skip pushing a checkpoint when the previous push is not finished.
- **hub_revision** (`str`, *optional*) --
  The revision to use when pushing to the Hub. Can be a branch name, a tag, or a commit hash.

Best Model Tracking

- **load_best_model_at_end** (`bool`, *optional*, defaults to `False`) --
  Load the best checkpoint at the end of training. Requires `eval_strategy` to be set.
  When enabled, the best checkpoint is always saved (see `save_total_limit`).
  
  When `True`, `save_strategy` must match `eval_strategy` (unless `save_strategy` is `"best"`), and if using `"steps"`,
  `save_steps` must be a multiple of `eval_steps`.
  
- **metric_for_best_model** (`str`, *optional*) --
  Metric to use for comparing models when `load_best_model_at_end=True`. Must be a metric
  name returned by evaluation, with or without the `"eval_"` prefix. Defaults to `"loss"`.
  If you set this, `greater_is_better` will default to `True` unless the name ends with
  `"loss"`. Examples: `"accuracy"`, `"f1"`, `"eval_bleu"`.
- **greater_is_better** (`bool`, *optional*) --
  Whether higher metric values are better. Defaults based on `metric_for_best_model`:
  `True` if the metric name doesn't end in `"loss"`, `False` otherwise.

Resuming Training

- **ignore_data_skip** (`bool`, *optional*, defaults to `False`) --
  When resuming training, skip fast-forwarding through the dataset to reach the previous
  state. If `True`, training starts from the beginning of the dataset (faster resume but
  results won't match interrupted training). If `False`, skips seen data (slower resume
  but exact continuation).
- **restore_callback_states_from_checkpoint** (`bool`, *optional*, defaults to `False`) --
  Restore callback states from checkpoint when resuming. If `True`, will override callbacks
  passed to Trainer if they exist in the checkpoint.

Reproducibility

- **full_determinism** (`bool`, *optional*, defaults to `False`) --
  If `True`, [enable_full_determinism()](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.enable_full_determinism) is called instead of [set_seed()](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.set_seed) to ensure reproducible results in
  distributed training. Important: this will negatively impact the performance, so only use it for debugging.
- **seed** (`int`, *optional*, defaults to 42) --
  Random seed that will be set at the beginning of training. To ensure reproducibility across runs, use the
  `~Trainer.model_init` function to instantiate the model if it has some randomly initialized parameters.
- **data_seed** (`int`, *optional*) --
  Random seed to be used with data samplers. If not set, random generators for data sampling will use the
  same seed as `seed`. This can be used to ensure reproducibility of data sampling, independent of the model
  seed.

Hardware Configuration

- **use_cpu** (`bool`, *optional*, defaults to `False`) --
  Whether or not to use cpu. If set to False, we will use the available torch device/backend.

Accelerate Configuration

- **accelerator_config** (`str`, `dict`, or `AcceleratorConfig`, *optional*) --
  Configuration for the internal Accelerate integration. Can be:
  - Path to JSON config file: `"accelerator_config.json"`
  - Dictionary with config options
  - `AcceleratorConfig` instance
  Key options:
  - `split_batches` (`bool`, defaults to `False`): Whether to split batches across devices.
    If `True`, actual batch size is the same on all devices (total must be divisible by
    num_processes). If `False`, each device gets the specified batch size.
  - `dispatch_batches` (`bool`): If `True`, only main process iterates through dataloader
    and dispatches batches to devices. Defaults to `True` for `IterableDataset`, `False`
    otherwise.
  - `even_batches` (`bool`, defaults to `True`): Duplicate samples from dataset start to
    ensure all workers get equal batch sizes.
  - `use_seedable_sampler` (`bool`, defaults to `True`): Use fully seedable random sampler
    for reproducibility.
  - `use_configured_state` (`bool`, defaults to `False`): Use pre-initialized
    `AcceleratorState`/`PartialState` instead of creating new one. May cause issues with
    hyperparameter tuning.

- **parallelism_config** (`ParallelismConfig`, *optional*) --
  Parallelism configuration for the training run. Requires Accelerate `1.10.1`

Dataloader

- **dataloader_drop_last** (`bool`, *optional*, defaults to `False`) --
  Whether to drop the last incomplete batch (if the length of the dataset is not divisible by the batch size)
  or not.
- **dataloader_num_workers** (`int`, *optional*, defaults to 0) --
  Number of subprocesses to use for data loading (PyTorch only). 0 means that the data will be loaded in the
  main process.
- **dataloader_pin_memory** (`bool`, *optional*, defaults to `True`) --
  Whether you want to pin memory in data loaders or not. Will default to `True`.
- **dataloader_persistent_workers** (`bool`, *optional*, defaults to `False`) --
  If True, the data loader will not shut down the worker processes after a dataset has been consumed once.
  This allows to maintain the workers Dataset instances alive. Can potentially speed up training, but will
  increase RAM usage. Will default to `False`.
- **dataloader_prefetch_factor** (`int`, *optional*) --
  Number of batches loaded in advance by each worker.
  2 means there will be a total of 2 * num_workers batches prefetched across all workers.
- **remove_unused_columns** (`bool`, *optional*, defaults to `True`) --
  Whether or not to automatically remove the columns unused by the model forward method.
- **label_names** (`list[str]`, *optional*) --
  The list of keys in your dictionary of inputs that correspond to the labels.
  Will eventually default to the list of argument names accepted by the model that contain the word "label",
  except if the model used is one of the `XxxForQuestionAnswering` in which case it will also include the
  `["start_positions", "end_positions"]` keys.
  You should only specify `label_names` if you're using custom label names or if your model's `forward` consumes multiple label tensors (e.g., extractive QA).
- **train_sampling_strategy** (`str`, *optional*, defaults to `"random"`) --
  The sampler to use for the training dataloader. Possible values are:

  - `"random"`: Uses `RandomSampler` (default).
  - `"sequential"`: Uses `SequentialSampler`.
  - `"group_by_length"`: Uses `LengthGroupedSampler` to group samples of roughly the same length
    together (to minimize padding and be more efficient).

  Note: When using an `IterableDataset`, this argument is ignored.
- **length_column_name** (`str`, *optional*, defaults to `"length"`) --
  Column name for precomputed lengths. If the column exists, grouping by length will use these values rather
  than computing them on train startup. Ignored unless `train_sampling_strategy` is `"group_by_length"` and the dataset
  is an instance of `Dataset`.

DDP (DistributedDataParallel)

- **ddp_find_unused_parameters** (`bool`, *optional*) --
  When using distributed training, the value of the flag `find_unused_parameters` passed to
  `DistributedDataParallel`. Will default to `False` if gradient checkpointing is used, `True` otherwise.
- **ddp_bucket_cap_mb** (`int`, *optional*) --
  When using distributed training, the value of the flag `bucket_cap_mb` passed to `DistributedDataParallel`.
- **ddp_broadcast_buffers** (`bool`, *optional*) --
  When using distributed training, the value of the flag `broadcast_buffers` passed to
  `DistributedDataParallel`. Will default to `False` if gradient checkpointing is used, `True` otherwise.
- **ddp_static_graph** (`bool`, *optional*) --
  When using distributed training, the value of the flag `static_graph` passed to
  `DistributedDataParallel`.
- **ddp_backend** (`str`, *optional*) --
  The backend to use for distributed training. Must be one of `"nccl"`, `"mpi"`, `"xccl"`, `"gloo"`, `"hccl"`.
- **ddp_timeout** (`int`, *optional*, defaults to 1800) --
  The timeout for `torch.distributed.init_process_group` calls, used to avoid GPU socket timeouts when
  performing slow operations in distributed runnings. Please refer to the [PyTorch documentation](https://pytorch.org/docs/stable/distributed.html#torch.distributed.init_process_group) for more
  information.

FSDP (Fully Sharded Data Parallel)

- **fsdp** (`bool`, *optional*, defaults to `None`) --
  Enable PyTorch Fully Sharded Data Parallel (FSDP) for distributed training. Pass `True` to enable FSDP.
- **fsdp_config** (`str` or `dict`, *optional*) --
  Configuration settings for when `fsdp` is enabled. Pass a path to a JSON config file, such
  as `fsdp_config.json`, or an already-loaded dict.

  Supported keys:
  - version (`int`, *optional*, defaults to `2`):
    The version of FSDP to use (`2` for FSDP2, `1` for the legacy FSDP1).
  - reshard_after_forward (`bool`, *optional*, defaults to `True`):
    Whether to reshard parameters after the forward pass. Set to `False` to keep parameters
    gathered between the forward and backward passes, avoids the re-all-gather, and use higher peak memory.
  - cpu_offload (`bool`, *optional*, defaults to `False`):
    Offload parameters and gradients to CPU when not in use to save GPU memory.
  - activation_checkpointing (`bool`, *optional*, defaults to `False`):
    Set to `True` to reduce memory by recomputing activations during the backward pass. Prefer
    `activation_checkpointing` over `gradient_checkpointing` when using FSDP. `gradient_checkpointing`
    introduces a redundant all-gather in the backward pass.
  - cpu_ram_efficient_loading (`bool`, *optional*, defaults to `False`):
    Set to `True` to load the pretrained checkpoint on the first process only. Other processes start
    with empty weights and receive the weights by broadcast.
  - state_dict_type (`str`, *optional*, defaults to `"FULL_STATE_DICT"`):
    Checkpoint format: `"FULL_STATE_DICT"` (single HF-compatible file) or
    `"SHARDED_STATE_DICT"` (one file per rank, faster for large models).
  - auto_wrap_policy (`str`, *optional*, defaults to `"TRANSFORMER_BASED_WRAP"`):
    Auto-wrap policy to use. Choose `"TRANSFORMER_BASED_WRAP"`, `"SIZE_BASED_WRAP"`, or `"NO_WRAP"`.
  - transformer_layer_cls_to_wrap (`list[str]`, *optional*):
    Transformer layer class names (case-sensitive) to wrap, e.g. `LlamaDecoderLayer`. Usually
    unnecessary: the wrap policy falls back to the model's `_no_split_modules`, which covers
    most transformers models.
  - min_num_params (`int`, *optional*, defaults to `0`):
    Minimum number of parameters per module for size-based auto-wrapping (used with
    `auto_wrap_policy="SIZE_BASED_WRAP"`).
  - xla (`bool`, *optional*, defaults to `False`):
    Whether to use PyTorch/XLA Fully Sharded Data Parallel Training. Experimental.
  - xla_fsdp_settings (`dict`, *optional*):
    Dictionary of XLA FSDP wrapping parameters. For a complete list of options, see the
    [XLA FSDP source](https://github.com/pytorch/xla/blob/master/torch_xla/distributed/fsdp/xla_fully_sharded_data_parallel.py).
  - xla_fsdp_grad_ckpt (`bool`, *optional*, defaults to `False`):
    Set to `True` to use gradient checkpointing over each nested XLA FSDP wrapped layer. Requires
    `xla=True` and an auto-wrapping policy (`min_num_params` or `transformer_layer_cls_to_wrap`).

DeepSpeed

- **deepspeed** (`str` or `dict`, *optional*) --
  Enable [DeepSpeed](https://github.com/deepspeedai/DeepSpeed) integration. Value is either:
  - Path to DeepSpeed JSON config file: `"ds_config.json"`
  - Loaded config as dictionary
  > [!TIP]
  > If using ZeRO initialization, instantiate your model *after* initializing
  `TrainingArguments`, otherwise ZeRO won't be applied.

Debugging & Profiling (Experimental)

- **debug** (`str` or list of `DebugOption`, *optional*, defaults to `""`) --
  Enable one or more debug features. This is an experimental feature.
  Possible options are:
  - "underflow_overflow": detects overflow in model's input/outputs and reports the last frames that led to
    the event
  - "tpu_metrics_debug": print debug metrics on TPU
- **skip_memory_metrics** (`bool`, *optional*, defaults to `True`) --
  Whether to skip adding of memory profiler reports to metrics. This is skipped by default because it slows
  down the training and evaluation speed.

External Script Flags (not used by Trainer)

- **do_train** (`bool`, *optional*, defaults to `False`) --
  Whether to run training or not. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used
  by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **do_eval** (`bool`, *optional*) --
  Whether to run evaluation on the validation set or not. Will be set to `True` if `eval_strategy` is
  different from `"no"`. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used by your
  training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **do_predict** (`bool`, *optional*, defaults to `False`) --
  Whether to run predictions on the test set or not. This argument is not directly used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's
  intended to be used by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.
- **resume_from_checkpoint** (`str`, *optional*) --
  The path to a folder with a valid checkpoint for your model. This argument is not directly used by
  [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer), it's intended to be used by your training/evaluation scripts instead. See the [example
  scripts](https://github.com/huggingface/transformers/tree/main/examples) for more details.

- **sortish_sampler** (`bool`, *optional*, defaults to `False`) --
  Whether to use a *sortish sampler* or not. Only possible if the underlying datasets are *Seq2SeqDataset*
  for now but will become generally available in the near future.

  It sorts the inputs according to lengths in order to minimize the padding size, with a bit of randomness
  for the training set.
- **predict_with_generate** (`bool`, *optional*, defaults to `False`) --
  Whether to use generate to calculate generative metrics (ROUGE, BLEU).
- **generation_max_length** (`int`, *optional*) --
  The `max_length` to use on each evaluation loop when `predict_with_generate=True`. Will default to the
  `max_length` value of the model configuration.
- **generation_num_beams** (`int`, *optional*) --
  The `num_beams` to use on each evaluation loop when `predict_with_generate=True`. Will default to the
  `num_beams` value of the model configuration.
- **generation_config** (`str` or `Path` or [GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) --
  Allows to load a [GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig) from the `from_pretrained` method. This can be either:

  - a string, the *model id* of a pretrained model configuration hosted inside a model repo on
    huggingface.co.
  - a path to a *directory* containing a configuration file saved using the
    [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig.save_pretrained) method, e.g., `./my_model_directory/`.
  - a [GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig) object.

Configuration class for controlling all aspects of model training with the Trainer.
TrainingArguments centralizes all hyperparameters, optimization settings, logging preferences, and infrastructure choices needed for training.

[HfArgumentParser](/docs/transformers/v5.14.0/en/internal/trainer_utils#transformers.HfArgumentParser) can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

Serializes this instance while replace `Enum` by their values and `GenerationConfig` by dictionaries (for JSON
serialization support). It obfuscates the token values by removing their value.
