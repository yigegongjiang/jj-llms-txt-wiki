# DataLoaders, Optimizers, and Schedulers

The internal classes Accelerate uses to prepare objects for distributed training
when calling [prepare()](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator.prepare).

## DataLoader utilities[[accelerate.data_loader.prepare_data_loader]]

#### accelerate.data_loader.prepare_data_loader[[accelerate.data_loader.prepare_data_loader]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L1014)

Wraps a PyTorch `DataLoader` to generate batches for one of the processes only.

Depending on the value of the `drop_last` attribute of the `dataloader` passed, it will either stop the iteration
at the first batch that would be too small / not present on all processes or loop with indices from the beginning.

`BatchSampler`s with varying batch sizes are not enabled by default. To enable this behaviour, set `even_batches`
equal to `False`

**Parameters:**

dataloader (`torch.utils.data.dataloader.DataLoader`) : The data loader to split across several devices.

device (`torch.device`) : The target device for the returned `DataLoader`.

num_processes (`int`, *optional*) : The number of processes running concurrently. Will default to the value given by [PartialState](/docs/accelerate/v1.14.0/en/package_reference/state#accelerate.PartialState).

process_index (`int`, *optional*) : The index of the current process. Will default to the value given by [PartialState](/docs/accelerate/v1.14.0/en/package_reference/state#accelerate.PartialState).

split_batches (`bool`, *optional*, defaults to `False`) : Whether the resulting `DataLoader` should split the batches of the original data loader across devices or yield full batches (in which case it will yield batches starting at the `process_index`-th and advancing of `num_processes` batches at each iteration).  Another way to see this is that the observed batch size will be the same as the initial `dataloader` if this option is set to `True`, the batch size of the initial `dataloader` multiplied by `num_processes` otherwise.  Setting this option to `True` requires that the batch size of the `dataloader` is a round multiple of `batch_size`.

put_on_device (`bool`, *optional*, defaults to `False`) : Whether or not to put the batches on `device` (only works if the batches are nested list, tuples or dictionaries of tensors).

rng_types (list of `str` or [RNGType](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.utils.RNGType)) : The list of random number generators to synchronize at the beginning of each iteration. Should be one or several of:  - `"torch"`: the base torch random number generator - `"cuda"`: the CUDA random number generator (GPU only) - `"xla"`: the XLA random number generator (TPU only) - `"generator"`: the `torch.Generator` of the sampler (or batch sampler if there is no sampler in your dataloader) or of the iterable dataset (if it exists) if the underlying dataset is of that type. 

dispatch_batches (`bool`, *optional*) : If set to `True`, the dataloader prepared is only iterated through on the main process and then the batches are split and broadcast to each process. Will default to `True` when the underlying dataset is an `IterableDataset`, `False` otherwise.

even_batches (`bool`, *optional*, defaults to `True`) : If set to `True`, in cases where the total batch size across all processes does not exactly divide the dataset, samples at the start of the dataset will be duplicated so the batch can be divided equally among all workers.

slice_fn_for_dispatch (`Callable`, *optional*`) : If passed, this function will be used to slice tensors across `num_processes`. Will default to [slice_tensors()](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.utils.slice_tensors). This argument is used only when `dispatch_batches` is set to `True` and will be ignored otherwise.

use_seedable_sampler (`bool`, *optional*, defaults to `False`) : Whether to use the `SeedableRandomSampler` instead of a `RandomSampler` for better reproducibility. Comes at a cost of potentially different performances due to different shuffling algorithms but ensures results will be the *exact* same. Should be paired with `set_seed()` at every `self.set_epoch`

data_seed (`int`, *optional*, defaults to `None`) : The seed to use for the underlying generator when using `use_seedable_sampler`. If `None`, the generator will use the current default seed from torch.

non_blocking (`bool`, *optional*, defaults to `False`) : If set to `True`, dataloader will utilize non-blocking host-to-device transfers. If the dataloader has `pin_memory` set to `True`, this will help to increase overlap between data transfer and computations.

use_stateful_dataloader (`bool`, *optional*, defaults to `False`) : "If set to true, the dataloader prepared by the Accelerator will be backed by " "[torchdata.StatefulDataLoader](https://github.com/pytorch/data/tree/main/torchdata/stateful_dataloader). This requires `torchdata` version 0.8.0 or higher that supports StatefulDataLoader to be installed."

torch_device_mesh (`torch.distributed.DeviceMesh`, *optional*, defaults to `None`) : PyTorch device mesh.

**Returns:**

``torch.utils.data.dataloader.DataLoader``

A new data loader that will yield the portion of the batches

#### accelerate.skip_first_batches[[accelerate.skip_first_batches]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L1393)

Creates a `torch.utils.data.DataLoader` that will efficiently skip the first `num_batches`. Should not be used if
the original dataloader is a `StatefulDataLoader`.

## BatchSamplerShard[[accelerate.data_loader.BatchSamplerShard]]

#### accelerate.data_loader.BatchSamplerShard[[accelerate.data_loader.BatchSamplerShard]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L110)

Wraps a PyTorch `BatchSampler` to generate batches for one of the processes only. Instances of this class will
always yield a number of batches that is a round multiple of `num_processes` and that all have the same size.
Depending on the value of the `drop_last` attribute of the batch sampler passed, it will either stop the iteration
at the first batch that would be too small / not present on all processes or loop with indices from the beginning.

`BatchSampler`s with varying batch sizes are not enabled by default. To enable this behaviour, set `even_batches`
equal to `False`

**Parameters:**

batch_sampler (`torch.utils.data.sampler.BatchSampler`) : The batch sampler to split in several shards.

num_processes (`int`, *optional*, defaults to 1) : The number of processes running concurrently.

process_index (`int`, *optional*, defaults to 0) : The index of the current process.

split_batches (`bool`, *optional*, defaults to `False`) : Whether the shards should be created by splitting a batch to give a piece of it on each process, or by yielding different full batches on each process.  On two processes with a sampler of `[[0, 1, 2, 3], [4, 5, 6, 7]]`, this will result in:  - the sampler on process 0 to yield `[0, 1, 2, 3]` and the sampler on process 1 to yield `[4, 5, 6, 7]` if this argument is set to `False`. - the sampler on process 0 to yield `[0, 1]` then `[4, 5]` and the sampler on process 1 to yield `[2, 3]` then `[6, 7]` if this argument is set to `True`.

even_batches (`bool`, *optional*, defaults to `True`) : Whether or not to loop back at the beginning of the sampler when the number of samples is not a round multiple of (original batch size / number of processes).

## IterableDatasetShard[[accelerate.data_loader.IterableDatasetShard]]

#### accelerate.data_loader.IterableDatasetShard[[accelerate.data_loader.IterableDatasetShard]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L274)

Wraps a PyTorch `IterableDataset` to generate samples for one of the processes only. Instances of this class will
always yield a number of samples that is a round multiple of the actual batch size (depending of the value of
`split_batches`, this is either `batch_size` or `batch_size x num_processes`). Depending on the value of the
`drop_last` attribute of the batch sampler passed, it will either stop the iteration at the first batch that would
be too small or loop with indices from the beginning.

**Parameters:**

dataset (`torch.utils.data.dataset.IterableDataset`) : The batch sampler to split in several shards.

batch_size (`int`, *optional*, defaults to 1) : The size of the batches per shard (if `split_batches=False`) or the size of the batches (if `split_batches=True`).

drop_last (`bool`, *optional*, defaults to `False`) : Whether or not to drop the last incomplete batch or complete the last batches by using the samples from the beginning.

num_processes (`int`, *optional*, defaults to 1) : The number of processes running concurrently.

process_index (`int`, *optional*, defaults to 0) : The index of the current process.

split_batches (`bool`, *optional*, defaults to `False`) : Whether the shards should be created by splitting a batch to give a piece of it on each process, or by yielding different full batches on each process.  On two processes with an iterable dataset yielding of `[0, 1, 2, 3, 4, 5, 6, 7]`, this will result in:  - the shard on process 0 to yield `[0, 1, 2, 3]` and the shard on process 1 to yield `[4, 5, 6, 7]` if this argument is set to `False`. - the shard on process 0 to yield `[0, 1, 4, 5]` and the sampler on process 1 to yield `[2, 3, 6, 7]` if this argument is set to `True`.

## DataLoaderShard[[accelerate.data_loader.DataLoaderShard]]

#### accelerate.data_loader.DataLoaderShard[[accelerate.data_loader.DataLoaderShard]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L510)

Subclass of `DataLoaderAdapter` that will deal with device placement and current distributed setup.

**Available attributes:**

- **total_batch_size** (`int`) -- Total batch size of the dataloader across all processes.
  Equal to the original batch size when `split_batches=True`; otherwise the original batch size * the total
  number of processes

- **total_dataset_length** (`int`) -- Total length of the inner dataset across all processes.

**Parameters:**

dataset (`torch.utils.data.dataset.Dataset`) : The dataset to use to build this dataloader.

device (`torch.device`, *optional*) : If passed, the device to put all batches on.

rng_types (list of `str` or [RNGType](/docs/accelerate/v1.14.0/en/package_reference/utilities#accelerate.utils.RNGType)) : The list of random number generators to synchronize at the beginning of each iteration. Should be one or several of:  - `"torch"`: the base torch random number generator - `"cuda"`: the CUDA random number generator (GPU only) - `"xla"`: the XLA random number generator (TPU only) - `"generator"`: an optional `torch.Generator`

synchronized_generator (`torch.Generator`, *optional*) : A random number generator to keep synchronized across processes.

skip_batches (`int`, *optional*, defaults to 0) : The number of batches to skip at the beginning.

use_stateful_dataloader (`bool`, *optional*, defaults to `False`) : Whether to have this class adapt `StatefulDataLoader` from `torchdata` instead of the regular `DataLoader`.

- ****kwargs** (additional keyword arguments, *optional*) : All other keyword arguments to pass to the regular `DataLoader` initialization.

## DataLoaderDispatcher[[accelerate.data_loader.DataLoaderDispatcher]]

#### accelerate.data_loader.DataLoaderDispatcher[[accelerate.data_loader.DataLoaderDispatcher]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/data_loader.py#L722)

Subclass of `DataLoaderAdapter` that will iterate and preprocess on process 0 only, then dispatch on each process
their part of the batch.

**Available attributes:**

- **total_batch_size** (`int`) -- Total batch size of the dataloader across all processes.
  Equal to the original batch size when `split_batches=True`; otherwise the original batch size * the total
  number of processes

- **total_dataset_length** (`int`) -- Total length of the inner dataset across all processes.

**Parameters:**

split_batches (`bool`, *optional*, defaults to `False`) : Whether the resulting `DataLoader` should split the batches of the original data loader across devices or yield full batches (in which case it will yield batches starting at the `process_index`-th and advancing of `num_processes` batches at each iteration). Another way to see this is that the observed batch size will be the same as the initial `dataloader` if this option is set to `True`, the batch size of the initial `dataloader` multiplied by `num_processes` otherwise. Setting this option to `True` requires that the batch size of the `dataloader` is a round multiple of `batch_size`.

skip_batches (`int`, *optional*, defaults to 0) : The number of batches to skip at the beginning of an iteration.

use_stateful_dataloader (`bool`, *optional*, defaults to `False`) : Whether to have this class adapt `StatefulDataLoader` from `torchdata` instead of the regular `DataLoader`.

## AcceleratedOptimizer[[accelerate.optimizer.AcceleratedOptimizer]]

#### accelerate.optimizer.AcceleratedOptimizer[[accelerate.optimizer.AcceleratedOptimizer]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/optimizer.py#L38)

Internal wrapper around a torch optimizer.

Conditionally will perform `step` and `zero_grad` if gradients should be synchronized when performing gradient
accumulation.

evalaccelerate.optimizer.AcceleratedOptimizer.evalhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/optimizer.py#L138[]

Sets the optimizer to "eval" mode. Useful for optimizers like `schedule_free`

**Parameters:**

optimizer (`torch.optim.optimizer.Optimizer`) : The optimizer to wrap.

device_placement (`bool`, *optional*, defaults to `True`) : Whether or not the optimizer should handle device placement. If so, it will place the state dictionary of `optimizer` on the right device.

scaler (`torch.amp.GradScaler` or `torch.cuda.amp.GradScaler`, *optional*) : The scaler to use in the step function if training with mixed precision.
#### train[[accelerate.optimizer.AcceleratedOptimizer.train]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/optimizer.py#L124)

Sets the optimizer to "train" mode. Useful for optimizers like `schedule_free`

## AcceleratedScheduler[[accelerate.scheduler.AcceleratedScheduler]]

#### accelerate.scheduler.AcceleratedScheduler[[accelerate.scheduler.AcceleratedScheduler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/scheduler.py#L25)

A wrapper around a learning rate scheduler that will only step when the optimizer(s) have a training step. Useful
to avoid making a scheduler step too fast when gradients went overflow and there was no training step (in mixed
precision training)

When performing gradient accumulation scheduler lengths should not be changed accordingly, Accelerate will always
step the scheduler to account for it.

**Parameters:**

scheduler (`torch.optim.lr_scheduler._LRScheduler`) : The scheduler to wrap.

optimizers (one or a list of `torch.optim.Optimizer`) : The optimizers used.

step_with_optimizer (`bool`, *optional*, defaults to `True`) : Whether or not the scheduler should be stepped at each optimizer step.

split_batches (`bool`, *optional*, defaults to `False`) : Whether or not the dataloaders split one batch across the different processes (so batch size is the same regardless of the number of processes) or create batches on each process (so batch size is the original batch size multiplied by the number of processes).
