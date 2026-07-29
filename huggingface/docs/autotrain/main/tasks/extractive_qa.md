# Extractive Question Answering with AutoTrain

Extractive Question Answering (QA) enables AI models to find and extract precise answers from text passages. This guide shows you how to train custom QA models using AutoTrain, supporting popular architectures like BERT, RoBERTa, and DeBERTa.

## What is Extractive Question Answering?

Extractive QA models learn to:
- Locate exact answer spans within longer text passages
- Understand questions and match them to relevant context
- Extract precise answers rather than generating them
- Handle both simple and complex queries about the text

## Preparing your Data

Your dataset needs these essential columns:
- `text`: The passage containing potential answers (also called context)
- `question`: The query you want to answer
- `answer`: Answer span information including text and position

Here is an example of how your dataset should look:

```
{"context":"Architecturally, the school has a Catholic character. Atop the Main Building's gold dome is a golden statue of the Virgin Mary. Immediately in front of the Main Building and facing it, is a copper statue of Christ with arms upraised with the legend \"Venite Ad Me Omnes\". Next to the Main Building is the Basilica of the Sacred Heart. Immediately behind the basilica is the Grotto, a Marian place of prayer and reflection. It is a replica of the grotto at Lourdes, France where the Virgin Mary reputedly appeared to Saint Bernadette Soubirous in 1858. At the end of the main drive (and in a direct line that connects through 3 statues and the Gold Dome), is a simple, modern stone statue of Mary.","question":"To whom did the Virgin Mary allegedly appear in 1858 in Lourdes France?","answers":{"text":["Saint Bernadette Soubirous"],"answer_start":[515]}}
{"context":"Architecturally, the school has a Catholic character. Atop the Main Building's gold dome is a golden statue of the Virgin Mary. Immediately in front of the Main Building and facing it, is a copper statue of Christ with arms upraised with the legend \"Venite Ad Me Omnes\". Next to the Main Building is the Basilica of the Sacred Heart. Immediately behind the basilica is the Grotto, a Marian place of prayer and reflection. It is a replica of the grotto at Lourdes, France where the Virgin Mary reputedly appeared to Saint Bernadette Soubirous in 1858. At the end of the main drive (and in a direct line that connects through 3 statues and the Gold Dome), is a simple, modern stone statue of Mary.","question":"What is in front of the Notre Dame Main Building?","answers":{"text":["a copper statue of Christ"],"answer_start":[188]}}
{"context":"Architecturally, the school has a Catholic character. Atop the Main Building's gold dome is a golden statue of the Virgin Mary. Immediately in front of the Main Building and facing it, is a copper statue of Christ with arms upraised with the legend \"Venite Ad Me Omnes\". Next to the Main Building is the Basilica of the Sacred Heart. Immediately behind the basilica is the Grotto, a Marian place of prayer and reflection. It is a replica of the grotto at Lourdes, France where the Virgin Mary reputedly appeared to Saint Bernadette Soubirous in 1858. At the end of the main drive (and in a direct line that connects through 3 statues and the Gold Dome), is a simple, modern stone statue of Mary.","question":"The Basilica of the Sacred heart at Notre Dame is beside to which structure?","answers":{"text":["the Main Building"],"answer_start":[279]}}
```

Note: the preferred format for question answering is JSONL, if you want to use CSV, the `answer` column should be stringified JSON with the keys `text` and `answer_start`.

Example dataset from Hugging Face Hub: [lhoestq/squad](https://huggingface.co/datasets/lhoestq/squad)

P.S. You can use both squad and squad v2 data format with correct column mappings.

## Training Options

### Local Training
Train models on your own hardware with full control over the process.

To train an Extractive QA model locally, you need a config file:

```yaml
task: extractive-qa
base_model: google-bert/bert-base-uncased
project_name: autotrain-bert-ex-qa1
log: tensorboard
backend: local

data:
  path: lhoestq/squad
  train_split: train
  valid_split: validation
  column_mapping:
    text_column: context
    question_column: question
    answer_column: answers

params:
  max_seq_length: 512
  max_doc_stride: 128
  epochs: 3
  batch_size: 4
  lr: 2e-5
  optimizer: adamw_torch
  scheduler: linear
  gradient_accumulation: 1
  mixed_precision: fp16

hub:
  username: ${HF_USERNAME}
  token: ${HF_TOKEN}
  push_to_hub: true
```

To train the model, run the following command:

```bash
$ autotrain --config config.yaml
```

Here, we are training a BERT model on the SQuAD dataset using the Extractive QA task. The model is trained for 3 epochs with a batch size of 4 and a learning rate of 2e-5. The training process is logged using TensorBoard. The model is trained locally and pushed to the Hugging Face Hub after training.

### Cloud Training on Hugging Face
Train models using Hugging Face's cloud infrastructure for better scalability.

![AutoTrain Extractive Question Answering on Hugging Face Spaces](https://raw.githubusercontent.com/huggingface/autotrain-advanced/main/static/ext_qa.png)

As always, pay special attention to column mapping.

## Parameter Reference[[autotrain.trainers.extractive_question_answering.params.ExtractiveQuestionAnsweringParams]]

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
