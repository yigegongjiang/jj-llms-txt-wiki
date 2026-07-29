# Distributed training with Optimum for Intel Gaudi

As models get bigger, parallelism has emerged as a strategy for training larger models on limited hardware and accelerating training speed by several orders of magnitude.

All the [PyTorch examples](/examples) and the `GaudiTrainer` script work out of the box with distributed training.
There are two ways of launching them:

1. Using the [gaudi_spawn.py](https://github.com/huggingface/optimum-habana/blob/main/examples/gaudi_spawn.py) script:

   - Use MPI for distributed training:

     ```bash
     python gaudi_spawn.py \
         --world_size number_of_hpu_you_have --use_mpi \
         path_to_script.py --args1 --args2 ... --argsN
     ```

     where `--argX` is an argument of the script to run in a distributed way.
     Examples are given for question answering [here](https://github.com/huggingface/optimum-habana/blob/main/examples/question-answering/README.md#multi-card-training) and text classification [here](/examples/text-classification#multi-card-training).

   - Use DeepSpeed for distributed training:

     ```bash
     python gaudi_spawn.py \
         --world_size number_of_hpu_you_have --use_deepspeed \
         path_to_script.py --args1 --args2 ... --argsN
     ```

     where `--argX` is an argument of the script to run in a distributed way.
     Examples are given for question answering [here](https://github.com/huggingface/optimum-habana/blob/main/examples/question-answering/README.md#using-deepspeed) and text classification [here](/examples/text-classification#using-deepspeed).

2. Using the `DistributedRunner` directly in code:

   ```python
   from optimum.habana.distributed import DistributedRunner
   from optimum.utils import logging

   world_size=8 # Number of HPUs to use (1 or 8)

   # define distributed runner
   distributed_runner = DistributedRunner(
       command_list=["scripts/train.py --args1 --args2 ... --argsN"],
       world_size=world_size,
       use_mpi=True,
   )

   # start job
   ret_code = distributed_runner.run()
   ```

You can set the training argument `--distribution_strategy fast_ddp` for simpler and usually faster distributed training management. More information [here](../usage_guides/accelerate_training#fast-ddp).

To go further, we invite you to read our guides about:
- [Accelerating training](../usage_guides/accelerate_training)
- [Pretraining](../usage_guides/pretraining)
- [DeepSpeed](../usage_guides/deepspeed) to train bigger models
- [Multi-node training](../usage_guides/multi_node_training) to speed up even more your distributed runs
