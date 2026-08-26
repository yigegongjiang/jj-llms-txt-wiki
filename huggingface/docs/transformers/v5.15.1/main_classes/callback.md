# Callbacks

Callbacks are objects that can customize the behavior of the training loop in the PyTorch
[Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) that can inspect the training loop state (for progress reporting, logging on TensorBoard or other ML
platforms...) and take decisions (like early stopping).

Callbacks are "read only" pieces of code, apart from the [TrainerControl](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerControl) object they return, they
cannot change anything in the training loop. For customizations that require changes in the training loop, you should
subclass [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) and override the methods you need (see [trainer](trainer) for examples).

By default, `TrainingArguments.report_to` is set to `"none"`.

The main class that implements callbacks is [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback). It gets the
[TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments) used to instantiate the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer), can access that
Trainer's internal state via [TrainerState](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerState), and can take some actions on the training loop via
[TrainerControl](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerControl).

## Available Callbacks[[transformers.integrations.CometCallback]]

Here is the list of the available [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) in the library:

#### transformers.integrations.CometCallback[[transformers.integrations.CometCallback]]

```python
transformers.integrations.CometCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1133)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [Comet ML](https://www.comet.com/site/).

#### setup[[transformers.integrations.CometCallback.setup]]

```python
setup(args, state, model)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1147)

Setup the optional Comet integration.

Environment:
- **COMET_MODE** (`str`, *optional*, default to `get_or_create`):
  Control whether to create and log to a new Comet experiment or append to an existing experiment.
  It accepts the following values:
  * `get_or_create`: Decides automatically depending if
    `COMET_EXPERIMENT_KEY` is set and whether an Experiment
    with that key already exists or not.
  * `create`: Always create a new Comet Experiment.
  * `get`: Always try to append to an Existing Comet Experiment.
    Requires `COMET_EXPERIMENT_KEY` to be set.
- **COMET_START_ONLINE** (`bool`, *optional*):
  Whether to create an online or offline Experiment.
- **COMET_PROJECT_NAME** (`str`, *optional*):
  Comet project name for experiments.
- **COMET_LOG_ASSETS** (`str`, *optional*, defaults to `TRUE`):
  Whether or not to log training assets (checkpoints, etc), to Comet. Can be `TRUE`, or
  `FALSE`.

For a number of configurable items in the environment, see
[here](https://www.comet.com/docs/v2/guides/experiment-management/configure-sdk/#explore-comet-configuration-options).

#### transformers.DefaultFlowCallback[[transformers.DefaultFlowCallback]]

```python
transformers.DefaultFlowCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L563)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that handles the default flow of the training loop for logs, evaluation and checkpoints.

#### transformers.PrinterCallback[[transformers.PrinterCallback]]

```python
transformers.PrinterCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L698)

