# Token Classification

Token classification is the task of classifying each token in a sequence. This can be used
for Named Entity Recognition (NER), Part-of-Speech (POS) tagging, and more. Get your data ready in
proper format and then with just a few clicks, your state-of-the-art model will be ready to
be used in production.

## Data Format

The data should be in the following CSV format:

```csv
tokens,tags
"['I', 'love', 'Paris']","['O', 'O', 'B-LOC']"
"['I', 'live', 'in', 'New', 'York']","['O', 'O', 'O', 'B-LOC', 'I-LOC']"
.
.
.
```

or you can also use JSONL format:

```json
{"tokens": ["I", "love", "Paris"],"tags": ["O", "O", "B-LOC"]}
{"tokens": ["I", "live", "in", "New", "York"],"tags": ["O", "O", "O", "B-LOC", "I-LOC"]}
.
.
.
```

As you can see, we have two columns in the CSV file. One column is the tokens and the other
is the tags. Both the columns are stringified lists! The tokens column contains the tokens
of the sentence and the tags column contains the tags for each token.

If your CSV is huge, you can divide it into multiple CSV files and upload them separately.
Please make sure that the column names are the same in all CSV files.

One way to divide the CSV file using pandas is as follows:

```python
import pandas as pd

# Set the chunk size
chunk_size = 1000
i = 1

# Open the CSV file and read it in chunks
for chunk in pd.read_csv('example.csv', chunksize=chunk_size):
    # Save each chunk to a new file
    chunk.to_csv(f'chunk_{i}.csv', index=False)
    i += 1
```

Sample dataset from HuggingFace Hub: [conll2003](https://huggingface.co/datasets/eriktks/conll2003)

## Columns

Your CSV/JSONL dataset must have two columns: `tokens` and `tags`.

## Parameters[[autotrain.trainers.token_classification.params.TokenClassificationParams]]

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
