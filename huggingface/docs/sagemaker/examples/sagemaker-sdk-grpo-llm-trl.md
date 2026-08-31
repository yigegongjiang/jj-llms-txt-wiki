# Improve tool calling with GRPO on Amazon SageMaker

Last updated 2026-08-31

In this notebook, you'll fine-tune **`HuggingFaceTB/SmolLM3-3B`** to produce better single-turn tool calls. The training signal is fully verifiable: instead of asking another model to judge the answer, we compare the model's generated tool call with the ground-truth call from the dataset.

We'll use GRPO from TRL because it can optimize directly from a Python reward function. For each prompt, the model samples several completions, the reward function scores each completion, and GRPO updates the model toward the completions that scored better than the rest of the group.

You'll walk through the full SageMaker training path:

- prepare a function-calling dataset for GRPO,
- write an exact-match reward for tool name and arguments,
- add a lightweight format reward for well-formed `<tool_call>` JSON,
- package the training script SageMaker will run,
- launch a multi-GPU training job,
- plot the reward curves from the training logs.

You need AWS credentials, a SageMaker execution role with S3 access, quota for `ml.p4d.24xlarge` training in `us-east-1`, and a Hugging Face token that can download the base model.

## What is GRPO?

Group Relative Policy Optimization (GRPO) is an RL fine-tuning method for language models. DeepSeek-R1 made it widely known: DeepSeek-R1-Zero used GRPO with rule-based rewards, while DeepSeek-R1 added cold-start data and a broader multi-stage training recipe around the RL stage.

The key idea is simple: for each prompt, sample several completions, score each one, and update the model based on how each completion did relative to the other completions from the same prompt. Unlike PPO-style RLHF setups, GRPO does not need a separate value model. The group itself provides the baseline.

```text
prompt
  -> sample N completions from the current model
  -> score each completion with a reward function
  -> compare each reward to the group mean/std
  -> increase probability of better-than-group completions
  -> decrease probability of worse-than-group completions
  -> apply a KL penalty so the model does not drift too far from the reference model
```

For reasoning models, rewards often check things like answer correctness and output format. In this notebook, the same pattern is used for tool calling: the model gets reward when it emits the correct tool name and arguments, plus a smaller formatting reward for valid `<tool_call>` JSON.

## Setup

Install only the packages used by the notebook kernel. The training dependencies are already inside the container that SageMaker runs.

This notebook uses SageMaker SDK v3 (`ModelTrainer`). The older v2 `HuggingFace` estimator uses different launch objects.

```python
%pip install -q "sagemaker>=3" datasets
```

The training job needs:

- `HF_TOKEN` in the container environment, so the model download can authenticate with the Hub.
- A SageMaker execution role, so SageMaker can pull the image, read the S3 input channel, and write the model artifact.

If `get_execution_role()` cannot find a role, replace the placeholder ARN with a role SageMaker can assume.

```python
from huggingface_hub import get_token, notebook_login

if not (HF_TOKEN := get_token()):
    notebook_login()
    HF_TOKEN = get_token()

assert HF_TOKEN, "No HF token found - set HF_TOKEN or run notebook_login()"
print("HF token loaded")
```

```python
import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

REGION = "us-east-1"
sess = Session(boto_session=boto3.Session(region_name=REGION))
bucket = sess.default_bucket()

try:
    role = get_execution_role(sagemaker_session=sess)
except Exception:
    role = "arn:aws:iam::<ACCOUNT_ID>:role/<SageMakerExecutionRole>"  # set this when running outside SageMaker

print("region:", REGION)
print("bucket:", bucket)
print("role:  ", role)
```

## Configuration

`MODEL_ID` is the base model and `INSTANCE_TYPE` the training hardware. `PUBLIC_IMAGE` is the public image that carries the TRL/GRPO training stack; the next step mirrors it into a private ECR repo (`ECR_REPO`) that SageMaker can actually pull from, and sets `TRAINING_IMAGE` to that private URI.