A bare [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that just prints the logs.

#### transformers.ProgressCallback[[transformers.ProgressCallback]]

```python
transformers.ProgressCallback(max_str_len: int = 100)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L624)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that displays the progress of training or evaluation.
You can modify `max_str_len` to control how long strings are truncated when logging.

#### transformers.EarlyStoppingCallback[[transformers.EarlyStoppingCallback]]

```python
transformers.EarlyStoppingCallback(early_stopping_patience: int = 1, early_stopping_threshold: float | None = 0.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L711)

**Parameters:**

early_stopping_patience (`int`) : Use with `metric_for_best_model` to stop training when the specified metric worsens for `early_stopping_patience` evaluation calls.

early_stopping_threshold(`float`, *optional*) : Use with TrainingArguments `metric_for_best_model` and `early_stopping_patience` to denote how much the specified metric must improve to satisfy early stopping conditions. `

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that handles early stopping.

This callback depends on [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments) argument *load_best_model_at_end* functionality to set best_metric
in [TrainerState](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerState). Note that if the [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments) argument *save_steps* differs from *eval_steps*, the
early stopping will not occur until the next save step.

#### transformers.integrations.TensorBoardCallback[[transformers.integrations.TensorBoardCallback]]

```python
transformers.integrations.TensorBoardCallback(tb_writer = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L577)

**Parameters:**

tb_writer (`SummaryWriter`, *optional*) : The writer to use. Will instantiate one if not set.

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [TensorBoard](https://www.tensorflow.org/tensorboard).

Environment:
- **TENSORBOARD_LOGGING_DIR** (`str`, *optional*, defaults to `None`):
  The logging dir to log the results. Default value is os.path.join(args.output_dir, default_logdir())

#### transformers.integrations.TrackioCallback[[transformers.integrations.TrackioCallback]]

```python
transformers.integrations.TrackioCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L935)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that logs metrics to Trackio.

#### setup[[transformers.integrations.TrackioCallback.setup]]

```python
setup(args, state, model, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L969)

Setup the optional Trackio integration.

To customize the setup you can also set `project`, `trackio_space_id`, `trackio_bucket_id`,
`trackio_static_space_id`, and `hub_private_repo` in [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments).

#### transformers.integrations.WandbCallback[[transformers.integrations.WandbCallback]]

```python
transformers.integrations.WandbCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L695)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that logs metrics, media, model checkpoints to [Weight and Biases](https://www.wandb.com/).

#### setup[[transformers.integrations.WandbCallback.setup]]

```python
setup(args, state, model, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L711)

Setup the optional Weights & Biases (*wandb*) integration.

One can subclass and override this method to customize the setup if needed. Find more information
[here](https://docs.wandb.ai/guides/integrations/huggingface). You can also override the following environment
variables:

Environment:
- **WANDB_LOG_MODEL** (`str`, *optional*, defaults to `"false"`):
  Whether to log model and checkpoints during training. Can be `"end"`, `"checkpoint"` or `"false"`. If set
  to `"end"`, the model will be uploaded at the end of training. If set to `"checkpoint"`, the checkpoint
  will be uploaded every `args.save_steps` . If set to `"false"`, the model will not be uploaded. Use along
  with `load_best_model_at_end()` to upload best model.
- **WANDB_WATCH** (`str`, *optional* defaults to `"false"`):
  Can be `"gradients"`, `"all"`, `"parameters"`, or `"false"`. Set to `"all"` to log gradients and
  parameters.
- **WANDB_PROJECT** (`str`, *optional*, defaults to `"huggingface"`):
  Set this to a custom string to store results in a different project.

#### transformers.integrations.MLflowCallback[[transformers.integrations.MLflowCallback]]

```python
transformers.integrations.MLflowCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1286)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [MLflow](https://www.mlflow.org/). Can be disabled by setting
environment variable `DISABLE_MLFLOW_INTEGRATION = TRUE`.

#### setup[[transformers.integrations.MLflowCallback.setup]]

```python
setup(args, state, model)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1305)

Setup the optional MLflow integration.

Environment:
- **HF_MLFLOW_LOG_ARTIFACTS** (`str`, *optional*):
  Whether to use MLflow `.log_artifact()` facility to log artifacts. This only makes sense if logging to a
  remote server, e.g. s3 or GCS. If set to `True` or *1*, will copy each saved checkpoint on each save in
  [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments)'s `output_dir` to the local or remote artifact storage. Using it without a remote
  storage will just copy the files to your artifact location.
- **MLFLOW_TRACKING_URI** (`str`, *optional*):
  Whether to store runs at a specific path or remote server. Unset by default, which skips setting the
  tracking URI entirely.
- **MLFLOW_EXPERIMENT_NAME** (`str`, *optional*, defaults to `None`):
  Whether to use an MLflow experiment_name under which to launch the run. Default to `None` which will point
  to the `Default` experiment in MLflow. Otherwise, it is a case sensitive name of the experiment to be
  activated. If an experiment with this name does not exist, a new experiment with this name is created.
- **MLFLOW_TAGS** (`str`, *optional*):
  A string dump of a dictionary of key/value pair to be added to the MLflow run as tags. Example:
  `os.environ['MLFLOW_TAGS']='{"release.candidate": "RC1", "release.version": "2.2.0"}'`.
- **MLFLOW_NESTED_RUN** (`str`, *optional*):
  Whether to use MLflow nested runs. If set to `True` or *1*, will create a nested run inside the current
  run.
- **MLFLOW_RUN_ID** (`str`, *optional*):
  Allow to reattach to an existing run which can be useful when resuming training from a checkpoint. When
  `MLFLOW_RUN_ID` environment variable is set, `start_run` attempts to resume a run with the specified run ID
  and other parameters are ignored.
- **MLFLOW_FLATTEN_PARAMS** (`str`, *optional*, defaults to `False`):
  Whether to flatten the parameters dictionary before logging.
- **MLFLOW_MAX_LOG_PARAMS** (`int`, *optional*):
  Set the maximum number of parameters to log in the run.

#### transformers.integrations.AzureMLCallback[[transformers.integrations.AzureMLCallback]]

```python
transformers.integrations.AzureMLCallback(azureml_run = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1263)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [AzureML](https://pypi.org/project/azureml-sdk/).

#### transformers.integrations.CodeCarbonCallback[[transformers.integrations.CodeCarbonCallback]]

```python
transformers.integrations.CodeCarbonCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1809)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that tracks the CO2 emission of training.

#### transformers.integrations.ClearMLCallback[[transformers.integrations.ClearMLCallback]]

```python
transformers.integrations.ClearMLCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1843)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [ClearML](https://clear.ml/).

Environment:
- **CLEARML_PROJECT** (`str`, *optional*, defaults to `HuggingFace Transformers`):
  ClearML project name.
- **CLEARML_TASK** (`str`, *optional*, defaults to `Trainer`):
  ClearML task name.
- **CLEARML_LOG_MODEL** (`bool`, *optional*, defaults to `False`):
  Whether to log models as artifacts during training.

#### transformers.integrations.DagsHubCallback[[transformers.integrations.DagsHubCallback]]

```python
transformers.integrations.DagsHubCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1467)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that logs to [DagsHub](https://dagshub.com/). Extends `MLflowCallback`

#### setup[[transformers.integrations.DagsHubCallback.setup]]

```python
setup(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L1481)

Setup the DagsHub's Logging integration.

Environment:
- **HF_DAGSHUB_LOG_ARTIFACTS** (`str`, *optional*):
  Whether to save the data and model artifacts for the experiment. Default to `False`.

#### transformers.integrations.FlyteCallback[[transformers.integrations.FlyteCallback]]

```python
transformers.integrations.FlyteCallback(save_log_history: bool = True, sync_checkpoints: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2094)

**Parameters:**

save_log_history (`bool`, *optional*, defaults to `True`) : When set to True, the training logs are saved as a Flyte Deck. 

sync_checkpoints (`bool`, *optional*, defaults to `True`) : When set to True, checkpoints are synced with Flyte and can be used to resume training in the case of an interruption.

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [Flyte](https://flyte.org/).
NOTE: This callback only works within a Flyte task.

Example:

```python
# Note: This example skips over some setup steps for brevity.
from flytekit import current_context, task

@task
def train_hf_transformer():
    cp = current_context().checkpoint
    trainer = Trainer(..., callbacks=[FlyteCallback()])
    output = trainer.train(resume_from_checkpoint=cp.restore())
```

#### transformers.integrations.KubeflowCallback[[transformers.integrations.KubeflowCallback]]

```python
transformers.integrations.KubeflowCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2436)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that reports training progress to [Kubeflow Trainer](https://github.com/kubeflow/trainer).

This callback is automatically registered when training inside a Kubeflow TrainJob with the
`TrainJobRuntimeStatus` feature gate enabled. The Kubeflow controller injects the required
environment variables into the training pod.

**Environment Variables (injected by controller):**

- `KUBEFLOW_TRAINER_SERVER_URL`: HTTPS endpoint for status updates
- `KUBEFLOW_TRAINER_SERVER_CA_CERT`: Path to CA certificate for TLS verification
- `KUBEFLOW_TRAINER_SERVER_TOKEN`: Path to service account token for authentication

**Reported Information:**

- Progress percentage (0-100%)
- Estimated time remaining (seconds)
- Training metrics (loss, learning_rate, etc.)

**Features:**

- Automatic throttling (max 1 update per 5 seconds) to avoid overwhelming the controller
- Token caching (5 minutes) to minimize file I/O
- Only rank 0 reports progress in distributed training
- Silent failures - network issues won't interrupt training

Can be disabled by setting environment variable `DISABLE_KUBEFLOW_INTEGRATION=TRUE`.

#### transformers.integrations.DVCLiveCallback[[transformers.integrations.DVCLiveCallback]]

```python
transformers.integrations.DVCLiveCallback(live: typing.Optional[typing.Any] = None, log_model: typing.Union[typing.Literal['all'], bool, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2157)

**Parameters:**

live (`dvclive.Live`, *optional*, defaults to `None`) : Optional Live instance. If None, a new instance will be created using **kwargs.

log_model (Union[Literal["all"], bool], *optional*, defaults to `None`) : Whether to use `dvclive.Live.log_artifact()` to log checkpoints created by [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer). If set to `True`, the final checkpoint is logged at the end of training. If set to `"all"`, the entire [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments)'s `output_dir` is logged at each checkpoint.

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that sends the logs to [DVCLive](https://www.dvc.org/doc/dvclive).

Use the environment variables below in `setup` to configure the integration. To customize this callback beyond
those environment variables, see [here](https://dvc.org/doc/dvclive/ml-frameworks/huggingface).

#### setup[[transformers.integrations.DVCLiveCallback.setup]]

```python
setup(args, state, model)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2198)

Setup the optional DVCLive integration. To customize this callback beyond the environment variables below, see
[here](https://dvc.org/doc/dvclive/ml-frameworks/huggingface).

Environment:
- **HF_DVCLIVE_LOG_MODEL** (`str`, *optional*):
  Whether to use `dvclive.Live.log_artifact()` to log checkpoints created by [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer). If set to `True` or
  *1*, the final checkpoint is logged at the end of training. If set to `all`, the entire
  [TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments)'s `output_dir` is logged at each checkpoint.

#### transformers.integrations.SwanLabCallback[[transformers.integrations.SwanLabCallback]]

```python
transformers.integrations.SwanLabCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2262)

A [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) that logs metrics, media, model checkpoints to [SwanLab](https://swanlab.cn/).

#### setup[[transformers.integrations.SwanLabCallback.setup]]

```python
setup(args, state, model, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/integration_utils.py#L2276)

Setup the optional SwanLab (*swanlab*) integration.

One can subclass and override this method to customize the setup if needed. Find more information
[here](https://docs.swanlab.cn/guide_cloud/integration/integration-huggingface-transformers.html).

You can also override the following environment variables. Find more information about environment
variables [here](https://docs.swanlab.cn/en/api/environment-variable.html#environment-variables)

Environment:
- **SWANLAB_API_KEY** (`str`, *optional*, defaults to `None`):
  Cloud API Key. During login, this environment variable is checked first. If it doesn't exist, the system
  checks if the user is already logged in. If not, the login process is initiated.

  - If a string is passed to the login interface, this environment variable is ignored.
  - If the user is already logged in, this environment variable takes precedence over locally stored
  login information.

- **SWANLAB_PROJECT** (`str`, *optional*, defaults to `None`):
  Set this to a custom string to store results in a different project. If not specified, the name of the current
  running directory is used.

- **SWANLAB_LOG_DIR** (`str`, *optional*, defaults to `swanlog`):
  This environment variable specifies the storage path for log files when running in local mode.
  By default, logs are saved in a folder named swanlog under the working directory.

- **SWANLAB_MODE** (`Literal["local", "cloud", "disabled"]`, *optional*, defaults to `cloud`):
  SwanLab's parsing mode, which involves callbacks registered by the operator. Currently, there are three modes:
  local, cloud, and disabled. Note: Case-sensitive. Find more information
  [here](https://docs.swanlab.cn/en/api/py-init.html#swanlab-init)

- **SWANLAB_LOG_MODEL** (`str`, *optional*, defaults to `None`):
  SwanLab does not currently support the save mode functionality.This feature will be available in a future
  release

- **SWANLAB_WEB_HOST** (`str`, *optional*, defaults to `None`):
  Web address for the SwanLab cloud environment for private version (its free)

- **SWANLAB_API_HOST** (`str`, *optional*, defaults to `None`):
  API address for the SwanLab cloud environment for private version (its free)

- **SWANLAB_RUN_ID** (`str`, *optional*, defaults to `None`):
  Experiment ID to resume a previous run. Use with `SWANLAB_RESUME` to continue an existing experiment.

- **SWANLAB_RESUME** (`str`, *optional*, defaults to `None`):
  Resume mode (`"must"`, `"allow"`, `"never"`). Defaults to `"allow"` when `resume_from_checkpoint` is used.

## TrainerCallback[[transformers.TrainerCallback]]

#### transformers.TrainerCallback[[transformers.TrainerCallback]]

```python
transformers.TrainerCallback()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L295)

**Parameters:**

args ([TrainingArguments](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.TrainingArguments)) : The training arguments used to instantiate the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer).

state ([TrainerState](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerState)) : The current state of the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer).

control ([TrainerControl](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerControl)) : The object that is returned to the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) and can be used to make some decisions.

model ([PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel) or `torch.nn.Module`) : The model being trained.

processing_class ([`PreTrainedTokenizer` or `BaseImageProcessor` or `ProcessorMixin` or `FeatureExtractionMixin`]) : The processing class used for encoding the data. Can be a tokenizer, a processor, an image processor or a feature extractor.

optimizer (`torch.optim.Optimizer`) : The optimizer used for the training steps.

lr_scheduler (`torch.optim.lr_scheduler.LambdaLR`) : The scheduler used for setting the learning rate.

train_dataloader (`torch.utils.data.DataLoader`, *optional*) : The current dataloader used for training.

eval_dataloader (`torch.utils.data.DataLoader`, *optional*) : The current dataloader used for evaluation.

metrics (`dict[str, float]`) : The metrics computed by the last evaluation phase.  Those are only accessible in the event `on_evaluate`.

logs  (`dict[str, float]`) : The values to log.  Those are only accessible in the event `on_log`.

A class for objects that will inspect the state of the training loop at some events and take some decisions. At
each of those events the following arguments are available:

The `control` object is the only one that can be changed by the callback, in which case the event that changes it
should return the modified version.

The argument `args`, `state` and `control` are positionals for all events, all the others are grouped in `kwargs`.
You can unpack the ones you need in the signature of the event using them. As an example, see the code of the
simple [PrinterCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.PrinterCallback).

Example:

```python
class PrinterCallback(TrainerCallback):
    def on_log(self, args, state, control, logs=None, **kwargs):
        _ = logs.pop("total_flos", None)
        if state.is_local_process_zero:
            print(logs)
```

#### on_epoch_begin[[transformers.TrainerCallback.on_epoch_begin]]

```python
on_epoch_begin(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L361)

Event called at the beginning of an epoch.

#### on_epoch_end[[transformers.TrainerCallback.on_epoch_end]]

```python
on_epoch_end(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L366)

Event called at the end of an epoch.

#### on_evaluate[[transformers.TrainerCallback.on_evaluate]]

```python
on_evaluate(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L398)

Event called after an evaluation phase.

#### on_init_end[[transformers.TrainerCallback.on_init_end]]

```python
on_init_end(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L346)

Event called at the end of the initialization of the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer).

