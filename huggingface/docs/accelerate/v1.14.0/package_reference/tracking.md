# Experiment Trackers

## GeneralTracker[[accelerate.tracking.GeneralTracker]]

#### accelerate.tracking.GeneralTracker[[accelerate.tracking.GeneralTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L101)

A base Tracker class to be used for all logging integration implementations.

Each function should take in `**kwargs` that will automatically be passed in from a base dictionary provided to
[Accelerator](/docs/accelerate/v1.14.0/en/package_reference/accelerator#accelerate.Accelerator).

Should implement `name`, `requires_logging_directory`, and `tracker` properties such that:

`name` (`str`): String representation of the tracker class name, such as "TensorBoard" `requires_logging_directory`
(`bool`): Whether the logger requires a directory to store their logs. `tracker` (`object`): Should return internal
tracking mechanism used by a tracker class (such as the `run` for wandb)

Implementations can also include a `main_process_only` (`bool`) attribute to toggle if relevant logging, init, and
other functions should occur on the main process or across all processes (by default will use `True`)

finishaccelerate.tracking.GeneralTracker.finishhttps://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L171[]

Should run any finalizing functions within the tracking API. If the API should not have one, just don't
overwrite that method.
#### log[[accelerate.tracking.GeneralTracker.log]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L159)

Logs `values` to the current run. Base `log` implementations of a tracking API should go in here, along with
special behavior for the `step parameter.

**Parameters:**

values (Dictionary `str` to `str`, `float`, or `int`) : Values to be logged as key-value pairs. The values need to have type `str`, `float`, or `int`.

step (`int`, *optional*) : The run step. If included, the log will be affiliated with this step.
#### start[[accelerate.tracking.GeneralTracker.start]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L142)

Lazy initialization of the tracker inside Accelerator to avoid initializing PartialState before
InitProcessGroupKwargs.
#### store_init_configuration[[accelerate.tracking.GeneralTracker.store_init_configuration]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L148)

Logs `values` as hyperparameters for the run. Implementations should use the experiment configuration
functionality of a tracking API.

**Parameters:**

values (Dictionary `str` to `bool`, `str`, `float` or `int`) : Values to be stored as initial hyperparameters as key-value pairs. The values need to have type `bool`, `str`, `float`, `int`, or `None`.

## TensorBoardTracker[[accelerate.tracking.TensorBoardTracker]]

#### accelerate.tracking.TensorBoardTracker[[accelerate.tracking.TensorBoardTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L178)

A `Tracker` class that supports `tensorboard`. Should be initialized at the start of your script.

__init__accelerate.tracking.TensorBoardTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L194[{"name": "run_name", "val": ": str"}, {"name": "logging_dir", "val": ": typing.Union[str, os.PathLike]"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run

logging_dir (`str`, `os.PathLike`) : Location for TensorBoard logs to be stored.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `tensorboard.SummaryWriter.__init__` method.

## WandBTracker[[accelerate.tracking.WandBTracker]]

#### accelerate.tracking.WandBTracker[[accelerate.tracking.WandBTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L293)

A `Tracker` class that supports `wandb`. Should be initialized at the start of your script.

__init__accelerate.tracking.WandBTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L308[{"name": "run_name", "val": ": str"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `wandb.init` method.

## Trackio[[accelerate.tracking.TrackioTracker]]

#### accelerate.tracking.TrackioTracker[[accelerate.tracking.TrackioTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L418)

A `Tracker` class that supports `trackio`. Should be initialized at the start of your script.

__init__accelerate.tracking.TrackioTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L435[{"name": "run_name", "val": ": str"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run. Will be used as the `project` name when instantiating trackio.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `trackio.init` method. Refer to this [init](https://github.com/gradio-app/trackio/blob/814809552310468b13f84f33764f1369b4e5136c/trackio/__init__.py#L22) to see all supported key word arguments.

## CometMLTracker[[accelerate.tracking.CometMLTracker]]

#### accelerate.tracking.CometMLTracker[[accelerate.tracking.CometMLTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L495)

A `Tracker` class that supports `comet_ml`. Should be initialized at the start of your script.

API keys must be stored in a Comet config file.

Note:
For `comet_ml` versions 

__init__accelerate.tracking.CometMLTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L516[{"name": "run_name", "val": ": str"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `comet_ml.start` method: https://www.comet.com/docs/v2/api-and-sdk/python-sdk/reference/start/

## AimTracker[[accelerate.tracking.AimTracker]]

#### accelerate.tracking.AimTracker[[accelerate.tracking.AimTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L589)

A `Tracker` class that supports `aim`. Should be initialized at the start of your script.

__init__accelerate.tracking.AimTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L603[{"name": "run_name", "val": ": str"}, {"name": "logging_dir", "val": ": typing.Union[str, os.PathLike, NoneType] = '.'"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `Run.__init__` method.

## MLflowTracker[[accelerate.tracking.MLflowTracker]]

#### accelerate.tracking.MLflowTracker[[accelerate.tracking.MLflowTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L692)

A `Tracker` class that supports `mlflow`. Should be initialized at the start of your script.

__init__accelerate.tracking.MLflowTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L723[{"name": "experiment_name", "val": ": typing.Optional[str] = None"}, {"name": "logging_dir", "val": ": typing.Union[str, os.PathLike, NoneType] = None"}, {"name": "run_id", "val": ": typing.Optional[str] = None"}, {"name": "tags", "val": ": typing.Union[dict[str, typing.Any], str, NoneType] = None"}, {"name": "nested_run", "val": ": typing.Optional[bool] = False"}, {"name": "run_name", "val": ": typing.Optional[str] = None"}, {"name": "description", "val": ": typing.Optional[str] = None"}]

**Parameters:**

experiment_name (`str`, *optional*) : Name of the experiment. Environment variable MLFLOW_EXPERIMENT_NAME has priority over this argument.

logging_dir (`str` or `os.PathLike`, defaults to `"."`) : Location for mlflow logs to be stored.

run_id (`str`, *optional*) : If specified, get the run with the specified UUID and log parameters and metrics under that run. The run’s end time is unset and its status is set to running, but the run’s other attributes (source_version, source_type, etc.) are not changed. Environment variable MLFLOW_RUN_ID has priority over this argument.

tags (`Dict[str, str]`, *optional*) : An optional `dict` of `str` keys and values, or a `str` dump from a `dict`, to set as tags on the run. If a run is being resumed, these tags are set on the resumed run. If a new run is being created, these tags are set on the new run. Environment variable MLFLOW_TAGS has priority over this argument.

nested_run (`bool`, *optional*, defaults to `False`) : Controls whether run is nested in parent run. True creates a nested run. Environment variable MLFLOW_NESTED_RUN has priority over this argument.

run_name (`str`, *optional*) : Name of new run (stored as a mlflow.runName tag). Used only when `run_id` is unspecified.

description (`str`, *optional*) : An optional string that populates the description box of the run. If a run is being resumed, the description is set on the resumed run. If a new run is being created, the description is set on the new run.

## ClearMLTracker[[accelerate.tracking.ClearMLTracker]]

#### accelerate.tracking.ClearMLTracker[[accelerate.tracking.ClearMLTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L901)

A `Tracker` class that supports `clearml`. Should be initialized at the start of your script.

__init__accelerate.tracking.ClearMLTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L916[{"name": "run_name", "val": ": typing.Optional[str] = None"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`, *optional*) : Name of the experiment. Environment variables `CLEARML_PROJECT` and `CLEARML_TASK` have priority over this argument.

- ****kwargs** (additional keyword arguments, *optional*) : Kwargs passed along to the `Task.__init__` method.

## SwanLabTracker[[accelerate.tracking.SwanLabTracker]]

#### accelerate.tracking.SwanLabTracker[[accelerate.tracking.SwanLabTracker]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L1147)

A `Tracker` class that supports `swanlab`. Should be initialized at the start of your script.

__init__accelerate.tracking.SwanLabTracker.__init__https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/tracking.py#L1162[{"name": "run_name", "val": ": str"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `swanlab.init` method.
