# Fine-Tuning LLMs with TRL CLI on SageMaker

Last updated 2026-08-05

This notebook shows how to fine-tune language models on AWS SageMaker using the **`trl sft` CLI** - the same command used by [trl-jobs](https://github.com/huggingface/trl-jobs) on HuggingFace.

**Hardware:** `ml.g6e.12xlarge` (4x L40S) or `ml.p4de.24xlarge` (8x A100)

**Why TRL CLI?**
- Zero Python code - just config and run
- Multi-GPU training via `accelerate`
- Compatible with [trl-jobs configs](https://github.com/huggingface/trl-jobs/tree/main/trl_jobs/configs)

## Prerequisites

- AWS credentials configured
- SageMaker execution role with S3 access
- HuggingFace token

## ⚙️ Setup

```python
>> %pip install sagemaker --upgrade --quiet
```

[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m[33mWARNING: Ignoring invalid distribution ~ackaging (/opt/pytorch/lib/python3.12/site-packages)[0m[33m
[0m
[1m[[0m[34;49mnotice[0m[1;39;49m][0m[39;49m A new release of pip is available: [0m[31;49m25.1.1[0m[39;49m -> [0m[32;49m25.3[0m
[1m[[0m[34;49mnotice[0m[1;39;49m][0m[39;49m To update, run: [0m[32;49mpip install --upgrade pip[0m
Note: you may need to restart the kernel to use updated packages.

This example uses the [SageMaker Python SDK v3](https://github.com/aws/sagemaker-python-sdk). v3 introduces a new, framework-agnostic API built around `ModelBuilder` (inference) and `ModelTrainer` (training), which replaces the v2 `HuggingFaceModel` and `HuggingFace` classes.

```python
import sagemaker
```

```python
>> # Authenticate with Hugging Face
>> import os
>> from huggingface_hub import login, get_token

>> login()

>> HF_TOKEN = get_token()
>> os.environ["HF_TOKEN"] = HF_TOKEN
>> print(f"HF_TOKEN set: {HF_TOKEN[:8]}...")
```

HF_TOKEN set: hf_MICph...

```python
>> import json
>> import time
>> import uuid
>> import tempfile
>> import shutil
>> from pathlib import Path

>> import boto3
>> import sagemaker
>> from sagemaker.train.model_trainer import ModelTrainer
>> from sagemaker.train.configs import SourceCode, Compute, StoppingCondition, OutputDataConfig
>> from sagemaker.core.helper.session_helper import Session
```

sagemaker.config INFO - Not applying SDK defaults from location: /etc/xdg/sagemaker/config.yaml
sagemaker.config INFO - Not applying SDK defaults from location: /home/ec2-user/.config/sagemaker/config.yaml

```python
>> # Initialize SageMaker session
>> sagemaker_session = Session()
>> iam = boto3.client('iam')
>> role = iam.get_role(RoleName='sagemaker-dlcs')['Role']['Arn']
>> region = sagemaker_session.boto_region_name
>> account_id = boto3.client("sts").get_caller_identity()["Account"]

>> print(f"Region: {region}")
>> print(f"Account: {account_id}")
>> print(f"Role: {role}")
```

Region: us-east-1
Account: 754289655784
Role: arn:aws:iam::754289655784:role/sagemaker-dlcs

## ⚙️ Configuration

We use **YAML config files** exactly like [trl-jobs](https://github.com/huggingface/trl-jobs/blob/main/trl_jobs/configs/Qwen3-4B-a100-large.yaml).

**Hardware:** 4x L40S GPUs on `ml.g6e.12xlarge` (192GB total VRAM)

```python
>> # =============================================================================
>> # MODEL & INSTANCE CONFIGURATION
>> # =============================================================================
>> # Based on trl-jobs optimal configs:
>> # https://github.com/huggingface/trl-jobs/blob/main/trl_jobs/configs/

>> MODEL_NAME = "Qwen/Qwen3-4B-Instruct-2507"  # trl-jobs optimal config
>> DATASET_NAME = "OpenMed/Medical-Reasoning-SFT-GPT-OSS-120B"

>> # Instance: ml.g6e.12xlarge = 4x L40S GPUs (48GB each = 192GB total)
>> # Note: g6e.16xlarge only has 1 GPU, g6e.12xlarge/g6e.24xlarge have 4 GPUs
>> INSTANCE_TYPE = "ml.p4de.24xlarge"
>> NUM_GPUS = 8

>> print(f"Model: {MODEL_NAME}")
>> print(f"Dataset: {DATASET_NAME}")
>> print(f"Instance: {INSTANCE_TYPE} ({NUM_GPUS}x L40S GPUs)")
```

Model: Qwen/Qwen3-4B-Instruct-2507
Dataset: OpenMed/Medical-Reasoning-SFT-GPT-OSS-120B
Instance: ml.p4de.24xlarge (8x L40S GPUs)

```python
>> # Generate unique output model name for hub_model_id and run_name
>> timestamp = time.strftime("%Y%m%d%H%M%S", time.gmtime())
>> OUTPUT_MODEL_NAME = f"Qwen3-4B-Base-SFT-{timestamp}"

>> print(f"Output model name: {OUTPUT_MODEL_NAME}")
>> print("(Copy this to hub_model_id and run_name in the YAML config cell)")
```

Output model name: Qwen3-4B-Base-SFT-20260120162518
(Copy this to hub_model_id and run_name in the YAML config cell)

## 🔧 TRL CLI with YAML Config

TRL CLI has **native accelerate support** - just put `num_processes` in the YAML config:

```bash
trl sft --config sft_config.yaml
```

No need for `accelerate launch` wrapper! See [TRL CLI docs](https://huggingface.co/docs/trl/en/clis#scaling-up-with-accelerate).

```python
>> # Preview the command that will be executed
>> print("Command:")
>> print("  trl sft --config sft_config.yaml")
>> print()
>> print("Config: scripts/sft_config.yaml")
>> print("Model: Qwen/Qwen3-4B-Base")
>> print("Dataset: trl-lib/Capybara")
>> print("GPUs: 4 (via num_processes in YAML)")
```

Command:
  trl sft --config sft_config.yaml

Config: scripts/sft_config.yaml
Model: Qwen/Qwen3-4B-Base
Dataset: trl-lib/Capybara
GPUs: 4 (via num_processes in YAML)

## 📝 Create Training Files

We create:
1. **`sft_config.yaml`** - TRL SFT config (same format as trl-jobs)
2. **`run_sft.py`** - Entry script that calls `trl sft --config`
3. **`requirements.txt`** - Python dependencies

### 📊 Training Logging with Trackio

This notebook uses [Trackio](https://huggingface.co/docs/trl/en/trackio_integration) for real-time training metrics visualization. Trackio is TRL's native integration for logging training runs to a Hugging Face Space.

**To use Trackio:**
1. Set `report_to: trackio` in your config
2. Configure `TRACKIO_SPACE_ID` (your HF Space) and `TRACKIO_PROJECT` (project name) as environment variables

**To disable:** Change `report_to: trackio` to `report_to: none` in the config and remove the `TRACKIO_*` environment variables.

```python
>> # Create scripts directory
>> import shutil

>> source_dir = Path("scripts")
>> if source_dir.exists():
...     shutil.rmtree(source_dir)
>> source_dir.mkdir(exist_ok=True)

>> print(f"✓ Created {source_dir}/")
```

✓ Created scripts/

```python
>> %%writefile scripts/run_sft.py
>> #!/usr/bin/env python3
>> """Entry script for TRL SFT training on SageMaker.

... Runs: trl sft --config sft_config.yaml

... TRL CLI natively supports accelerate - num_processes is in the YAML config.
... See: https://huggingface.co/docs/trl/en/clis#scaling-up-with-accelerate
... """
>> import os
>> import sys
>> import subprocess
>> from pathlib import Path

>> def main():
...     print("=" * 60)
...     print("TRL SFT Training on SageMaker")
...     print("=" * 60)

...     # Environment info
...     hf_token = os.environ.get("HF_TOKEN", "")
...     print(f"HF_TOKEN: {hf_token[:8]}..." if hf_token else "HF_TOKEN: NOT SET")
...     print(f"Working directory: {os.getcwd()}")

...     # Find config file
...     config_path = Path("/opt/ml/input/data/code/sft_config.yaml")
...     if not config_path.exists():
...         config_path = Path("sft_config.yaml")

...     print(f"Config file: {config_path}")
...     print()

...     # Print config contents
...     print("Configuration:")
...     print("-" * 40)
...     print(config_path.read_text())
...     print("-" * 40)
...     print()

...     # TRL CLI has native accelerate support - just pass the config
...     # num_processes, mixed_precision, etc. are read from the YAML
...     cmd = ["trl", "sft", "--config", str(config_path)]

...     print(f"Running: {' '.join(cmd)}")
...     print("=" * 60)

...     # Execute
...     result = subprocess.run(cmd, check=False)
...     sys.exit(result.returncode)

>> if __name__ == "__main__":
...     main()
```

Overwriting scripts/run_sft.py

```python
>> %%writefile scripts/requirements.txt
>> trl>=0.12.0
>> transformers>=4.45.0
>> datasets>=3.0.0
>> peft>=0.13.0
>> accelerate>=1.0.0
>> huggingface_hub>=0.26.0
>> liger-kernel>=0.4.0
>> flash-attn>=2.0.0
>> trackio
>> hf_transfer
```

Overwriting scripts/requirements.txt

```python
>> %%writefile scripts/sft_config.yaml
>> # TRL SFT Config - Based on trl-jobs
>> # https://github.com/huggingface/trl-jobs/blob/main/trl_jobs/configs/Qwen3-4B-a100-large.yaml
>> # Adapted for 4x L40S GPUs (ml.g6e.12xlarge)

>> # Accelerate arguments
>> num_processes: 8
>> num_machines: 1
>> mixed_precision: "no"
>> dynamo_backend: "no"

>> # Model arguments
>> model_name_or_path: Qwen/Qwen3-4B-Instruct-2507
>> model_revision: main
>> torch_dtype: bfloat16
>> attn_implementation: kernels-community/flash-attn2
>> use_peft: true

>> # Data arguments
>> dataset_name: OpenMed/Medical-Reasoning-SFT-GPT-OSS-120B
>> dataset_num_proc: 96

>> # Training arguments
>> bf16: true
>> do_eval: false
>> eval_strategy: "no"
>> gradient_accumulation_steps: 8
>> gradient_checkpointing: true
>> gradient_checkpointing_kwargs:
...   use_reentrant: false
>> learning_rate: 2.0e-04
>> log_level: info
>> logging_strategy: steps
>> logging_steps: 10
>> lr_scheduler_type: cosine_with_min_lr
>> lr_scheduler_kwargs:
...   min_lr_rate: 0.1
>> max_grad_norm: 0.2
>> max_length: 24576
>> num_train_epochs: 1
>> output_dir: /opt/ml/model
>> overwrite_output_dir: true
>> packing: true
>> pad_to_multiple_of: 4096
>> per_device_eval_batch_size: 1
>> per_device_train_batch_size: 1
>> seed: 42
>> use_liger_kernel: true
>> warmup_ratio: 0.03

>> # Checkpointing
>> save_steps: 100
>> save_strategy: steps
>> save_total_limit: 2

>> # Hub & Logging
>> push_to_hub: true
>> hub_strategy: every_save
>> hub_model_id: Qwen3-4B-Instruct-OpenMed  # <-- UPDATE with OUTPUT_MODEL_NAME from Cell 10
>> run_name: Qwen3-4B-Instruct-OpenMed-SFT      # <-- UPDATE with OUTPUT_MODEL_NAME from Cell 10
>> report_to: trackio
```

Overwriting scripts/sft_config.yaml

```python
>> # Verify all files created
>> print("Contents of scripts/:")
>> for f in sorted(source_dir.iterdir()):
...     print(f"  ✓ {f.name}")
```

Contents of scripts/:
  ✓ requirements.txt
  ✓ run_sft.py
  ✓ sft_config.yaml

## 🚀 Launch Training Job

We use the SageMaker `ModelTrainer` with `SourceCode` to launch the training job.

The TRL CLI arguments are embedded directly in the entry script to avoid SageMaker/accelerate argument parsing conflicts.

```python
>> # Job configuration
>> PROJECT_NAME = "trl-sft-yaml"
>> BUCKET_NAME = sagemaker_session.default_bucket()
>> S3_OUTPUT_URI = f"s3://{BUCKET_NAME}/{PROJECT_NAME}"

>> unique_id = uuid.uuid4().hex[:8]
>> base_job_name = f"{PROJECT_NAME}-{unique_id}"

>> # Training image - PyTorch DLC with GPU support
>> TRAINING_IMAGE = f"763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-pytorch-training:2.8.0-transformers4.56.2-gpu-py312-cu129-ubuntu22.04-v1.2"

>> # Create ModelTrainer with SourceCode
>> trainer = ModelTrainer(
...     sagemaker_session=sagemaker_session,
...     role=role,
...     training_mode="SAGEMAKER_TRAINING_JOB",
...     source_code=SourceCode(
...         source_dir=str(source_dir),
...         entry_script="run_sft.py",
...         requirements="requirements.txt",
...     ),
...     compute=Compute(
...         instance_type=INSTANCE_TYPE,
...         instance_count=1,
...         volume_size_in_gb=200,  # Larger for 4B model + checkpoints
...     ),
...     stopping_condition=StoppingCondition(
...         max_runtime_in_seconds=6 * 60 * 60,  # 6 hours
...     ),
...     output_data_config=OutputDataConfig(
...         s3_output_path=f"{S3_OUTPUT_URI}/output/",
...     ),
...     base_job_name=base_job_name,
...     environment={
...         "HF_TOKEN": HF_TOKEN,
...         "TRANSFORMERS_VERBOSITY": "info",
...         "NCCL_DEBUG": "WARN",  # For multi-GPU debugging
...         # Trackio logging - https://huggingface.co/docs/trl/en/trackio_integration
...         "TRACKIO_SPACE_ID": "florentgbelidji/trackio",
...         "TRACKIO_PROJECT": "trl-sft",
...         "HF_HUB_ENABLE_HF_TRANSFER": "1",
...     },
...     training_image=TRAINING_IMAGE,
... )

>> print(f"Job name: {base_job_name}")
>> print(f"Instance: {INSTANCE_TYPE} ({NUM_GPUS}x L40S GPUs)")
>> print(f"Output: {S3_OUTPUT_URI}")
>> print(f"Image: {TRAINING_IMAGE.split('/')[-1]}")
>> print(f"\nReady to launch!")
```

Job name: trl-sft-yaml-97d3b6c5
Instance: ml.p4de.24xlarge (8x L40S GPUs)
Output: s3://sagemaker-us-east-1-754289655784/trl-sft-yaml
Image: huggingface-pytorch-training:2.8.0-transformers4.56.2-gpu-py312-cu129-ubuntu22.04-v1.2

Ready to launch!

```python
>> # Launch the training job
>> print(f"Launching training job: {base_job_name}")
>> print(f"Model: {MODEL_NAME}")
>> print(f"Dataset: {DATASET_NAME}")
>> print(f"Instance: {INSTANCE_TYPE} ({NUM_GPUS}x L40S)")
>> print(f"\nUsing: trl sft --config sft_config.yaml")
>> print()

>> # Set wait=True to stream logs, wait=False to run in background
>> trainer.train(wait=True)
```

Launching training job: trl-sft-yaml-97d3b6c5
Model: Qwen/Qwen3-4B-Instruct-2507
Dataset: OpenMed/Medical-Reasoning-SFT-GPT-OSS-120B
Instance: ml.p4de.24xlarge (8x L40S)

Using: trl sft --config sft_config.yaml

## 📊 Monitor Training

If you launched with `wait=False`, you can monitor the job here.

```python
>> # Get the job name
>> training_job_name = trainer.latest_training_job.name if hasattr(trainer, 'latest_training_job') else base_job_name
>> print(f"Job name: {training_job_name}")

>> # View in console
>> console_url = f"https://{region}.console.aws.amazon.com/sagemaker/home?region={region}#/jobs/{training_job_name}"
>> print(f"Console: {console_url}")
```

Job name: trl-sft-yaml-dfe9c258
Console: https://us-east-1.console.aws.amazon.com/sagemaker/home?region=us-east-1#/jobs/trl-sft-yaml-dfe9c258

## 🧪 Load and Test the Model

After training completes, the model is pushed to the Hugging Face Hub.

```python
# Load the fine-tuned model from Hub
from transformers import pipeline

# Your HF username
HF_USERNAME = "florentgbelidji"  # Change this!

finetuned_model_id = f"{HF_USERNAME}/{OUTPUT_MODEL_NAME}"
print(f"Model will be available at: {finetuned_model_id}")

# Uncomment after training completes
# pipe = pipeline(
#     "text-generation",
#     model=finetuned_model_id,
#     torch_dtype="auto",
#     device_map="auto",
# )

# messages = [{"role": "user", "content": "What is the capital of France?"}]
# output = pipe(messages, max_new_tokens=100)
# print(output[0]["generated_text"][-1]["content"])
```

## 🧹 Cleanup

```python
shutil.rmtree(source_dir, ignore_errors=True)
print("Cleaned up temporary files.")
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/fine-tune-trl-cli/sagemaker-notebook.ipynb)!