```python
MODEL_ID = "HuggingFaceTB/SmolLM3-3B"  # SmolLM3, HF's 3B instruct model
INSTANCE_TYPE = "ml.p4d.24xlarge"  # 8 x A100 40GB

# Public image carrying the TRL/GRPO training stack. SageMaker can't pull a
# public registry directly (see the next cell), so we mirror it into a private
# ECR repo in your account and train from that.
PUBLIC_IMAGE = "public.ecr.aws/u9a4y4p1/huggingface/hubcap:aws-pytorch-training_2.11.0-transformers5.10.2-gpu-py312-cu130-amzn2023-sagemaker"
ECR_REPO = "hf-trl-grpo"  # private repo created in your account
```

## Copy the training image to a private ECR repo

SageMaker only pulls from private ECR, so the cell below mirrors the public image into your account once and sets `TRAINING_IMAGE`. Expand it if you want the details.

```python
import base64
import platform
import shutil
import subprocess
import tarfile
import tempfile
import urllib.request
from pathlib import Path

account_id = boto3.client("sts", region_name=REGION).get_caller_identity()["Account"]
registry = f"{account_id}.dkr.ecr.{REGION}.amazonaws.com"
tag = PUBLIC_IMAGE.rsplit(":", 1)[-1]
TRAINING_IMAGE = f"{registry}/{ECR_REPO}:{tag}"

def ensure_crane():
    """Path to a `crane` binary, downloading a release build if it isn't on PATH."""
    if found := shutil.which("crane"):
        return found
    system = platform.system()  # Darwin / Linux
    machine = {"aarch64": "arm64"}.get(platform.machine(), platform.machine())  # x86_64 / arm64
    asset = f"go-containerregistry_{system}_{machine}.tar.gz"
    url = f"https://github.com/google/go-containerregistry/releases/latest/download/{asset}"
    dest = Path(tempfile.gettempdir()) / "crane-bin"
    dest.mkdir(exist_ok=True)
    urllib.request.urlretrieve(url, dest / asset)
    with tarfile.open(dest / asset) as tf:
        tf.extract("crane", dest)
    crane = dest / "crane"
    crane.chmod(0o755)
    return str(crane)

# 1. Create the private repo (no-op if it already exists).
ecr = boto3.client("ecr", region_name=REGION)
try:
    ecr.create_repository(repositoryName=ECR_REPO)
except ecr.exceptions.RepositoryAlreadyExistsException:
    pass

# 2. Authenticate crane to your private ECR (the public source needs no login).
token = ecr.get_authorization_token()["authorizationData"][0]["authorizationToken"]
password = base64.b64decode(token).decode().split(":", 1)[1]
crane = ensure_crane()
subprocess.run([crane, "auth", "login", registry, "-u", "AWS", "-p", password], check=True)

# 3. Copy public -> private. Idempotent: only missing layers are transferred.
subprocess.run([crane, "copy", PUBLIC_IMAGE, TRAINING_IMAGE], check=True)
print("training image:", TRAINING_IMAGE)
```

## Prepare the dataset

`Salesforce/xlam-function-calling-60k` has the three fields needed for a verifiable tool-call reward:

- `query`: the user request.
- `tools`: the functions the model may call.
- `answers`: the correct tool call, including arguments.

```python
from datasets import load_dataset

raw = load_dataset("Salesforce/xlam-function-calling-60k", split="train")
{k: raw[0][k] for k in ("query", "tools", "answers")}
```

`GRPOTrainer` reads prompts from a `prompt` column. The system message contains the tool list and the required `<tool_call>` format; the user message contains the request.

Keep `answers` as a separate column. TRL passes extra dataset columns to the reward function as keyword arguments, so `answers` remains hidden from the model but available to the scorer.

```python
import shutil
from pathlib import Path
import json

N_TRAIN = 2000  # a small subset keeps this demo cheap; the full set is 60k

def has_one_answer(row):
    try:
        answers = json.loads(row["answers"])
    except json.JSONDecodeError:
        return False
    return isinstance(answers, list) and len(answers) == 1

SYSTEM_TMPL = """/no_think
You are an expert in composing function calls. Return exactly one function call that answers the user's request.

You have access to the following tools:
<tools>
{tools}
</tools>

Return a single JSON object with the function name and arguments within <tool_call></tool_call> XML tags:
<tool_call>
{{"name": <function-name>, "arguments": <arguments-json-object>}}
</tool_call>
Emit exactly one <tool_call> block and no other text."""

def to_grpo(row):
    return {
        "prompt": [
            {"role": "system", "content": SYSTEM_TMPL.format(tools=row["tools"])},
            {"role": "user", "content": row["query"]},
        ],
        "answers": row["answers"],
    }

single_call = raw.filter(has_one_answer)
train_source = single_call.shuffle(seed=42).select(range(min(N_TRAIN, len(single_call))))
prepared = train_source.map(to_grpo, remove_columns=raw.column_names)
if Path("prepared").exists():
    shutil.rmtree("prepared")
prepared.save_to_disk("prepared")
prepared
```