#### on_log[[transformers.TrainerCallback.on_log]]

```python
on_log(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L413)

Event called after logging the last logs.

#### on_optimizer_step[[transformers.TrainerCallback.on_optimizer_step]]

```python
on_optimizer_step(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L382)

Event called after the optimizer step but before gradients are zeroed out. Useful for monitoring gradients.

#### on_pre_optimizer_step[[transformers.TrainerCallback.on_pre_optimizer_step]]

```python
on_pre_optimizer_step(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L377)

Event called before the optimizer step but after gradient clipping. Useful for monitoring gradients.

#### on_predict[[transformers.TrainerCallback.on_predict]]

```python
on_predict(args: TrainingArguments, state: TrainerState, control: TrainerControl, metrics, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L403)

Event called after a successful prediction.

#### on_prediction_step[[transformers.TrainerCallback.on_prediction_step]]

```python
on_prediction_step(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L418)

Event called after a prediction step.

#### on_push_begin[[transformers.TrainerCallback.on_push_begin]]

```python
on_push_begin(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L423)

Event called before pushing the model to the hub, at the beginning of Trainer.push_to_hub and Trainer._push_from_checkpoint.

#### on_save[[transformers.TrainerCallback.on_save]]

```python
on_save(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L408)

Event called after a checkpoint save.

#### on_step_begin[[transformers.TrainerCallback.on_step_begin]]

```python
on_step_begin(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L371)

