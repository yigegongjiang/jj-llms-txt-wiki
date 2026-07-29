# Quickstart with Python

AutoTrain is a library that allows you to train state of the art models on Hugging Face Spaces, or locally. 
It provides a simple and easy-to-use interface to train models for various tasks like llm finetuning, text classification, 
image classification, object detection, and more.

In this quickstart guide, we will show you how to train a model using AutoTrain in Python.

## Getting Started

AutoTrain can be installed using pip:

```bash
$ pip install autotrain-advanced
```

The example code below shows how to finetune an LLM model using AutoTrain in Python:

```python
import os

from autotrain.params import LLMTrainingParams
from autotrain.project import AutoTrainProject

params = LLMTrainingParams(
    model="meta-llama/Llama-3.2-1B-Instruct",
    data_path="HuggingFaceH4/no_robots",
    chat_template="tokenizer",
    text_column="messages",
    train_split="train",
    trainer="sft",
    epochs=3,
    batch_size=1,
    lr=1e-5,
    peft=True,
    quantization="int4",
    target_modules="all-linear",
    padding="right",
    optimizer="paged_adamw_8bit",
    scheduler="cosine",
    gradient_accumulation=8,
    mixed_precision="bf16",
    merge_adapter=True,
    project_name="autotrain-llama32-1b-finetune",
    log="tensorboard",
    push_to_hub=True,
    username=os.environ.get("HF_USERNAME"),
    token=os.environ.get("HF_TOKEN"),
)

backend = "local"
project = AutoTrainProject(params=params, backend=backend, process=True)
project.create()
```

In this example, we are finetuning the `meta-llama/Llama-3.2-1B-Instruct` model on the `HuggingFaceH4/no_robots` dataset.
We are training the model for 3 epochs with a batch size of 1 and a learning rate of `1e-5`.
We are using the `paged_adamw_8bit` optimizer and the `cosine` scheduler.
We are also using mixed precision training with a gradient accumulation of 8.
The final model will be pushed to the Hugging Face Hub after training.

To train the model, run the following command:

```bash
$ export HF_USERNAME=
$ export HF_TOKEN=
$ python train.py
```

This will create a new project directory with the name `autotrain-llama32-1b-finetune` and start the training process.
Once the training is complete, the model will be pushed to the Hugging Face Hub.

Your HF_TOKEN and HF_USERNAME are only required if you want to push the model or if you are accessing a gated model or dataset.

## AutoTrainProject Class[[autotrain.project.AutoTrainProject]]

#### autotrain.project.AutoTrainProject[[autotrain.project.AutoTrainProject]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/project.py#L444)

A class to train an AutoTrain project

Attributes
----------
params : Union[
LLMTrainingParams,
TextClassificationParams,
TabularParams,
Seq2SeqParams,
ImageClassificationParams,
TextRegressionParams,
ObjectDetectionParams,
TokenClassificationParams,
SentenceTransformersParams,
ImageRegressionParams,
ExtractiveQuestionAnsweringParams,
VLMTrainingParams,
]
The parameters for the AutoTrain project.
backend : str
The backend to be used for the AutoTrain project. It should be one of the following:
- local
- spaces-a10g-large
- spaces-a10g-small
- spaces-a100-large
- spaces-t4-medium
- spaces-t4-small
- spaces-cpu-upgrade
- spaces-cpu-basic
- spaces-l4x1
- spaces-l4x4
- spaces-l40sx1
- spaces-l40sx4
- spaces-l40sx8
- spaces-a10g-largex2
- spaces-a10g-largex4
process : bool
Flag to indicate if the params and dataset should be processed. If your data format is not AutoTrain-readable, set it to True. Set it to True when in doubt. Defaults to False.

Methods
-------
__post_init__():
Validates the backend attribute.
create():
Creates a runner based on the backend and initializes the AutoTrain project.

## Parameters

### Text Tasks[[autotrain.trainers.clm.params.LLMTrainingParams]]

#### autotrain.trainers.clm.params.LLMTrainingParams[[autotrain.trainers.clm.params.LLMTrainingParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/clm/params.py#L8)