```python
print(json.dumps(prepared[0]["prompt"], indent=2)[:1500])
print("\nground truth:", prepared[0]["answers"])
```

Upload the prepared dataset to S3. The input channel is named `train`, so SageMaker mounts it at `$SM_CHANNEL_TRAIN`; `train.py` loads the dataset from that path.

```python
train_s3 = sess.upload_data("prepared", bucket=bucket, key_prefix="xlam-grpo/prepared")
print(train_s3)
```

## The reward function

The reward must not crash on bad model output. Bad JSON, missing tags, extra prose, or wrong arguments return a low score.

- `tool_call_reward`: exact match on tool name and arguments.
- `format_reward`: partial credit for valid `<tool_call>` JSON, useful before exact matches are common.

Both functions are written to `src/rewards.py` so the training job can import them.

```python
from pathlib import Path

Path("src").mkdir(exist_ok=True)
```

```python
%%writefile src/rewards.py
'''Verifiable rewards for GRPO tool-call training.

Parse the tool calls the model emits (`<tool_call>` format) and compare them
against the dataset's ground-truth `answers`. No LLM judge, no sandbox.
'''
import json
import re

_TOOL_CALL_RE = re.compile(r"<tool_call>\s*(.*?)\s*</tool_call>", re.DOTALL)

def _text(completion):
    # GRPO hands completions as a string or a list of chat messages; return the text.
    if isinstance(completion, str):
        return completion
    if isinstance(completion, list):
        for msg in reversed(completion):
            if isinstance(msg, dict) and msg.get("role") == "assistant":
                return msg.get("content") or ""
    return str(completion)

def parse_tool_calls(text):
    '''Extract {name, arguments} dicts from a completion. Never raises.

    If <tool_call> tags are used, reject any non-whitespace text outside them.
    '''
    text = _text(text)
    matches = list(_TOOL_CALL_RE.finditer(text))
    if matches:
        outside = _TOOL_CALL_RE.sub("", text)
        if outside.strip():
            return []
        blocks = [m.group(1) for m in matches]
    else:
        blocks = [text.strip()]  # fall back to bare JSON
    calls = []
    for block in blocks:
        try:
            obj = json.loads(block)
        except (json.JSONDecodeError, TypeError):
            continue
        for call in obj if isinstance(obj, list) else [obj]:
            if isinstance(call, dict) and "name" in call:
                calls.append({"name": call["name"], "arguments": call.get("arguments", {})})
    return calls

def _loose_tool_calls(text):
    '''Parse tagged calls even when the completion rambles outside the tags.'''
    text = _text(text)
    calls = []
    for match in _TOOL_CALL_RE.finditer(text):
        try:
            obj = json.loads(match.group(1))
        except (json.JSONDecodeError, TypeError):
            continue
        for call in obj if isinstance(obj, list) else [obj]:
            if isinstance(call, dict) and "name" in call:
                calls.append(call)
    return calls

def _key(call):
    return json.dumps({"name": call.get("name"), "arguments": call.get("arguments", {})},
                      sort_keys=True, ensure_ascii=False)

def _normalize(calls):
    return sorted(_key(c) for c in calls)  # order- and key-order-independent

def _ground_truth(entry):
    if isinstance(entry, str):
        try:
            entry = json.loads(entry)
        except json.JSONDecodeError:
            return []
    return entry if isinstance(entry, list) else []

def tool_call_reward(completions, answers=None, **kwargs):
    '''1.0 when the emitted tool calls exactly match the ground truth, else 0.0.'''
    answers = answers or [None] * len(completions)
    out = []
    for completion, gt in zip(completions, answers):
        pred = _normalize(parse_tool_calls(completion))
        out.append(1.0 if pred and pred == _normalize(_ground_truth(gt)) else 0.0)
    return out

def format_reward(completions, **kwargs):
    '''Dense formatting signal: clean call wins; rambling/long junk loses.'''
    scores = []
    for completion in completions:
        text = _text(completion).strip()
        penalty = min(len(text), 2000) / 20000.0
        if parse_tool_calls(text):
            scores.append(1.0)
        elif _loose_tool_calls(text):
            scores.append(0.2 - penalty)
        elif "<tool_call" in text or "{" in text or '"name"' in text:
            scores.append(0.0 - penalty)
        else:
            scores.append(-0.2 - penalty)
    return scores

def _selfcheck():
    gt = '[{"name": "get_weather", "arguments": {"city": "Paris", "unit": "c"}}]'
    ok = '<tool_call>{"name": "get_weather", "arguments": {"unit": "c", "city": "Paris"}}</tool_call>'
    ramble = ok + " and then some extra text"
    assert tool_call_reward([ok], answers=[gt]) == [1.0]
    assert tool_call_reward([ramble], answers=[gt]) == [0.0]
    assert tool_call_reward(["nope"], answers=[gt]) == [0.0]
    assert format_reward([ok]) == [1.0]
    assert format_reward([ramble])[0] < 0.2
    assert format_reward(["nope"])[0] < -0.2
    print("ok")

if __name__ == "__main__":
    _selfcheck()
```