Event called at the beginning of a training step. If using gradient accumulation, one training step might take
several inputs.

#### on_step_end[[transformers.TrainerCallback.on_step_end]]

```python
on_step_end(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L392)

Event called at the end of a training step. If using gradient accumulation, one training step might take
several inputs.

#### on_substep_end[[transformers.TrainerCallback.on_substep_end]]

```python
on_substep_end(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L387)

Event called at the end of an substep during gradient accumulation.

#### on_train_begin[[transformers.TrainerCallback.on_train_begin]]

```python
on_train_begin(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L351)

Event called at the beginning of training.

#### on_train_end[[transformers.TrainerCallback.on_train_end]]

```python
on_train_end(args: TrainingArguments, state: TrainerState, control: TrainerControl, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L356)

Event called at the end of training.

Here is an example of how to register a custom callback with the PyTorch [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer):

```python
class MyCallback(TrainerCallback):
    "A callback that prints a message at the beginning of training"

    def on_train_begin(self, args, state, control, **kwargs):
        print("Starting training")

trainer = Trainer(
    model,
    args,
    train_dataset=train_dataset,
    eval_dataset=eval_dataset,
    callbacks=[MyCallback],  # We can either pass the callback class this way or an instance of it (MyCallback())
)
```

Another way to register a callback is to call `trainer.add_callback()` as follows:

