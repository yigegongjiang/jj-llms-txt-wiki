# Multitask prompt tuning

[Multitask prompt tuning](https://huggingface.co/papers/2303.02861)  decomposes the soft prompts of each task into a single learned transferable prompt instead of a separate prompt for each task. The single learned prompt can be adapted for each task by multiplicative low rank updates.

The abstract from the paper is:

*Prompt tuning, in which a base pretrained model is adapted to each task via conditioning on learned prompt vectors, has emerged as a promising approach for efficiently adapting large language models to multiple downstream tasks. However, existing methods typically learn soft prompt vectors from scratch, and it has not been clear how to exploit the rich cross-task knowledge with prompt vectors in a multitask learning setting. We propose multitask prompt tuning (MPT), which first learns a single transferable prompt by distilling knowledge from multiple task-specific source prompts. We then learn multiplicative low rank updates to this shared prompt to efficiently adapt it to each downstream target task. Extensive experiments on 23 NLP datasets demonstrate that our proposed approach outperforms the state-of-the-art methods, including the full finetuning baseline in some cases, despite only tuning 0.035% as many task-specific parameters*.

    

Multitask prompt tuning enables parameter-efficient transfer learning.

MPT consists of two stages:

1. source training - for each task, its soft prompt is decomposed into task-specific vectors. The task-specific vectors are multiplied together to form another matrix W, and the Hadamard product is used between W and a shared prompt matrix P to generate a task-specific prompt matrix. The task-specific prompts are distilled into a single prompt matrix that is shared across all tasks. This prompt is trained with multitask training.
2. target adaptation - to adapt the single prompt for a target task, a target prompt is initialized and expressed as the Hadamard product of the shared prompt matrix and the task-specific low-rank prompt matrix.

    

Prompt decomposition.

## Benchmark overview

There is no benchmark for this method yet. Feel free to contribute an experiment
configuration but make sure to first create an issue
[here](https://github.com/huggingface/peft/issues).

# API

## MultitaskPromptTuningConfig[[peft.MultitaskPromptTuningConfig]]

"}, {"name": "prompt_tuning_init_text", "val": ": typing.Optional[str] = None"}, {"name": "tokenizer_name_or_path", "val": ": typing.Optional[str] = None"}, {"name": "tokenizer_kwargs", "val": ": typing.Optional[dict] = None"}, {"name": "prompt_tuning_init_state_dict_path", "val": ": typing.Optional[str] = None"}, {"name": "prompt_tuning_init_task", "val": ": typing.Optional[int] = 0"}, {"name": "num_ranks", "val": ": typing.Optional[int] = 1"}, {"name": "num_tasks", "val": ": typing.Optional[int] = 1"}]}>

## MultitaskPromptEmbedding[[peft.tuners.MultitaskPromptEmbedding]]
