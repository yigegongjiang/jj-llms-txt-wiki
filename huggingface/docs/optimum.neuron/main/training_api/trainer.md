# NeuronTrainer

Training classes for AWS Trainium accelerators.

## NeuronTrainingArguments[[optimum.neuron.NeuronTrainingArguments]]

#### optimum.neuron.NeuronTrainingArguments[[optimum.neuron.NeuronTrainingArguments]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L49)

get_process_log_leveloptimum.neuron.NeuronTrainingArguments.get_process_log_levelhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L761[]

Returns the log level to be used depending on whether this process is the main process of node 0, main process
of node non-0, or a non-main process.

For the main process the log level defaults to the logging level set (`logging.WARNING` if you didn't do
anything) unless overridden by `log_level` argument.

For the replica processes the log level defaults to `logging.WARNING` unless overridden by `log_level_replica`
argument.

The choice between the main and replica process settings is made according to the return value of `should_log`.
#### get_warmup_steps[[optimum.neuron.NeuronTrainingArguments.get_warmup_steps]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L783)

Get number of steps used for a linear warmup.
#### to_dict[[optimum.neuron.NeuronTrainingArguments.to_dict]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L803)

Serializes this instance while replace `Enum` by their values (for JSON serialization support). It obfuscates
the token values by removing their value.
#### to_json_string[[optimum.neuron.NeuronTrainingArguments.to_json_string]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L830)

Serializes this instance to a JSON string.
#### to_sanitized_dict[[optimum.neuron.NeuronTrainingArguments.to_sanitized_dict]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/training_args.py#L836)

Sanitized serialization to use with TensorBoard’s hparams

## NeuronTrainer[[optimum.neuron.NeuronTrainer]]

#### optimum.neuron.NeuronTrainer[[optimum.neuron.NeuronTrainer]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L119)

add_callbackoptimum.neuron.NeuronTrainer.add_callbackhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L375[{"name": "callback", "val": ": typing.Union[typing.Type[transformers.trainer_callback.TrainerCallback], transformers.trainer_callback.TrainerCallback]"}]- **callback** (`Type[TrainerCallback] | TrainerCallback`) --
  A `TrainerCallback` class or an instance of a `TrainerCallback`. In the
  first case, will instantiate a member of that class.0

Add a callback to the current list of `TrainerCallback`.

**Parameters:**

callback (`Type[TrainerCallback] | TrainerCallback`) : A `TrainerCallback` class or an instance of a `TrainerCallback`. In the first case, will instantiate a member of that class.
#### autocast_smart_context_manager[[optimum.neuron.NeuronTrainer.autocast_smart_context_manager]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L743)

A helper wrapper that creates an appropriate context manager for `autocast` while feeding it the desired
arguments, depending on the situation.
#### create_accelerator_and_postprocess[[optimum.neuron.NeuronTrainer.create_accelerator_and_postprocess]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L291)

Creates NeuronAccelerator instance and prepares model for distributed training.
#### create_optimizer[[optimum.neuron.NeuronTrainer.create_optimizer]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L569)

Setup the optimizer.

We provide a reasonable default that works well. If you want to use something else, you can pass a tuple in the
NeuronTrainer's init through `optimizers`, or subclass and override this method in a subclass.
#### create_optimizer_and_scheduler[[optimum.neuron.NeuronTrainer.create_optimizer_and_scheduler]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L547)

Setup the optimizer and the learning rate scheduler.

We provide a reasonable default that works well. If you want to use something else, you can pass a tuple in the
NeuronTrainer's init through `optimizers`, or subclass and override this method (or `create_optimizer` and/or
`create_scheduler`) in a subclass.
#### create_scheduler[[optimum.neuron.NeuronTrainer.create_scheduler]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L683)

Setup the scheduler. The optimizer of the trainer must have been set up either before this method is called or
passed as an argument.

**Parameters:**

num_training_steps (int) : The number of training steps to do.
#### get_decay_parameter_names[[optimum.neuron.NeuronTrainer.get_decay_parameter_names]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L558)

Get all parameter names that weight decay will be applied to.