```python
trainer = Trainer(...)
trainer.add_callback(MyCallback)
# Alternatively, we can pass an instance of the callback class
trainer.add_callback(MyCallback())
```

## TrainerState[[transformers.TrainerState]]

#### transformers.TrainerState[[transformers.TrainerState]]

```python
transformers.TrainerState(epoch: float = 0, global_step: int = 0, max_steps: int = 0, logging_steps: int = 500, eval_steps: int = 500, save_steps: int = 500, train_batch_size: int | None = None, num_train_epochs: int = 0, num_input_tokens_seen: int = 0, total_flos: float = 0, log_history: list = None, best_metric: float | None = None, best_global_step: int | None = None, best_model_checkpoint: str | None = None, is_local_process_zero: bool = True, is_world_process_zero: bool = True, is_hyper_param_search: bool = False, trial_name: str | None = None, trial_params: dict[str, str | float | int | bool] | None = None, stateful_callbacks: list['TrainerCallback'] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L35)

**Parameters:**

epoch (`float`, *optional*) : Only set during training, will represent the epoch the training is at (the decimal part being the percentage of the current epoch completed).

global_step (`int`, *optional*, defaults to 0) : During training, represents the number of update steps completed.

max_steps (`int`, *optional*, defaults to 0) : The number of update steps to do during the current training.

logging_steps (`int`, *optional*, defaults to 500) : Log every X updates steps

eval_steps (`int`, *optional*) : Run an evaluation every X steps.

save_steps (`int`, *optional*, defaults to 500) : Save checkpoint every X updates steps.

train_batch_size (`int`, *optional*) : The batch size for the training dataloader. Only needed when `auto_find_batch_size` has been used.

num_input_tokens_seen (`int`, *optional*, defaults to 0) : When tracking the inputs tokens, the number of tokens seen during training (number of input tokens, not the number of prediction tokens).

total_flos (`float`, *optional*, defaults to 0) : The total number of floating operations done by the model since the beginning of training (stored as floats to avoid overflow).

log_history (`list[dict[str, float]]`, *optional*) : The list of logs done since the beginning of training.

best_metric (`float`, *optional*) : When tracking the best model, the value of the best metric encountered so far.

best_global_step (`int`, *optional*) : When tracking the best model, the step at which the best metric was encountered. Used for setting `best_model_checkpoint`.

best_model_checkpoint (`str`, *optional*) : When tracking the best model, the value of the name of the checkpoint for the best model encountered so far.

is_local_process_zero (`bool`, *optional*, defaults to `True`) : Whether or not this process is the local (e.g., on one machine if training in a distributed fashion on several machines) main process.

is_world_process_zero (`bool`, *optional*, defaults to `True`) : Whether or not this process is the global main process (when training in a distributed fashion on several machines, this is only going to be `True` for one process).

is_hyper_param_search (`bool`, *optional*, defaults to `False`) : Whether we are in the process of a hyper parameter search using Trainer.hyperparameter_search. This will impact the way data will be logged in TensorBoard.

stateful_callbacks (`list[StatefulTrainerCallback]`, *optional*) : Callbacks attached to the `Trainer` that should have their states be saved or restored. Relevant callbacks should implement a `state` and `from_state` function.

A class containing the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) inner state that will be saved along the model and optimizer when checkpointing
and passed to the [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback).

In all this class, one step is to be understood as one update step. When using gradient accumulation, one update
step may require several forward and backward passes: if you use `gradient_accumulation_steps=n`, then one update
step requires going through *n* batches.

#### compute_steps[[transformers.TrainerState.compute_steps]]

```python
compute_steps(args, max_steps)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L156)