LLMTrainingParams: Parameters for training a language model using the autotrain library.

**Parameters:**

model (str) : Model name to be used for training. Default is "gpt2".

project_name (str) : Name of the project and output directory. Default is "project-name". 

data_path (str) : Path to the dataset. Default is "data".

train_split (str) : Configuration for the training data split. Default is "train".

valid_split (Optional[str]) : Configuration for the validation data split. Default is None.

add_eos_token (bool) : Whether to add an EOS token at the end of sequences. Default is True.

block_size (Union[int, List[int]]) : Size of the blocks for training, can be a single integer or a list of integers. Default is -1.

model_max_length (int) : Maximum length of the model input. Default is 2048.

padding (Optional[str]) : Side on which to pad sequences (left or right). Default is "right". 

trainer (str) : Type of trainer to use. Default is "default".

use_flash_attention_2 (bool) : Whether to use flash attention version 2. Default is False.

log (str) : Logging method for experiment tracking. Default is "none".

disable_gradient_checkpointing (bool) : Whether to disable gradient checkpointing. Default is False.

logging_steps (int) : Number of steps between logging events. Default is -1.

eval_strategy (str) : Strategy for evaluation (e.g., 'epoch'). Default is "epoch".

save_total_limit (int) : Maximum number of checkpoints to keep. Default is 1.

auto_find_batch_size (bool) : Whether to automatically find the optimal batch size. Default is False.

mixed_precision (Optional[str]) : Type of mixed precision to use (e.g., 'fp16', 'bf16', or None). Default is None.

lr (float) : Learning rate for training. Default is 3e-5.

epochs (int) : Number of training epochs. Default is 1.

batch_size (int) : Batch size for training. Default is 2.

warmup_ratio (float) : Proportion of training to perform learning rate warmup. Default is 0.1.

gradient_accumulation (int) : Number of steps to accumulate gradients before updating. Default is 4.

optimizer (str) : Optimizer to use for training. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler to use. Default is "linear".

weight_decay (float) : Weight decay to apply to the optimizer. Default is 0.0.

max_grad_norm (float) : Maximum norm for gradient clipping. Default is 1.0.

seed (int) : Random seed for reproducibility. Default is 42.

chat_template (Optional[str]) : Template for chat-based models, options include: None, zephyr, chatml, or tokenizer. Default is None. 

quantization (Optional[str]) : Quantization method to use (e.g., 'int4', 'int8', or None). Default is "int4".

target_modules (Optional[str]) : Target modules for quantization or fine-tuning. Default is "all-linear".

merge_adapter (bool) : Whether to merge the adapter layers. Default is False.

peft (bool) : Whether to use Parameter-Efficient Fine-Tuning (PEFT). Default is False.

lora_r (int) : Rank of the LoRA matrices. Default is 16.

lora_alpha (int) : Alpha parameter for LoRA. Default is 32.

lora_dropout (float) : Dropout rate for LoRA. Default is 0.05. 

model_ref (Optional[str]) : Reference model for DPO trainer. Default is None.

dpo_beta (float) : Beta parameter for DPO trainer. Default is 0.1. 

max_prompt_length (int) : Maximum length of the prompt. Default is 128.

max_completion_length (Optional[int]) : Maximum length of the completion. Default is None. 

prompt_text_column (Optional[str]) : Column name for the prompt text. Default is None.

text_column (str) : Column name for the text data. Default is "text".

rejected_text_column (Optional[str]) : Column name for the rejected text data. Default is None. 

push_to_hub (bool) : Whether to push the model to the Hugging Face Hub. Default is False.

username (Optional[str]) : Hugging Face username for authentication. Default is None.

token (Optional[str]) : Hugging Face token for authentication. Default is None. 

unsloth (bool) : Whether to use the unsloth library. Default is False.

distributed_backend (Optional[str]) : Backend to use for distributed training. Default is None.

#### autotrain.trainers.sent_transformers.params.SentenceTransformersParams[[autotrain.trainers.sent_transformers.params.SentenceTransformersParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/sent_transformers/params.py#L8)