Run the reward self-check before launching SageMaker. If parsing or scoring is broken, fail here while the logs are local.

```python
from src.rewards import parse_tool_calls, tool_call_reward, format_reward

sample = '<tool_call>{"name": "get_weather", "arguments": {"city": "Paris", "unit": "c"}}</tool_call>'
gt = '[{"name": "get_weather", "arguments": {"unit": "c", "city": "Paris"}}]'

print("parsed:     ", parse_tool_calls(sample))
print("exact-match:", tool_call_reward([sample], answers=[gt]))  # [1.0] (argument order ignored)
print("format:     ", format_reward([sample]))  # [1.0]
print(
    "wrong call: ", tool_call_reward(['<tool_call>{"name": "x", "arguments": {}}</tool_call>'], answers=[gt])
)  # [0.0]
```

## The training script

`train.py` reads SageMaker paths and CLI hyperparameters, builds `GRPOConfig`, trains, and saves to `$SM_MODEL_DIR`.

Two generation settings are deliberate:

- `renormalize_logits=True` makes sampling robust if a logits processor creates invalid probabilities.
- `attn_implementation="sdpa"` uses the stable attention path for this stack.

This notebook leaves TRL's optional vLLM generation path disabled and uses the Transformers generation backend inside `GRPOTrainer`.