Calculates and stores the absolute value for logging,
eval, and save steps based on if it was a proportion
or not.

#### init_training_references[[transformers.TrainerState.init_training_references]]

```python
init_training_references(trainer, max_steps, num_train_epochs, trial)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L169)

Stores the initial training references needed in `self`

#### load_from_json[[transformers.TrainerState.load_from_json]]

```python
load_from_json(json_path: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L149)

Create an instance from the content of `json_path`.

#### save_to_json[[transformers.TrainerState.save_to_json]]

```python
save_to_json(json_path: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L143)

Save the content of this instance in JSON format inside `json_path`.

## TrainerControl[[transformers.TrainerControl]]

#### transformers.TrainerControl[[transformers.TrainerControl]]

```python
transformers.TrainerControl(should_training_stop: bool = False, should_epoch_stop: bool = False, should_save: bool = False, should_evaluate: bool = False, should_log: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/trainer_callback.py#L234)

**Parameters:**

should_training_stop (`bool`, *optional*, defaults to `False`) : Whether or not the training should be interrupted.  If `True`, this variable will not be set back to `False`. The training will just stop.

should_epoch_stop (`bool`, *optional*, defaults to `False`) : Whether or not the current epoch should be interrupted.  If `True`, this variable will be set back to `False` at the beginning of the next epoch.

should_save (`bool`, *optional*, defaults to `False`) : Whether or not the model should be saved at this step.  If `True`, this variable will be set back to `False` at the beginning of the next step.

should_evaluate (`bool`, *optional*, defaults to `False`) : Whether or not the model should be evaluated at this step.  If `True`, this variable will be set back to `False` at the beginning of the next step.

should_log (`bool`, *optional*, defaults to `False`) : Whether or not the logs should be reported at this step.  If `True`, this variable will be set back to `False` at the beginning of the next step.

A class that handles the [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) control flow. This class is used by the [TrainerCallback](/docs/transformers/v5.15.1/en/main_classes/callback#transformers.TrainerCallback) to activate some
switches in the training loop.
