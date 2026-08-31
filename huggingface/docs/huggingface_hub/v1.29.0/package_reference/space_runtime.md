# Managing your Space runtime

Check the [HfApi](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi) documentation page for the reference of methods to manage your Space on the Hub.

- Duplicate a Space: [duplicate_space()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.duplicate_space)
- Fetch current runtime: [get_space_runtime()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.get_space_runtime)
- Fetch build or run logs: [fetch_space_logs()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.fetch_space_logs)
- Manage secrets: [add_space_secret()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.add_space_secret) and [delete_space_secret()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.delete_space_secret)
- Manage hardware: [request_space_hardware()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.request_space_hardware)
- Manage state: [pause_space()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.pause_space), [restart_space()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.restart_space), [set_space_sleep_time()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.set_space_sleep_time)
- Wait until Space is ready: [wait_for_space()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.wait_for_space)

## Data structures

### SpaceRuntime[[huggingface_hub.SpaceRuntime]]

#### huggingface_hub.SpaceRuntime[[huggingface_hub.SpaceRuntime]]

```python
huggingface_hub.SpaceRuntime(data: dict)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L195)

**Parameters:**

stage (`str`) : Current stage of the space. Example: RUNNING.

hardware (`str` or `None`) : Current hardware of the space. Example: "cpu-basic". Can be `None` if Space is `BUILDING` for the first time.

requested_hardware (`str` or `None`) : Requested hardware. Can be different from `hardware` especially if the request has just been made. Example: "t4-medium". Can be `None` if no hardware has been requested yet.

sleep_time (`int` or `None`) : Number of seconds the Space will be kept alive after the last request. By default (if value is `None`), the Space will never go to sleep if it's running on an upgraded hardware, while it will go to sleep after 48 hours on a free 'cpu-basic' hardware. For more details, see https://huggingface.co/docs/hub/spaces-gpus#sleep-time.

volumes (`list[Volume]` or `None`) : List of volumes mounted in the Space. Each volume is a [Volume](/docs/huggingface_hub/v1.29.0/en/package_reference/jobs#huggingface_hub.Volume) object describing its type, source, mount path, and optional settings. `None` if no volumes are attached.

raw (`dict`) : Raw response from the server. Contains more information about the Space runtime like number of replicas, number of cpu, memory size,...

Contains information about the current runtime of a Space.

### SpaceHardware[[huggingface_hub.SpaceHardware]]

#### huggingface_hub.SpaceHardware[[huggingface_hub.SpaceHardware]]

```python
huggingface_hub.SpaceHardware(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L68)

Enumeration of hardwares available to run your Space on the Hub.

Value can be compared to a string:
```py
assert SpaceHardware.CPU_BASIC == "cpu-basic"
```

Taken from https://github.com/huggingface-internal/moon-landing/blob/main/server/repo_types/SpaceHardwareFlavor.ts (private url).

### SpaceStage[[huggingface_hub.SpaceStage]]

#### huggingface_hub.SpaceStage[[huggingface_hub.SpaceStage]]

```python
huggingface_hub.SpaceStage(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L22)

Enumeration of possible stage of a Space on the Hub.

Value can be compared to a string:
```py
assert SpaceStage.BUILDING == "BUILDING"
```

Taken from https://github.com/huggingface/moon-landing/blob/main/server/repo_types/SpaceInfo.ts#L61 (private url).

### SpaceStorage[[huggingface_hub.SpaceStorage]]

#### huggingface_hub.SpaceStorage[[huggingface_hub.SpaceStorage]]

```python
huggingface_hub.SpaceStorage(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L104)

Enumeration of persistent storage available for your Space on the Hub.

Value can be compared to a string:
```py
assert SpaceStorage.SMALL == "small"
```

Taken from https://github.com/huggingface/moon-landing/blob/main/server/repo_types/SpaceHardwareFlavor.ts#L24 (private url).

### SpaceVariable[[huggingface_hub.SpaceVariable]]

#### huggingface_hub.SpaceVariable[[huggingface_hub.SpaceVariable]]

```python
huggingface_hub.SpaceVariable(key: str, values: dict)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L273)

**Parameters:**

key (`str`) : Variable key. Example: `"MODEL_REPO_ID"`

value (`str`) : Variable value. Example: `"the_model_repo_id"`.

description (`str` or None) : Description of the variable. Example: `"Model Repo ID of the implemented model"`.

updatedAt (`datetime` or None) : datetime of the last update of the variable (if the variable has been updated at least once).

Contains information about the current variables of a Space.

### SpaceTemplate[[huggingface_hub.SpaceTemplate]]

#### huggingface_hub.SpaceTemplate[[huggingface_hub.SpaceTemplate]]

```python
huggingface_hub.SpaceTemplate(data: dict)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_space_api.py#L368)

**Parameters:**

name (`str`) : Human-friendly name of the template (e.g. `"JupyterLab"`, `"chatbot"`).

repo_id (`str`) : Repo id of the template Space (e.g. `"SpacesExamples/jupyterlab"`).

sdk (`str`) : SDK the template is built with (e.g. `"gradio"`, `"docker"`, `"static"`).

preferred_private (`bool`) : Whether Spaces created from this template are recommended to be private.

Contains information about a Space template available on the Hub.

Returned by [HfApi.list_space_templates()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.list_space_templates). The `repo_id` can be passed as `space_template`
to [HfApi.create_repo()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.create_repo) to seed a new Space from that template.