```python
%%writefile src/train.py
'''SageMaker entry script: GRPO tool-call training with TRL.

Runs once per GPU under torchrun. Reads data from $SM_CHANNEL_TRAIN, writes the
model to $SM_MODEL_DIR. Hyperparameters arrive as --key value CLI args.
'''
import argparse
import json
import os
import shutil

from datasets import load_from_disk
from transformers import AutoTokenizer, TrainerCallback
from trl import GRPOConfig, GRPOTrainer

from rewards import format_reward, tool_call_reward

def str2bool(v):
    return str(v).lower() in ("1", "true", "yes")

class JsonlLogCallback(TrainerCallback):
    def __init__(self, paths, stop_on_collapse=False):
        self.paths = paths
        self.stop_on_collapse = stop_on_collapse

    def on_log(self, args, state, control, logs=None, **kwargs):
        if not logs:
            return
        row = {"step": state.global_step, **logs}
        collapsed = (
            self.stop_on_collapse
            and state.global_step >= 2
            and (logs.get("completions/clipped_ratio") or 0) >= 0.9
            and (logs.get("rewards/tool_call_reward/mean") or 0) <= 0
        )
        if collapsed:
            row["early_stop_reason"] = "collapse: clipped completions with zero exact-match reward"
            control.should_training_stop = True
        if state.is_world_process_zero:
            for path in self.paths:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                with open(path, "a", encoding="utf-8") as f:
                    f.write(json.dumps(row) + "\n")
        return control

def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--model_id", default="HuggingFaceTB/SmolLM3-3B")
    p.add_argument("--train_dir", default=os.environ.get("SM_CHANNEL_TRAIN", "/opt/ml/input/data/train"))
    p.add_argument("--output_dir", default=os.environ.get("SM_MODEL_DIR", "/opt/ml/model"))
    # global batch (per_device * num_gpus * grad_accum) must be divisible by num_generations
    p.add_argument("--num_generations", type=int, default=8)
    p.add_argument("--per_device_train_batch_size", type=int, default=8)
    p.add_argument("--gradient_accumulation_steps", type=int, default=4)
    p.add_argument("--learning_rate", type=float, default=1e-8)
    p.add_argument("--beta", type=float, default=0.1)
    p.add_argument("--temperature", type=float, default=0.7)
    p.add_argument("--top_p", type=float, default=1.0)
    p.add_argument("--top_k", type=int, default=0)
    p.add_argument("--min_p", type=float, default=0.0)
    p.add_argument("--max_grad_norm", type=float, default=0.05)
    p.add_argument("--max_completion_length", type=int, default=128)
    p.add_argument("--max_steps", type=int, default=50)
    p.add_argument("--reward_weights", default="1.0,0.5")
    p.add_argument("--repetition_penalty", type=float, default=1.05)
    p.add_argument("--gradient_checkpointing", type=str2bool, default=True)
    p.add_argument("--deepspeed", default="")
    p.add_argument("--report_to", default="none")
    p.add_argument("--log_completions", type=str2bool, default=False)
    p.add_argument("--num_completions_to_print", type=int, default=8)
    p.add_argument("--stop_on_collapse", type=str2bool, default=True)
    p.add_argument("--remove_invalid_values", type=str2bool, default=False)
    p.add_argument("--renormalize_logits", type=str2bool, default=True)
    p.add_argument("--seed", type=int, default=42)
    return p.parse_args()

def main():
    args = parse_args()

    tokenizer = AutoTokenizer.from_pretrained(args.model_id)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    train_dataset = load_from_disk(args.train_dir)

    generation_kwargs = {
        "remove_invalid_values": args.remove_invalid_values,
        "renormalize_logits": args.renormalize_logits,
        "eos_token_id": tokenizer.eos_token_id,
        "pad_token_id": tokenizer.pad_token_id,
    }
    if args.min_p > 0:
        generation_kwargs["min_p"] = args.min_p

    config = GRPOConfig(
        output_dir=args.output_dir,
        per_device_train_batch_size=args.per_device_train_batch_size,
        gradient_accumulation_steps=args.gradient_accumulation_steps,
        learning_rate=args.learning_rate,
        beta=args.beta,
        temperature=args.temperature,
        top_p=args.top_p,
        top_k=args.top_k,
        num_generations=args.num_generations,
        max_completion_length=args.max_completion_length,
        max_steps=args.max_steps,
        max_grad_norm=args.max_grad_norm,
        bf16=True,
        repetition_penalty=args.repetition_penalty,
        gradient_checkpointing=args.gradient_checkpointing,
        gradient_checkpointing_kwargs={"use_reentrant": False},
        generation_kwargs=generation_kwargs,
        loss_type="dapo",
        mask_truncated_completions=False,
        scale_rewards="batch",
        reward_weights=[float(w) for w in args.reward_weights.split(",")],
        model_init_kwargs={"dtype": "bfloat16", "attn_implementation": "sdpa"},
        deepspeed=args.deepspeed or None,
        logging_steps=1,
        log_completions=args.log_completions,
        num_completions_to_print=args.num_completions_to_print,
        save_strategy="no",
        report_to=args.report_to,
        seed=args.seed,
    )

    metrics_paths = [os.path.join(args.output_dir, "training_metrics.jsonl")]
    if os.environ.get("SM_OUTPUT_DATA_DIR"):
        metrics_paths.append(os.path.join(os.environ["SM_OUTPUT_DATA_DIR"], "training_metrics.jsonl"))

    trainer = GRPOTrainer(
        model=args.model_id,
        args=config,
        reward_funcs=[tool_call_reward, format_reward],
        train_dataset=train_dataset,
        processing_class=tokenizer,
        callbacks=[JsonlLogCallback(metrics_paths, args.stop_on_collapse)],
    )
    trainer.train()

    completions_dir = os.path.join(args.output_dir, "completions")
    if os.environ.get("SM_OUTPUT_DATA_DIR") and os.path.isdir(completions_dir):
        shutil.copytree(
            completions_dir,
            os.path.join(os.environ["SM_OUTPUT_DATA_DIR"], "completions"),
            dirs_exist_ok=True,
        )

    trainer.save_model(args.output_dir)
    tokenizer.save_pretrained(args.output_dir)

if __name__ == "__main__":
    main()
```