SentenceTransformersParams is a configuration class for setting up parameters for training sentence transformers.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the pre-trained model to use. Default is "microsoft/mpnet-base".

lr (float) : Learning rate for training. Default is 3e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length for the input. Default is 128.

batch_size (int) : Batch size for training. Default is 8.

warmup_ratio (float) : Proportion of training to perform learning rate warmup. Default is 0.1.

gradient_accumulation (int) : Number of steps to accumulate gradients before updating. Default is 1.

optimizer (str) : Optimizer to use. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler to use. Default is "linear".

weight_decay (float) : Weight decay to apply. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm for clipping. Default is 1.0.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split. Default is None.

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find the optimal batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision training mode (fp16, bf16, or None). Default is None.

save_total_limit (int) : Maximum number of checkpoints to save. Default is 1.

token (Optional[str]) : Token for accessing Hugging Face Hub. Default is None.

push_to_hub (bool) : Whether to push the model to Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy to use. Default is "epoch".

username (Optional[str]) : Hugging Face username. Default is None.

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Number of epochs with no improvement after which training will be stopped. Default is 5.

early_stopping_threshold (float) : Threshold for measuring the new optimum, to qualify as an improvement. Default is 0.01.

trainer (str) : Name of the trainer to use. Default is "pair_score".

sentence1_column (str) : Name of the column containing the first sentence. Default is "sentence1".

sentence2_column (str) : Name of the column containing the second sentence. Default is "sentence2".

sentence3_column (Optional[str]) : Name of the column containing the third sentence (if applicable). Default is None.

target_column (Optional[str]) : Name of the column containing the target variable. Default is None.

#### autotrain.trainers.seq2seq.params.Seq2SeqParams[[autotrain.trainers.seq2seq.params.Seq2SeqParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/seq2seq/params.py#L8)

Seq2SeqParams is a configuration class for sequence-to-sequence training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to be used. Default is "google/flan-t5-base".

username (Optional[str]) : Hugging Face Username.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

project_name (str) : Name of the project or output directory. Default is "project-name".

token (Optional[str]) : Hub Token for authentication.

push_to_hub (bool) : Whether to push the model to the Hugging Face Hub. Default is False.

text_column (str) : Name of the text column in the dataset. Default is "text".

target_column (str) : Name of the target text column in the dataset. Default is "target".

lr (float) : Learning rate for training. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length for input text. Default is 128.

max_target_length (int) : Maximum sequence length for target text. Default is 128.

batch_size (int) : Training batch size. Default is 2.

warmup_ratio (float) : Proportion of warmup steps. Default is 0.1.

gradient_accumulation (int) : Number of gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to be used. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler to be used. Default is "linear".

weight_decay (float) : Weight decay for the optimizer. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm for clipping. Default is 1.0.

logging_steps (int) : Number of steps between logging. Default is -1 (disabled).

eval_strategy (str) : Evaluation strategy. Default is "epoch".

auto_find_batch_size (bool) : Whether to automatically find the batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision training mode (fp16, bf16, or None).

save_total_limit (int) : Maximum number of checkpoints to save. Default is 1.

peft (bool) : Whether to use Parameter-Efficient Fine-Tuning (PEFT). Default is False.

quantization (Optional[str]) : Quantization mode (int4, int8, or None). Default is "int8".

lora_r (int) : LoRA-R parameter for PEFT. Default is 16.

lora_alpha (int) : LoRA-Alpha parameter for PEFT. Default is 32.

lora_dropout (float) : LoRA-Dropout parameter for PEFT. Default is 0.05.

target_modules (str) : Target modules for PEFT. Default is "all-linear".

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Patience for early stopping. Default is 5.

early_stopping_threshold (float) : Threshold for early stopping. Default is 0.01.

#### autotrain.trainers.token_classification.params.TokenClassificationParams[[autotrain.trainers.token_classification.params.TokenClassificationParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/token_classification/params.py#L8)

TokenClassificationParams is a configuration class for token classification training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to use. Default is "bert-base-uncased".

