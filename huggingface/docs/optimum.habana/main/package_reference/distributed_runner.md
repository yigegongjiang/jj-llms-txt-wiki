# DistributedRunner[[optimum.habana.distributed.DistributedRunner]]

#### optimum.habana.distributed.DistributedRunner[[optimum.habana.distributed.DistributedRunner]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L32)

Set up training/inference hardware configurations and run distributed commands.

create_multi_node_setupoptimum.habana.distributed.DistributedRunner.create_multi_node_setuphttps://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L188[]

Multi-node configuration setup for DeepSpeed.
#### create_single_card_setup[[optimum.habana.distributed.DistributedRunner.create_single_card_setup]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L147)

Single-card setup.
#### create_single_node_setup[[optimum.habana.distributed.DistributedRunner.create_single_node_setup]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L177)

Single-node multi-card configuration setup.
#### create_single_node_setup_deepspeed[[optimum.habana.distributed.DistributedRunner.create_single_node_setup_deepspeed]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L168)

Single-node multi-card configuration setup for DeepSpeed.
#### create_single_node_setup_mpirun[[optimum.habana.distributed.DistributedRunner.create_single_node_setup_mpirun]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L157)

Single-node multi-card configuration setup for mpirun.
#### process_hostfile[[optimum.habana.distributed.DistributedRunner.process_hostfile]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L230)

Returns the master address to use for multi-node runs with DeepSpeed.
Directly inspired from https://github.com/microsoft/DeepSpeed/blob/316c4a43e0802a979951ee17f735daf77ea9780f/deepspeed/autotuning/utils.py#L145.

**Returns:**

`str`

address of the master node.
#### run[[optimum.habana.distributed.DistributedRunner.run]]

[Source](https://github.com/huggingface/optimum-habana/blob/main/optimum/habana/distributed/distributed_runner.py#L196)

Runs the desired command with configuration specified by the user.