DeepSpeed ZeRO-3 shards the model, gradients, and optimizer state across the 8 x A100 40GB GPUs. This run keeps `beta` enabled, so TRL also loads a reference model for the KL term.

```python
%%writefile src/ds_zero3.json
{
  "bf16": { "enabled": "auto" },
  "zero_optimization": {
    "stage": 3,
    "overlap_comm": true,
    "contiguous_gradients": true,
    "reduce_bucket_size": "auto",
    "stage3_prefetch_bucket_size": "auto",
    "stage3_param_persistence_threshold": "auto",
    "stage3_gather_16bit_weights_on_model_save": true
  },
  "gradient_accumulation_steps": "auto",
  "gradient_clipping": "auto",
  "train_batch_size": "auto",
  "train_micro_batch_size_per_gpu": "auto"
}
```

## Launch the training job

Multi-GPU GRPO needs one process per GPU. `Torchrun` lets the SageMaker SDK start `train.py` directly on all 8 x A100 40GB GPUs, so the notebook does not need a shell launcher.

This configuration is intentionally small: it is meant to verify that the dataset, reward functions, distributed launch, and logging are wired correctly. Do not expect this short run to materially improve the model; for a real RL training run, increase the number of steps and validate on a held-out set.

Set `DEBUG_NO_UPDATE=True` for a smoke test with `learning_rate=0.0`. `beta` stays enabled so the run still exercises the reference model and KL path.

```python
import time
from sagemaker.train.model_trainer import ModelTrainer
from sagemaker.train.configs import SourceCode, Compute, InputData, OutputDataConfig, StoppingCondition
from sagemaker.train.distributed import Torchrun

DEBUG_NO_UPDATE = False

base_job_name = "smolm3-grpo-toolcall-" + time.strftime("%Y%m%d-%H%M%S", time.gmtime())

trainer = ModelTrainer(
    sagemaker_session=sess,
    role=role,
    training_image=TRAINING_IMAGE,
    base_job_name=base_job_name,
    source_code=SourceCode(source_dir="src", entry_script="train.py"),
    distributed=Torchrun(process_count_per_node=8),
    compute=Compute(instance_type=INSTANCE_TYPE, instance_count=1, volume_size_in_gb=200),
    stopping_condition=StoppingCondition(max_runtime_in_seconds=6 * 60 * 60),
    output_data_config=OutputDataConfig(s3_output_path=f"s3://{bucket}/xlam-grpo/output/"),
    environment={
        "HF_TOKEN": HF_TOKEN,
        "NCCL_DEBUG": "WARN",
    },
    hyperparameters={
        "model_id": MODEL_ID,
        "max_steps": 20 if DEBUG_NO_UPDATE else 50,
        "per_device_train_batch_size": 2,
        "gradient_accumulation_steps": 8,  # global batch = 8 GPUs x 2 x 8
        "num_generations": 8,
        "learning_rate": 0.0 if DEBUG_NO_UPDATE else 1e-8,
        "beta": 0.1,  # keep the reference model + KL term enabled
        "temperature": 0.7,
        "top_p": 1.0,
        "top_k": 0,
        "max_grad_norm": 0.05,
        "max_completion_length": 128,
        "reward_weights": "1.0,0.5",  # exact match + lighter format shaping
        "repetition_penalty": 1.05,  # discourage max-length loops
        "log_completions": False,
        "num_completions_to_print": 8,
        "stop_on_collapse": True,
        "remove_invalid_values": False,
        "renormalize_logits": True,
        "gradient_checkpointing": True,
        "deepspeed": "ds_zero3.json",  # shard model + optimizer state across the 8 GPUs
    },
)

trainer.train(input_data_config=[InputData(channel_name="train", data_source=train_s3)], wait=True)
```

