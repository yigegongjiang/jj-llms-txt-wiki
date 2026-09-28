# Experiment Trackers

## GeneralTracker[[accelerate.tracking.GeneralTracker]]

#### accelerate.tracking.GeneralTracker[[accelerate.tracking.GeneralTracker]]

```python
accelerate.tracking.GeneralTracker(_blank = False)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L103)

A base Tracker class to be used for all logging integration implementations.

Each function should take in `**kwargs` that will automatically be passed in from a base dictionary provided to
[Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator).

Should implement `name`, `requires_logging_directory`, and `tracker` properties such that:

`name` (`str`): String representation of the tracker class name, such as "TensorBoard" `requires_logging_directory`
(`bool`): Whether the logger requires a directory to store their logs. `tracker` (`object`): Should return internal
tracking mechanism used by a tracker class (such as the `run` for wandb)

Implementations can also include a `main_process_only` (`bool`) attribute to toggle if relevant logging, init, and
other functions should occur on the main process or across all processes (by default will use `True`)

#### finish[[accelerate.tracking.GeneralTracker.finish]]

```python
finish()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L173)

Should run any finalizing functions within the tracking API. If the API should not have one, just don't
overwrite that method.

#### log[[accelerate.tracking.GeneralTracker.log]]

```python
log(values: dict, step: typing.Optional[int] = None, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L161)

**Parameters:**

values (Dictionary `str` to `str`, `float`, or `int`) : Values to be logged as key-value pairs. The values need to have type `str`, `float`, or `int`.

step (`int`, *optional*) : The run step. If included, the log will be affiliated with this step.

Logs `values` to the current run. Base `log` implementations of a tracking API should go in here, along with
special behavior for the `step parameter.

#### start[[accelerate.tracking.GeneralTracker.start]]

```python
start()
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L144)

Lazy initialization of the tracker inside Accelerator to avoid initializing PartialState before
InitProcessGroupKwargs.

#### store_init_configuration[[accelerate.tracking.GeneralTracker.store_init_configuration]]

```python
store_init_configuration(values: dict)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L150)

**Parameters:**

values (Dictionary `str` to `bool`, `str`, `float` or `int`) : Values to be stored as initial hyperparameters as key-value pairs. The values need to have type `bool`, `str`, `float`, `int`, or `None`.

Logs `values` as hyperparameters for the run. Implementations should use the experiment configuration
functionality of a tracking API.

## register_tracker_class[[accelerate.tracking.register_tracker_class]]

#### accelerate.tracking.register_tracker_class[[accelerate.tracking.register_tracker_class]]

```python
accelerate.tracking.register_tracker_class(tracker_class: type)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L1264)

**Parameters:**

tracker_class (subclass of `GeneralTracker`) : The tracker class to register. It must subclass `GeneralTracker` and define a non-empty `name` class attribute and a `requires_logging_directory` class attribute. The class is instantiated with the `project_name` passed to `Accelerator.init_trackers()` as the first positional argument. When `requires_logging_directory` is `True`, the logging directory is passed as the second positional argument (matching the built-in tracker convention). Tracker-specific keyword arguments from `init_kwargs` are forwarded as `**kwargs`.

Registers a custom `GeneralTracker` subclass so it can be referenced by its `name` in the `log_with` argument of
[Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator), the same way as the built-in trackers.

The tracker must be registered before instantiating the [Accelerator](/docs/accelerate/v1.15.0/en/package_reference/accelerator#accelerate.Accelerator) that uses it.

Example:

```python
from accelerate import Accelerator
from accelerate.tracking import GeneralTracker, register_tracker_class

class MyTracker(GeneralTracker):
    name = "my_tracker"
    requires_logging_directory = False
    # ... implement the rest of the `GeneralTracker` interface

register_tracker_class(MyTracker)
accelerator = Accelerator(log_with="my_tracker")
```

## TensorBoardTracker[[accelerate.tracking.TensorBoardTracker]]

#### accelerate.tracking.TensorBoardTracker[[accelerate.tracking.TensorBoardTracker]]

```python
accelerate.tracking.TensorBoardTracker(run_name: str, logging_dir: typing.Union[str, os.PathLike], **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L180)

**Parameters:**

run_name (`str`) : The name of the experiment run

logging_dir (`str`, `os.PathLike`) : Location for TensorBoard logs to be stored.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `tensorboard.SummaryWriter.__init__` method.

A `Tracker` class that supports `tensorboard`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.TensorBoardTracker.__init__]]

```python
__init__(run_name: str, logging_dir: typing.Union[str, os.PathLike], **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L196)

## WandBTracker[[accelerate.tracking.WandBTracker]]

#### accelerate.tracking.WandBTracker[[accelerate.tracking.WandBTracker]]

```python
accelerate.tracking.WandBTracker(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L295)

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `wandb.init` method.

A `Tracker` class that supports `wandb`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.WandBTracker.__init__]]

```python
__init__(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L310)

## Trackio[[accelerate.tracking.TrackioTracker]]

#### accelerate.tracking.TrackioTracker[[accelerate.tracking.TrackioTracker]]

```python
accelerate.tracking.TrackioTracker(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L420)

**Parameters:**

run_name (`str`) : The name of the experiment run. Will be used as the `project` name when instantiating trackio.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `trackio.init` method. Refer to this [init](https://github.com/gradio-app/trackio/blob/814809552310468b13f84f33764f1369b4e5136c/trackio/__init__.py#L22) to see all supported key word arguments.

A `Tracker` class that supports `trackio`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.TrackioTracker.__init__]]

