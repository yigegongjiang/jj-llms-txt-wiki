# Evaluate LLMs with Hugging Face Lighteval on Amazon SageMaker

Last updated 2026-08-05

In this sagemaker example, we are going to learn how to evaluate LLMs using Hugging Face [lighteval](https://github.com/huggingface/lighteval/tree/main).  LightEval is a lightweight LLM evaluation suite that powers [Hugging Face Open LLM Leaderboard](https://huggingface.co/spaces/HuggingFaceH4/open_llm_leaderboard). 

Evaluating LLMs is crucial for understanding their capabilities and limitations, yet it poses significant challenges due to their complex and opaque nature. LightEval facilitates this evaluation process by enabling LLMs to be assessed on acamedic benchmarks like MMLU or IFEval, providing a structured approach to gauge their performance across diverse tasks.

In Detail you will learn how to:
1. Setup Development Environment
2. Prepare the evaluation configuraiton
3. Evaluate Zephyr 7B on TruthfulQA on Amazon SageMaker

```python
!pip install 'sagemaker>=3.0.0' --upgrade --quiet
```

This example uses the [SageMaker Python SDK v3](https://github.com/aws/sagemaker-python-sdk). v3 introduces a new, framework-agnostic API built around `ModelTrainer` (training) and `ModelBuilder` (inference), which replaces the v2 `HuggingFace` and `HuggingFaceModel` classes.

If you are going to use Sagemaker in a local environment. You need access to an IAM Role with the required permissions for Sagemaker. You can find [here](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html) more about it.

```python
import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

sess = Session()
# sagemaker session bucket -> used for uploading data, models and logs
# sagemaker will automatically create this bucket if it does not exist
sagemaker_session_bucket = sess.default_bucket()

try:
    role = get_execution_role()
except Exception:
    iam = boto3.client('iam')
    role = iam.get_role(RoleName='sagemaker_execution_role')['Role']['Arn']

print(f"sagemaker role arn: {role}")
print(f"sagemaker bucket: {sess.default_bucket()}")
print(f"sagemaker session region: {sess.boto_region_name}")
```

## 2. Prepare the evaluation configuration

[LightEval](https://github.com/huggingface/lighteval/tree/main) lets you evaluate LLMs on common benchmarks like MMLU, TruthfulQA, IFEval, and more. It powers the [Hugging Face Open LLM Leaderboard](https://huggingface.co/spaces/HuggingFaceH4/open_llm_leaderboard) and is built on top of the [Eleuther AI Harness](https://github.com/EleutherAI/lm-evaluation-harness) with additional features and improvements.

You can find all available benchmarks [here](https://github.com/huggingface/lighteval/blob/main/examples/tasks/all_tasks.txt).

We are going to use Amazon SageMaker Managed Training to evaluate the model. lighteval is now used through its command-line interface (`lighteval accelerate`); the older `run_evals_accelerate.py` script has been removed. The training code lives in the [`scripts`](scripts) folder next to this notebook and is uploaded to the training job:

- [`scripts/requirements.txt`](scripts/requirements.txt) installs lighteval into the Hugging Face DLC (which does not ship with it).
- [`scripts/run_lighteval.py`](scripts/run_lighteval.py) is a small launcher that invokes the `lighteval accelerate` CLI.

In lighteval, an evaluation is launched with the `lighteval accelerate` command. Tasks are passed as a positional argument using the format `suite|task|num_few_shot` (for example `leaderboard|truthfulqa:mc|0`). You can evaluate on several tasks at once by passing a comma-separated list (e.g. `leaderboard|truthfulqa:mc|0,leaderboard|gsm8k|5`) or by pointing to a tasks file whose path starts with `./`. You can list every available task with `lighteval tasks list`.

We are going to evaluate the model on the TruthfulQA benchmark with 0 few-shot examples. [TruthfulQA](https://paperswithcode.com/dataset/truthfulqa) is a benchmark designed to measure whether a language model generates truthful answers to questions, encompassing 817 questions across 38 categories including health, law, finance, and politics.

To evaluate a model on all the benchmarks of the Open LLM Leaderboard, you can pass the tasks listed in [this file](https://github.com/huggingface/lighteval/blob/v0.13.0/examples/tasks/open_llm_leaderboard_tasks.txt).

## 3. Evaluate Zephyr 7B on TruthfulQA on Amazon SageMaker

In this example we are going to evaluate [HuggingFaceH4/zephyr-7b-beta](https://huggingface.co/HuggingFaceH4/zephyr-7b-beta) on the TruthfulQA benchmark, which is part of the Open LLM Leaderboard.

In addition to the tasks, the `lighteval accelerate` command takes:
* `model-args`: a `key=value` string describing the model, e.g. `model_name=HuggingFaceH4/zephyr-7b-beta,dtype=bfloat16`. Useful keys include `model_name` (Hugging Face Model ID or path) and `dtype` (`bfloat16`, `float16` or `float32`).
* `--output-dir`: the directory where the evaluation results are saved. We use `/opt/ml/model` so that SageMaker uploads the results to S3 once the job finishes.

You can see all available options with `lighteval accelerate --help`.

```python
from sagemaker.train.model_trainer import ModelTrainer
from sagemaker.train.configs import SourceCode, Compute, OutputDataConfig
from sagemaker.core import image_uris

# evaluation configuration
model_id = "HuggingFaceH4/zephyr-7b-beta"   # Hugging Face Model ID to evaluate
task = "leaderboard|truthfulqa:mc|0"         # suite|task|num_few_shot (comma-separate for multiple tasks)
model_dtype = "bfloat16"                      # torch dtype used to load the model weights
output_dir = "/opt/ml/model"                  # SageMaker uploads this directory to S3 after the job

instance_type = "ml.g5.4xlarge"

# Retrieve the Hugging Face PyTorch training DLC image URI
training_image = image_uris.retrieve(
    framework="huggingface",
    region=sess.boto_region_name,
    version="4.56.2",
    base_framework_version="pytorch2.8.0",
    py_version="py312",
    image_scope="training",
    instance_type=instance_type,
)

# lighteval is invoked through its CLI. SageMaker installs requirements.txt first, then runs this command.
# The model-args and task are single-quoted so the shell keeps the `,` and `|` characters intact.
command = (
    "python run_lighteval.py accelerate "
    f"'model_name={model_id},dtype={model_dtype}' "
    f"'{task}' "
    f"--output-dir {output_dir}"
)

# create the ModelTrainer
huggingface_estimator = ModelTrainer(
    sagemaker_session=sess,
    role=role,
    base_job_name="lighteval",                     # the name of the training job
    training_image=training_image,
    source_code=SourceCode(
        source_dir="scripts",                      # directory uploaded to the job (contains requirements.txt)
        requirements="requirements.txt",           # dependencies installed before running the command
        command=command,                           # lighteval CLI invocation
    ),
    compute=Compute(
        instance_type=instance_type,               # instance type used for the evaluation job
        instance_count=1,                          # the number of instances used
        volume_size_in_gb=300,                     # the size of the EBS volume in GB
    ),
    output_data_config=OutputDataConfig(
        s3_output_path=f"s3://{sess.default_bucket()}/lighteval/output",
    ),
    environment={
        "HUGGINGFACE_HUB_CACHE": "/tmp/.cache",
        # "HF_TOKEN": "REPLACE_WITH_YOUR_TOKEN"     # needed for gated/private models
    },  # set env variable to cache models in /tmp
)
```

```python
# start the evaluation job
huggingface_estimator.train(wait=True)
```

After the evaluation job is finished, we can download the evaluation results from the S3 bucket. Lighteval will save the results and generations in the `output_dir`. The results are savedas json and include detailed information about each task and the model's performance. The results are available in the `results` key. 

```python
import tarfile
import json
import io
import os
import boto3
from urllib.parse import urlparse

# get the S3 URI of the uploaded output artifacts (model.tar.gz)
training_job = huggingface_estimator._latest_training_job
model_data = training_job.model_artifacts.s3_model_artifacts

# download the artifacts from s3
parsed = urlparse(model_data)
s3 = boto3.client("s3")
results_tar = s3.get_object(Bucket=parsed.netloc, Key=parsed.path.lstrip("/"))["Body"].read()

result = {}

# lighteval writes the scores to results/<model_id>/results_<timestamp>.json
with tarfile.open(fileobj=io.BytesIO(results_tar), mode="r:gz") as tar:
    for member in tar.getmembers():
        if os.path.join("results", model_id) in member.name and member.name.endswith(".json"):
            f = tar.extractfile(member)
            if f is not None:
                result = json.loads(f.read())
                break

# print results
print(result["results"])
# {'leaderboard|truthfulqa:mc|0': {'truthfulqa_mc1': 0.406, 'truthfulqa_mc2': 0.575, ...}, 'all': {...}}
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/evaluate-llm-lighteval/sagemaker-notebook.ipynb)!
