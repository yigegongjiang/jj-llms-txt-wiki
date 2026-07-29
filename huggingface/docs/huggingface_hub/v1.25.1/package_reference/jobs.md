# Jobs

Check the [HfApi](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi) documentation page for the reference of methods to manage your Jobs on the Hub.

- Run a Job: [run_job()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.run_job)
- Fetch logs: [fetch_job_logs()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.fetch_job_logs)
- Fetch metrics: [fetch_job_metrics()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.fetch_job_metrics)
- Inspect Job: [inspect_job()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.inspect_job)
- Wait until Job(s) finish: [wait_for_job()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.wait_for_job)
- List Jobs: [list_jobs()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.list_jobs)
- Cancel Job: [cancel_job()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.cancel_job)
- Run a UV Job: [run_uv_job()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.run_uv_job)
- Sync a local directory to mount it in a Job: [sync_job_volume()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.sync_job_volume)

## Data structures

### JobInfo[[huggingface_hub.JobInfo]]

- **id** (`str`) --
  Job ID.
- **created_at** (`datetime` or `None`) --
  When the Job was created.
- **started_at** (`datetime` or `None`) --
  When the Job started running. None while the Job is still scheduling.
- **finished_at** (`datetime` or `None`) --
  When the Job finished. None while the Job is still scheduling or running.
- **docker_image** (`str` or `None`) --
  The Docker image from Docker Hub used for the Job.
  Can be None if space_id is present instead.
- **space_id** (`str` or `None`) --
  The Docker image from Hugging Face Spaces used for the Job.
  Can be None if docker_image is present instead.
- **command** (`list[str]` or `None`) --
  Command of the Job, e.g. `["python", "-c", "print('hello world')"]`
- **arguments** (`list[str]` or `None`) --
  Arguments passed to the command
- **environment** (`dict[str]` or `None`) --
  Environment variables of the Job as a dictionary.
- **secrets** (`dict[str]` or `None`) --
  Secret environment variables of the Job (encrypted).
- **flavor** (`str` or `None`) --
  Flavor for the hardware. See `JobHardware` for possible values.
  E.g. `"cpu-basic"`.
- **labels** (`dict[str, str]` or `None`) --
  Labels to attach to the job (key-value pairs).
- **volumes** (`list[Volume]` or `None`) --
  Volumes mounted in the job container (buckets, models, datasets, spaces).
- **status** -- (`JobStatus` or `None`):
  Status of the Job, e.g. `JobStatus(stage="RUNNING", message=None)`
  See [JobStage](/docs/huggingface_hub/v1.25.1/en/package_reference/jobs#huggingface_hub.JobStage) for possible stage values.
- **durations** (`JobDurations` or `None`) --
  Timing breakdown of the Job. Present for all job states including SCHEDULING.
- **owner** -- (`JobOwner` or `None`):
  Owner of the Job, e.g. `JobOwner(id="5e9ecfc04957053f60648a3e", name="lhoestq", type="user")`
- **initiator** (`JobInitiator` or `None`) --
  What triggered the Job, e.g. `JobInitiator(type="scheduled-job", id="...")` for a cron-triggered run.
- **expose_urls** (`list[str]` or `None`) --
  Public URLs through which the Job's exposed ports are reachable (one per port exposed via `expose=`),
  e.g. `["https://687fb701029421ae5549d998--8000.hf.jobs"]`. `None` when no port is exposed.
  Accessing a URL requires an HF token with read access to the Job's namespace.
- **ssh_url** (`str` or `None`) --
  SSH endpoint of the Job, e.g. `"ssh://687fb701029421ae5549d998@ssh.hf.jobs"`. Only present when the Job
  was started with `ssh=True`. Connecting requires write access to the Job's namespace and an SSH public
  key registered on the Hub (https://huggingface.co/settings/keys).

Contains information about a Job.

Example:

```python
>>> from huggingface_hub import run_job
>>> job = run_job(
...     image="python:3.12",
...     command=["python", "-c", "print('Hello from the cloud!')"]
... )
>>> job
JobInfo(id='687fb701029421ae5549d998', created_at=datetime.datetime(2025, 7, 22, 16, 6, 25, 79000, tzinfo=datetime.timezone.utc), started_at=datetime.datetime(2025, 7, 22, 16, 6, 31, 79000, tzinfo=datetime.timezone.utc), finished_at=None, docker_image='python:3.12', space_id=None, command=['python', '-c', "print('Hello from the cloud!')"], arguments=[], environment={}, secrets={}, flavor='cpu-basic', labels=None, status=JobStatus(stage='RUNNING', message=None), durations=JobDurations(scheduling_secs=6, running_secs=2, total_secs=8), owner=JobOwner(id='5e9ecfc04957053f60648a3e', name='lhoestq', type='user'), initiator=JobInitiator(type='user', id='5e9ecfc04957053f60648a3e', name='lhoestq'), endpoint='https://huggingface.co', url='https://huggingface.co/jobs/lhoestq/687fb701029421ae5549d998')
>>> job.id
'687fb701029421ae5549d998'
>>> job.url
'https://huggingface.co/jobs/lhoestq/687fb701029421ae5549d998'
>>> job.status.stage
'RUNNING'
```

### JobOwner[[huggingface_hub.JobOwner]]

### JobStage[[huggingface_hub.JobStage]]

Enumeration of possible stage of a Job on the Hub.

Value can be compared to a string:
```py
assert JobStage.COMPLETED == "COMPLETED"
```

Possible values are: `COMPLETED`, `CANCELED`, `ERROR`, `DELETED`, `SCHEDULING`, `RUNNING`.
Taken from https://github.com/huggingface/moon-landing/blob/main/server/job_types/JobInfo.ts#L61 (private url).

### JobStatus[[huggingface_hub.JobStatus]]

### Volume[[huggingface_hub.Volume]]

- **type** (`str`) --
  Type of volume: `"bucket"`, `"model"`, `"dataset"`, or `"space"`.
- **source** (`str`) --
  Source identifier, e.g. `"username/my-bucket"` or `"username/my-model"`.
- **mount_path** (`str`) --
  Mount path inside the container, e.g. `"/data"`. Must start with `/`.
- **revision** (`str` or `None`) --
  Git revision (only for repos, defaults to `"main"`).
- **read_only** (`bool` or `None`) --
  Read-only mount. Forced `True` for repos, defaults to `False` for buckets.
- **path** (`str` or `None`) --
  Subfolder prefix inside the bucket/repo to mount, e.g. `"path/to/dir"`.

Describes a volume to mount in a Space or Job container.

Serialize to the JSON payload expected by the Hub API.

Return the volume as an HF mount URI in the format expected by the CLI.