```python
__init__(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L437)

## CometMLTracker[[accelerate.tracking.CometMLTracker]]

#### accelerate.tracking.CometMLTracker[[accelerate.tracking.CometMLTracker]]

```python
accelerate.tracking.CometMLTracker(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L497)

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `comet_ml.start` method: https://www.comet.com/docs/v2/api-and-sdk/python-sdk/reference/start/

A `Tracker` class that supports `comet_ml`. Should be initialized at the start of your script.

API keys must be stored in a Comet config file.

Note:
For `comet_ml` versions < 3.41.0, additional keyword arguments are passed to `comet_ml.Experiment` instead:
https://www.comet.com/docs/v2/api-and-sdk/python-sdk/reference/Experiment/#comet_ml.Experiment.__init__

#### __init__[[accelerate.tracking.CometMLTracker.__init__]]

```python
__init__(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L518)

## AimTracker[[accelerate.tracking.AimTracker]]

#### accelerate.tracking.AimTracker[[accelerate.tracking.AimTracker]]

```python
accelerate.tracking.AimTracker(run_name: str, logging_dir: typing.Union[str, os.PathLike, NoneType] = '.', **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L591)

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `Run.__init__` method.

A `Tracker` class that supports `aim`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.AimTracker.__init__]]

```python
__init__(run_name: str, logging_dir: typing.Union[str, os.PathLike, NoneType] = '.', **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L605)

## MLflowTracker[[accelerate.tracking.MLflowTracker]]

#### accelerate.tracking.MLflowTracker[[accelerate.tracking.MLflowTracker]]

```python
accelerate.tracking.MLflowTracker(experiment_name: typing.Optional[str] = None, logging_dir: typing.Union[str, os.PathLike, NoneType] = None, run_id: typing.Optional[str] = None, tags: typing.Union[dict[str, typing.Any], str, NoneType] = None, nested_run: typing.Optional[bool] = False, run_name: typing.Optional[str] = None, description: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L694)

**Parameters:**

experiment_name (`str`, *optional*) : Name of the experiment. Environment variable MLFLOW_EXPERIMENT_NAME has priority over this argument.

logging_dir (`str` or `os.PathLike`, defaults to `"."`) : Location for mlflow logs to be stored.

run_id (`str`, *optional*) : If specified, get the run with the specified UUID and log parameters and metrics under that run. The run’s end time is unset and its status is set to running, but the run’s other attributes (source_version, source_type, etc.) are not changed. Environment variable MLFLOW_RUN_ID has priority over this argument.

tags (`Dict[str, str]`, *optional*) : An optional `dict` of `str` keys and values, or a `str` dump from a `dict`, to set as tags on the run. If a run is being resumed, these tags are set on the resumed run. If a new run is being created, these tags are set on the new run. Environment variable MLFLOW_TAGS has priority over this argument.

nested_run (`bool`, *optional*, defaults to `False`) : Controls whether run is nested in parent run. True creates a nested run. Environment variable MLFLOW_NESTED_RUN has priority over this argument.

run_name (`str`, *optional*) : Name of new run (stored as a mlflow.runName tag). Used only when `run_id` is unspecified.

description (`str`, *optional*) : An optional string that populates the description box of the run. If a run is being resumed, the description is set on the resumed run. If a new run is being created, the description is set on the new run.

A `Tracker` class that supports `mlflow`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.MLflowTracker.__init__]]

```python
__init__(experiment_name: typing.Optional[str] = None, logging_dir: typing.Union[str, os.PathLike, NoneType] = None, run_id: typing.Optional[str] = None, tags: typing.Union[dict[str, typing.Any], str, NoneType] = None, nested_run: typing.Optional[bool] = False, run_name: typing.Optional[str] = None, description: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L725)

## ClearMLTracker[[accelerate.tracking.ClearMLTracker]]

#### accelerate.tracking.ClearMLTracker[[accelerate.tracking.ClearMLTracker]]

```python
accelerate.tracking.ClearMLTracker(run_name: typing.Optional[str] = None, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L905)

**Parameters:**

run_name (`str`, *optional*) : Name of the experiment. Environment variables `CLEARML_PROJECT` and `CLEARML_TASK` have priority over this argument.

- ****kwargs** (additional keyword arguments, *optional*) : Kwargs passed along to the `Task.__init__` method.

A `Tracker` class that supports `clearml`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.ClearMLTracker.__init__]]

```python
__init__(run_name: typing.Optional[str] = None, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L920)

## SwanLabTracker[[accelerate.tracking.SwanLabTracker]]

#### accelerate.tracking.SwanLabTracker[[accelerate.tracking.SwanLabTracker]]

```python
accelerate.tracking.SwanLabTracker(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L1151)

**Parameters:**

run_name (`str`) : The name of the experiment run.

- ****kwargs** (additional keyword arguments, *optional*) : Additional key word arguments passed along to the `swanlab.init` method.

A `Tracker` class that supports `swanlab`. Should be initialized at the start of your script.

#### __init__[[accelerate.tracking.SwanLabTracker.__init__]]

```python
__init__(run_name: str, **kwargs)
```

[Source](https://github.com/huggingface/accelerate/blob/v1.15.0/src/accelerate/tracking.py#L1166)
