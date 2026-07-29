# Fine-Tune Gemma on Google TPU

This tutorial will teach how to fine-tune open LLMs like [Google Gemma](https://huggingface.co/google/gemma-7b) on Google Cloud's TPUs. In our example, we are going to leverage Hugging Face Optimum TPU, [🤗 Transformers](https://huggingface.co/docs/transformers/index) and datasets.

### Google's TPU

Google Cloud TPUs are custom-designed AI accelerators, which are optimized for training and inference of large AI models. They are ideal for a variety of use cases, such as chatbots, code generation, media content generation, synthetic speech, vision services, recommendation engines, personalization models, among others.

Advantages of using TPUs include:

* Designed to scale cost-efficiently for a wide range of AI workloads, spanning training, fine-tuning, and inference.
* Optimized for TensorFlow, PyTorch, and JAX, and are available in a variety of form factors, including edge devices, workstations, and cloud-based infrastructure.
* TPUs are available in Google Cloud, and have been integrated with Vertex AI, and Google Kubernetes Engine (GKE).

### Environment Setup

For this example, a single-host `v5litepod8` TPU will be enough. To set up a TPU environment with Pytorch XLA, this [Google Cloud guide](https://cloud.google.com/tpu/docs/run-calculation-pytorch) shows how to do that.

We can use `ssh` or `gcloud` commands to log in to the remote TPU. Enable port-forwarding for the port `8888`, e.g.:

```python
gcloud compute tpus tpu-vm ssh $TPU_NAME \
        --zone=$ZONE \
        -- -L 8888:localhost:8888
```

Once we have access to the TPU VM, we can clone the `optimum-tpu` repository containing the related notebook. Then we can install few packages used in this tutorial and launch the notebook:

```python
git clone https://github.com/huggingface/optimum-tpu.git
# Install Optimum tpu
pip install -e . -f https://storage.googleapis.com/libtpu-releases/index.html
# Install TRL and PEFT for training (see later how they are used)
pip install trl peft
# Install Jupyter notebook
pip install -U jupyterlab notebook
# Optionally, install widgets extensions for better rendering
pip install ipywidgets widgetsnbextension
# Change directory and launch Jupyter notebook
cd optimum-tpu/examples/language-modeling
jupyter notebook --port 8888
```

We should then see the familiar Jupyter output that shows the address accessible from a browser:

```
http://localhost:8888/tree?token=3ceb24619d0a2f99acf5fba41c51b475b1ddce7cadb2a133
```

Since we are going to use the gated `gemma` model, we will need to log in using a [Hugging Face token](https://huggingface.co/settings/tokens):

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

### Load and Prepare Dataset

We will use [Dolly](https://huggingface.co/datasets/databricks/databricks-dolly-15k), an open source dataset of instruction-following records on categories outlined in the [InstructGPT](https://arxiv.org/abs/2203.02155) paper, including brainstorming, classification, closed QA, generation, information extraction, open QA, and summarization.

We will load the dataset from the hub:

```python
from datasets import load_dataset

dataset = load_dataset("databricks/databricks-dolly-15k", split="train")
```

We can take a look to a sample:

```python
dataset[321]
```

We obtain a result similar to this:

```python
{
    "instruction": "When was the 8088 processor released?",
    "context": "The 8086 (also called iAPX 86) is a 16-bit microprocessor chip designed by Intel between early 1976 and June 8, 1978, when it was released. The Intel 8088, released July 1, 1979, is a slightly modified chip with an external 8-bit data bus (allowing the use of cheaper and fewer supporting ICs),[note 1] and is notable as the processor used in the original IBM PC design.",
    "response": "The Intel 8088 processor was released July 1, 1979.",
    "category": "information_extraction",
}
```

We will define a formatting function that combines `instruction`, `context` and `response` fields, and tokenizes them in a complete prompt. We will use a tokenizer compatible with the model we intend to use.

```python
from transformers import AutoTokenizer

model_id = "google/gemma-2b"

tokenizer = AutoTokenizer.from_pretrained(model_id)

def preprocess_function(sample):
    instruction = f"### Instruction\n{sample['instruction']}"
    context = f"### Context\n{sample['context']}" if len(sample["context"]) > 0 else None
    response = f"### Answer\n{sample['response']}"
    # join all the parts together
    prompt = "\n\n".join([i for i in [instruction, context, response] if i is not None])
    prompt += tokenizer.eos_token
    sample["prompt"] = prompt
    return sample
```

It is now possible to use this function to map the dataset, where original columns can now be removed:

```python
data = dataset.map(preprocess_function, remove_columns=list(dataset.features))
```

### Preparing the Model for Tuning

We can now load the model that will be used for tuning. The dataset is now ready to be used for fine-tuning:

```python
import torch
from transformers import AutoModelForCausalLM

model = AutoModelForCausalLM.from_pretrained(model_id, use_cache=False, torch_dtype=torch.bfloat16)
```

We're now going to use [Parameter Efficient FineTuning PEFT](https://huggingface.co/blog/peft) and [Low-Rank Adaptation (LoRA)](https://huggingface.co/papers/2106.09685) to efficiently fine tune the model on the prepared dataset. In the `LoraConfig` instance we will define the `nn.Linear` operations that will be fine tuned.

```python
from peft import LoraConfig

# Set up PEFT LoRA for fine-tuning.
lora_config = LoraConfig(
    r=8,
    target_modules=["k_proj", "v_proj"],
    task_type="CAUSAL_LM",
)
```

The `optimum-tpu` dedicated function will help us obtain arguments so we can create the trainer instance.

```python
from transformers import TrainingArguments
from trl import SFTTrainer

# Set up the FSDP arguments
fsdp_training_args = fsdp_v2.get_fsdp_training_args(model)

# Set up the trainer
trainer = SFTTrainer(
    model=model,
    train_dataset=data,
    args=TrainingArguments(
        per_device_train_batch_size=64,
        num_train_epochs=32,
        max_steps=-1,
        output_dir="./output",
        optim="adafactor",
        logging_steps=1,
        dataloader_drop_last=True,  # Required for FSDPv2.
        **fsdp_training_args,
    ),
    peft_config=lora_config,
    dataset_text_field="prompt",
    max_seq_length=1024,
    packing=True,
)
```

Once everything is ready it tuning the model is as simple as calling a function!

```python
trainer.train()
```

After this, we have successfully fine-tuned the model on the Dolly dataset.
