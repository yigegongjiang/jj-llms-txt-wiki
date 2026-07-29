# Apps

## Core App Classes[[reachy_mini.ReachyMiniApp]]

Base class for Reachy Mini applications.

- **reachy_mini** (ReachyMini) -- The Reachy Mini instance to interact with.
- **stop_event** (threading.Event) -- An event that can be set to stop the app gracefully.
Run the main logic of the app.

Stop the app gracefully.

Wrap the run method with Reachy Mini context management.

Manager for Reachy Mini apps.

Clean up the AppManager, stopping any running app.

Get the current status of the app.

Install a new app by name.

Check if an app is currently running or stopping.

List available apps while preserving curated-only entries.

List available apps for given source kind.

Remove an installed app by name.

Restart the current app.

Start the app as a subprocess.

Raises RuntimeError if an app is already running. When
`evict_remote` is false, a remote WebRTC session holding the app slot
makes the start fail instead of being evicted.

Stop the current app subprocess.

- **app_name** -- Name of the app to update.
- **logger** -- Logger for progress output.- ``RuntimeError`` -- If app is running, not found, or update fails.</raises><raisederrors>``RuntimeError``
Update an installed app by reinstalling it from HuggingFace.

This preserves the original source info and reinstalls to get the latest version.

## App Management[[reachy_mini.apps.manager.AppState]]

Status of a running app.

Status of an app.

Information about a running app.

## App Information[[reachy_mini.apps.AppInfo]]

"}]}>

Metadata about an app.

Kinds of app source.

## App Assistant

### Assistant Functions[[reachy_mini.apps.assistant.validate_app_name]]

Validate the app name.

Check if the given path is inside a git repository.

Validate the location where to create the app project.

Validate the location where to create the app project, ensuring it's not in a git repo.

Create a new Reachy Mini app project using a CLI.

- **console** (Console) -- The console object for printing messages.
- **app_name** (str) -- The name of the app to create.
- **app_path** (Path) -- The directory where the app project will be created.PathThe path to the created app project.
Create a new Reachy Mini app project with the given name at the specified path.

Install the app in a temporary virtual environment with a progress spinner.

- **console** (Console) -- The console object for printing messages.
- **app_path** (str) -- Local path to the app to check.
Check an existing Reachy Mini app project.

Request to add the new app to the official Reachy Mini app store.

Try to push changes to the remote repository.

- **console** (Console) -- The console object for printing messages.
- **app_path** (str) -- Local path to the app to publish.
- **commit_message** (str) -- Commit message for the app publish.
- **official** (bool) -- Request to publish the app as an official Reachy Mini app.
- **no_check** (bool) -- Don't run checks before publishing the app.
- **private** (bool | None) -- If True, make private. If False, make public. If None, prompt.
Publish the app to the Reachy Mini app store.

## App Sources[[reachy_mini.apps.sources.hf_auth.save_hf_token]]

- **token** -- The HuggingFace access token to save.A dict containing- "status": "success" or "error"
- "username": the associated Hugging Face username if successful
- "message": an error description if unsuccessful
Save a HuggingFace access token securely.

Validates the token against the Hugging Face API and, if valid,
stores it using the standard Hugging Face authentication mechanism
for reuse across sessions.

The stored token, or None if no token is stored.
Get stored HuggingFace token.

Delete stored HuggingFace token(s).

Note: logout() without arguments logs out from all saved access tokens.

Status dict with is_logged_in and username.
Check if a token is stored and valid.

List apps available on Hugging Face Spaces.

List all apps available on Hugging Face Spaces (including private ones when authenticated).

Public API to get the site-packages directory for a given app's venv.

For separate venvs: returns the app's venv site-packages
For shared environment (SDK mode): returns the current environment's site-packages

Get the Python executable path for an app (cross-platform).

For separate venvs: returns the app's venv Python
For shared environment: returns the current Python interpreter

List apps available from entry points or separate venvs.

- **app** -- AppInfo with package details.
- **logger** -- Logger for progress output.
- **wireless_version** -- Whether running on wireless version.
- **desktop_app_daemon** -- Whether running as desktop app daemon.
- **force_reinstall** -- If True, force reinstall even if already installed (for updates).
Install a package given an AppInfo object, streaming logs.

Get the module name for an app without loading it (for subprocess execution).

Uninstall a package given an app name.

## App Utilities[[reachy_mini.apps.utils.running_command]]

- **command** -- The command to run as a list of strings.
- **logger** -- Logger instance for output streaming.
- **env** -- Optional environment variables dict. If None, inherits current environment.
Run a shell command and stream its output to the provided logger.
