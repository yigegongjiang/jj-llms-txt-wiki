# Apps

## Core App Classes[[reachy_mini.ReachyMiniApp]]

#### reachy_mini.ReachyMiniApp[[reachy_mini.ReachyMiniApp]]

```python
reachy_mini.ReachyMiniApp(running_on_wireless: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/app.py#L27)

Base class for Reachy Mini applications.

#### run[[reachy_mini.ReachyMiniApp.run]]

```python
run(reachy_mini: ReachyMini, stop_event: Event)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/app.py#L153)

**Parameters:**

reachy_mini (ReachyMini) : The Reachy Mini instance to interact with.

stop_event (threading.Event) : An event that can be set to stop the app gracefully.

Run the main logic of the app.

#### stop[[reachy_mini.ReachyMiniApp.stop]]

```python
stop()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/app.py#L164)

Stop the app gracefully.

#### wrapped_run[[reachy_mini.ReachyMiniApp.wrapped_run]]

```python
wrapped_run(*args: Any, **kwargs: Any)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/app.py#L100)

Wrap the run method with Reachy Mini context management.

#### reachy_mini.apps.manager.AppManager[[reachy_mini.apps.manager.AppManager]]

```python
reachy_mini.apps.manager.AppManager(wireless_version: bool = False, desktop_app_daemon: bool = False, daemon: typing.Optional[ForwardRef('Daemon')] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L62)

Manager for Reachy Mini apps.

#### close[[reachy_mini.apps.manager.AppManager.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L79)

Clean up the AppManager, stopping any running app.

#### current_app_status[[reachy_mini.apps.manager.AppManager.current_app_status]]

```python
current_app_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L399)

Get the current status of the app.

#### get_running_app_url[[reachy_mini.apps.manager.AppManager.get_running_app_url]]

```python
get_running_app_url()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L108)

Return the running app's `custom_app_url`, or `None`.

The JSON-RPC relay uses this to reach the app's `/rpc` endpoint. The
URL is read from the app's `main.py` (same cheap scrape the launcher
uses); the relay normalizes the host (`0.0.0.0` -> `127.0.0.1`).

#### install_new_app[[reachy_mini.apps.manager.AppManager.install_new_app]]

```python
install_new_app(app: AppInfo, logger: Logger)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L450)

Install a new app by name.

#### is_app_running[[reachy_mini.apps.manager.AppManager.is_app_running]]

```python
is_app_running()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L99)

Check if an app is currently running or stopping.

#### list_all_available_apps[[reachy_mini.apps.manager.AppManager.list_all_available_apps]]

```python
list_all_available_apps()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L406)

List available apps while preserving curated-only entries.

#### list_available_apps[[reachy_mini.apps.manager.AppManager.list_available_apps]]

```python
list_available_apps(source: SourceKind)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L434)

List available apps for given source kind.

#### remove_app[[reachy_mini.apps.manager.AppManager.remove_app]]

```python
remove_app(app_name: str, logger: Logger)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L461)

Remove an installed app by name.

#### restart_current_app[[reachy_mini.apps.manager.AppManager.restart_current_app]]

```python
restart_current_app()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L385)

Restart the current app.

#### start_app[[reachy_mini.apps.manager.AppManager.start_app]]

```python
start_app(app_name: str, *args: Any, evict_remote: bool = True, keep_remote: bool = False, **kwargs: Any)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L123)

Start the app as a subprocess.

Raises RuntimeError if an app is already running. When
`evict_remote` is false, a remote WebRTC session holding the app slot
makes the start fail instead of being evicted.

`keep_remote` (used when the start is *requested by* the connected
remote client, e.g. the mobile app driving a conversation) takes the
local-app slot **without** evicting the remote session: the client is a
controller of this app, not a competitor for the robot, so its
DataChannel must survive. Takes precedence over `evict_remote`.

#### stop_current_app[[reachy_mini.apps.manager.AppManager.stop_current_app]]

```python
stop_current_app(timeout: float | None = 20.0)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L301)

Stop the current app subprocess.

#### update_app[[reachy_mini.apps.manager.AppManager.update_app]]

```python
update_app(app_name: str, logger: Logger)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L472)

**Parameters:**

app_name : Name of the app to update.

logger : Logger for progress output.

**Raises:** ``RuntimeError``

- ``RuntimeError`` -- If app is running, not found, or update fails.

Update an installed app by reinstalling it from HuggingFace.

This preserves the original source info and reinstalls to get the latest version.

## App Management[[reachy_mini.apps.manager.AppState]]

#### reachy_mini.apps.manager.AppState[[reachy_mini.apps.manager.AppState]]

```python
reachy_mini.apps.manager.AppState(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L29)

Status of a running app.

#### reachy_mini.apps.manager.AppStatus[[reachy_mini.apps.manager.AppStatus]]

```python
reachy_mini.apps.manager.AppStatus(info: AppInfo, state: AppState, error: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L39)

Status of an app.

