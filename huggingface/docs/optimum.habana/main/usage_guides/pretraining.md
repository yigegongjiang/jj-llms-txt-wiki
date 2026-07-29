# Pretraining Transformers with Optimum for Intel Gaudi

Pretraining a model from Transformers, like BERT, is as easy as fine-tuning it.
The model should be instantiated from a configuration with `.from_config` and not from a pretrained checkpoint with `.from_pretrained`.
Here is how it should look with GPT2 for instance:
```python
from transformers import AutoConfig, AutoModelForXXX

config = AutoConfig.from_pretrained("gpt2")
model = AutoModelForXXX.from_config(config)
```
with XXX the task to perform, such as `ImageClassification` for example.

The following is a working example where BERT is pretrained for masked language modeling:
```python
from datasets import load_dataset
from optimum.habana import GaudiTrainer, GaudiTrainingArguments
from transformers import AutoConfig, AutoModelForMaskedLM, AutoTokenizer, DataCollatorForLanguageModeling

# Load the training set (this one has already been preprocessed)
training_set = load_dataset("philschmid/processed_bert_dataset", split="train")
# Load the tokenizer
tokenizer = AutoTokenizer.from_pretrained("philschmid/bert-base-uncased-2022-habana")

# Instantiate an untrained model
config = AutoConfig.from_pretrained("bert-base-uncased")
model = AutoModelForMaskedLM.from_config(config)

model.resize_token_embeddings(len(tokenizer))

# The data collator will take care of randomly masking the tokens
data_collator = DataCollatorForLanguageModeling(tokenizer=tokenizer)

training_args = GaudiTrainingArguments(
    output_dir="/tmp/bert-base-uncased-mlm",
    num_train_epochs=1,
    per_device_train_batch_size=8,
    use_habana=True,
    use_lazy_mode=True,
    gaudi_config_name="Habana/bert-base-uncased",
)

# Initialize our Trainer
trainer = GaudiTrainer(
    model=model,
    args=training_args,
    train_dataset=training_set,
    tokenizer=tokenizer,
    data_collator=data_collator,
)

trainer.train()
```

You can see another example of pretraining in [this blog post](https://huggingface.co/blog/pretraining-bert).
