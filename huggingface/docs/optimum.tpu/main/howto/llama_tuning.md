# Fine-Tune Llama on Google TPU

Training Large Language Models (LLMs) on Google Tensor Processing Units (TPUs) with Single Program Multiple Data (SPMD) offers a multitude of benefits. TPUs provide competitive processing power, enabling good training times and allowing researchers to experiment with larger models and datasets efficiently. SPMD architecture optimizes resource utilization by distributing tasks across multiple TPUs, enhancing parallelism and scalability.
The easiest approach to tune a model with SPMD is using Fully Sharded Data Parallel [(FSDP)](https://engineering.fb.com/2021/07/15/open-source/fsdp/). Pytorch/XLA most recent and performant implementation is [FSDP v2](https://github.com/pytorch/xla/blob/master/docs/fsdpv2.md), that allows to shard weights, activations and outputs.

This example shows to tune one of Meta's Llama models on single host TPUs. For information on TPUs architecture, you can consult the [documentation](https://cloud.google.com/tpu/docs/system-architecture-tpu-vm).

### Prerequisites

We consider you have already created a single-host TPU VM, such as a `v5litepod8` setup, and you have ssh access to the machine.
You need to clone `optimum-tpu` and install few modules:

```python
git clone https://github.com/huggingface/optimum-tpu.git
# Install Optimum TPU
pip install -e . -f https://storage.googleapis.com/libtpu-releases/index.html
# Install TRL and PEFT for training (see later how they are used)
pip install trl peft
# Install Jupyter notebook
pip install -U jupyterlab notebook
# Optionally, install widgets extensions for better rendering
pip install ipywidgets widgetsnbextension
# This will be necessary for the language modeling example
pip install datasets evaluate accelerate
# Change directory and launch Jupyter notebook
cd optimum-tpu/examples/language-modeling
jupyter notebook --port 8888
```

We should then see the familiar Jupyter output that shows the address accessible from a browser:

```
http://localhost:8888/tree?token=3ceb24619d0a2f99acf5fba41c51b475b1ddce7cadb2a133
```

Since we are going to use the gated `llama` model, we will need to log in using a [Hugging Face token](https://huggingface.co/settings/tokens):

```python
!huggingface-cli login --token YOUR_HF_TOKEN
```

### Enable FSDPv2

To fine-tune an LLM, it might be necessary to shard the model across the TPUs to prevent memory issues and enhance tuning performances. Fully Sharded Data Parallel is an algorithm that has been implemented on Pytorch and that allows to wrap modules to distribute them.
When using Pytorch/XLA on TPUs, [FSDPv2](https://pytorch.org/xla/master/#fully-sharded-data-parallel-via-spmd) is an utility that re-expresses the famous FSDP algorithm using SPMD (Single Program Multiple Data). In `optimum-tpu` it is possible to use dedicated helpers to use FSPDv2. To enable it, you can use the dedicated function, that should be called at the beginning of the execution:

```python
from optimum.tpu import fsdp_v2

fsdp_v2.use_fsdp_v2()
```

Then, the tokenizer and model need to be loaded. We will choose [`meta-llama/Llama-3.2-1B`](https://huggingface.co/meta-llama/Llama-3.2-1B) for this example.

```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

model_id = "meta-llama/Llama-3.2-1B"
tokenizer = AutoTokenizer.from_pretrained(model_id)
# Add custom token for padding Llama
tokenizer.add_special_tokens({"pad_token": tokenizer.eos_token})
model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.bfloat16)
```

To tune the model with the [Abirate/english_quotes](https://huggingface.co/datasets/Abirate/english_quotes) dataset, you can load it and obtain the `quote` column:

```python
from datasets import load_dataset

data = load_dataset("Abirate/english_quotes")

def preprocess_function(samples):
    # Add a simple prompt format to the quotes
    prompts = [f"Generate a quote:\n\n{quote}\n" for quote in samples["quote"]]
    # Add EOS token to each prompt
    prompts = [p + tokenizer.eos_token for p in prompts]
    return {"prompt": prompts}

# data = data.map(lambda samples: tokenizer(samples["quote"]), batched=True)
data = data.map(preprocess_function, batched=True, remove_columns=data["train"].column_names)
```

You then need to specify the FSDP training arguments to enable the sharding feature, the function will deduce the classes that should be sharded:

```python
fsdp_training_args = fsdp_v2.get_fsdp_training_args(model)
```

The `fsdp_training_args` will specify the Pytorch module that needs to be sharded:

```python
fsdp_training_args
```

Now training can be done as simply as using the standard `Trainer` class:

```python
from peft import LoraConfig

lora_config = LoraConfig(
    lora_alpha=128,
    lora_dropout=0.05,
    r=256,
    bias="none",
    target_modules="all-linear",
    task_type="CAUSAL_LM",
)
```

```python
from transformers import TrainingArguments
from trl import SFTTrainer

trainer = SFTTrainer(
    model=model,
    train_dataset=data["train"],
    args=TrainingArguments(
        per_device_train_batch_size=32,
        num_train_epochs=10,
        max_steps=-1,
        output_dir="/tmp/output",
        optim="adafactor",
        logging_steps=1,
        dataloader_drop_last=True,  # Required by FSDP v2
        **fsdp_training_args,
    ),
    peft_config=lora_config,
    dataset_text_field="prompt",
    max_seq_length=512,
    packing=True,
)

trainer.train()
```