lr (float) : Learning rate. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length. Default is 128.

batch_size (int) : Training batch size. Default is 8.

warmup_ratio (float) : Warmup proportion. Default is 0.1.

gradient_accumulation (int) : Gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to use. Default is "adamw_torch".

scheduler (str) : Scheduler to use. Default is "linear".

weight_decay (float) : Weight decay. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm. Default is 1.0.

seed (int) : Random seed. Default is 42.

train_split (str) : Name of the training split. Default is "train".

valid_split (Optional[str]) : Name of the validation split. Default is None.

tokens_column (str) : Name of the tokens column. Default is "tokens".

tags_column (str) : Name of the tags column. Default is "tags".

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find the batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision setting (fp16, bf16, or None). Default is None.

save_total_limit (int) : Total number of checkpoints to save. Default is 1.

token (Optional[str]) : Hub token for authentication. Default is None.

push_to_hub (bool) : Whether to push the model to the Hugging Face hub. Default is False.

eval_strategy (str) : Evaluation strategy. Default is "epoch".

username (Optional[str]) : Hugging Face username. Default is None.

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Patience for early stopping. Default is 5.

early_stopping_threshold (float) : Threshold for early stopping. Default is 0.01.

#### autotrain.trainers.extractive_question_answering.params.ExtractiveQuestionAnsweringParams[[autotrain.trainers.extractive_question_answering.params.ExtractiveQuestionAnsweringParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/extractive_question_answering/params.py#L8)

ExtractiveQuestionAnsweringParams

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Pre-trained model name. Default is "bert-base-uncased".

lr (float) : Learning rate for the optimizer. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length for inputs. Default is 128.

max_doc_stride (int) : Maximum document stride for splitting context. Default is 128.

batch_size (int) : Batch size for training. Default is 8.

warmup_ratio (float) : Warmup proportion for learning rate scheduler. Default is 0.1.

gradient_accumulation (int) : Number of gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer type. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler type. Default is "linear".

weight_decay (float) : Weight decay for the optimizer. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm for clipping. Default is 1.0.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split. Default is None.

text_column (str) : Column name for context/text. Default is "context".

question_column (str) : Column name for questions. Default is "question".

answer_column (str) : Column name for answers. Default is "answers".

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Automatically find optimal batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision training mode (fp16, bf16, or None). Default is None.

save_total_limit (int) : Maximum number of checkpoints to save. Default is 1.

token (Optional[str]) : Authentication token for Hugging Face Hub. Default is None.

push_to_hub (bool) : Whether to push the model to Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy during training. Default is "epoch".

username (Optional[str]) : Hugging Face username for authentication. Default is None.

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Number of epochs with no improvement for early stopping. Default is 5.

early_stopping_threshold (float) : Threshold for early stopping improvement. Default is 0.01.

#### autotrain.trainers.text_classification.params.TextClassificationParams[[autotrain.trainers.text_classification.params.TextClassificationParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/text_classification/params.py#L8)

`TextClassificationParams` is a configuration class for text classification training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to use. Default is "bert-base-uncased".

lr (float) : Learning rate. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length. Default is 128.

batch_size (int) : Training batch size. Default is 8.

warmup_ratio (float) : Warmup proportion. Default is 0.1.

gradient_accumulation (int) : Number of gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to use. Default is "adamw_torch".

scheduler (str) : Scheduler to use. Default is "linear".

weight_decay (float) : Weight decay. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm. Default is 1.0.

seed (int) : Random seed. Default is 42.

train_split (str) : Name of the training split. Default is "train".

valid_split (Optional[str]) : Name of the validation split. Default is None.

text_column (str) : Name of the text column in the dataset. Default is "text".

target_column (str) : Name of the target column in the dataset. Default is "target".

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find the batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision setting (fp16, bf16, or None). Default is None.

save_total_limit (int) : Total number of checkpoints to save. Default is 1.

token (Optional[str]) : Hub token for authentication. Default is None.

push_to_hub (bool) : Whether to push the model to the hub. Default is False.

