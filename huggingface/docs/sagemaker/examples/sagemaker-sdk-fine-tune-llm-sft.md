# Fine-Tune an LLM with the SageMaker SDK and TRL

Last updated 2026-09-22

This notebook shows how to fine-tune a small language model on Amazon SageMaker AI with the SageMaker Python SDK `ModelTrainer` and a [TRL](https://huggingface.co/docs/trl) `SFTTrainer` training script. The job runs on the Hugging Face PyTorch Training DLC, which ships with Transformers, TRL, Datasets, and PyTorch pre-installed.

**Hardware:** `ml.g6.xlarge` (1x NVIDIA L4) — one GPU is enough for a 0.6B model.

You will learn how to:

- Write a training script that reads hyperparameters and SageMaker environment variables.
- Retrieve the Hugging Face training DLC for your region and instance type.
- Create a `ModelTrainer` with CloudWatch metrics and a runtime safety cap.
- Start the training job and locate the trained model artifacts in S3.

## Setup

Install the SageMaker Python SDK v3:

```python
>> %pip install "sagemaker>=3.0.0" --upgrade --quiet
```

/Users/dwarez/hf/repos/hub-docs/docs/sagemaker/notebooks/sagemaker-sdk/fine-tune-llm-sft/.venv/bin/python: No module named pip
Note: you may need to restart the kernel to use updated packages.

This example uses the [SageMaker Python SDK v3](https://github.com/aws/sagemaker-python-sdk). v3 introduces a new, framework-agnostic API built around `ModelBuilder` (inference) and `ModelTrainer` (training), which replaces the v2 `HuggingFaceModel` and `HuggingFace` classes.

## Session and execution role

The training job runs under an IAM execution role with access to S3. Inside SageMaker Studio or a notebook instance, `get_execution_role()` finds the role automatically. Locally, the role is looked up by name — adjust `role_name` to your setup.

```python
>> import boto3
>> from sagemaker.core.helper.session_helper import Session, get_execution_role

>> REGION = boto3.Session().region_name or "us-east-1"
>> boto_sess = boto3.Session(region_name=REGION)
>> sess = Session(boto_session=boto_sess)

>> try:
...     role = get_execution_role(sagemaker_session=sess)
...     print(f"Using the SageMaker execution role: {role}")
>> except Exception:
...     role_name = "sagemaker_execution_role"
...     role = boto_sess.client("iam").get_role(RoleName=role_name)["Role"]["Arn"]
...     print(f"Using the IAM role: {role}")
```

sagemaker.config INFO - Not applying SDK defaults from location: /Library/Application Support/sagemaker/config.yaml
sagemaker.config INFO - Not applying SDK defaults from location: /Users/dwarez/Library/Application Support/sagemaker/config.yaml

```python
role = "arn:aws:iam::754289655784:role/sagemaker_execution_role"
```

## The training script

The `ModelTrainer` runs your own script inside the training container. The script for this notebook lives in [`scripts/train.py`](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/fine-tune-llm-sft/scripts/train.py) and does three things:

1. Reads the hyperparameters as command-line arguments (`--model_name`, `--max_steps`, ...).
2. Loads the dataset from the Hugging Face Hub and fine-tunes the model with TRL `SFTTrainer`.
3. Saves the model and tokenizer to `SM_MODEL_DIR`, which SageMaker archives to S3 as `model.tar.gz` when the job finishes.

## Create the ModelTrainer

The `ModelTrainer` ties everything together:

- `source_code` points at the script directory and entry point.
- `compute` defines the instance. `enable_managed_spot_training=True` uses [managed spot instances](https://docs.aws.amazon.com/sagemaker/latest/dg/model-managed-spot-training.html) for up to 90% savings — fine here because the job takes a few minutes.
- `training_image` is the Hugging Face training DLC, retrieved for your region and instance type with `image_uris.retrieve`.
- `with_metric_definitions` parses the training logs and sends metrics to CloudWatch.

```python
from sagemaker.train.model_trainer import ModelTrainer
from sagemaker.train.configs import SourceCode, Compute, StoppingCondition, MetricDefinition
from sagemaker.core import image_uris

hyperparameters = {
    "model_name": "Qwen/Qwen3-0.6B",  # any small causal LM from the Hub works
    "dataset_name": "trl-lib/Capybara",  # conversational SFT dataset
    "max_steps": 50,  # short run: enough to see the loss go down
    "train_batch_size": 4,
    "learning_rate": 2e-5,
}

instance_type = "ml.p4de.24xlarge"

# Retrieve the Hugging Face PyTorch training DLC image URI
training_image = image_uris.retrieve(
    framework="huggingface",
    region=REGION,
    version="5.3.0",  # Transformers version
    base_framework_version="pytorch2.9.0",  # PyTorch version
    py_version="py312",  # Python version
    image_scope="training",
    instance_type=instance_type,
)

# SFTTrainer logs lines like {'loss': 2.34, ...}; parse the loss into CloudWatch
metric_definitions = [
    MetricDefinition(name="train-loss", regex="'loss': ([0-9.]+)"),
]

model_trainer = ModelTrainer(
    sagemaker_session=sess,
    role=role,
    training_image=training_image,
    source_code=SourceCode(
        source_dir="./scripts",  # directory with the training script
        entry_script="train.py",  # script to run in the training job
    ),
    compute=Compute(
        instance_type=instance_type,
        instance_count=1,
        enable_managed_spot_training=True,  # use managed spot instances
    ),
    # max_wait_time_in_seconds should be equal to or greater than max_runtime_in_seconds
    stopping_condition=StoppingCondition(
        max_runtime_in_seconds=3600,
        max_wait_time_in_seconds=7200,
    ),
    hyperparameters=hyperparameters,
).with_metric_definitions(metric_definitions)
```

## Start the training job

Call `train` to launch the job. SageMaker starts the instance, runs `train.py` with your hyperparameters, streams the logs, and uploads the model artifacts to S3 when done. The dataset downloads from the Hub inside the container, so there is no data to upload.

```python
model_trainer.train()
```

## The trained model

When the job completes, the model artifacts (`model.tar.gz`) are in S3:

```python
model_data = model_trainer._latest_training_job.model_artifacts.s3_model_artifacts
print(f"Trained model artifacts: {model_data}")
```

## What's next

Deploy the trained model to an endpoint with `ModelBuilder` by pointing it at this S3 URI — see [Deploy models](https://huggingface.co/docs/sagemaker/main/en/tutorials/sagemaker-sdk/deploy-sagemaker-sdk) for the full guide.

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/fine-tune-llm-sft/sagemaker-notebook.ipynb)!