## Training curves

`GRPOTrainer` logs reward metrics during training. Plot those first: they show whether the model is still producing parseable tool calls and whether exact-match reward stays alive.

A noisy reward curve is normal. A fast drop to zero with clipped completions means generation collapsed.

```python
import boto3

TRAINING_JOB_NAME = ""  # set to a specific completed job name, or leave blank for latest
JOB_PREFIX = "smolm3-grpo-toolcall-"

sm = boto3.client("sagemaker", region_name=REGION)

if TRAINING_JOB_NAME:
    job = TRAINING_JOB_NAME
elif "trainer" in globals() and hasattr(trainer, "latest_training_job"):
    job = trainer.latest_training_job.name
elif "base_job_name" in globals():
    job = base_job_name
else:
    jobs = sm.list_training_jobs(
        NameContains=JOB_PREFIX,
        SortBy="CreationTime",
        SortOrder="Descending",
        MaxResults=1,
    )["TrainingJobSummaries"]
    assert jobs, f"No SageMaker jobs found with prefix {JOB_PREFIX!r}; set TRAINING_JOB_NAME manually."
    job = jobs[0]["TrainingJobName"]

desc = sm.describe_training_job(TrainingJobName=job)
print("job:   ", job)
print("status:", desc["TrainingJobStatus"])
print("model: ", desc.get("ModelArtifacts", {}).get("S3ModelArtifacts"))
print(f"console: https://{REGION}.console.aws.amazon.com/sagemaker/home?region={REGION}#/jobs/{job}")
```

### Plot reward metrics

Read metrics from the SageMaker output artifact. If the artifact is not ready, use CloudWatch logs or notebook output.

