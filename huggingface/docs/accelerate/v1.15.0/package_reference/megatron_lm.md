# Megatron-LM utilities

## MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

#### accelerate.utils.MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

```python
accelerate.utils.MegatronLMPlugin(tp_degree: int = None, pp_degree: int = None, use_custom_fsdp: bool = None, overlap_cpu_optimizer_d2h_h2d: bool = None, no_load_optim: bool = None, eod_mask_loss: bool = None, no_save_optim: bool = None, optimizer_cpu_offload: bool = None, use_precision_aware_optimizer: bool = None, decoder_last_pipeline_num_layers: int = None, recompute_granularity: str = None, recompute_method: str = None, recompute_num_layers: int = None, attention_backend: bool = None, expert_model_parallel_size: int = None, context_parallel_size: int = None, attention_dropout: float = None, hidden_dropout: float = None, attention_softmax_in_fp32: bool = None, expert_tensor_parallel_size: int = None, calculate_per_token_loss: bool = None, use_rotary_position_embeddings: bool = None, num_micro_batches: int = None, gradient_clipping: float = None, sequence_parallelism: bool = None, recompute_activations: bool = None, use_distributed_optimizer: bool = None, pipeline_model_parallel_split_rank: int = None, num_layers_per_virtual_pipeline_stage: int = None, is_train_batch_min: str = True, train_iters: int = None, train_samples: int = None, weight_decay_incr_style: str = 'constant', start_weight_decay: float = None, end_weight_decay: float = None, lr_decay_style: str = 'linear', lr_decay_iters: int = None, lr_decay_samples: int = None, lr_warmup_iters: int = None, lr_warmup_samples: int = None, lr_warmup_fraction: float = None, min_lr: float = 0, consumed_samples: list = None, no_wd_decay_cond: typing.Optional[typing.Callable] = None, scale_lr_cond: typing.Optional[typing.Callable] = None, lr_mult: float = 1.0, megatron_dataset_flag: bool = False, seq_length: int = None, encoder_seq_length: int = None, decoder_seq_length: int = None, tensorboard_dir: str = None, set_all_logging_options: bool = False, eval_iters: int = 100, eval_interval: int = 1000, return_logits: bool = False, custom_train_step_class: typing.Optional[typing.Any] = None, custom_train_step_kwargs: typing.Optional[dict[str, typing.Any]] = None, custom_model_provider_function: typing.Optional[typing.Callable] = None, custom_prepare_model_function: typing.Optional[typing.Callable] = None, custom_megatron_datasets_provider_function: typing.Optional[typing.Callable] = None, custom_get_batch_function: typing.Optional[typing.Callable] = None, custom_loss_function: typing.Optional[typing.Callable] = None, other_megatron_args: typing.Optional[dict[str, typing.Any]] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/dataclasses.py#L2335)

**Parameters:**

tp_degree (`int`, defaults to `None`) : Tensor parallelism degree.

pp_degree (`int`, defaults to `None`) : Pipeline parallelism degree.

num_micro_batches (`int`, defaults to `None`) : Number of micro-batches.

gradient_clipping (`float`, defaults to `None`) : Gradient clipping value based on global L2 Norm (0 to disable).

sequence_parallelism (`bool`, defaults to `None`) : Enable sequence parallelism.

recompute_activations (`bool`, defaults to `None`) : Enable selective activation recomputation.

use_distributed_optimizer (`bool`, defaults to `None`) : Enable distributed optimizer.

pipeline_model_parallel_split_rank (`int`, defaults to `None`) : Rank where encoder and decoder should be split.

num_layers_per_virtual_pipeline_stage (`int`, defaults to `None`) : Number of layers per virtual pipeline stage.

is_train_batch_min (`str`, defaults to `True`) : If both tran & eval dataloaders are specified, this will decide the `micro_batch_size`.

train_iters (`int`, defaults to `None`) : Total number of samples to train over all training runs. Note that either train-iters or train-samples should be provided when using `MegatronLMDummyScheduler`.

train_samples (`int`, defaults to `None`) : Total number of samples to train over all training runs. Note that either train-iters or train-samples should be provided when using `MegatronLMDummyScheduler`.

weight_decay_incr_style (`str`, defaults to `'constant'`) : Weight decay increment function. choices=["constant", "linear", "cosine"].

start_weight_decay (`float`, defaults to `None`) : Initial weight decay coefficient for L2 regularization.

end_weight_decay (`float`, defaults to `None`) : End of run weight decay coefficient for L2 regularization.

lr_decay_style (`str`, defaults to `'linear'`) : Learning rate decay function. choices=['constant', 'linear', 'cosine'].

lr_decay_iters (`int`, defaults to `None`) : Number of iterations for learning rate decay. If None defaults to `train_iters`.

lr_decay_samples (`int`, defaults to `None`) : Number of samples for learning rate decay. If None defaults to `train_samples`.

lr_warmup_iters (`int`, defaults to `None`) : Number of iterations to linearly warmup learning rate over.

lr_warmup_samples (`int`, defaults to `None`) : Number of samples to linearly warmup learning rate over.

lr_warmup_fraction (`float`, defaults to `None`) : Fraction of lr-warmup-(iters/samples) to linearly warmup learning rate over.

min_lr (`float`, defaults to `0`) : Minimum value for learning rate. The scheduler clip values below this threshold.

consumed_samples (`List`, defaults to `None`) : Number of samples consumed in the same order as the dataloaders to `accelerator.prepare` call.

no_wd_decay_cond (`Optional`, defaults to `None`) : Condition to disable weight decay.

scale_lr_cond (`Optional`, defaults to `None`) : Condition to scale learning rate.

lr_mult (`float`, defaults to `1.0`) : Learning rate multiplier.

megatron_dataset_flag (`bool`, defaults to `False`) : Whether the format of dataset follows Megatron-LM Indexed/Cached/MemoryMapped format.

seq_length (`int`, defaults to `None`) : Maximum sequence length to process.

encoder_seq_length (`int`, defaults to `None`) : Maximum sequence length to process for the encoder.

decoder_seq_length (`int`, defaults to `None`) : Maximum sequence length to process for the decoder.

tensorboard_dir (`str`, defaults to `None`) : Path to save tensorboard logs.

set_all_logging_options (`bool`, defaults to `False`) : Whether to set all logging options.

eval_iters (`int`, defaults to `100`) : Number of iterations to run for evaluation validation/test for.

eval_interval (`int`, defaults to `1000`) : Interval between running evaluation on validation set.

return_logits (`bool`, defaults to `False`) : Whether to return logits from the model.

custom_train_step_class (`Optional`, defaults to `None`) : Custom train step class.

custom_train_step_kwargs (`Optional`, defaults to `None`) : Custom train step kwargs.

custom_model_provider_function (`Optional`, defaults to `None`) : Custom model provider function.

custom_prepare_model_function (`Optional`, defaults to `None`) : Custom prepare model function.

custom_megatron_datasets_provider_function (`Optional`, defaults to `None`) : Custom megatron train_valid_test datasets provider function.

custom_get_batch_function (`Optional`, defaults to `None`) : Custom get batch function.

custom_loss_function (`Optional`, defaults to `None`) : Custom loss function.

other_megatron_args (`Optional`, defaults to `None`) : Other Megatron-LM arguments. Please refer Megatron-LM.

Plugin for Megatron-LM to enable tensor, pipeline, sequence and data parallelism. Also to enable selective
activation recomputation and optimized fused kernels.

## MegatronLMDummyScheduler[[accelerate.utils.MegatronLMDummyScheduler]]

#### accelerate.utils.MegatronLMDummyScheduler[[accelerate.utils.MegatronLMDummyScheduler]]

```python
accelerate.utils.MegatronLMDummyScheduler(optimizer, total_num_steps = None, warmup_num_steps = 0, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L378)