#### reachy_mini.apps.manager.RunningApp[[reachy_mini.apps.manager.RunningApp]]

```python
reachy_mini.apps.manager.RunningApp(process: Process, monitor_task: Task, status: AppStatus)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/manager.py#L48)

Information about a running app.

## App Information[[reachy_mini.apps.AppInfo]]

#### reachy_mini.apps.AppInfo[[reachy_mini.apps.AppInfo]]

```python
reachy_mini.apps.AppInfo(name: str, source_kind: SourceKind, description: str = '', url: str | None = None, extra: typing.Dict[str, typing.Any] = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/__init__.py#L18)

Metadata about an app.

#### reachy_mini.apps.SourceKind[[reachy_mini.apps.SourceKind]]

```python
reachy_mini.apps.SourceKind(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/__init__.py#L8)

Kinds of app source.

## App Assistant

### Assistant Functions[[reachy_mini.apps.assistant.validate_app_name]]

#### reachy_mini.apps.assistant.validate_app_name[[reachy_mini.apps.assistant.validate_app_name]]

```python
reachy_mini.apps.assistant.validate_app_name(text: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L21)

Validate the app name.

#### reachy_mini.apps.assistant.is_git_repo[[reachy_mini.apps.assistant.is_git_repo]]

```python
reachy_mini.apps.assistant.is_git_repo(path: Path)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L36)

Check if the given path is inside a git repository.

#### reachy_mini.apps.assistant.validate_location[[reachy_mini.apps.assistant.validate_location]]

```python
reachy_mini.apps.assistant.validate_location(text: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L48)

Validate the location where to create the app project.

#### reachy_mini.apps.assistant.validate_location_and_git_repo[[reachy_mini.apps.assistant.validate_location_and_git_repo]]

```python
reachy_mini.apps.assistant.validate_location_and_git_repo(text: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L57)

Validate the location where to create the app project, ensuring it's not in a git repo.

#### reachy_mini.apps.assistant.create_cli[[reachy_mini.apps.assistant.create_cli]]

```python
reachy_mini.apps.assistant.create_cli(console: Console, app_name: str | None, app_path: pathlib.Path | None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L68)

Create a new Reachy Mini app project using a CLI.

#### reachy_mini.apps.assistant.create[[reachy_mini.apps.assistant.create]]

```python
reachy_mini.apps.assistant.create(console: Console, app_name: str, app_path: Path)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L131)

**Parameters:**

console (Console) : The console object for printing messages.

app_name (str) : The name of the app to create.

app_path (Path) : The directory where the app project will be created.

**Returns:** `Path`

The path to the created app project.

Create a new Reachy Mini app project with the given name at the specified path.

#### reachy_mini.apps.assistant.install_app_with_progress[[reachy_mini.apps.assistant.install_app_with_progress]]

```python
reachy_mini.apps.assistant.install_app_with_progress(console: Console, python_executable: str, app_path: Path, env: dict[str, str] | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L216)

Install the app in a temporary virtual environment with a progress spinner.

#### reachy_mini.apps.assistant.check[[reachy_mini.apps.assistant.check]]

```python
reachy_mini.apps.assistant.check(console: Console, app_path: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L260)

**Parameters:**

console (Console) : The console object for printing messages.

app_path (str) : Local path to the app to check.

Check an existing Reachy Mini app project.

#### reachy_mini.apps.assistant.request_app_addition[[reachy_mini.apps.assistant.request_app_addition]]

```python
reachy_mini.apps.assistant.request_app_addition(new_app_repo_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L576)

Request to add the new app to the official Reachy Mini app store.

#### reachy_mini.apps.assistant.try_to_push[[reachy_mini.apps.assistant.try_to_push]]

```python
reachy_mini.apps.assistant.try_to_push(console: Console, _app_path: Path)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L660)

Try to push changes to the remote repository.

#### reachy_mini.apps.assistant.publish[[reachy_mini.apps.assistant.publish]]

```python
reachy_mini.apps.assistant.publish(console: Console, app_path: str, commit_message: str, official: bool = False, no_check: bool = False, private: bool | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/assistant.py#L710)

**Parameters:**

console (Console) : The console object for printing messages.

app_path (str) : Local path to the app to publish.

commit_message (str) : Commit message for the app publish.

official (bool) : Request to publish the app as an official Reachy Mini app.

no_check (bool) : Don't run checks before publishing the app.

private (bool | None) : If True, make private. If False, make public. If None, prompt.

Publish the app to the Reachy Mini app store.

## App Sources[[reachy_mini.apps.sources.hf_auth.save_hf_token]]

#### reachy_mini.apps.sources.hf_auth.save_hf_token[[reachy_mini.apps.sources.hf_auth.save_hf_token]]

```python
reachy_mini.apps.sources.hf_auth.save_hf_token(token: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_auth.py#L656)

**Parameters:**

token : The HuggingFace access token to save.

**Returns:** `A dict containing`

- "status": "success" or "error"
- "username": the associated Hugging Face username if successful
- "message": an error description if unsuccessful

Save a HuggingFace access token securely.

Validates the token against the Hugging Face API and, if valid,
stores it using the standard Hugging Face authentication mechanism
for reuse across sessions.

#### reachy_mini.apps.sources.hf_auth.get_hf_token[[reachy_mini.apps.sources.hf_auth.get_hf_token]]

```python
reachy_mini.apps.sources.hf_auth.get_hf_token()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_auth.py#L702)

**Returns:**

The stored token, or None if no token is stored.

Get stored HuggingFace token.

#### reachy_mini.apps.sources.hf_auth.delete_hf_token[[reachy_mini.apps.sources.hf_auth.delete_hf_token]]

```python
reachy_mini.apps.sources.hf_auth.delete_hf_token()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_auth.py#L712)

Delete stored HuggingFace token(s).

Note: logout() without arguments logs out from all saved access tokens.

#### reachy_mini.apps.sources.hf_auth.check_token_status[[reachy_mini.apps.sources.hf_auth.check_token_status]]

```python
reachy_mini.apps.sources.hf_auth.check_token_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_auth.py#L726)

**Returns:**

Status dict with is_logged_in and username.

Check if a token is stored and valid.

#### reachy_mini.apps.sources.hf_space.list_available_apps[[reachy_mini.apps.sources.hf_space.list_available_apps]]

```python
reachy_mini.apps.sources.hf_space.list_available_apps()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_space.py#L148)

List apps available on Hugging Face Spaces.

#### reachy_mini.apps.sources.hf_space.list_all_apps[[reachy_mini.apps.sources.hf_space.list_all_apps]]

```python
reachy_mini.apps.sources.hf_space.list_all_apps()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/hf_space.py#L182)

List all apps available on Hugging Face Spaces (including private ones when authenticated).

#### reachy_mini.apps.sources.local_common_venv.get_app_site_packages[[reachy_mini.apps.sources.local_common_venv.get_app_site_packages]]

```python
reachy_mini.apps.sources.local_common_venv.get_app_site_packages(app_name: str, wireless_version: bool = False, desktop_app_daemon: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L122)

Public API to get the site-packages directory for a given app's venv.

For separate venvs: returns the app's venv site-packages
For shared environment (SDK mode): returns the current environment's site-packages

#### reachy_mini.apps.sources.local_common_venv.get_app_python[[reachy_mini.apps.sources.local_common_venv.get_app_python]]

```python
reachy_mini.apps.sources.local_common_venv.get_app_python(app_name: str, wireless_version: bool = False, desktop_app_daemon: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L141)

Get the Python executable path for an app (cross-platform).

For separate venvs: returns the app's venv Python
For shared environment: returns the current Python interpreter

#### reachy_mini.apps.sources.local_common_venv.list_available_apps[[reachy_mini.apps.sources.local_common_venv.list_available_apps]]

```python
reachy_mini.apps.sources.local_common_venv.list_available_apps(wireless_version: bool = False, desktop_app_daemon: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L337)

List apps available from entry points or separate venvs.

#### reachy_mini.apps.sources.local_common_venv.install_package[[reachy_mini.apps.sources.local_common_venv.install_package]]

```python
reachy_mini.apps.sources.local_common_venv.install_package(app: AppInfo, logger: Logger, wireless_version: bool = False, desktop_app_daemon: bool = False, force_reinstall: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L453)

**Parameters:**

app : AppInfo with package details.

logger : Logger for progress output.

wireless_version : Whether running on wireless version.

desktop_app_daemon : Whether running as desktop app daemon.

force_reinstall : If True, force reinstall even if already installed (for updates).

Install a package given an AppInfo object, streaming logs.

#### reachy_mini.apps.sources.local_common_venv.get_app_module[[reachy_mini.apps.sources.local_common_venv.get_app_module]]

```python
reachy_mini.apps.sources.local_common_venv.get_app_module(app_name: str, wireless_version: bool = False, desktop_app_daemon: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L700)

Get the module name for an app without loading it (for subprocess execution).

#### reachy_mini.apps.sources.local_common_venv.uninstall_package[[reachy_mini.apps.sources.local_common_venv.uninstall_package]]

```python
reachy_mini.apps.sources.local_common_venv.uninstall_package(app_name: str, logger: Logger, wireless_version: bool = False, desktop_app_daemon: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/sources/local_common_venv.py#L733)

Uninstall a package given an app name.

## App Utilities[[reachy_mini.apps.utils.running_command]]

#### reachy_mini.apps.utils.running_command[[reachy_mini.apps.utils.running_command]]

```python
reachy_mini.apps.utils.running_command(command: list, logger: Logger, env: dict[str, str] | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/apps/utils.py#L9)

**Parameters:**

command : The command to run as a list of strings.

logger : Logger instance for output streaming.

env : Optional environment variables dict. If None, inherits current environment.

Run a shell command and stream its output to the provided logger.
