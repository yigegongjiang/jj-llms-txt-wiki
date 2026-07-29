# Getting started with AWS Trainium and Hugging Face Transformers

*This tutorial is available in two different formats, as [web page](https://huggingface.co/docs/optimum-neuron/training_tutorials/fine_tune_bert) and [notebook version](https://github.com/huggingface/optimum-neuron/blob/main/notebooks/text-classification/fine_tune_bert.ipynb)*.

This guide will help you to get started with [AWS Trainium](https://aws.amazon.com/machine-learning/trainium/?nc1=h_ls) and Hugging Face Transformers. It will cover how to set up a Trainium instance on AWS, load & fine-tune a transformers model for text-classification.

You will learn how to:

1. Setup AWS environment
2. Load and process the dataset
3. Fine-tune BERT using Hugging Face Transformers and Optimum Neuron

Before we can start, make sure you have a [Hugging Face Account](https://huggingface.co/join) to save artifacts and experiments.

## Quick intro: AWS Trainium

[AWS Trainium (Trn1)](https://aws.amazon.com/de/ec2/instance-types/trn1/) is a purpose-built EC2 for deep learning (DL) training workloads. Trainium is the successor of [AWS Inferentia](https://aws.amazon.com/ec2/instance-types/inf1/?nc1=h_ls) focused on high-performance training workloads claiming up to 50% cost-to-train savings over comparable GPU-based instances.

Trainium has been optimized for training natural language processing, computer vision, and recommender models used. The accelerator supports a wide range of data types, including FP32, TF32, BF16, FP16, UINT8, and configurable FP8.

The biggest Trainium instance, the `trn1.32xlarge` comes with over 500GB of memory, making it easy to fine-tune ~10B parameter models on a single instance. Below you will find an overview of the available instance types. More details [here](https://aws.amazon.com/en/ec2/instance-types/trn1/#Product_details):

| instance size | accelerators | accelerator memory | vCPU | CPU Memory | price per hour |
| --- | --- | --- | --- | --- | --- |
| trn1.2xlarge | 1 | 32 | 8 | 32 | $1.34 |
| trn1.32xlarge | 16 | 512 | 128 | 512 | $21.50 |
| trn1n.32xlarge (2x bandwidth) | 16 | 512 | 128 | 512 | $24.78 |

---

Now we know what Trainium offers, let's get started. 🚀

*Note: This tutorial was created on a trn1.2xlarge AWS EC2 Instance.* 

## 1. Setup AWS environment

In this tutorial, we will use the `trn1.2xlarge` instance on AWS with 1 Accelerator, including two Neuron Cores and the [Hugging Face Neuron Deep Learning AMI](https://aws.amazon.com/marketplace/pp/prodview-gr3e6yiscria2).

Once the instance is up and running, we can ssh into it. But instead of developing inside a terminal we want to use a `Jupyter` environment, which we can use for preparing our dataset and launching the training. For this, we need to add a port for forwarding in the `ssh` command, which will tunnel our localhost traffic to the Trainium instance.

```bash
PUBLIC_DNS="" # IP address, e.g. ec2-3-80-....
KEY_PATH="" # local path to key, e.g. ssh/trn.pem

ssh -L 8080:localhost:8080 -i ${KEY_NAME}.pem ubuntu@$PUBLIC_DNS
```

We need to make sure we have the  `training` extra installed, to get all the necessary dependencies:

```bash
python -m pip install .[training]
```

We can now start our **`jupyter`** server.

```bash
python -m notebook --allow-root --port=8080
```

You should see a familiar **`jupyter`** output with a URL to the notebook.

**`http://localhost:8080/?token=8c1739aff1755bd7958c4cfccc8d08cb5da5234f61f129a9`**

We can click on it, and a **`jupyter`** environment opens in our local browser.

![jupyter.webp](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/optimum/neuron/tutorial-fine-tune-bert-jupyter.png)

We are going to use the Jupyter environment only for preparing the dataset and then `torchrun` for launching our training script on both neuron cores for distributed training. Lets create a new notebook and get started. 

## 2. Load and process the dataset

We are training a Text Classification model on the [emotion](https://huggingface.co/datasets/dair-ai/emotion) dataset to keep the example straightforward. The `emotion` is a dataset of English Twitter messages with six basic emotions: anger, fear, joy, love, sadness, and surprise.

We will use the `load_dataset()` method from the [🤗 Datasets](https://huggingface.co/docs/datasets/index) library to load the `emotion`.

```python
from datasets import load_dataset

# Dataset id from huggingface.co/dataset
dataset_id = "dair-ai/emotion"

# Load raw dataset
raw_dataset = load_dataset(dataset_id)

print(f"Train dataset size: {len(raw_dataset['train'])}")
print(f"Test dataset size: {len(raw_dataset['test'])}")

# Train dataset size: 16000
# Test dataset size: 2000
```

Let’s check out an example of the dataset.

```python
from random import randrange

random_id = randrange(len(raw_dataset['train']))
raw_dataset['train'][random_id]
# {'text': 'i also like to listen to jazz whilst painting it makes me feel more artistic and ambitious actually look to the rainbow', 'label': 1}
```

We must convert our "Natural Language" to token IDs to train our model. This is done by a Tokenizer, which tokenizes the inputs (including converting the tokens to their corresponding IDs in the pre-trained vocabulary). if you want to learn more about this, out [chapter 6](https://huggingface.co/course/chapter6/1?fw=pt) of the [Hugging Face Course](https://huggingface.co/course/chapter1/1).

In order to avoid graph recompilation, inputs should have a fixed shape. We need to truncate or pad all samples to the same length.

```python
import os

from transformers import AutoTokenizer

# Model id to load the tokenizer
model_id = "bert-base-uncased"

# Load Tokenizer
tokenizer = AutoTokenizer.from_pretrained(model_id)

# Tokenize helper function
def tokenize(batch):
    return tokenizer(batch['text'], padding='max_length', truncation=True,return_tensors="pt")
def tokenize_function(example):
        return tokenizer(
            example["text"],
            padding="max_length",
            truncation=True,
        )

# Tokenize dataset
tokenized_emotions = raw_dataset.map(tokenize, batched=True, remove_columns=["text"])
```

## 3. Fine-tune BERT using Hugging Face Transformers

We can use the **[Trainer](https://huggingface.co/docs/transformers/en/main_classes/trainer#transformers.Trainer)** and **[TrainingArguments](https://huggingface.co/docs/transformers/en/main_classes/trainer#transformers.TrainingArguments)** to fine-tune PyTorch-based transformer models.

We prepared a simple [train.py](https://github.com/huggingface/optimum-neuron/blob/main/notebooks/ec2/text-classification/scripts/train.py) training script to perform training and evaluation on the dataset. Below is an excerpt:

```python
from transformers import Trainer, TrainingArguments

def parse_args():
	...

def training_function(args):

    ...

    # Download the model from huggingface.co/models
    model = AutoModelForSequenceClassification.from_pretrained(
        args.model_id, num_labels=num_labels, label2id=label2id, id2label=id2label
    )

    training_args = TrainingArguments(
			...
    )

    # Create Trainer instance
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_emotions["train"],
        eval_dataset=tokenized_emotions["validation"],
        processing_class=tokenizer,
    )

    # Start training
    trainer.train()
```

We can load the training script into our environment using the `wget` command or manually copy it into the notebook from [here](https://github.com/huggingface/optimum-neuron/blob/main/notebooks/ec2/text-classification/scripts/train.py).

```python
!wget https://raw.githubusercontent.com/huggingface/optimum-neuron/main/notebooks/ec2/text-classification/scripts/train.py
```

We will use `torchrun` to launch our training script on both neuron cores for distributed training, thus allowing data parallelism. `torchrun` is a tool that automatically distributes a PyTorch model across multiple accelerators. We can pass the number of accelerators as `nproc_per_node` arguments alongside our hyperparameters.

We'll use the following command to launch  training:

```python
!torchrun --nproc_per_node=2 train.py \
 --model_id bert-base-uncased \
 --lr 5e-5 \
 --per_device_train_batch_size 8 \
 --bf16 True \
 --epochs 3
```

After compilation, it will only take few minutes to complete the training.

```python
***** train metrics *****
  epoch                    =        3.0
  eval_loss                =     0.1761
  eval_runtime             = 0:00:03.73
  eval_samples_per_second  =    267.956
  eval_steps_per_second    =     16.881
  total_flos               =  1470300GF
  train_loss               =     0.2024
  train_runtime            = 0:07:27.14
  train_samples_per_second =     53.674
  train_steps_per_second   =      6.709

```

Last but not least, terminate the EC2 instance to avoid unnecessary charges. Looking at the price-performance, our training only costs **`20ct`** (**`1.34$/h * 0.13h = 0.18$`**)