This function filters out parameters in two ways:
1. By layer type (instances of layers specified in ALL_LAYERNORM_LAYERS)
2. By parameter name patterns (containing 'bias', 'layernorm', or 'rmsnorm')
#### get_learning_rates[[optimum.neuron.NeuronTrainer.get_learning_rates]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L626)

Returns the learning rate of each parameter from self.optimizer.
#### get_num_trainable_parameters[[optimum.neuron.NeuronTrainer.get_num_trainable_parameters]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L620)

Get the number of trainable parameters.
#### get_optimizer_cls_and_kwargs[[optimum.neuron.NeuronTrainer.get_optimizer_cls_and_kwargs]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L650)

Returns the optimizer class and optimizer parameters based on the training arguments.

**Parameters:**

args (`transformers.training_args.TrainingArguments`) : The training arguments for the training session.
#### get_optimizer_group[[optimum.neuron.NeuronTrainer.get_optimizer_group]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L634)

Returns optimizer group for a parameter if given, else returns all optimizer groups for params.

**Parameters:**

param (`str | torch.nn.parameter.Parameter | None`, defaults to `None`) : The parameter for which optimizer group needs to be returned.
#### get_train_dataloader[[optimum.neuron.NeuronTrainer.get_train_dataloader]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L528)

Returns the training DataLoader with appropriate sampler and batch size.
#### is_local_process_zero[[optimum.neuron.NeuronTrainer.is_local_process_zero]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1251)

Whether or not this process is the local (e.g., on one machine if training in a distributed fashion on several
machines) main process.
#### is_world_process_zero[[optimum.neuron.NeuronTrainer.is_world_process_zero]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1258)

Whether or not this process is the global main process (when training in a distributed fashion on several
machines, this is only going to be `True` for one process).
#### log[[optimum.neuron.NeuronTrainer.log]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1318)

Log training metrics to the state history and callbacks.
#### maybe_log_train_step_metrics[[optimum.neuron.NeuronTrainer.maybe_log_train_step_metrics]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1028)

Log training step metrics if logging is due.
#### maybe_save_checkpoint[[optimum.neuron.NeuronTrainer.maybe_save_checkpoint]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1080)

Save checkpoint if saving is due.
#### num_examples[[optimum.neuron.NeuronTrainer.num_examples]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L703)

Helper to get number of samples in a `~torch.utils.data.DataLoader` by accessing its dataset. When
dataloader.dataset does not exist or has no length, estimates as best it can
#### num_tokens[[optimum.neuron.NeuronTrainer.num_tokens]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L717)

Helper to get number of tokens in a `~torch.utils.data.DataLoader` by enumerating dataloader.
#### pop_callback[[optimum.neuron.NeuronTrainer.pop_callback]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L386)

Remove a callback from the current list of `TrainerCallback` and returns it.

If the callback is not found, returns `None` (and no error is raised).

**Parameters:**

callback (`Type[TrainerCallback] | TrainerCallback`) : A `TrainerCallback` class or an instance of a `TrainerCallback`. In the first case, will pop the first member of that class found in the list of callbacks.

**Returns:**

``TrainerCallback | None``

The callback removed, if found.
#### remove_callback[[optimum.neuron.NeuronTrainer.remove_callback]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L402)

Remove a callback from the current list of `TrainerCallback`.

**Parameters:**

callback (`Type[TrainerCallback] | TrainerCallback`) : A `TrainerCallback` class or an instance of a `TrainerCallback`. In the first case, will remove the first member of that class found in the list of callbacks.
#### report_and_save_summary_metrics[[optimum.neuron.NeuronTrainer.report_and_save_summary_metrics]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1326)

Report and save comprehensive training summary metrics at the end of training.
#### set_initial_training_values[[optimum.neuron.NeuronTrainer.set_initial_training_values]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L754)

Calculates and returns the following values:
- `num_train_epochs`
- `num_update_steps_per_epoch`
- `num_examples`
- `num_train_samples`
- `epoch_based`
- `len_dataloader`
- `max_steps`
#### setup_training[[optimum.neuron.NeuronTrainer.setup_training]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L818)

Setup everything to prepare for the training loop.
This methods does not return anything but initializes many attributes of the class for training.
#### train[[optimum.neuron.NeuronTrainer.train]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/transformers.py#L1240)

Main training entry point.
Wraps around `self._train()` to handle cache synchronization.