**Parameters:**

optimizer (`torch.optim.optimizer.Optimizer`) : The optimizer to wrap.

total_num_steps (int) : Total number of steps.

warmup_num_steps (int) : Number of steps for warmup.

- ****kwargs** (additional keyword arguments, *optional*) : Other arguments.

Dummy scheduler presents model parameters or param groups, this is primarily used to follow conventional training
loop when scheduler config is specified in the deepspeed config file.

## MegatronLMDummyDataLoader[[accelerate.utils.MegatronLMDummyDataLoader]]

#### accelerate.utils.MegatronLMDummyDataLoader[[accelerate.utils.MegatronLMDummyDataLoader]]

```python
accelerate.utils.MegatronLMDummyDataLoader(**dataset_kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L162)

**Parameters:**

- ****dataset_kwargs** : Megatron data arguments.

Dummy dataloader presents model parameters or param groups, this is primarily used to follow conventional training

## AbstractTrainStep[[accelerate.utils.AbstractTrainStep]]

#### accelerate.utils.AbstractTrainStep[[accelerate.utils.AbstractTrainStep]]

```python
accelerate.utils.AbstractTrainStep(name)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L415)

Abstract class for batching, forward pass and loss handler.

## GPTTrainStep[[accelerate.utils.GPTTrainStep]]

#### accelerate.utils.GPTTrainStep[[accelerate.utils.GPTTrainStep]]

```python
accelerate.utils.GPTTrainStep(accelerator, args)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L574)

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

GPT train step class.

## BertTrainStep[[accelerate.utils.BertTrainStep]]

#### accelerate.utils.BertTrainStep[[accelerate.utils.BertTrainStep]]

```python
accelerate.utils.BertTrainStep(accelerator, args)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L432)

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

Bert train step class.

## T5TrainStep[[accelerate.utils.T5TrainStep]]

#### accelerate.utils.T5TrainStep[[accelerate.utils.T5TrainStep]]

```python
accelerate.utils.T5TrainStep(accelerator, args)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L718)

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

T5 train step class.

## avg_losses_across_data_parallel_group[[accelerate.utils.avg_losses_across_data_parallel_group]]

#### accelerate.utils.avg_losses_across_data_parallel_group[[accelerate.utils.avg_losses_across_data_parallel_group]]

```python
accelerate.utils.avg_losses_across_data_parallel_group(losses)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/utils/megatron_lm.py#L1217)

**Parameters:**

losses (List[Tensor]) : List of losses to average across data parallel group.

Average losses across data parallel group.
