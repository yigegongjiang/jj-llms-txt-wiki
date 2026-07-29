# Utilities

## Configure logging[[huggingface_hub.utils.logging.get_verbosity]]

The `huggingface_hub` package exposes a `logging` utility to control the logging level of the package itself.
You can import it as such:

```py
from huggingface_hub import logging
```

Then, you may define the verbosity in order to update the amount of logs you'll see:

```python
from huggingface_hub import logging

logging.set_verbosity_error()
logging.set_verbosity_warning()
logging.set_verbosity_info()
logging.set_verbosity_debug()

logging.set_verbosity(...)
```

The levels should be understood as follows:

- `error`: only show critical logs about usage which may result in an error or unexpected behavior.
- `warning`: show logs that aren't critical but usage may result in unintended behavior.
  Additionally, important informative logs may be shown.
- `info`: show most logs, including some verbose logging regarding what is happening under the hood.
  If something is behaving in an unexpected manner, we recommend switching the verbosity level to this in order
  to get more information.
- `debug`: show all logs, including some internal logs which may be used to track exactly what's happening
  under the hood.

Logging level, e.g., `huggingface_hub.logging.DEBUG` and
`huggingface_hub.logging.INFO`.
Return the current level for the HuggingFace Hub's root logger.

> [!TIP]
> HuggingFace Hub has following logging levels:
>
> - `huggingface_hub.logging.CRITICAL`, `huggingface_hub.logging.FATAL`
> - `huggingface_hub.logging.ERROR`
> - `huggingface_hub.logging.WARNING`, `huggingface_hub.logging.WARN`
> - `huggingface_hub.logging.INFO`
> - `huggingface_hub.logging.DEBUG`

- **verbosity** (`int`) --
  Logging level, e.g., `huggingface_hub.logging.DEBUG` and
  `huggingface_hub.logging.INFO`.

Sets the level for the HuggingFace Hub's root logger.

Sets the verbosity to `logging.INFO`.

Sets the verbosity to `logging.DEBUG`.

Sets the verbosity to `logging.WARNING`.

Sets the verbosity to `logging.ERROR`.

Disable propagation of the library log outputs. Note that log propagation is
disabled by default.

Enable propagation of the library log outputs. Please disable the
HuggingFace Hub's default handler to prevent double logging if the root
logger has been configured.

### Repo-specific helper methods[[huggingface_hub.utils.logging.get_logger]]

The methods exposed below are relevant when modifying modules from the `huggingface_hub` library itself.
Using these shouldn't be necessary if you use `huggingface_hub` and you don't modify them.

- **name** (`str`, *optional*) --
  The name of the logger to get, usually the filename

Returns a logger with the specified name. This function is not supposed
to be directly accessed by library users.

Example:

```python
>>> from huggingface_hub import get_logger

>>> logger = get_logger(__file__)
>>> logger.set_verbosity_info()
```

## Configure progress bars

Progress bars are a useful tool to display information to the user while a long-running task is being executed (e.g.
when downloading or uploading files). `huggingface_hub` exposes a `tqdm` wrapper to display progress bars in a
consistent way across the library.

By default, progress bars are enabled. You can disable them globally by setting `HF_HUB_DISABLE_PROGRESS_BARS`
environment variable. You can also enable/disable them using [enable_progress_bars()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.utils.enable_progress_bars) and
[disable_progress_bars](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.utils.disable_progress_bars). If set, the environment variable has priority on the helpers.

```py
>>> from huggingface_hub import snapshot_download
>>> from huggingface_hub.utils import are_progress_bars_disabled, disable_progress_bars, enable_progress_bars

>>> # Disable progress bars globally
>>> disable_progress_bars()

>>> # Progress bar will not be shown !
>>> snapshot_download("gpt2")

>>> are_progress_bars_disabled()
True

>>> # Re-enable progress bars globally
>>> enable_progress_bars()
```

### Group-specific control of progress bars

You can also enable or disable progress bars for specific groups. This allows you to manage progress bar visibility more granularly within different parts of your application or library. When a progress bar is disabled for a group, all subgroups under it are also affected unless explicitly overridden.