```python
import ast
import html
import json
import os
import re
import tarfile
import tempfile
from pathlib import Path
from urllib.parse import urlparse

from botocore.exceptions import ClientError

os.environ.setdefault("MPLCONFIGDIR", "/tmp/matplotlib")
import matplotlib.pyplot as plt
import pandas as pd

metric_re = re.compile(r"\{.*?(?:\'reward\'|\"reward\").*?\}", re.DOTALL)
ansi_re = re.compile(r"\x1b\[[0-?]*[ -/]*[@-~]")
tag_re = re.compile(r"<[^>]+>")

def maybe_float(value):
    try:
        return float(value)
    except (TypeError, ValueError):
        return value

def parse_metric_dicts(chunks):
    records, seen = [], set()
    for chunk in chunks:
        text = html.unescape(str(chunk))
        text = tag_re.sub("", text)
        text = ansi_re.sub("", text).replace("#015", "").replace("\r", "")
        for match in metric_re.finditer(text):
            raw = " ".join(match.group(0).split())
            if raw in seen:
                continue
            seen.add(raw)
            try:
                record = ast.literal_eval(raw)
            except (SyntaxError, ValueError):
                continue
            if isinstance(record, dict) and "reward" in record:
                records.append({k: maybe_float(v) for k, v in record.items()})
    return records

def s3_metric_records(training_desc):
    artifact = training_desc.get("ModelArtifacts", {}).get("S3ModelArtifacts")
    assert artifact, "No model artifact URI found."
    output_uri = artifact.rsplit("/", 1)[0] + "/output.tar.gz"
    parsed = urlparse(output_uri)
    local_tar = Path(tempfile.gettempdir()) / f"{job}-output.tar.gz"
    boto3.client("s3", region_name=REGION).download_file(parsed.netloc, parsed.path.lstrip("/"), str(local_tar))
    with tarfile.open(local_tar) as tar:
        member = next(m for m in tar.getmembers() if m.name.endswith("training_metrics.jsonl"))
        lines = tar.extractfile(member).read().decode().splitlines()
    Path("training_metrics.jsonl").write_text("\n".join(lines) + "\n")
    return [json.loads(line) for line in lines if line.strip()]

def cloudwatch_metric_records(job_name):
    logs = boto3.client("logs", region_name=REGION)
    kwargs = {
        "logGroupName": "/aws/sagemaker/TrainingJobs",
        "logStreamNamePrefix": job_name,
        "startFromHead": True,
    }
    chunks = []
    while True:
        page = logs.filter_log_events(**kwargs)
        chunks.extend(event.get("message", "") for event in page.get("events", []))
        token = page.get("nextToken")
        if not token:
            break
        kwargs["nextToken"] = token
    return parse_metric_dicts(chunks)

def notebook_metric_records(path="notebook.ipynb"):
    nb = json.loads(Path(path).read_text())
    chunks = []
    for cell in nb["cells"]:
        for output in cell.get("outputs", []):
            if "text" in output:
                value = output["text"]
                chunks.append("".join(value) if isinstance(value, list) else value)
            for value in output.get("data", {}).values():
                chunks.append("".join(value) if isinstance(value, list) else str(value))
    return parse_metric_dicts(chunks)

try:
    records = s3_metric_records(desc)
except Exception as e:
    print(f"Could not read metrics from S3 output artifact: {e}")
    metrics_file = Path("training_metrics.jsonl")
    if metrics_file.exists():
        records = [json.loads(line) for line in metrics_file.read_text().splitlines() if line.strip()]
    else:
        try:
            records = cloudwatch_metric_records(job)
        except ClientError as e:
            if e.response.get("Error", {}).get("Code") != "AccessDeniedException":
                raise
            print("CloudWatch Logs access denied; parsing saved notebook output instead.")
            records = notebook_metric_records()

records = [r for r in records if "reward" in r]
assert records, f"No GRPO reward metrics found for {job}."

metrics = pd.DataFrame(records)
metrics["step"] = range(1, len(metrics) + 1)
metrics["reward_smooth"] = metrics["reward"].rolling(5, min_periods=1).mean()
metrics.to_csv("training_metrics.csv", index=False)

cols = [
    "step",
    "reward",
    "reward_smooth",
    "rewards/tool_call_reward/mean",
    "rewards/format_reward/mean",
    "completions/clipped_ratio",
    "loss",
    "entropy",
]
display(metrics[[c for c in cols if c in metrics]].head())

plot_cols = [
    "reward",
    "reward_smooth",
    "rewards/tool_call_reward/mean",
    "rewards/format_reward/mean",
]
plot_cols = [c for c in plot_cols if c in metrics]

ax = metrics.plot(x="step", y=plot_cols, figsize=(10, 5), marker="o")
ax.set_title(f"GRPO reward curves: {MODEL_ID}")
ax.set_xlabel("logged training step")
ax.set_ylabel("reward")
ax.grid(True, alpha=0.3)
plt.tight_layout()
plt.savefig("training_reward_curves.png", dpi=160)
plt.show()
```

## Conclusion

You now have the full shape of a verifiable-reward RL training job on SageMaker: a dataset with hidden ground-truth tool calls, reward functions that score generated calls, a TRL GRPO training script, and a multi-GPU SageMaker launch.

The run above is deliberately short so the notebook stays practical to execute. To turn it into a real training run, train for more steps, use a held-out evaluation split, and track exact-match tool-call accuracy in addition to the reward curves.

## References

- [DeepSeek-R1: Incentivizing Reasoning Capability in LLMs via Reinforcement Learning](https://arxiv.org/abs/2501.12948)
- [TRL GRPO Trainer](https://huggingface.co/docs/trl/main/en/grpo_trainer)
- [SageMaker `ModelTrainer`](https://sagemaker.readthedocs.io/en/stable/api/training/model_trainer.html)
- [DeepSpeed ZeRO](https://www.deepspeed.ai/tutorials/zero/)
- [`HuggingFaceTB/SmolLM3-3B`](https://huggingface.co/HuggingFaceTB/SmolLM3-3B)
- [`Salesforce/xlam-function-calling-60k`](https://huggingface.co/datasets/Salesforce/xlam-function-calling-60k)

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/grpo-llm-trl/sagemaker-notebook.ipynb)!
