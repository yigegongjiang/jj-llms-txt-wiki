# Sentence Transformers

This task lets you easily train or fine-tune a Sentence Transformer model on your own dataset.

AutoTrain supports the following types of sentence transformer finetuning:

- `pair`: dataset with two sentences: anchor and positive 
- `pair_class`: dataset with two sentences: premise and hypothesis and a target label
- `pair_score`: dataset with two sentences: sentence1 and sentence2 and a target score
- `triplet`: dataset with three sentences: anchor, positive and negative
- `qa`: dataset with two sentences: query and answer

## Data Format

Sentence Transformers finetuning accepts data in CSV/JSONL format. You can also use a dataset from Hugging Face Hub.

### `pair`

For `pair` training, the data should be in the following format:

| anchor | positive |
|--------|----------|
| hello  | hi       |
| how are you | I am fine |
| What is your name? | My name is Abhishek |
| Which is the best programming language? | Python |

### `pair_class`

For `pair_class` training, the data should be in the following format:

| premise | hypothesis | label |
|---------|------------|-------|
| hello   | hi         | 1     |
| how are you | I am fine | 0 |
| What is your name? | My name is Abhishek | 1 |
| Which is the best programming language? | Python | 1 |

### `pair_score`

For `pair_score` training, the data should be in the following format:

| sentence1 | sentence2 | score |
|-----------|-----------|-------|
| hello     | hi        | 0.8   |
| how are you | I am fine | 0.2 |
| What is your name? | My name is Abhishek | 0.9 |
| Which is the best programming language? | Python | 0.7 |

### `triplet`

For `triplet` training, the data should be in the following format:

| anchor | positive | negative |
|--------|----------|----------|
| hello  | hi       | bye      |
| how are you | I am fine | I am not fine |
| What is your name? | My name is Abhishek | Whats it to you? | 
| Which is the best programming language? | Python | Javascript |

### `qa`

For `qa` training, the data should be in the following format:

| query | answer |
|-------|--------|
| hello | hi     |
| how are you | I am fine |
| What is your name? | My name is Abhishek |
| Which is the best programming language? | Python |

## Parameters[[autotrain.trainers.sent_transformers.params.SentenceTransformersParams]]
    
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