```py
# Disable progress bars for a specific group
>>> disable_progress_bars("peft.foo")
>>> assert not are_progress_bars_disabled("peft")
>>> assert not are_progress_bars_disabled("peft.something")
>>> assert are_progress_bars_disabled("peft.foo")
>>> assert are_progress_bars_disabled("peft.foo.bar")

# Re-enable progress bars for a subgroup
>>> enable_progress_bars("peft.foo.bar")
>>> assert are_progress_bars_disabled("peft.foo")
>>> assert not are_progress_bars_disabled("peft.foo.bar")

# Use groups with tqdm
# No progress bar for `name="peft.foo"`
>>> for _ in tqdm(range(5), name="peft.foo"):
...     pass

# Progress bar will be shown for `name="peft.foo.bar"`
>>> for _ in tqdm(range(5), name="peft.foo.bar"):
...     pass
100%|███████████████████████████████████████| 5/5 [00:00<00:00, 117817.53it/s]
```

### are_progress_bars_disabled[[huggingface_hub.utils.are_progress_bars_disabled]]

- **name** (`str`, *optional*) --
  The group name to check; if None, checks the global setting.`bool`True if progress bars are disabled, False otherwise.

Check if progress bars are disabled globally or for a specific group.

This function returns whether progress bars are disabled for a given group or globally.
It checks the `HF_HUB_DISABLE_PROGRESS_BARS` environment variable first, then the programmatic
settings.

### disable_progress_bars[[huggingface_hub.utils.disable_progress_bars]]

- **name** (`str`, *optional*) --
  The name of the group for which to disable the progress bars. If None,
  progress bars are disabled globally.- ``Warning`` -- If the environment variable precludes changes.</raises><raisederrors>``Warning``

Disable progress bars either globally or for a specified group.

This function updates the state of progress bars based on a group name.
If no group name is provided, all progress bars are disabled. The operation
respects the `HF_HUB_DISABLE_PROGRESS_BARS` environment variable's setting.

Works as both a regular call and a context manager:
disable_progress_bars()           # disables until enable_progress_bars()
with disable_progress_bars():     # disables for the block, re-enables on exit
...

### enable_progress_bars[[huggingface_hub.utils.enable_progress_bars]]

- **name** (`str`, *optional*) --
  The name of the group for which to enable the progress bars. If None,
  progress bars are enabled globally.- ``Warning`` -- If the environment variable precludes changes.</raises><raisederrors>``Warning``

Enable progress bars either globally or for a specified group.

This function sets the progress bars to enabled for the specified group or globally
if no group is specified. The operation is subject to the `HF_HUB_DISABLE_PROGRESS_BARS`
environment setting.

## Configuring the HTTP Backend[[huggingface_hub.set_client_factory]]

In `huggingface_hub` v0.x, HTTP requests were handled with `requests`, and configuration was done via `configure_http_backend`. Since we now use `httpx`, configuration works differently: you must provide a factory function that takes no arguments and returns an `httpx.Client`. You can review the [default implementation here](https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/utils/_http.py) to see which parameters are used by default.

