# Neuron TRL Trainers

[TRL](https://huggingface.co/docs/trl/en/index)-compatible trainers for AWS Trainium accelerators.

## NeuronSFTTrainer

### NeuronSFTConfig[[optimum.neuron.NeuronSFTConfig]]

#### optimum.neuron.NeuronSFTConfig[[optimum.neuron.NeuronSFTConfig]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/sft_config.py#L34)

Configuration class for Neuron-optimized SFT training.

Inherits from both NeuronTrainingArguments (for Trainium-specific settings) and
trl's SFTConfig (for SFT-specific settings).

Key Neuron-specific behavior:
- padding_free is always set to False to avoid recompilation on Trainium devices
- All other SFT parameters from trl 0.24.0+ are supported

### NeuronSFTTrainer[[optimum.neuron.NeuronSFTTrainer]]

#### optimum.neuron.NeuronSFTTrainer[[optimum.neuron.NeuronSFTTrainer]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/sft_trainer.py#L132)

`SFTTrainer` adapted for Neuron (Trainium) devices.

compute_lossoptimum.neuron.NeuronSFTTrainer.compute_losshttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/sft_trainer.py#L403[{"name": "model", "val": ""}, {"name": "inputs", "val": ""}, {"name": "return_outputs", "val": " = False"}, {"name": "num_items_in_batch", "val": " = None"}]

Compute training loss for Neuron-optimized training.
#### log[[optimum.neuron.NeuronSFTTrainer.log]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/sft_trainer.py#L387)

Override SFTTrainer's log method to use NeuronTrainer's implementation.

SFTTrainer has custom metrics tracking that we don't use for Neuron training.
#### training_step[[optimum.neuron.NeuronSFTTrainer.training_step]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/trainers/sft_trainer.py#L413)

Perform a training step for Neuron-optimized training.
