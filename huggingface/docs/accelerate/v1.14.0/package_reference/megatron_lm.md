# Megatron-LM utilities

## MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

#### accelerate.utils.MegatronLMPlugin[[accelerate.utils.MegatronLMPlugin]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/dataclasses.py#L2316)

Plugin for Megatron-LM to enable tensor, pipeline, sequence and data parallelism. Also to enable selective
activation recomputation and optimized fused kernels.

**Parameters:**

tp_degree (`int`, defaults to `None`) : Tensor parallelism degree.

pp_degree (`int`, defaults to `None`) : Pipeline parallelism degree.

num_micro_batches (`int`, defaults to `None`) : Number of micro-batches.

gradient_clipping (`float`, defaults to `None`) : Gradient clipping value based on global L2 Norm (0 to disable).

sequence_parallelism (`bool`, defaults to `None`) : Enable sequence parallelism.

recompute_activations (`bool`, defaults to `None`) : Enable selective activation recomputation.

use_distributed_optimizr (`bool`, defaults to `None`) : Enable distributed optimizer.

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

## MegatronLMDummyScheduler[[accelerate.utils.MegatronLMDummyScheduler]]

#### accelerate.utils.MegatronLMDummyScheduler[[accelerate.utils.MegatronLMDummyScheduler]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L378)

Dummy scheduler presents model parameters or param groups, this is primarily used to follow conventional training
loop when scheduler config is specified in the deepspeed config file.

**Parameters:**

optimizer (`torch.optim.optimizer.Optimizer`) : The optimizer to wrap.

total_num_steps (int) : Total number of steps.

warmup_num_steps (int) : Number of steps for warmup.

- ****kwargs** (additional keyword arguments, *optional*) : Other arguments.

## MegatronLMDummyDataLoader[[accelerate.utils.MegatronLMDummyDataLoader]]

#### accelerate.utils.MegatronLMDummyDataLoader[[accelerate.utils.MegatronLMDummyDataLoader]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L162)

Dummy dataloader presents model parameters or param groups, this is primarily used to follow conventional training

**Parameters:**

- ****dataset_kwargs** : Megatron data arguments.

## AbstractTrainStep[[accelerate.utils.AbstractTrainStep]]

#### accelerate.utils.AbstractTrainStep[[accelerate.utils.AbstractTrainStep]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L415)

Abstract class for batching, forward pass and loss handler.

## GPTTrainStep[[accelerate.utils.GPTTrainStep]]

#### accelerate.utils.GPTTrainStep[[accelerate.utils.GPTTrainStep]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L574)

GPT train step class.

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

## BertTrainStep[[accelerate.utils.BertTrainStep]]

#### accelerate.utils.BertTrainStep[[accelerate.utils.BertTrainStep]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L432)

Bert train step class.

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

## T5TrainStep[[accelerate.utils.T5TrainStep]]

#### accelerate.utils.T5TrainStep[[accelerate.utils.T5TrainStep]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L718)

T5 train step class.

**Parameters:**

args (`argparse.Namespace`) : Megatron-LM arguments.

## avg_losses_across_data_parallel_group[[accelerate.utils.avg_losses_across_data_parallel_group]]

#### accelerate.utils.avg_losses_across_data_parallel_group[[accelerate.utils.avg_losses_across_data_parallel_group]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/utils/megatron_lm.py#L1217)

Average losses across data parallel group.

**Parameters:**

losses (List[Tensor]) : List of losses to average across data parallel group.