In some setups, you may need to control how HTTP requests are made, for example when working behind a proxy. The `huggingface_hub` library allows you to configure this globally with [set_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_client_factory). After configuration, all requests to the Hub will use your custom settings. Since `huggingface_hub` relies on `httpx.Client` under the hood, you can check the [`httpx` documentation](https://www.python-httpx.org/advanced/clients/) for details on available parameters.

If you are building a third-party library and need to make direct requests to the Hub, use [get_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_session) to obtain a correctly configured `httpx` client. Replace any direct `httpx.get(...)` calls with `get_session().get(...)` to ensure proper behavior.

Set the HTTP client factory to be used by `huggingface_hub`.

The client factory is a method that returns a `httpx.Client` object. On the first call to [get_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_session) the client factory
will be used to create a new `httpx.Client` object that will be shared between all calls made by `huggingface_hub`.

This can be useful if you are running your scripts in a specific environment requiring custom configuration (e.g. custom proxy or certifications).

Use [get_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_session) to get a correctly configured `httpx.Client`.

Get a `httpx.Client` object, using the transport factory from the user.

This client is shared between all calls made by `huggingface_hub`. Therefore you should not close it manually.

Use [set_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_client_factory) to customize the `httpx.Client`.

In rare cases, you may want to manually close the current session (for example, after a transient `SSLError`). You can do this with [close_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.close_session). A new session will automatically be created on the next call to [get_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_session).

Sessions are always closed automatically when the process exits.

Close the global `httpx.Client` used by `huggingface_hub`.

If a Client is closed, it will be recreated on the next call to [get_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_session).

Can be useful if e.g. an SSL certificate has been updated.

For async code, use [set_async_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_async_client_factory) to configure an `httpx.AsyncClient` and [get_async_session()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.get_async_session) to retrieve one.

Set the HTTP async client factory to be used by `huggingface_hub`.

The async client factory is a method that returns a `httpx.AsyncClient` object.
This can be useful if you are running your scripts in a specific environment requiring custom configuration (e.g. custom proxy or certifications).
Use `get_async_client` to get a correctly configured `httpx.AsyncClient`.

> [!WARNING]
> Contrary to the `httpx.Client` that is shared between all calls made by `huggingface_hub`, the `httpx.AsyncClient` is not shared.
> It is recommended to use an async context manager to ensure the client is properly closed when the context is exited.

Return a `httpx.AsyncClient` object, using the transport factory from the user.

Use [set_async_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_async_client_factory) to customize the `httpx.AsyncClient`.

> [!WARNING]
> Contrary to the `httpx.Client` that is shared between all calls made by `huggingface_hub`, the `httpx.AsyncClient` is not shared.
> It is recommended to use an async context manager to ensure the client is properly closed when the context is exited.

Unlike the synchronous client, the lifecycle of the async client is not managed automatically. Use an async context manager to handle it properly.

## Handle HTTP errors

`huggingface_hub` defines its own HTTP errors to refine the `HTTPError` raised by
`httpx` with additional information sent back by the server.

### Raise for status[[huggingface_hub.hf_raise_for_status]]

[hf_raise_for_status()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.hf_raise_for_status) is meant to be the central method to "raise for status" from any
request made to the Hub. It wraps the base `httpx.Response.raise_for_status` to provide
additional information. Any `HTTPError` thrown is converted into a `HfHubHTTPError`.

```py
from huggingface_hub.utils import get_session, hf_raise_for_status, HfHubHTTPError

response = get_session().post(...)
try:
    hf_raise_for_status(response)
except HfHubHTTPError as e:
    print(str(e)) # formatted message
    e.request_id, e.server_message # details returned by server

    # Complete the error message with additional information once it's raised
    e.append_to_message("\n`create_commit` expects the repository to exist.")
    raise
```

- **response** (`Response`) --
  Response from the server.
- **endpoint_name** (`str`, *optional*) --
  Name of the endpoint that has been called. If provided, the error message will be more complete.

Internal version of `response.raise_for_status()` that will refine a potential HTTPError.
Raised exception will be an instance of [HfHubHTTPError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.HfHubHTTPError).

This helper is meant to be the unique method to raise_for_status when making a call to the Hugging Face Hub.

> [!WARNING]
> Raises when the request has failed:
>
>     - [RepositoryNotFoundError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.RepositoryNotFoundError)
>         If the repository to download from cannot be found. This may be because it
>         doesn't exist, because `repo_type` is not set correctly, or because the repo
>         is `private` and you do not have access.
>     - [GatedRepoError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.GatedRepoError)
>         If the repository exists but is gated and the user is not on the authorized
>         list.
>     - [RevisionNotFoundError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.RevisionNotFoundError)
>         If the repository exists but the revision couldn't be found.
>     - [EntryNotFoundError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.EntryNotFoundError)
>         If the repository exists but the entry (e.g. the requested file) couldn't be
>         find.
>     - [BadRequestError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.BadRequestError)
>         If request failed with a HTTP 400 BadRequest error.
>     - [HfHubHTTPError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.HfHubHTTPError)
>         If request failed for a reason not listed above.

### Check offline mode[[huggingface_hub.is_offline_mode]]

You can programmatically check if offline mode is enabled using `is_offline_mode`. Offline mode is enabled by setting `HF_HUB_OFFLINE=1` as environment variable.

Returns whether we are in offline mode for the Hub.

When offline mode is enabled, all HTTP requests made with `get_session` will raise an `OfflineModeIsEnabled` exception.

Example:
```py
from huggingface_hub import is_offline_mode

def list_files(repo_id: str):
    if is_offline_mode():
        ... # list files from local cache (degraded experience but still functional)
    else:
        ... # list files from Hub (complete experience)
```

### HTTP errors

Here is a list of HTTP errors thrown in `huggingface_hub`.

#### HfHubHTTPError[[huggingface_hub.errors.HfHubHTTPError]]

`HfHubHTTPError` is the parent class for any HF Hub HTTP error. It takes care of parsing
the server response and format the error message to provide as much information to the
user as possible.

HTTPError to inherit from for any custom HTTP Error raised in HF Hub.

Any HTTPError is converted at least into a `HfHubHTTPError`. If some information is
sent back by the server, it will be added to the error message.

Added details:
- Request ID sourced from headers in order of precedence: "X-Request-Id", "X-Amzn-Trace-Id", "X-Amz-Cf-Id".
- Server error message from the header "X-Error-Message".
- Server error message if we can found one in the response body.

Example:
```py
    import httpx
    from huggingface_hub.utils import get_session, hf_raise_for_status, HfHubHTTPError

    response = get_session().post(...)
    try:
        hf_raise_for_status(response)
    except HfHubHTTPError as e:
        print(str(e)) # formatted message
        e.request_id, e.server_message # details returned by server

        # Complete the error message with additional information once it's raised
        e.append_to_message("
ate_commit` expects the repository to exist.")
        raise
```

Append additional information to the `HfHubHTTPError` initial message.

#### RepositoryNotFoundError[[huggingface_hub.errors.RepositoryNotFoundError]]

- **repo_id** (`str` or `None`) --
  The repo id that was not found, if it could be determined from the request URL.
- **repo_type** (`str` or `None`) --
  The repo type ("model", "dataset", or "space"), if it could be determined from the request URL.

Raised when trying to access a hf.co URL with an invalid repository name, or
with a private repo name the user does not have access to.

Example:

```py
>>> from huggingface_hub import model_info
>>> model_info("<non_existent_repository>")
(...)
huggingface_hub.errors.RepositoryNotFoundError: 401 Client Error. (Request ID: PvMw_VjBMjVdMz53WKIzP)

Repository Not Found for url: https://huggingface.co/api/models/%3Cnon_existent_repository%3E.
Please make sure you specified the correct `repo_id` and `repo_type`.
If the repo is private, make sure you are authenticated and your token has the required permissions.
Invalid username or password.
```

#### GatedRepoError[[huggingface_hub.errors.GatedRepoError]]

Raised when trying to access a gated repository for which the user is not on the
authorized list.

Note: derives from `RepositoryNotFoundError` to ensure backward compatibility.

Example:

```py
>>> from huggingface_hub import model_info
>>> model_info("<gated_repository>")
(...)
huggingface_hub.errors.GatedRepoError: 403 Client Error. (Request ID: ViT1Bf7O_026LGSQuVqfa)

Cannot access gated repo for url https://huggingface.co/api/models/ardent-figment/gated-model.
Access to model ardent-figment/gated-model is restricted and you are not in the authorized list.
Visit https://huggingface.co/ardent-figment/gated-model to ask for access.
```

#### RevisionNotFoundError[[huggingface_hub.errors.RevisionNotFoundError]]

- **repo_id** (`str` or `None`) --
  The repo id, if it could be determined from the request URL.
- **repo_type** (`str` or `None`) --
  The repo type ("model", "dataset", or "space"), if it could be determined from the request URL.

Raised when trying to access a hf.co URL with a valid repository but an invalid
revision.

Example:

```py
>>> from huggingface_hub import hf_hub_download
>>> hf_hub_download('bert-base-cased', 'config.json', revision='<non-existent-revision>')
(...)
huggingface_hub.errors.RevisionNotFoundError: 404 Client Error. (Request ID: Mwhe_c3Kt650GcdKEFomX)

Revision Not Found for url: https://huggingface.co/bert-base-cased/resolve/%3Cnon-existent-revision%3E/config.json.
```

#### BadRequestError[[huggingface_hub.errors.BadRequestError]]

Raised by `hf_raise_for_status` when the server returns a HTTP 400 error.

Example:

```py
>>> resp = httpx.post("hf.co/api/check", ...)
>>> hf_raise_for_status(resp, endpoint_name="check")
huggingface_hub.errors.BadRequestError: Bad request for check endpoint: {details} (Request ID: XXX)
```

#### EntryNotFoundError[[huggingface_hub.errors.EntryNotFoundError]]

Raised when entry not found, either locally or remotely.

Example:

```py
>>> from huggingface_hub import hf_hub_download
>>> hf_hub_download('bert-base-cased', '<non-existent-file>')
(...)
huggingface_hub.errors.RemoteEntryNotFoundError (...)
>>> hf_hub_download('bert-base-cased', '<non-existent-file>', local_files_only=True)
(...)
huggingface_hub.utils.errors.LocalEntryNotFoundError (...)
```

#### RemoteEntryNotFoundError[[huggingface_hub.errors.RemoteEntryNotFoundError]]

- **repo_id** (`str` or `None`) --
  The repo id, if it could be determined from the request URL.
- **repo_type** (`str` or `None`) --
  The repo type ("model", "dataset", or "space"), if it could be determined from the request URL.

Raised when trying to access a hf.co URL with a valid repository and revision
but an invalid filename.

Example:

```py
>>> from huggingface_hub import hf_hub_download
>>> hf_hub_download('bert-base-cased', '<non-existent-file>')
(...)
huggingface_hub.errors.EntryNotFoundError: 404 Client Error. (Request ID: 53pNl6M0MxsnG5Sw8JA6x)

Entry Not Found for url: https://huggingface.co/bert-base-cased/resolve/main/%3Cnon-existent-file%3E.
```

#### LocalEntryNotFoundError[[huggingface_hub.errors.LocalEntryNotFoundError]]

Raised when trying to access a file or snapshot that is not on the disk when network is
disabled or unavailable (connection issue). The entry may exist on the Hub.

Example:

```py
>>> from huggingface_hub import hf_hub_download
>>> hf_hub_download('bert-base-cased', '<non-cached-file>',  local_files_only=True)
(...)
huggingface_hub.errors.LocalEntryNotFoundError: Cannot find the requested files in the disk cache and outgoing traffic has been disabled. To enable hf.co look-ups and downloads online, set 'local_files_only' to False.
```

#### IncompleteSnapshotError[[huggingface_hub.errors.IncompleteSnapshotError]]

Raised by [snapshot_download()](/docs/huggingface_hub/v1.25.1/en/package_reference/file_download#huggingface_hub.snapshot_download) when the Hub cannot be reached (offline, connection issue, or
`local_files_only=True`) and the cached snapshot is known to be incomplete: some files listed in
the repository's cached tree listing are missing from the local snapshot.

This is a subclass of `LocalEntryNotFoundError` for backward compatibility.

The `snapshot_path` attribute holds the path to the incomplete local snapshot, so a downstream library can locate
the latest cached files even though they are known to be incomplete.

#### CachedRepoTreeNotFoundError[[huggingface_hub.errors.CachedRepoTreeNotFoundError]]

Raised by [get_cached_repo_tree()](/docs/huggingface_hub/v1.25.1/en/package_reference/file_download#huggingface_hub.get_cached_repo_tree) when no tree listing is cached for the requested revision.

The tree listing is populated as a side effect of [snapshot_download()](/docs/huggingface_hub/v1.25.1/en/package_reference/file_download#huggingface_hub.snapshot_download).

#### OfflineModeIsEnabled[[huggingface_hub.errors.OfflineModeIsEnabled]]

Raised when a request is made but `HF_HUB_OFFLINE=1` is set as environment variable.

## Telemetry[[huggingface_hub.utils.send_telemetry]]

`huggingface_hub` includes a helper to send telemetry data. This information helps us debug issues and prioritize new features.
Users can disable telemetry collection at any time by setting the `HF_HUB_DISABLE_TELEMETRY=1` environment variable.
Telemetry is also disabled in offline mode (i.e. when setting HF_HUB_OFFLINE=1).

If you are maintainer of a third-party library, sending telemetry data is as simple as making a call to `send_telemetry`.
Data is sent in a separate thread to reduce as much as possible the impact for users.

- **topic** (`str`) --
  Name of the topic that is monitored. The topic is directly used to build the URL. If you want to monitor
  subtopics, just use "/" separation. Examples: "gradio", "transformers/examples",...
- **library_name** (`str`, *optional*) --
  The name of the library that is making the HTTP request. Will be added to the user-agent header.
- **library_version** (`str`, *optional*) --
  The version of the library that is making the HTTP request. Will be added to the user-agent header.
- **user_agent** (`str`, `dict`, *optional*) --
  The user agent info in the form of a dictionary or a single string. It will be completed with information about the installed packages.

Sends telemetry that helps track usage of different HF libraries.

This usage data helps us debug issues and prioritize new features. However, we understand that not everyone wants
to share additional information, and we respect your privacy. You can disable telemetry collection by setting the
`HF_HUB_DISABLE_TELEMETRY=1` as environment variable. Telemetry is also disabled in offline mode (i.e. when setting
`HF_HUB_OFFLINE=1`).

Telemetry collection is run in a separate thread to minimize impact for the user.

Example:
```py
>>> from huggingface_hub.utils import send_telemetry

# Send telemetry without library information
>>> send_telemetry("ping")

# Send telemetry to subtopic with library information
>>> send_telemetry("gradio/local_link", library_name="gradio", library_version="3.22.1")

# Send telemetry with additional data
>>> send_telemetry(
...     topic="examples",
...     library_name="transformers",
...     library_version="4.26.0",
...     user_agent={"pipeline": "text_classification", "framework": "flax"},
... )
```

## Validators

`huggingface_hub` includes custom validators to validate method arguments automatically.
Validation is inspired by the work done in [Pydantic](https://pydantic-docs.helpmanual.io/)
to validate type hints but with more limited features.

### Generic decorator

[validate_hf_hub_args()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.utils.validate_hf_hub_args) is a generic decorator to encapsulate
methods that have arguments following `huggingface_hub`'s naming. By default, all
arguments that has a validator implemented will be validated.

If an input is not valid, a [HFValidationError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.HFValidationError) is thrown. Only
the first non-valid value throws an error and stops the validation process.

Usage:

```py
>>> from huggingface_hub.utils import validate_hf_hub_args

>>> @validate_hf_hub_args
... def my_cool_method(repo_id: str):
...     print(repo_id)

>>> my_cool_method(repo_id="valid_repo_id")
valid_repo_id

>>> my_cool_method("other..repo..id")
huggingface_hub.utils._validators.HFValidationError: Cannot have -- or .. in repo_id: 'other..repo..id'.

>>> my_cool_method(repo_id="other..repo..id")
huggingface_hub.utils._validators.HFValidationError: Cannot have -- or .. in repo_id: 'other..repo..id'.
```

#### validate_hf_hub_args[[huggingface_hub.utils.validate_hf_hub_args]]

- [HFValidationError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.HFValidationError) -- 
  If an input is not valid.[HFValidationError](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.errors.HFValidationError)
Validate values received as argument for any public method of `huggingface_hub`.

The goal of this decorator is to harmonize validation of arguments reused
everywhere. By default, all defined validators are tested.

Validators:
- [validate_repo_id()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.utils.validate_repo_id): `repo_id` must be `"repo_name"`
  or `"namespace/repo_name"`. Namespace is a username or an organization.
- `~utils.smoothly_deprecate_legacy_arguments`: Ignore `proxies` when downloading files (should be set globally).

Example:
```py
>>> from huggingface_hub.utils import validate_hf_hub_args

>>> @validate_hf_hub_args
... def my_cool_method(repo_id: str):
...     print(repo_id)

>>> my_cool_method(repo_id="valid_repo_id")
valid_repo_id

>>> my_cool_method("other..repo..id")
huggingface_hub.utils._validators.HFValidationError: Cannot have -- or .. in repo_id: 'other..repo..id'.

>>> my_cool_method(repo_id="other..repo..id")
huggingface_hub.utils._validators.HFValidationError: Cannot have -- or .. in repo_id: 'other..repo..id'.
```

#### HFValidationError[[huggingface_hub.errors.HFValidationError]]

Generic exception thrown by `huggingface_hub` validators.

Inherits from [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError).

### Argument validators

Validators can also be used individually. Here is a list of all arguments that can be
validated.

#### repo_id[[huggingface_hub.utils.validate_repo_id]]

Validate `repo_id` is valid.

This is not meant to replace the proper validation made on the Hub but rather to
avoid local inconsistencies whenever possible (example: passing `repo_type` in the
`repo_id` is forbidden).

Rules:
- Between 1 and 96 characters.
- Either "repo_name" or "namespace/repo_name"
- [a-zA-Z0-9] or "-", "_", "."
- "--" and ".." are forbidden

Valid: `"foo"`, `"foo/bar"`, `"123"`, `"Foo-BAR_foo.bar123"`

Not valid: `"datasets/foo/bar"`, `".repo_id"`, `"foo--bar"`, `"foo.git"`

Example:
```py
>>> from huggingface_hub.utils import validate_repo_id
>>> validate_repo_id(repo_id="valid_repo_id")
>>> validate_repo_id(repo_id="other..repo..id")
huggingface_hub.utils._validators.HFValidationError: Cannot have -- or .. in repo_id: 'other..repo..id'.
```

Discussed in https://github.com/huggingface/huggingface_hub/issues/1008.
In moon-landing (internal repository):
- https://github.com/huggingface/moon-landing/blob/main/server/lib/Names.ts#L27
- https://github.com/huggingface/moon-landing/blob/main/server/views/components/NewRepoForm/NewRepoForm.svelte#L138

#### smoothly_deprecate_legacy_arguments[[huggingface_hub.utils._validators.smoothly_deprecate_legacy_arguments]]

Not exactly a validator, but ran as well.

Smoothly deprecate legacy arguments in the `huggingface_hub` codebase.

This function ignores some deprecated arguments from the kwargs and warns the user they are ignored.
The goal is to avoid breaking existing code while guiding the user to the new way of doing things.

List of deprecated arguments:
- `proxies`:
  To set up proxies, user must either use the HTTP_PROXY environment variable or configure the `httpx.Client`
  manually using the [set_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_client_factory) function.

  In huggingface_hub 0.x, `proxies` was a dictionary directly passed to `requests.request`.
  In huggingface_hub 1.x, we migrated to `httpx` which does not support `proxies` the same way.
  In particular, it is not possible to configure proxies on a per-request basis. The solution is to configure
  it globally using the [set_client_factory()](/docs/huggingface_hub/v1.25.1/en/package_reference/utilities#huggingface_hub.set_client_factory) function or using the HTTP_PROXY environment variable.

  For more details, see:
  - https://www.python-httpx.org/advanced/proxies/
  - https://www.python-httpx.org/compatibility/#proxy-keys.

- `resume_download`: deprecated without replacement. `huggingface_hub` always resumes downloads whenever possible.
- `force_filename`: deprecated without replacement. Filename is always the same as on the Hub.
- `local_dir_use_symlinks`: deprecated without replacement. Downloading to a local directory does not use symlinks anymore.
