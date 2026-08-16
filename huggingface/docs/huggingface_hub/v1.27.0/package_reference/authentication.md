# Authentication

The `huggingface_hub` library allows users to programmatically manage authentication to the Hub. This includes logging in, logging out, switching between tokens, and listing available tokens.

For more details about authentication, check out [this section](../quick-start#authentication).

## login[[huggingface_hub.login]]

#### huggingface_hub.login[[huggingface_hub.login]]

```python
huggingface_hub.login(token: str | None = None, add_to_git_credential: bool = False, skip_if_logged_in: bool = True)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L57)

**Parameters:**

token (`str`, *optional*) : User access token to generate from https://huggingface.co/settings/token.

add_to_git_credential (`bool`, defaults to `False`) : If `True`, token will be set as git credential. If no git credential helper is configured, a warning will be displayed to the user. Only used when `token` is provided; ignored by the browser-based flow.

skip_if_logged_in (`bool`, defaults to `True`) : If `True`, do not prompt for token if user is already logged in. Set to `False` to force re-login. In CLI, use `--force` instead.

**Raises:** ``ValueError`` or `DeviceCodeError`

- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If an organization token is passed. Only personal account tokens are valid
  to log in.
- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If token is invalid.
- `DeviceCodeError` -- 
  If the browser-based login fails (authorization denied, code expired, ...).

Login the machine to access the Hub.

The `token` is persisted in cache and set as a git credential. Once done, the machine
is logged in and the access token will be available across all `huggingface_hub`
components. If `token` is not provided, a browser-based OAuth flow is used to
authenticate: open a URL, enter a short code, and the token is retrieved and saved.
In a terminal, you can also choose to paste an existing access token instead.

To log in from outside of a script, one can also use `hf auth login` which is
a cli command that wraps [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login).

> [!TIP]
> When the token is not passed, [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login) will automatically detect if the script runs
> in a notebook or not. However, this detection might not be accurate due to the
> variety of notebooks that exists nowadays. If that is the case, you can always force
> the UI by using [notebook_login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.notebook_login) or [interpreter_login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.interpreter_login).

## interpreter_login[[huggingface_hub.interpreter_login]]

#### huggingface_hub.interpreter_login[[huggingface_hub.interpreter_login]]

```python
huggingface_hub.interpreter_login(skip_if_logged_in: bool = True)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L284)

**Parameters:**

skip_if_logged_in (`bool`, defaults to `True`) : If `True`, do not prompt for token if user is already logged in. Set to `False` to force re-login. In CLI, use `--force` instead.

Displays a prompt to log in to the HF website and store the token.

This is equivalent to [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login) without passing a token when not run in a notebook.
[interpreter_login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.interpreter_login) is useful if you want to force the use of the terminal prompt
instead of a notebook flow.

For more details, see [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login).

## notebook_login[[huggingface_hub.notebook_login]]

#### huggingface_hub.notebook_login[[huggingface_hub.notebook_login]]

```python
huggingface_hub.notebook_login(skip_if_logged_in: bool = True)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L335)

**Parameters:**

skip_if_logged_in (`bool`, defaults to `True`) : If `True`, do not prompt for token if user is already logged in. Set to `False` to force re-login. In CLI, use `--force` instead.

Displays a prompt to log in to the HF website and store the token.

This is equivalent to [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login) without passing a token when run in a notebook.
[notebook_login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.notebook_login) is useful if you want to force the use of the notebook flow
instead of a prompt in the terminal.

For more details, see [login()](/docs/huggingface_hub/v1.27.0/en/package_reference/authentication#huggingface_hub.login).

## logout[[huggingface_hub.logout]]

#### huggingface_hub.logout[[huggingface_hub.logout]]

```python
huggingface_hub.logout(token_name: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L120)

**Parameters:**

token_name (`str`, *optional*) : Name of the access token to logout from. If `None`, will log out from all saved access tokens.

**Raises:** ``ValueError``

- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If the access token name is not found.

Logout the machine from the Hub.

Token is deleted from the machine and removed from git credential.

## auth_switch[[huggingface_hub.auth_switch]]

#### huggingface_hub.auth_switch[[huggingface_hub.auth_switch]]

```python
huggingface_hub.auth_switch(token_name: str, add_to_git_credential: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L162)

**Parameters:**

token_name (`str`) : Name of the access token to switch to.

add_to_git_credential (`bool`, defaults to `False`) : If `True`, token will be set as git credential. If no git credential helper is configured, a warning will be displayed to the user. If `token` is `None`, the value of `add_to_git_credential` is ignored and will be prompted again to the end user.

**Raises:** ``ValueError``

- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If the access token name is not found.

Switch to a different access token.

## auth_list[[huggingface_hub.auth_list]]

#### huggingface_hub.auth_list[[huggingface_hub.auth_list]]

```python
huggingface_hub.auth_list()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_login.py#L191)

List all stored access tokens.
