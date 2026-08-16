# Sandboxes

Check out the [Sandboxes guide](../guides/sandbox) to learn how to use them.

## Sandbox[[huggingface_hub.Sandbox]]

#### huggingface_hub.Sandbox[[huggingface_hub.Sandbox]]

```python
huggingface_hub.Sandbox(id: str, server: _SandboxServer, local_id: str | None, owns_sandbox: bool, owns_server: bool)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L471)

An isolated cloud machine running on Hugging Face Jobs.

Create a dedicated one with [Sandbox.create()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.create) (one job per sandbox), or get many cheap shared ones from a [SandboxPool](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxPool).

Reattach to a running sandbox from anywhere with [Sandbox.connect()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.connect). Use as a context manager to terminate it on exit:

```python
>>> from huggingface_hub import Sandbox
>>> with Sandbox.create(image="python:3.12") as sbx:
...     print(sbx.run("python --version").stdout)
```

#### close[[huggingface_hub.Sandbox.close]]

```python
close()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L680)

Release the local HTTP client without terminating the sandbox. Idempotent.

No-op for pool sandboxes (the client belongs to the pool's host).

#### connect[[huggingface_hub.Sandbox.connect]]

```python
connect(sandbox_id: str, namespace: str | None = None, token: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L614)

Reattach to a running sandbox from anywhere, using only its id.

#### create[[huggingface_hub.Sandbox.create]]

```python
create(image: str = 'python:3.12', flavor: str = 'cpu-basic', idle_timeout: int | float | str | None = 600, env: dict[str, typing.Any] | None = None, secrets: dict[str, typing.Any] | None = None, volumes: typing.Optional[typing.List[huggingface_hub._space_api.Volume]] = None, namespace: str | None = None, forward_hf_token: bool = False, start_timeout: float = 120.0, token: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L513)

**Parameters:**

image (`str`, *optional*, defaults to `"python --3.12"`): Any Docker image with `/bin/sh` (Docker Hub or `hf.co/spaces/...`).

flavor (`str`, *optional*, defaults to `"cpu-basic"`) : Hardware flavor, e.g. `"cpu-basic"`, `"a10g-small"`. See `hf jobs hardware`.

idle_timeout (`int` or `float` or `str`, *optional*, defaults to `600`) : Auto-shutdown after this much inactivity (no API calls, no running processes). Defaults to 10 minutes; pass `None` to disable.

env (`dict[str, Any]`, *optional*) : Environment variables available in the sandbox.

secrets (`dict[str, Any]`, *optional*) : Secret environment variables (encrypted server-side).

volumes (`List[Volume]`, *optional*) : HF repos/buckets to mount, see [Volume](/docs/huggingface_hub/v1.27.0/en/package_reference/jobs#huggingface_hub.Volume).

namespace (`str`, *optional*) : User or org namespace to run under (defaults to current user).

forward_hf_token (`bool`, *optional*, defaults to `False`) : If True, your HF token is injected as `HF_TOKEN` (opt-in).

start_timeout (`float`, *optional*, defaults to `120.0`) : Max seconds to wait for the sandbox to become ready.

token (`str`, *optional*) : HF token override.

Create a dedicated sandbox (one HF Job) and block until it is ready (~7s on cpu-basic).

Each sandbox is a full isolated VM, so this is the right choice for GPU
workloads or untrusted code. To fan out many cheap CPU sandboxes instead, use
[SandboxPool](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxPool).

The job runs with a fixed 24h maximum lifetime; `idle_timeout` is the real
keeper — an idle sandbox shuts itself down well before that.

The image only needs `/bin/sh`. The sandbox server is downloaded at startup with
`wget`/`curl` if available, otherwise read off an always-mounted server bucket (which
adds ~2-3s to cold start, so shipping `wget`/`curl` keeps it fast).

#### processes[[huggingface_hub.Sandbox.processes]]

```python
processes()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L819)

List the background processes of this sandbox.

Returns the processes started with [Sandbox.run()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.run)`(..., background=True)`; stop one
with [SandboxProcess.kill()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess.kill). Completed processes stay listed (with `running=False` and
their `exit_code`) until the sandbox is deleted.

#### proxy_url_for[[huggingface_hub.Sandbox.proxy_url_for]]

```python
proxy_url_for(port: int | str, path: str = '/', scheme: str = 'https://')
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L853)

**Parameters:**

port (`int` or `str`) : The port (pool: the `<port>` of the unix socket) the inner server listens on.

path (`str`, *optional*, defaults to `"/"`) : Path on the inner server to point at, e.g. `"/ws"`.

scheme (`str`, *optional*, defaults to `"https --//"`): URL scheme to build the link with. Defaults to `"https://"`; pass `"wss://"` for a WebSocket client (the proxy is protocol-agnostic, so only the client-side scheme changes).

**Returns:** `str`

a URL like `https://<job_id>--49983.hf.jobs/v1/.../proxy/8000/ws` (or
`wss://...` with `scheme="wss://"`).

Public URL that proxies through to a server running *inside* this sandbox.

Requests to the returned URL are forwarded by the in-job sandbox server to a
server you started in the sandbox on `port`, including WebSocket (`ws(s)://`)
upgrades and streamed responses. Pair it with `proxy_headers` for auth.

How the sandbox must listen on `port`:

- **Pool / shared sandbox**: it cannot bind a TCP port (Landlock), so bind a
  **unix socket** at `$SBX_PROXY_DIR/<port>.sock` (the `SBX_PROXY_DIR` env var
  is set in every sandbox). E.g. `uvicorn app:app --uds $SBX_PROXY_DIR/8000.sock`.
- **Dedicated sandbox**: bind a normal TCP port on `127.0.0.1:<port>`. (You can
  also expose the port directly via the job proxy without going through here.)

Example:
```python
>>> url = sandbox.proxy_url_for(8000, "/ws", scheme="wss://")
>>> import websockets
>>> async with websockets.connect(url, additional_headers=sandbox.proxy_headers) as ws:
...     await ws.send("hello")
```

#### run[[huggingface_hub.Sandbox.run]]

```python
run(cmd: typing.Union[str, typing.List[str]], shell: bool | None = None, env: dict[str, typing.Any] | None = None, cwd: str | None = None, timeout: float | None = None, stdin: str | None = None, on_stdout: typing.Optional[typing.Callable[[str], NoneType]] = None, on_stderr: typing.Optional[typing.Callable[[str], NoneType]] = None, check: bool = True, background: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L726)

**Parameters:**

cmd (`str` or `List[str]`) : A shell command string (run with `/bin/sh -c`) or an argv list (exec'd directly).

shell (`bool`, *optional*) : Force the execution mode instead of inferring it from the type of `cmd`. `True` runs through `/bin/sh -c` and requires `cmd` to be a string; `False` exec's `cmd` directly and requires it to be an argv list. `None` (default) infers from the type. Set it explicitly to avoid the type-driven footgun (e.g. `["echo hi"]` being exec'd as a single program named `"echo hi"`).

env (`dict[str, Any]`, *optional*) : Extra environment variables for this command.

cwd (`str`, *optional*) : Working directory.

timeout (`float`, *optional*) : Kill the command (whole process group) after this many seconds.

stdin (`str`, *optional*) : Data to write to the command's stdin.

on_stdout (`Callable[[str], None]`, *optional*) : Callback invoked with stdout chunks as they arrive.

on_stderr (`Callable[[str], None]`, *optional*) : Callback invoked with stderr chunks as they arrive.

check (`bool`, *optional*, defaults to `True`) : If True, raise `SandboxCommandError` on non-zero exit.

background (`bool`, *optional*, defaults to `False`) : If True, start the command detached and return a [SandboxProcess](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess) right away instead of waiting for it and returning a [SandboxCommandResult](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxCommandResult).

Run a command in the sandbox and wait for it, streaming output live.

With `background=True` the command is started detached and `run` returns a
[SandboxProcess](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess) immediately, without waiting for it to finish — handy for
servers and other long-running processes. List them later with [Sandbox.processes()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.processes)
and stop one with [SandboxProcess.kill()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess.kill). The streaming/wait-only options
(`timeout`, `stdin`, `on_stdout`, `on_stderr`, `check`) don't apply in that mode.

Returns: a [SandboxCommandResult](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxCommandResult) (with `exit_code`, `stdout`, `stderr`,
`duration_ms`), or a [SandboxProcess](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess) when `background=True`.

## SandboxPool[[huggingface_hub.SandboxPool]]

#### huggingface_hub.SandboxPool[[huggingface_hub.SandboxPool]]

```python
huggingface_hub.SandboxPool(image: str = 'python:3.12', flavor: str = 'cpu-basic', sandboxes_per_host: int = 50, warm_up: int = 1, max_hosts: int | None = None, name: str | None = None, idle_timeout: int | float | str | None = 600, namespace: str | None = None, start_timeout: float = 120.0, token: str | None = None, _connect_mode: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L917)

A fleet of shared "host" jobs, each packing many landlock-isolated sandboxes.

One host is one billed HF Job (a VM); it runs the sandbox server and multiplexes
up to `sandboxes_per_host` lightweight sandboxes, isolated from each other by
uid + the Landlock LSM. This makes large fan-outs cheap (the VM cost is shared
across all its sandboxes) and fast (creating a sandbox is ~one proxy round-trip
once a host is warm). Best for many parallel CPU sandboxes such as RL rollouts;
for GPU or strong VM-level isolation between mutually-distrusting workloads, use
[Sandbox.create()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.create) instead.

The constructor pre-provisions `warm_up` hosts (default 1) and blocks until they are
ready; further hosts are then provisioned on demand as sandboxes are requested, and all

are torn down on `close()` (or when idle, via `idle_timeout`). The user never manages jobs:

```python
>>> from huggingface_hub import SandboxPool
>>> with SandboxPool(image="python:3.12", flavor="cpu-basic", warm_up=2) as pool:
...     boxes = [pool.create() for _ in range(100)]   # packed across the warm hosts
...     print(boxes[0].run("echo hi").stdout)
hi
```

`create()` makes **one** sandbox at a time: it reuses a host that still has free
capacity before booting a new one, so you grow on demand as work arrives. To avoid
a cold start on the first few calls, pre-provision hosts with `warm_up` (or
`warm`). Warm hosts are discovered via job labels, so reuse works **across
processes** too (a fresh pool with the same `image`/`flavor`/`name` attaches to

hosts an earlier run left behind):

```python
>>> pool = SandboxPool(image="python:3.12")
>>> sbx = pool.create()    # finds a warm host (here or in another process), else boots one
```

#### close[[huggingface_hub.SandboxPool.close]]

```python
close()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L1256)

Release the pool. Idempotent.

For a pool we created, this terminates all host jobs (and therefore all their
sandboxes). For a `connect()`'d handle it only releases the local HTTP clients: the
shared hosts may be serving other clients, so — like [Sandbox.connect()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.connect) — leaving a
`with` block must not tear them down. Terminate a connected pool's hosts explicitly
with `hf sandbox pool delete <id>`.

#### connect[[huggingface_hub.SandboxPool.connect]]

```python
connect(pool_id: str, namespace: str | None = None, token: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L1053)

**Parameters:**

pool_id (`str`) : The id returned when the pool was first created.

namespace (`str`, *optional*) : Namespace to search for the pool's hosts (defaults to yours).

token (`str`, *optional*) : HF token override.

Reattach to a running pool by id, from any machine — no local state needed.

Finds a running host labelled with `pool_id` and rebuilds the pool's config
(image/flavor/density/host-idle) from that host job's spec and env vars, returning
a [SandboxPool](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxPool) ready to `create()` more sandboxes — packing onto the running
hosts, or booting a duplicate (same config) when they are full.

Raises `SandboxError` if no running host is found (a pool stops existing once
all of its hosts are gone — idle-timed-out or killed).

#### create[[huggingface_hub.SandboxPool.create]]

```python
create(env: dict[str, typing.Any] | None = None, idle_timeout: int | float | str | None = 600, forward_hf_token: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L1135)

**Parameters:**

env (`dict[str, Any]`, *optional*) : Environment variables for this sandbox (each sandbox gets its own).

idle_timeout (`int` or `float` or `str`, *optional*, defaults to `600`) : Per-sandbox idle timeout — a sandbox is evicted from its host after this much inactivity (no API calls, no running process). Distinct from the host idle timeout. Pass `None` to disable.

forward_hf_token (`bool`, *optional*, defaults to `False`) : If True, inject your HF token as `HF_TOKEN` in the sandbox (opt-in). Unlike a dedicated sandbox's `secrets`, a pooled sandbox's env is delivered to the host server at creation (never stored in the host job), so it doesn't appear in any job's metadata.

Create one sandbox, provisioning a host if needed.

Reuses a host with free capacity (this pool's, or a warm host found via job labels
/ the local cache) before booting a new one, so a `create()` against a warm host
costs ~one round-trip. Call it repeatedly to fan out; use `warm_up` (or `warm`)
to pre-provision hosts and avoid a cold start on the first calls. If a host fills
up under us (another process packed it) or a cached host is gone, the sandbox is
re-placed on another host (or a fresh one).

#### warm[[huggingface_hub.SandboxPool.warm]]

```python
warm(num_hosts: int = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L1108)

Ensure `num_hosts` empty host(s) are running and leave them running. Returns the
pool's host job ids.

Used to "create" a pool up front: the hosts carry the pool label and config (in
their env vars), so a later `SandboxPool.connect(pool_id)` (even from another
machine) finds them and spawns sandboxes without a cold start. The hosts keep
billing until killed or idle.

Adopts hosts already running for this pool (found via job labels) before booting,
so a `warm()` after `connect()` — or a repeated `warm()` — tops up to `num_hosts`
instead of duplicating live hosts and blowing past `max_hosts`.

## Data structures

### SandboxCommandResult[[huggingface_hub.SandboxCommandResult]]

#### huggingface_hub.SandboxCommandResult[[huggingface_hub.SandboxCommandResult]]

```python
huggingface_hub.SandboxCommandResult(exit_code: int | None, stdout: str, stderr: str, signal: int | None = None, timed_out: bool = False, duration_ms: int = 0)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L112)

Result of a command executed in a sandbox with [Sandbox.run()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.run).

### SandboxProcess[[huggingface_hub.SandboxProcess]]

#### huggingface_hub.SandboxProcess[[huggingface_hub.SandboxProcess]]

```python
huggingface_hub.SandboxProcess(pid: int, cmd: typing.Union[str, typing.List[str]], _sandbox: Sandbox, tag: str | None = None, started_at_ms: int | None = None, running: bool = True, exit_code: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L132)

A background process started in a sandbox with [Sandbox.run()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.run)`(..., background=True)`.

List a sandbox's processes with [Sandbox.processes()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.Sandbox.processes) and stop one with [SandboxProcess.kill()](/docs/huggingface_hub/v1.27.0/en/package_reference/sandbox#huggingface_hub.SandboxProcess.kill).
Completed processes stay in the listing until the sandbox is deleted, so `running` and
`exit_code` tell whether a process is still alive or already exited (as of when it was listed).

#### kill[[huggingface_hub.SandboxProcess.kill]]

```python
kill()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L150)

Terminate the background process (idempotent server-side).

### FileEntry[[huggingface_hub._sandbox.FileEntry]]

#### huggingface_hub._sandbox.FileEntry[[huggingface_hub._sandbox.FileEntry]]

```python
huggingface_hub._sandbox.FileEntry(name: str, path: str, type: typing.Literal['file', 'dir', 'symlink'], size: int, mtime_ms: int | None = None, mode: str = '')
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_sandbox.py#L156)

A file or directory inside a sandbox.

## Errors

### SandboxError[[huggingface_hub.errors.SandboxError]]

#### huggingface_hub.errors.SandboxError[[huggingface_hub.errors.SandboxError]]

```python
huggingface_hub.errors.SandboxError(message: str, status_code: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L586)

**Parameters:**

status_code : The HTTP status returned by the in-sandbox server, if the error originated from an API response (e.g. `404` for a missing file). `None` otherwise.

Base exception for sandbox operations (see `huggingface_hub.Sandbox`).

### SandboxCommandError[[huggingface_hub.errors.SandboxCommandError]]

#### huggingface_hub.errors.SandboxCommandError[[huggingface_hub.errors.SandboxCommandError]]

```python
huggingface_hub.errors.SandboxCommandError(cmd, result)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L599)

**Parameters:**

cmd : The command that failed.

result : The full `SandboxCommandResult` (exit_code, stdout, stderr, ...).

Raised when a command run in a sandbox exits with a non-zero code.