eval_strategy (str) : Evaluation strategy. Default is "epoch".

username (Optional[str]) : Hugging Face username. Default is None.

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Number of epochs with no improvement after which training will be stopped. Default is 5.

early_stopping_threshold (float) : Threshold for measuring the new optimum to continue training. Default is 0.01.

#### autotrain.trainers.text_regression.params.TextRegressionParams[[autotrain.trainers.text_regression.params.TextRegressionParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/text_regression/params.py#L8)

TextRegressionParams is a configuration class for setting up text regression training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the pre-trained model to use. Default is "bert-base-uncased".

lr (float) : Learning rate for the optimizer. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

max_seq_length (int) : Maximum sequence length for the inputs. Default is 128.

batch_size (int) : Batch size for training. Default is 8.

warmup_ratio (float) : Proportion of training to perform learning rate warmup. Default is 0.1.

gradient_accumulation (int) : Number of steps to accumulate gradients before updating. Default is 1.

optimizer (str) : Optimizer to use. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler to use. Default is "linear".

weight_decay (float) : Weight decay to apply. Default is 0.0.

max_grad_norm (float) : Maximum norm for the gradients. Default is 1.0.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split. Default is None.

text_column (str) : Name of the column containing text data. Default is "text".

target_column (str) : Name of the column containing target data. Default is "target".

logging_steps (int) : Number of steps between logging. Default is -1 (no logging).

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find the batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision training mode (fp16, bf16, or None). Default is None.

save_total_limit (int) : Maximum number of checkpoints to save. Default is 1.

token (Optional[str]) : Token for accessing Hugging Face Hub. Default is None.

push_to_hub (bool) : Whether to push the model to Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy to use. Default is "epoch".

username (Optional[str]) : Hugging Face username. Default is None.

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Number of epochs with no improvement after which training will be stopped. Default is 5.

early_stopping_threshold (float) : Threshold for measuring the new optimum, to qualify as an improvement. Default is 0.01.

### Image Tasks[[autotrain.trainers.image_classification.params.ImageClassificationParams]]

#### autotrain.trainers.image_classification.params.ImageClassificationParams[[autotrain.trainers.image_classification.params.ImageClassificationParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/image_classification/params.py#L8)

ImageClassificationParams is a configuration class for image classification training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Pre-trained model name or path. Default is "google/vit-base-patch16-224".

username (Optional[str]) : Hugging Face account username.

lr (float) : Learning rate for the optimizer. Default is 5e-5.

epochs (int) : Number of epochs for training. Default is 3.

batch_size (int) : Batch size for training. Default is 8.

warmup_ratio (float) : Warmup ratio for learning rate scheduler. Default is 0.1.

gradient_accumulation (int) : Number of gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer type. Default is "adamw_torch".

scheduler (str) : Learning rate scheduler type. Default is "linear".

weight_decay (float) : Weight decay for the optimizer. Default is 0.0.

max_grad_norm (float) : Maximum gradient norm for clipping. Default is 1.0.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Automatically find optimal batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision training mode (fp16, bf16, or None).

save_total_limit (int) : Maximum number of checkpoints to keep. Default is 1.

token (Optional[str]) : Hugging Face Hub token for authentication.

push_to_hub (bool) : Whether to push the model to Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy during training. Default is "epoch".

image_column (str) : Column name for images in the dataset. Default is "image".

target_column (str) : Column name for target labels in the dataset. Default is "target".

log (str) : Logging method for experiment tracking. Default is "none".

early_stopping_patience (int) : Number of epochs with no improvement for early stopping. Default is 5.

early_stopping_threshold (float) : Threshold for early stopping. Default is 0.01.

#### autotrain.trainers.image_regression.params.ImageRegressionParams[[autotrain.trainers.image_regression.params.ImageRegressionParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/image_regression/params.py#L8)

ImageRegressionParams is a configuration class for image regression training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to use. Default is "google/vit-base-patch16-224".

username (Optional[str]) : Hugging Face Username.

lr (float) : Learning rate. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

batch_size (int) : Training batch size. Default is 8.

warmup_ratio (float) : Warmup proportion. Default is 0.1.

gradient_accumulation (int) : Gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to use. Default is "adamw_torch".

scheduler (str) : Scheduler to use. Default is "linear".

weight_decay (float) : Weight decay. Default is 0.0.

max_grad_norm (float) : Max gradient norm. Default is 1.0.

seed (int) : Random seed. Default is 42.

train_split (str) : Train split name. Default is "train".

valid_split (Optional[str]) : Validation split name.

logging_steps (int) : Logging steps. Default is -1.

project_name (str) : Output directory name. Default is "project-name".

auto_find_batch_size (bool) : Whether to auto find batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision type (fp16, bf16, or None).

save_total_limit (int) : Save total limit. Default is 1.

token (Optional[str]) : Hub Token.

push_to_hub (bool) : Whether to push to hub. Default is False.

eval_strategy (str) : Evaluation strategy. Default is "epoch".

image_column (str) : Image column name. Default is "image".

target_column (str) : Target column name. Default is "target".

log (str) : Logging using experiment tracking. Default is "none".

early_stopping_patience (int) : Early stopping patience. Default is 5.

early_stopping_threshold (float) : Early stopping threshold. Default is 0.01.

#### autotrain.trainers.object_detection.params.ObjectDetectionParams[[autotrain.trainers.object_detection.params.ObjectDetectionParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/object_detection/params.py#L8)

ObjectDetectionParams is a configuration class for object detection training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to be used. Default is "google/vit-base-patch16-224".

username (Optional[str]) : Hugging Face Username.

lr (float) : Learning rate. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

batch_size (int) : Training batch size. Default is 8.

warmup_ratio (float) : Warmup proportion. Default is 0.1.

gradient_accumulation (int) : Gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to be used. Default is "adamw_torch".

scheduler (str) : Scheduler to be used. Default is "linear".

weight_decay (float) : Weight decay. Default is 0.0.

max_grad_norm (float) : Max gradient norm. Default is 1.0.

seed (int) : Random seed. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision type (fp16, bf16, or None).

save_total_limit (int) : Total number of checkpoints to save. Default is 1.

token (Optional[str]) : Hub Token for authentication.

push_to_hub (bool) : Whether to push the model to the Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy. Default is "epoch".

image_column (str) : Name of the image column in the dataset. Default is "image".

objects_column (str) : Name of the target column in the dataset. Default is "objects".

log (str) : Logging method for experiment tracking. Default is "none".

image_square_size (Optional[int]) : Longest size to which the image will be resized, then padded to square. Default is 600.

early_stopping_patience (int) : Number of epochs with no improvement after which training will be stopped. Default is 5.

early_stopping_threshold (float) : Minimum change to qualify as an improvement. Default is 0.01.

### Tabular Tasks[[autotrain.trainers.tabular.params.TabularParams]]

#### autotrain.trainers.tabular.params.TabularParams[[autotrain.trainers.tabular.params.TabularParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/tabular/params.py#L8)

TabularParams is a configuration class for tabular data training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to use. Default is "xgboost".

username (Optional[str]) : Hugging Face Username.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

project_name (str) : Name of the output directory. Default is "project-name".

token (Optional[str]) : Hub Token for authentication.

push_to_hub (bool) : Whether to push the model to the hub. Default is False.

id_column (str) : Name of the ID column. Default is "id".

target_columns (Union[List[str], str]) : Target column(s) in the dataset. Default is ["target"].

categorical_columns (Optional[List[str]]) : List of categorical columns.

numerical_columns (Optional[List[str]]) : List of numerical columns.

task (str) : Type of task (e.g., "classification"). Default is "classification".

num_trials (int) : Number of trials for hyperparameter optimization. Default is 10.

time_limit (int) : Time limit for training in seconds. Default is 600.

categorical_imputer (Optional[str]) : Imputer strategy for categorical columns.

numerical_imputer (Optional[str]) : Imputer strategy for numerical columns.

numeric_scaler (Optional[str]) : Scaler strategy for numerical columns.
