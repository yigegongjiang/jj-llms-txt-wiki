# Core API

The `openenv.core` package provides the core abstractions for building and running environments. For an end-to-end tutorial on building environments with OpenEnv, see the [building an environment](../getting_started/environment-builder) guide.

If you are trying to understand when OpenEnv exposes the training loop versus direct MCP access, see the [simulation vs production mode](../guides/simulation-vs-production) guide.

For a high-level explanation of how MCP-backed environments move through `step()`, `step_async()`, and convenience tool helpers, see the [MCP environment lifecycle](../guides/mcp-environment-lifecycle) guide.

## Server

### Environment server primitives[[openenv.core.Message]]

A message in a conversation.

Compatible with Huggingface chat template format.

Protocol for tokenizers that support chat templates.

This protocol defines the interface that tokenizers must implement
to work with chat-based environments. It's compatible with
Huggingface transformers tokenizers.

- **conversation** (`list[Message]`) --
  List of message dictionaries with 'role' and 'content'.
- **tokenize** (`bool`, *optional*, defaults to `True`) --
  Whether to tokenize the output.
- **return_tensors** (`str`, *optional*) --
  Format for returned tensors ('pt' for PyTorch).
- ****kwargs** --
  Additional arguments.Formatted and optionally tokenized conversation.
Apply a chat template to format and optionally tokenize a conversation.

- **token_ids** (`Any`) --
  Token IDs to decode.
- **skip_special_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether to skip special tokens in output.
- ****kwargs** --
  Additional arguments.`str`Decoded text string.
Decode token IDs back to text.

Transform observations to add rewards, metrics, or other modifications.

Transforms follow the TorchRL pattern where they take an observation
and return a (potentially modified) observation. This allows for
flexible reward computation and observation augmentation.

- **transform** (*Transform*, *optional*) --
Optional transform to apply to observations.
- **rubric** (*Rubric*, *optional*) --
  Optional rubric for reward computation. When provided, the
  rubric's output can be used to set the observation's reward in step().
- **SUPPORTS_CONCURRENT_SESSIONS** (*bool*) --
  Whether this environment supports concurrent sessions. When `True`,
  multiple WebSocket connections can each have their own environment
  instance (up to `max_concurrent_envs`). When `False` (default),
  the environment should only be used with a single session at a time.

  Set this to `True` in your subclass if the environment uses proper
  session isolation (unique working dirs, no shared mutable state, and
  external resources that can handle concurrent access).
- **rubric** (*Rubric*, *optional*) --
  Optional rubric for computing rewards. Set in `__init__` and use in
  `step()` to compute observation rewards. Training infrastructure can
  access it for introspection:

```python
for name, r in env.rubric.named_rubrics():
    print(f"&amp;lcub;name}: &amp;lcub;r.last_score}")
```
Base class for all environment servers following Gym/Gymnasium API.

See [rfcs/004-rubrics.md](https://github.com/huggingface/OpenEnv/blob/main/rfcs/004-rubrics.md) for rubric design details.

Clean up resources used by the environment.

Override this method to implement custom cleanup logic.
Called when the environment is being destroyed or reset.

`EnvironmentMetadata` with environment information.

Get metadata about this environment.

Override this method to provide custom metadata for the environment.
Default implementation returns basic metadata derived from class name.

Reset the environment and return initial observation.

Async version of reset. Default implementation calls sync reset.

Override to provide true async implementation.

Take a step in the environment.

Async version of step. Default implementation calls sync step.

Override to provide true async implementation.

### Types[[openenv.core.ServerMode]]

Server operation mode.

Server health status values.

WebSocket error codes for structured error handling.

"}]}>

Base class for all environment actions.

All action subclasses should inherit from this base class.
Uses Pydantic for automatic validation and serialization.

"}]}>

Base class for all environment observations.

All observation subclasses should inherit from this base class.
Uses Pydantic for automatic validation and serialization.

Request model for environment reset.

Response model for environment reset.

Request model for environment step.

Response model for environment step.

Base class for WebSocket messages with shared configuration.

Base class for environment state.

Represents internal environment state, separate from observations.

Result of code execution containing stdout, stderr, and exit code.

Metadata about an environment for documentation and UI purposes.

Response model for the combined schema endpoint.

"}]}>

Response model for health check endpoint.

"}]}>

WebSocket message to reset the environment.

WebSocket message to execute a step.

WebSocket message to request current state.

WebSocket message to close the session.

WebSocket response containing an observation.

WebSocket response containing environment state.

WebSocket response for errors.

Configuration for concurrent environment sessions.

Status of server capacity for concurrent sessions.

Create status from active and max session counts.

Information about an active session.

### Exceptions[[openenv.core.OpenEnvError]]

Base exception for all OpenEnv errors.

Raised when an environment is misconfigured for concurrent sessions.

This error is raised during server startup when max_concurrent_envs > 1
is specified for an environment that is not marked as SUPPORTS_CONCURRENT_SESSIONS.

Raised when the server cannot accept new sessions due to capacity limits.

This error is raised when a new WebSocket connection is attempted but
the server has already reached max_concurrent_envs active sessions.

Raised when attempting to access a session that does not exist.

Raised when a session cannot be created.

Raised when the environment factory fails to create an instance.

### HTTP server utilities[[openenv.core.HTTPEnvServer]]

HTTP server wrapper for Environment instances.

This class wraps an Environment and exposes its reset(), step(), and state
methods as HTTP and WebSocket endpoints compatible with EnvClient.

The server expects:
- Action deserialization: Converts JSON dict to Action subclass
- Observation serialization: Converts Observation subclass to JSON dict

Examples:

```python
from core.env_server import HTTPEnvServer
from envs.coding_env.server import CodeExecutionEnvironment
from envs.coding_env.models import CodeAction, CodeObservation

# Pass environment class (factory pattern)
server = HTTPEnvServer(
    env=CodeExecutionEnvironment,
    action_cls=CodeAction,
    observation_cls=CodeObservation,
    max_concurrent_envs=4,
)

# Register routes with FastAPI
from fastapi import FastAPI
app = FastAPI()
server.register_routes(app)
```

`ServerCapacityStatus` with current session counts and availability.

Get the current capacity status of the server.

- **session_id** (`str`) --
  The session ID to query.`SessionInfo` if the session exists, `None` otherwise.

Get information about a specific session.

"}]}>
- **app** (`FastAPI`) --
  FastAPI application instance.
- **mode** (`ServerMode` or `str`, *optional*, defaults to `ServerMode.SIMULATION`) --
  Server mode. In production mode, simulation control endpoints (/reset, /step,
  /state) are NOT registered. Only safe endpoints (/health, /schema, /metadata,
  /ws) are available.- ``ValueError`` -- If `mode` is not a valid `ServerMode` or string equivalent.</raises><raisederrors>``ValueError``

Register HTTP routes on a FastAPI application.

- **env** (`Callable[[], Environment]`) --
  Environment factory (callable) that creates new instances.
- **action_cls** (`Type[Action]`) --
  The Action subclass this environment expects.
- **observation_cls** (`Type[Observation]`) --
  The Observation subclass this environment returns.
- **env_name** (`str`, *optional*) --
  Environment name for README loading.
- **max_concurrent_envs** (`int`, *optional*) --
  Maximum concurrent WebSocket sessions. Mutually exclusive with
  `concurrency_config`.
- **concurrency_config** (`ConcurrencyConfig`, *optional*) --
  Advanced concurrency settings. Mutually exclusive with
  `max_concurrent_envs`.
- **gradio_builder** (`Callable`, *optional*) --
  Callable to build a custom Gradio UI at /web. Signature:
  `(web_manager, action_fields, metadata, is_chat_env, title, quick_start_md) -> gr.Blocks`.
  When `None`, the default Gradio app is used.
- **custom_tab_name** (`str`, *optional*, defaults to `"Custom"`) --
  Label for the env-specific tab when `gradio_builder` is provided.
- **custom_tab_primary** (`bool`, *optional*, defaults to `False`) --
  When `True`, the env-specific tab is active first; the auto-generated
  Playground becomes secondary.
- **show_default_tab** (`bool`, *optional*, defaults to `True`) --
  When `False`, mount the env's `gradio_builder` output alone (no
  auto-generated Playground, no tab chrome). Only meaningful when
  `gradio_builder` is provided.
- **title_override** (`str`, *optional*) --
  If set, used as the Gradio app title instead of the default
  `"OpenEnv Agentic Environment: {name}"`.`FastAPI` application instance with or without web interface and README integration.

Create a FastAPI application with or without web interface.

This function creates a FastAPI app with the web interface enabled by default,
including README integration for better user experience.

- **env** (`Callable[[], Environment]`) --
  Environment factory (callable) that creates new instances.
- **action_cls** (`Type[Action]`) --
  The Action subclass this environment expects.
- **observation_cls** (`Type[Observation]`) --
  The Observation subclass this environment returns.
- **max_concurrent_envs** (`int`, *optional*) --
  Maximum concurrent WebSocket sessions. Mutually exclusive with
  `concurrency_config`.
- **concurrency_config** (`ConcurrencyConfig`, *optional*) --
  Advanced concurrency settings. Mutually exclusive with
  `max_concurrent_envs`.
- **env_name** (`str`, *optional*) --
  Optional environment name for task/split endpoints.`FastAPI` application instance.

Create a FastAPI application with comprehensive documentation.

### Web interface helpers[[openenv.core.env_server.web_interface.ActionLog]]

Log entry for an action taken.

"}, {"name": "is_reset", "val": ": bool = True"}]}>

Current episode state for the web interface.

Manages the web interface for an environment.

Connect a new WebSocket client.

Disconnect a WebSocket client.

Get current environment state.

Reset the environment and update state.

Execute a step in the environment and update state.

- **env** -- The Environment instance to serve
- **action_cls** -- The Action subclass this environment expects
- **observation_cls** -- The Observation subclass this environment returns
- **env_name** -- Optional environment name for README loading
- **max_concurrent_envs** -- Maximum concurrent WebSocket sessions
- **concurrency_config** -- Optional ConcurrencyConfig for advanced concurrency settings
- **gradio_builder** -- Optional callable (web_manager, action_fields, metadata,
  is_chat_env, title, quick_start_md) -> gr.Blocks to use instead of the
  default Gradio UI. Lets envs replace or customize the /web interface.
- **custom_tab_name** -- Label shown on the env-specific tab when `gradio_builder`
  is provided. Defaults to `"Custom"` for backwards compatibility; envs
  that ship a rich custom UI should pass a descriptive name
  (e.g. `"REPL"`). Ignored when `show_default_tab=False` (no tab chrome
  is rendered).
- **custom_tab_primary** -- When True, the env-specific tab is rendered first and
  selected by default; the auto-generated Playground becomes secondary.
  Use this for envs whose custom tab is the real interaction surface
  (so visitors don't land on a less informative schema form). Ignored
  when `show_default_tab=False`.
- **show_default_tab** -- When False, the auto-generated Playground tab is not
  rendered and the env's `gradio_builder` output is mounted directly
  (single-view UI, no tab chrome). Only meaningful when
  `gradio_builder` is provided.
- **title_override** -- If set, used verbatim as the Gradio app/browser-tab
  title instead of the default `"OpenEnv Agentic Environment: &amp;lcub;name}"`.FastAPI application instance with web interface

Create a FastAPI application with web interface for the given environment.

### Serialization[[openenv.core.deserialize_action]]

- **action_data** (*dict*) --
  Dictionary containing action data.
- **action_cls** (*type*) --
  The Action subclass to instantiate.*Action* instance.- `ValidationError` -- If *action_data* is invalid for the action class.`ValidationError`

Convert JSON dict to Action instance using Pydantic validation.

MCP action types (`list_tools`, `call_tool`) are recognised
automatically via the `"type"` discriminator field, regardless of
the environment's configured `action_cls`.  All other payloads
fall through to `action_cls.model_validate()`.

For special cases (e.g., tensor fields, custom type conversions),
use deserialize_action_with_preprocessing().

- **action_data** (`dict`) --
  Dictionary containing action data.
- **action_cls** (`type`) --
  The Action subclass to instantiate.`Action` instance.- ``ValidationError`` -- If `action_data` is invalid for the action class.</raises><raisederrors>``ValidationError``

Convert JSON dict to Action instance with preprocessing for special types.

This version handles common type conversions needed for web interfaces:
- Converting lists/strings to tensors for 'tokens' field
- Converting string action_id to int
- Other custom preprocessing as needed

- **observation** (`Observation`) --
  Observation instance to serialize.`dict` compatible with `EnvClient._parse_result()`, with keys- `observation` (`dict`): Observation fields.
- `reward` (`float` or `None`): Reward value.
- `done` (`bool`): Whether the episode is done.
- `metadata` (`dict`, *optional*): Additional observation metadata.

Convert Observation instance to JSON-compatible dict using Pydantic.

### Transforms[[openenv.core.CompositeTransform]]

Combines multiple transforms into a single transform.

Default transform that passes through unchanged.

### Route configuration[[openenv.core.GetEndpointConfig]]

Configuration for a simple GET endpoint.

- **app** (`~fastapi.FastAPI`) --
  FastAPI application instance.
- **configs** (`List[GetEndpointConfig]`) --
  List of GET endpoint configurations.

Register multiple GET endpoints from configuration.

## Clients

### Base client[[openenv.core.EnvClient]]

Async environment client for persistent sessions.

This client maintains a persistent WebSocket connection to an environment
server, enabling efficient multi-step interactions. Each client instance
corresponds to a dedicated environment session on the server.

The client is async by default. For synchronous usage, use the `.sync()`
method to get a `SyncEnvClient` wrapper.

Features:
- Lower latency for sequential interactions
- Session state is maintained server-side
- Better suited for long-running episodes
- Async by default for modern Python async/await patterns

Examples:

Async usage:

```python
from envs.coding_env.client import CodingEnv

async with CodingEnv(base_url="ws://localhost:8000") as env:
    result = await env.reset(seed=42)
    while not result.done:
        action = agent.predict(result.observation)
        result = await env.step(action)
```

Sync usage via `.sync()` wrapper:

```python
env = CodingEnv(base_url="ws://localhost:8000").sync()
with env:
    result = env.reset(seed=42)
    result = env.step(action)
```

- **image** (`str`) --
  Docker image name to run (e.g., `"coding-env:latest"`).
- **provider** (`ContainerProvider`, *optional*) --
  Container provider to use. Defaults to `LocalDockerProvider`.
- ****kwargs** --
  Additional arguments to pass to `provider.start_container()`.`_BootstrapResult``await` for a connected async client, or call
`.sync()` for a connected `SyncEnvClient`.

Create an environment client by spinning up a Docker container.

Returns a bootstrap handle so the same call works from both async and
synchronous code: `await` it for the connected async client, or chain
`.sync()` for a connected `SyncEnvClient` (e.g. a TRL GRPO rollout loop
that cannot `await`). Bootstrap is lazy — the container starts when the
handle is resolved.

Examples:

```python
# Async
env = await MyEnv.from_docker_image("coding-env:latest")

# Sync
env = MyEnv.from_docker_image("coding-env:latest").sync()
result = env.reset()
```

- **repo_id** (`str`) --
  Hugging Face space identifier `{org}/{space}`.
- **use_docker** (`bool`, *optional*, defaults to `True`) --
  When `True`, pull from the HF registry and launch via `LocalDockerProvider`.
  When `False`, run the space locally with `UVProvider`.
- **provider** (`ContainerProvider` or `RuntimeProvider`, *optional*) --
  Provider instance to reuse. Must be a `ContainerProvider` when
  `use_docker=True` and a `RuntimeProvider` otherwise.
- ****provider_kwargs** --
  Additional keyword arguments forwarded to either the container provider's
  `start_container` (docker) or to the `UVProvider` constructor/start (uv).
  When `use_docker=False`, the `project_path` argument can be used to override
  the default git URL (`git+https://huggingface.co/spaces/{repo_id}`).`_BootstrapResult``await` for a connected async client, or call
`.sync()` for a connected `SyncEnvClient`.

Create a client from a Hugging Face Space.

Returns a bootstrap handle: `await` it for the connected async client, or
chain `.sync()` for a connected `SyncEnvClient`. Bootstrap is lazy — the
Space starts when the handle is resolved.

Examples:

```python
# Async: pull and run from HF Docker registry
env = await MyEnv.from_env("openenv/echo-env")

# Sync: chain .sync()
env = MyEnv.from_env("openenv/echo-env").sync()

# Run locally with UV (clones the space)
env = await MyEnv.from_env("openenv/echo-env", use_docker=False)

# Run from a local checkout
env = await MyEnv.from_env(
    "openenv/echo-env",
    use_docker=False,
    project_path="/path/to/local/checkout"
)
```

`EnvClient`A connected child client of the same concrete type.

Create and connect a new session against the same environment server.

The child session is tracked by this parent and closed when the parent
is closed. Server-side capacity still applies: when the server is at
`MAX_CONCURRENT_ENVS`, opening the child WebSocket can fail and is
surfaced as a connection error.

SyncEnvClient wrapper that provides synchronous methods

Return a synchronous wrapper around this async client.

Use this method when you need synchronous access to the environment
without async/await syntax. This is useful for:
- Integration with synchronous codebases
- Interactive/REPL usage
- Stopping async from "infecting" the call stack

Examples:

```python
async_client = GenericEnvClient(base_url="http://localhost:8000")
sync_client = async_client.sync()

with sync_client:
    result = sync_client.reset()
    result = sync_client.step({"code": "print('hello')"})
```

### Synchronous client[[openenv.SyncEnvClient]]

- **_async** -- The wrapped async EnvClient instance

Synchronous wrapper around an async EnvClient.

This class provides a synchronous interface to an async EnvClient,
making it easier to use in synchronous code or to stop async from
"infecting" the entire call stack.

The wrapper executes async operations on a dedicated background event loop
so connection state remains bound to a single loop.

For guaranteed resource cleanup, use `with SyncEnvClient(...)` or call
`close()` explicitly. `__del__` is best-effort only and may not run
reliably (for example, during interpreter shutdown).

Examples:

```python
# From an async client
async_client = GenericEnvClient(base_url="http://localhost:8000")
sync_client = async_client.sync()

# Use synchronous context manager
with sync_client:
    result = sync_client.reset()
    result = sync_client.step({"action": "test"})
```

Close the connection and clean up resources.

self for method chaining

Establish connection to the server.

Close the connection.

`SyncEnvClient`A connected child wrapper around a child async
client of the same concrete type.

Create a new synchronous session against the same environment server.

The child session is tracked by this parent and closed when the parent
is closed. Call this after the parent has connected, because the child
reuses the parent's current base URL. Server-side capacity still
applies: when the server is at `MAX_CONCURRENT_ENVS`, opening the child
WebSocket can fail and is surfaced as a connection error.

- ****kwargs** --
  Optional parameters passed to the environment's reset method.StepResult containing initial observation

Reset the environment.

State object with environment state information

Get the current environment state.

- **action** --
  The action to execute.
- ****kwargs** --
  Optional parameters.StepResult containing observation, reward, and done status

Execute an action in the environment.

### Generic client[[openenv.GenericEnvClient]]

Environment client that works with raw dictionaries instead of typed classes.

This client doesn't require installing environment-specific packages, making it
ideal for:
- Connecting to remote servers without installing their packages
- Quick prototyping and testing
- Environments where type safety isn't needed
- Security-conscious scenarios where you don't want to run remote code

The trade-off is that you lose type safety and IDE autocomplete for actions
and observations. Instead of typed objects, you work with plain dictionaries.

Examples:

```python
# Direct connection to a running server (no installation needed)
with GenericEnvClient(base_url="http://localhost:8000") as env:
    result = env.reset()
    result = env.step({"code": "print('hello')"})
    print(result.observation)  # Dict[str, Any]
    print(result.observation.get("output"))

# From local Docker image (chain .sync() for synchronous use)
env = GenericEnvClient.from_docker_image("coding-env:latest").sync()
result = env.reset()
result = env.step({"code": "x = 1 + 2"})
env.close()

# From HuggingFace Hub (pulls Docker image, no pip install)
env = GenericEnvClient.from_env("user/my-env", use_docker=True).sync()
result = env.reset()
env.close()
```

`GenericEnvClient` inherits `from_docker_image()` and `from_env()` from
`EnvClient`, so you can use it with Docker containers and HuggingFace
Spaces without any package installation.

A dictionary subclass for creating actions when using GenericEnvClient.

This provides a semantic wrapper around dictionaries to make code more
readable when working with GenericEnvClient. It behaves exactly like a
dict but signals intent that this is an action for an environment.

Examples:

```python
# Without GenericAction (works fine)
env.step({"code": "print('hello')"})

# With GenericAction (more explicit)
action = GenericAction(code="print('hello')")
env.step(action)

# With multiple fields
action = GenericAction(code="x = 1", timeout=30, metadata={"tag": "test"})
env.step(action)
```

`GenericAction` is just a dict with a constructor that accepts keyword
arguments. It's provided for symmetry with typed Action classes and
to make code more readable.

### LLM client[[openenv.core.ToolCall]]

A single tool/function call returned by the model.

"}]}>

Normalized response from an LLM, with optional tool calls.

Convert to an OpenAI-format assistant message dict.

- **endpoint** (`str`) --
  The base URL of the LLM service (e.g. "http://localhost").
- **port** (`int`) --
  The port the service listens on.
Abstract base for LLM endpoint clients.

Subclass and implement `complete()` for your protocol.

- **prompt** (`str`) --
  The user prompt to send.
- ****kwargs** --
  Override default parameters (temperature, max_tokens, etc.).The model's text response.
Send a prompt, return the text response.

- **messages** (`list[dict[str, Any]]`) --
  Conversation history as OpenAI-format message dicts.
- **tools** (`list[dict[str, Any]]`) --
  MCP tool definitions.
- ****kwargs** --
  Override default parameters (temperature, max_tokens, etc.).An `LLMResponse` with the model's text and any tool calls.
Send messages with tool definitions, return a normalized response.

Messages use OpenAI-format dicts (`{"role": "...", "content": "..."}`).
Tools use MCP tool definitions; they are converted internally.

- **endpoint** (`str`) --
  The base URL (e.g. "http://localhost").
- **port** (`int`) --
  The port number.
- **model** (`str`) --
  Model name to pass to the API.
- **api_key** (`str`, *optional*) --
  API key. Defaults to "not-needed" for local endpoints.
- **system_prompt** (`str`, *optional*) --
  System message prepended to every request.
- **temperature** (`float`, *optional*, defaults to `0.0`) --
  Default sampling temperature.
- **max_tokens** (`int`, *optional*, defaults to `256`) --
  Default max tokens in the response.
- **use_max_completion_tokens** (`bool`, *optional*, defaults to `False`) --
  Use max_completion_tokens instead of max_tokens. Required for newer OpenAI models
  (gpt-5-mini, o1, o3). Not supported by self-hosted OpenAI-compatible endpoints.
Client for OpenAI-compatible APIs.

Works with: OpenAI, vLLM, TGI, Ollama, HuggingFace Inference API,
or any endpoint that speaks the OpenAI chat completions format.

- **prompt** (`str`) --
  The user message.
- ****kwargs** --
  Overrides for temperature, max_tokens.The assistant's response text.
Send a chat completion request.

- **endpoint** (`str`) --
  The base URL (e.g. `https://api.anthropic.com`).
- **port** (`int`) --
  The port number.
- **model** (`str`) --
  Model name (e.g. "claude-sonnet-4-20250514").
- **api_key** (`str`, *optional*) --
  Anthropic API key.
- **system_prompt** (`str`, *optional*) --
  System message prepended to every request.
- **temperature** (`float`, *optional*, defaults to `0.0`) --
  Default sampling temperature.
- **max_tokens** (`int`, *optional*, defaults to `256`) --
  Default max tokens in the response.
Client for Anthropic's Messages API.

Requires the `anthropic` package (lazy-imported at construction time).

- **provider** (`str`) --
  Provider name ("openai" or "anthropic").
- **model** (`str`) --
  Model identifier.
- **api_key** (`str`) --
  API key for the provider.
- **system_prompt** (`str`, *optional*) --
  System message prepended to every request.
- **temperature** (`float`, *optional*, defaults to `0.0`) --
  Sampling temperature.
- **max_tokens** (`int`, *optional*, defaults to `4096`) --
  Maximum tokens in the response.A configured `LLMClient` instance.
Create an LLM client for a hosted provider.

### Shared dataclasses[[openenv.core.client_types.StepResult]]

- **observation** --
  The environment's observation after the action.
- **reward** (`float`, *optional*) --
  Scalar reward for this step.
- **done** (`bool`, *optional*, defaults to `False`) --
  Whether the episode is finished.
- **metadata** (`dict`, *optional*) --
  Additional metadata returned alongside the observation.

Represents the result of one environment step.

## MCP (Model Context Protocol)

### MCP environment[[openenv.core.MCPEnvironment]]

- **mcp_server** -- A FastMCP server instance containing tool definitions.
  The server's tools will be validated against reserved names.
- **transform** -- Optional transform to apply to observations (inherited from Environment).- ``ValueError`` -- If any tool in the MCP server uses a reserved name
  (reset, step, state, close).``ValueError``

Base class for environments that expose tools via MCP (Model Context Protocol).

MCPEnvironment bridges FastMCP servers with OpenEnv's Gym-style API, allowing
agents to discover and invoke MCP tools through the standard step() interface.

The class automatically handles:
- ListToolsAction: Returns available tools from the MCP server
- CallToolAction: Invokes a specific tool with arguments

All other actions are delegated to the abstract _step_impl() method,
which subclasses must implement.

Examples:

```python
from fastmcp import FastMCP

mcp = FastMCP("calculator")

@mcp.tool()
def add(a: int, b: int) -> int:
    return a + b

env = MyMCPEnvironment(mcp)
obs = env.step(ListToolsAction())
obs.tools[0].name  # 'add'
```

Clean up resources used by the environment.

This method cleans up the MCP client and any other resources.
Subclasses should call super().close() if they override this method.

- **code** -- Python code to execute. Tools are available as functions
  in the execution namespace. Set a variable named 'result'
  to capture the return value.Observation with result in metadata["result"] or error in
metadata["error"].

Execute Python code with tools available as callables.

This enables the CodeAct pattern where agents write Python code
that calls tools directly as functions, avoiding JSON-RPC overhead.

Dictionary mapping tool names to callables.

Get callable functions for code mode.

Returns tool functions as direct Python callables, enabling code mode
where agents write Python code that calls tools directly (no JSON-RPC
overhead). Mode-specific tools are filtered by the current mode.

Context manager for MCP client sessions.

This wrapper serves two purposes:

1. **Null guard** — raises a clear error if `close()` has already
   been called (`mcp_client` is `None`).

2. **AsyncExitStack adapter** — FastMCP's `Client.__aenter__`
   creates a background `asyncio.Task` for session management.
   When entered directly via `AsyncExitStack` in the HTTP session
   path (`_create_session`), this task can be cancelled by ASGI
   harnesses (e.g. Starlette `TestClient`) between requests,
   corrupting session state.  Wrapping in an `asynccontextmanager`
   generator isolates the task lifecycle: the generator frame keeps
   `async with client:` suspended at `yield`, so cleanup only
   runs when the stack explicitly closes the generator — not when
   the event loop cancels orphaned tasks.

Delegates to FastMCP's `Client` context manager which is
reentrant: the first entry opens the transport and subsequent
(nested) entries simply increment an internal reference counter.
The transport is closed only when the outermost context exits.

No external lock is needed because `Client._connect` /
`Client._disconnect` already serialise connection state changes
through their own `anyio.Lock`.

- **action** (`Action`) --
  The action to execute. `ListToolsAction` returns available MCP tools,
  `CallToolAction` invokes a specific MCP tool, and any other action
  is delegated to _step_impl().
- **timeout_s** (`float`, *optional*) --
  Timeout in seconds for the action. Defaults to MCP_TOOL_CALL_TIMEOUT
  (30s) for MCP actions.
- ****kwargs** (`Any`) --
  Additional arguments passed to handlers.`Observation``ListToolsObservation` for `ListToolsAction`,
`CallToolObservation` for `CallToolAction`, or a subclass-defined
Observation for other actions.

Execute an action in the environment.

This method routes MCP-specific actions (ListToolsAction, CallToolAction)
to the appropriate handlers, while delegating all other actions to
the subclass's _step_impl() method.

Async step that routes MCP actions without going through run_async_safely.

The WebSocket handler calls this directly on the outer event loop, where
the MCP session is already open, avoiding the thread/event-loop deadlock
that occurs when the sync step() path is used via run_in_executor.

- **mode** -- Optional mode for the tool ("production" or "simulation").
  If None, tool is available in all modes.A decorator function for registering tools.- ``ValueError`` -- If mode is not None, "production", or "simulation".</raises><raisederrors>``ValueError``

Decorator for registering mode-aware tools.

### MCP types[[openenv.core.JsonRpcErrorCode]]

Standard JSON-RPC 2.0 error codes.

See: https://www.jsonrpc.org/specification#error_object

Supported MCP method names.

JSON-RPC 2.0 error object.

See: https://www.jsonrpc.org/specification#error_object

Create an error from a standard error code.

"}, {"name": "id", "val": ": typing.Union[str, int, NoneType] = None"}]}>

JSON-RPC 2.0 request object.

See: https://www.jsonrpc.org/specification#request_object

JSON-RPC 2.0 response object.

Per JSON-RPC 2.0 spec, a response has either 'result' or 'error', not both.
This model excludes None values during serialization to comply with the spec.

See: https://www.jsonrpc.org/specification#response_object

Create an error response from a standard error code.

Serialize to dict, excluding result or error when None (JSON-RPC compliance).

Serialize to JSON string, excluding result or error when None (JSON-RPC compliance).

Create a success response.

Strongly typed MCP tool specification.

Follows the MCP ToolSpec format for tool discovery.
See: https://modelcontextprotocol.io/specification/2025-06-18/server/tools

Types of errors that can occur during tool execution.

Structured error for tool execution failures.

This is used for transport/framework errors, NOT for errors returned
by the tool itself (those go in the result field).

"}, {"name": "type", "val": ": typing.Literal['list_tools'] = 'list_tools'"}]}>

Request list of available tools from the environment.

This action triggers MCP's tools/list operation and returns
all available tools with their schemas. Does NOT require reset()
to be called first.

"}, {"name": "type", "val": ": typing.Literal['call_tool'] = 'call_tool'"}, {"name": "tool_name", "val": ": str"}, {"name": "arguments", "val": ": typing.Dict[str, typing.Any] = "}]}>

Call a specific tool via MCP.

This action triggers MCP's tools/call operation with the
specified tool name and arguments.

"}, {"name": "tools", "val": ": typing.List[openenv.core.env_server.mcp_types.Tool]"}]}>

Response containing available tools.

Returned when processing a ListToolsAction.

"}, {"name": "tool_name", "val": ": str"}, {"name": "result", "val": ": typing.Any = None"}, {"name": "error", "val": ": typing.Optional[openenv.core.env_server.mcp_types.ToolError] = None"}]}>

Response from tool execution.

Contains the tool's result or an error if the call failed.
Tool-specific errors (from the tool itself) are included in the result.
Transport/framework errors use the error field.

WebSocket message for MCP JSON-RPC requests.

Allows direct MCP access via WebSocket for production inference,
bypassing the step() API.

WebSocket response for MCP JSON-RPC.

Contains the JSON-RPC response from the MCP server.

### MCP client[[openenv.core.MCPClientBase]]

- **_tools_cache** -- Cached list of tools (populated on first `list_tools()` call)

Base class for MCP clients with tool discovery.

This class provides the common `list_tools()` method for discovering
available tools from an MCP-enabled environment. Subclasses implement
specific interaction patterns (tool-calling or CodeAct).

Close client resources.

In production MCP mode, this also closes the server-side persistent
MCP session (best effort) before closing websocket/provider resources.

- **use_cache** (`bool`, *optional*, defaults to `True`) --
  If `True`, return cached tools if available. Set to `False` to force a fresh request.List of `Tool` objects with name, description, and input_schema.

Discover available tools from the environment.

Examples:

```python
tools = await env.list_tools()
for tool in tools:
    print(f"{tool.name}: {tool.description}")
```

Async client for tool-calling style MCP interactions.

Each step invokes a single tool. Use this for traditional function-calling
agent patterns where the agent decides which tool to call next.

This client provides convenience methods for tool discovery and invocation:
- `list_tools()`: Get all available tools with their schemas
- `call_tool(name, **kwargs)`: Invoke a tool by name with arguments

Examples:

```python
async with MCPToolClient(base_url="http://localhost:8000") as env:
    # Reset the environment
    await env.reset()

    # Discover available tools
    tools = await env.list_tools()
    print([t.name for t in tools])  # ['echo_message', 'echo_with_length']

    # Call a tool directly
    result = await env.call_tool("echo_message", message="Hello!")
    print(result)  # "Hello!"

    # Or use the full action interface
    from openenv.core.env_server.mcp_types import CallToolAction
    step_result = await env.step(CallToolAction(
        tool_name="echo_with_length",
        arguments={"message": "Test"}
    ))
    print(step_result.observation.result)
```

Sync wrapper:

```python
env = MCPToolClient(base_url="http://localhost:8000").sync()
with env:
    tools = env.list_tools()
    result = env.call_tool("echo_message", message="Hello!")
```

- **name** (`str`) --
  Name of the tool to invoke (must match a tool from `list_tools()`).
- ****kwargs** --
  Arguments to pass to the tool. Must match the tool's input_schema.The tool's result. The type depends on the tool being called.- ``RuntimeError`` -- If the server returns an error response.</raises><raisederrors>``RuntimeError``

Call a tool by name.

This is a convenience method that creates a CallToolAction, executes it,
and returns the result directly. For more control, use `step()` with
a CallToolAction directly.

Examples:

```python
result = await env.call_tool("add", a=5, b=3)
print(result)  # 8

result = await env.call_tool("greet", name="Claude")
print(result)  # "Hello, Claude!"
```

- **name** (`str`) --
  Name of the tool to find.The `Tool` object if found, `None` otherwise.

Get a specific tool by name.

Examples:

```python
tool = await env.get_tool("echo_message")
if tool:
    print(tool.description)
    print(tool.input_schema)
```

- **name** (`str`) --
  Name of the tool to check.`True` if the tool exists, `False` otherwise.

Check if a tool exists.

## Rubrics[[openenv.core.rubrics.Rubric]]

Abstract base class for reward computation.

A Rubric computes a reward signal from an action and observation.
Subclasses implement forward() to define the reward logic.

Examples:

```python
class MyRubric(Rubric):
    def forward(self, action, observation) -> float:
        return 1.0 if action.valid else 0.0

rubric = MyRubric()
reward = rubric(action, observation)
```

Child rubrics are auto-registered when assigned as attributes,
enabling hierarchical composition and introspection.

Iterate over immediate child rubrics.

- **action** -- The action taken by the agent.
- **observation** -- The resulting observation.`float`Reward value (typically 0.0 to 1.0).
Compute the reward. Implement this in subclasses.

- **path** (`str`) --
  Dot-separated path (e.g., "code.syntax").`Rubric`The rubric at the specified path.- ``KeyError`` -- If the path does not exist.</raises><raisederrors>``KeyError``
Access a nested rubric by dot-separated path.

Load rubric configuration from checkpoint.

Iterate over immediate child rubrics with names.

Iterate over all descendant rubrics with dot-separated names.

- **hook** (`Callable`) --
  Callable with signature (rubric, action, observation, result).
Register a hook called after forward().

- **hook** (`Callable`) --
  Callable with signature (rubric, action, observation).
Register a hook called before forward().

Reset any internal state. Override in subclasses if needed.

Iterate over all descendant rubrics (depth-first).

Serialize rubric configuration for checkpointing.

Run rubrics in order, fail-fast on zero.

Runs child rubrics in order. If any returns 0, stops immediately
and returns 0. This implements hierarchical gating patterns where
syntax checks run before execution checks.

Examples:

```python
rubric = Sequential(
    Gate(Compiles()),
    Gate(PassesTests(), threshold=0.5),
    WeightedSum([PassesTests(), StyleRubric()], weights=[0.7, 0.3])
)
```

Run rubrics in order, return 0 if any returns 0. Sync version.

Threshold wrapper - returns 0 if child score is below threshold.

Useful for hard constraints like "must pass 50% of tests".

Examples:

```python
rubric = Gate(PassesTests(), threshold=0.5)
# Returns PassesTests() score if >= 0.5, else 0.0
```

Return child score if >= threshold, else 0. Sync version.

Weighted combination of child rubrics.

Standard aggregation pattern for multi-criteria evaluation.

Examples:

```python
rubric = WeightedSum(
    [PassesTests(), StyleRubric()],
    weights=[0.7, 0.3]
)
```

Return weighted sum of child scores. Sync version.

Container for dynamic lists of rubrics.

Analogous to nn.ModuleList. Does not define aggregation - use within
a parent rubric that implements custom logic.

Examples:

```python
class MultiGameRubric(Rubric):
    def __init__(self, games: List[str]):
        super().__init__()
        self.games = RubricList([GameRubric(g) for g in games])

    def forward(self, action, obs) -> float:
        return self.games[obs.game_index](action, obs)
```

Add a rubric to the list.

Add multiple rubrics to the list.

RubricList does not define aggregation - override in parent.

Container for named rubrics with keyed access.

Analogous to nn.ModuleDict. Enables keyed access for multi-task
environments where different tasks require different rubrics.

Examples:

```python
class AtariRubric(Rubric):
    def __init__(self):
        super().__init__()
        self.games = RubricDict({
            "pong": PongRubric(),
            "breakout": BreakoutRubric(),
            "space_invaders": SpaceInvadersRubric(),
        })

    def forward(self, action, obs) -> float:
        return self.games[obs.game_id](action, obs)

# Access: env.rubric.games["pong"]
```

RubricDict does not define aggregation - override in parent.

Iterate over (key, rubric) pairs.

Iterate over keys.

Update with rubrics from a dictionary.

Iterate over rubrics.

Abstract base for rubrics that score based on full trajectories.

Subclasses implement:
- score_trajectory(): Compute final score from trajectory
- compute_step_rewards(): Define credit assignment strategy

The __call__ method accumulates steps and returns rewards according
to the subclass's implementation.

IMPORTANT: Trajectories are stored in CPU memory to avoid GPU pressure.
Environments with GPU tensors in observations must move them to CPU
before returning from step().

Known limitation: Very long episodes (thousands of steps) may consume
significant CPU memory. For such cases, consider streaming rubrics.

Examples:

```python
class WinLossRubric(TrajectoryRubric):
    def score_trajectory(self, trajectory):
        _, final_obs = trajectory[-1]
        return 1.0 if final_obs.metadata.get('won') else 0.0

    def compute_step_rewards(self):
        # Equal credit to all steps
        score = self.score_trajectory(self._trajectory)
        return [score] * len(self._trajectory)

rubric = WinLossRubric()
for action, obs in episode:
    reward = rubric(action, obs)  # 0.0 until done
step_rewards = rubric.compute_step_rewards()  # Credit assignment
```

`list[float]`Rewards, one per step. Length matches len(trajectory).
Compute per-step rewards from the accumulated trajectory.

Define your credit assignment strategy here (e.g., discounting,
assigning all credit to specific steps, etc.).

- **action** -- The action taken.
- **observation** -- The resulting observation. Must have a 'done' attribute.`float`intermediate_reward if not done, else score_trajectory() result.
Accumulate step and return reward.

Returns intermediate_reward until done, then computes trajectory score.

Load configuration from checkpoint.

Clear accumulated trajectory. Call on env.reset().

- **trajectory** (`list`) --
  List of (action, observation) tuples.`float`Final trajectory score (typically 0.0 to 1.0).
Score the complete trajectory. Return 0.0-1.0.

Called when observation.done=True.

Serialize configuration (not trajectory data).

TrajectoryRubric with exponential discounting for credit assignment.

Per-step reward:

```text
r_t = gamma^(T-1-t) * R_final
```

With gamma=0.99, later steps get higher reward (they're "closer" to the outcome).
With gamma=1.0, all steps get equal reward.
With gamma=0.0, only the final step gets reward.

This is the standard temporal discounting used in reinforcement learning,
applied retroactively once the episode outcome is known.

Examples:

```python
class ChessRubric(ExponentialDiscountingTrajectoryRubric):
    def score_trajectory(self, trajectory):
        _, final_obs = trajectory[-1]
        outcome = final_obs.metadata.get('winner')
        if outcome == 'agent': return 1.0
        elif outcome == 'opponent': return 0.0
        else: return 0.5  # Draw

rubric = ChessRubric(gamma=0.99)
reward = rubric(action, obs)  # 0.0 until done, then final score
step_rewards = rubric.compute_step_rewards()  # Discounted per-step rewards
```

`list[float]`Discounted rewards where `step_rewards[t] = gamma^(T-1-t) * R_final`,
T is the trajectory length and R_final is score_trajectory().
Apply exponential discounting from final reward.

Serialize configuration.

- **prompt_template** (*str*) --
  Template string with &amp;lcub;action} and &amp;lcub;observation} placeholders.
- **client** (*LLMClient*) --
  An LLMClient instance for making LLM calls.
- **score_pattern** (*str*, *optional*) --
  Regex to extract the score from the LLM response. Defaults to matching
  the first decimal number.
- **default_score** (*float*, *optional*, defaults to *0.0*) --
  Score returned when parsing fails.
- **normalize** (*bool*, *optional*, defaults to *True*) --
  If True, clamp extracted score to [0, 1].
Rubric that uses an LLM to evaluate agent actions/observations.

The prompt template is formatted with `&amp;lcub;action}` and `&amp;lcub;observation}`
placeholders. The LLM response is parsed for a numeric score.

- **action** -- The action taken by the agent.
- **observation** -- The resulting observation.`float`Parsed score from the LLM response.
Evaluate by sending a prompt to the LLM and parsing the score.

Serialize rubric configuration.

## Tools[[openenv.core.tools.RepoInfo]]

Information about a repository.

- **gitea_url** -- URL of the Gitea server (e.g., "http://gitea:3000")
- **username** -- Gitea username for authentication
- **password** -- Gitea password for authentication
- **workspace_dir** -- Local workspace directory for cloning repos

Client for connecting to an external Gitea server.

This client is optimized for task-based isolation where:
- Multiple tasks share the same Gitea instance
- Each task has its own isolated workspace
- Fast reset() via git operations (no server restart)
- Repos are pre-migrated to Gitea once

Examples:

```python
import os

# Connect to shared Gitea (credentials from environment)
client = GitServerClient(
    gitea_url=os.getenv("GITEA_URL"),
    username=os.getenv("GITEA_USERNAME"),
    password=os.getenv("GITEA_PASSWORD")
)
client.wait_for_ready()
# Clone repo to workspace
path = client.clone_to_workspace("my-repo", commit="abc123")
# Fast reset to base state
client.reset_workspace("my-repo", commit="abc123")
```

- **repo_name** (`str`) --
  Name of repository to clone.
- **target_dir** (`str`, *optional*) --
  Target directory name. Defaults to `repo_name`.
- **commit** (`str`, *optional*, defaults to `"main"`) --
  Commit hash or branch to check out.`str`Path to cloned repository.- ``RuntimeError`` -- If clone fails.</raises><raisederrors>``RuntimeError``

Clone a repository to the workspace at a specific commit.

This creates a fresh clone optimized for task isolation.

- **command** (`str`) --
  Git command to execute (without `git` prefix).
- **working_dir** (`str`, *optional*, defaults to `""`) --
  Working directory relative to workspace.`tuple` of (exit_code, stdout, stderr).

Execute a git command in the workspace.

- **repo_name** (`str`) --
  Name of repository in workspace.`str`Commit hash.

Get current commit hash of a workspace repository.

`list` of repository information dictionaries.

List all repositories in Gitea.

- **repo_name** (`str`) --
  Name of repository (directory in workspace).
- **commit** (`str`, *optional*, defaults to `"main"`) --
  Commit hash or branch to reset to.`bool`True if reset successful.- ``RuntimeError`` -- If reset fails.</raises><raisederrors>``RuntimeError``

Fast reset of workspace to base state (optimized for task resets).

This is much faster than re-cloning. It:
1. Checks out the target commit
2. Resets to that commit (hard)
3. Cleans untracked files

- **timeout** (`int`, *optional*, defaults to `30`) --
  Maximum seconds to wait.`bool`True if server is ready, False otherwise.

Wait for Gitea server to be ready.

Check if a repository exists in workspace.

## Container providers[[openenv.core.containers.runtime.ContainerProvider]]

Abstract base class for container providers.

Providers implement this interface to support different container platforms:
- LocalDockerProvider: Runs containers on local Docker daemon
- KubernetesProvider: Runs containers in Kubernetes cluster
- FargateProvider: Runs containers on AWS Fargate
- CloudRunProvider: Runs containers on Google Cloud Run

The provider manages a single container lifecycle and provides the base URL
for connecting to it.

Examples:

```python
provider = LocalDockerProvider()
base_url = provider.start_container("echo-env:latest")
print(base_url)  # http://localhost:8000
# Use the environment via base_url
provider.stop_container()
```

Release provider-held resources (e.g. SDK clients, connections).

Defaults to a no-op so existing providers are unaffected. Providers that
hold external resources beyond the container itself (such as a cloud SDK
client) override this to release them; it is also invoked on context-
manager exit. Lightweight providers need not override it.

- **image** (`str`) --
  Provider-specific container *source* identifier. For
  container-based providers this is a registry image name (e.g.
  `"echo-env:latest"`); other providers may map it to a
  provider-specific source (see the provider's documentation).
- **port** (`int`, *optional*) --
  Port to expose. If `None`, the provider chooses.
- **env_vars** (`dict`, *optional*) --
  Environment variables to pass to container.
- ****kwargs** --
  Provider-specific options.`str`Base URL to connect to the container (e.g., `"http://localhost:8000"`).- ``RuntimeError`` -- If container fails to start.</raises><raisederrors>``RuntimeError``

Start a container from the specified image.

Stop and remove the running container.

This cleans up the container that was started by start_container().

- **base_url** (`str`) --
  Base URL of the container.
- **timeout_s** (`float`, *optional*, defaults to `30.0`) --
  Maximum time to wait in seconds.- ``TimeoutError`` -- If container doesn't become ready in time.</raises><raisederrors>``TimeoutError``

Wait for the container to be ready to accept requests.

This typically polls the /health endpoint until it returns 200.

Container provider for local Docker daemon.

This provider runs containers on the local machine using Docker.
Useful for development and testing.

Examples:

```python
provider = LocalDockerProvider()
base_url = provider.start_container("echo-env:latest")
# Container running on http://localhost:<random-port>
provider.stop_container()
```

- **image** (`str`) --
  Docker image name.
- **port** (`int`, *optional*) --
  Port to expose. If `None`, finds an available port.
- **env_vars** (`dict`, *optional*) --
  Environment variables for the container.
- ****kwargs** --
  Additional Docker run options.`str`Base URL to connect to the container.

Start a Docker container locally.

Stop and remove the Docker container.

- **base_url** (`str`) --
  Base URL of the container.
- **timeout_s** (`float`, *optional*, defaults to `30.0`) --
  Maximum time to wait in seconds.- ``TimeoutError`` -- If container doesn't become ready.</raises><raisederrors>``TimeoutError``

Wait for container to be ready by polling /health endpoint.

Container provider that uses Docker Swarm services for local concurrency.

This provider creates a replicated Swarm service backed by the local Docker
engine. The built-in load-balancer fans requests across the replicas,
allowing multiple container instances to run concurrently on the developer
workstation (mirroring the workflow described in the Docker stack docs).

- **image** (`str`) --
  Docker image name.
- **port** (`int`, *optional*) --
  Port to expose. If `None`, finds an available port.
- **env_vars** (`dict`, *optional*) --
  Environment variables for the container.
- **replicas** (`int`, *optional*, defaults to `2`) --
  Number of container replicas.
- **cpu_limit** (`float` or `str`, *optional*) --
  CPU limit passed to `--limit-cpu`.
- **memory_limit** (`str`, *optional*) --
  Memory limit passed to `--limit-memory`.
- **constraints** (`Sequence[str]`, *optional*) --
  Placement constraints.
- **labels** (`dict`, *optional*) --
  Service labels.
- **command** (`Sequence[str]` or `str`, *optional*) --
  Override container command.`str`Base URL to connect to the service.

Start (or scale) a Swarm service for the given image.

Remove the Swarm service (and keep the Swarm manager running).

Wait for at least one replica to become healthy by polling /health.

With Swarm's load balancer, requests round-robin across replicas,
so this only verifies that at least one replica is responding. Some
replicas may still be starting when this returns.

Abstract base class for runtime providers that are not container providers.
Providers implement this interface to support different runtime platforms:
- UVProvider: Runs environments via `uv run`

The provider manages a single runtime lifecycle and provides the base URL
for connecting to it.

Examples:

```python
provider = UVProvider(project_path="/path/to/env")
base_url = provider.start()
print(base_url)  # http://localhost:8000
provider.stop()
```

- **port** (`int`, *optional*) --
  Port to expose. If `None`, the provider chooses.
- **env_vars** (`dict`, *optional*) --
  Environment variables for the runtime.
- ****kwargs** --
  Additional runtime options.`str`Base URL to connect to the runtime.

Start the runtime.

Stop the runtime.

Wait for the runtime to be ready to accept requests.

- **project_path** (*str*) --
  Local path to a uv project (passed to *uv run --project*), or a
  *git+&amp;lt;url>* spec that is cloned to a temp directory on *start()*.
- **app** (*str*, *optional*, defaults to *"server.app --app"*):
  ASGI application path for uvicorn.
- **host** (*str*, *optional*, defaults to *"0.0.0.0"*) --
  Host interface to bind to.
- **reload** (*bool*, *optional*, defaults to *False*) --
  Whether to enable uvicorn's reload mode.
- **env_vars** (*dict*, *optional*) --
  Environment variables to pass through to the spawned process.
- **context_timeout_s** (*float*, *optional*, defaults to *60.0*) --
  How long to wait for the environment to become ready.

RuntimeProvider implementation backed by `uv run`.

Examples:

```python
provider = UVProvider(project_path="/path/to/env")
base_url = provider.start()
print(base_url)  # http://localhost:8000
# Use the environment via base_url
provider.stop()
```

- **port** (`int`, *optional*) --
  The port to bind the environment to.
- **env_vars** (`dict`, *optional*) --
  Environment variables to pass to the environment.
- **workers** (`int`, *optional*, defaults to `1`) --
  The number of workers to use.`str`Base URL of the environment.- ``RuntimeError`` -- If the environment is already running.</raises><raisederrors>``RuntimeError``

Start the environment via `uv run`.

Stop the environment.

- **timeout_s** (`float`, *optional*) --
  Maximum time in seconds to wait for the environment to become
  ready. Defaults to the provider's `context_timeout_s`.- ``RuntimeError`` -- If the environment is not running.
- ``TimeoutError`` -- If the environment does not become ready within the timeout.</raises><raisederrors>``RuntimeError`` or ``TimeoutError``

Wait for the environment to become ready.

Container provider that runs environments in Daytona cloud sandboxes.

Example:
>>> provider = DaytonaProvider(api_key="your-key")
>>> image = DaytonaProvider.image_from_dockerfile("envs/echo_env/server/Dockerfile")
>>> base_url = provider.start_container(image)
>>> provider.wait_for_ready(base_url)
>>> provider.stop_container()

- **dockerfile_path** -- Path to the Dockerfile on disk.
- **context_dir** -- Build context directory.  Defaults to the
  Dockerfile's grandparent directory, matching the
  `openenv init` convention where Dockerfiles live in
  `&amp;lt;env>/server/Dockerfile` and the build context is
  `&amp;lt;env>/`.  Pass explicitly for non-standard layouts
  (e.g. `context_dir="."` for repo-root contexts).A `"dockerfile</rettype><retdesc>&amp;lt;abs_path>"` string to pass to `start_container`.- `FileNotFoundError` -- If *dockerfile_path* does not exist.
- `ValueError` -- If *context_dir* is given but does not exist,
  or if COPY sources in the Dockerfile cannot be found
  under the resolved context directory.`FileNotFoundError` or `ValueError`
Validate a Dockerfile and return a `dockerfile:` URI for
`start_container`.

Eagerly validates the Dockerfile (existence, COPY sources,
BuildKit stripping) and stores the processed content in an
internal registry.  The actual `daytona.Image` is created
later inside `start_container`.

Get a fresh signed preview URL (valid for 24h).

Daytona signed URLs expire after at most 24 hours.  Call this to
get a new one for long-running sessions.  The returned URL points
to the same sandbox — clients will need to reconnect using it.

- **image** -- Docker image name (e.g. `"echo-env:latest"`),
  `"snapshot:&amp;lt;name>"` to create from a pre-built snapshot,
  or `"dockerfile:&amp;lt;path>"` returned by
  `image_from_dockerfile`. May be omitted when supplied
  to the constructor.
- **port** -- Must be `None` or `8000`. Daytona exposes port 8000
  via its preview proxy; other ports raise `ValueError`.
- **env_vars** -- Environment variables forwarded to the sandbox.
- ****kwargs** -- `cmd` (str) to override the server command;
  remaining kwargs passed through to `Daytona.create()`.HTTPS preview URL for the sandbox (base_url).

Create a Daytona sandbox from a Docker image or snapshot.

Daytona does not execute the image's CMD (known bug — ENTRYPOINT
runs, CMD does not).  The server command is resolved in order:

1. Explicit `cmd` passed to the constructor.
2. `cmd` key in `**kwargs` (popped before forwarding).
3. Auto-discovered from `openenv.yaml` inside the sandbox.
4. `CMD` parsed from the Dockerfile (when *image* came from
   `image_from_dockerfile`).

Delete the Daytona sandbox.

Remove BuildKit `--mount=...` flags from `RUN` instructions.

Handles single-line flags, multi-line continuations, and multiple
`--mount` flags spread across continuation lines. Only leading
`--mount` flags are removed (before the actual command starts).

Daytona's `Image.from_dockerfile` does not support BuildKit
`--mount` syntax.  This helper strips the flags so that standard
Dockerfiles (like the ones generated by `openenv build`) can
be used directly.

- **base_url** -- Preview URL returned by `start_container()`.
- **timeout_s** -- Maximum seconds to wait.- `TimeoutError` -- If the sandbox doesn't become ready in time.
- `RuntimeError` -- If the server process died (detected via PID check).`TimeoutError` or `RuntimeError`

Poll the /health endpoint until the sandbox is ready.

Uses a longer default timeout (120s) than Docker providers because
Daytona sandboxes may have cold-start latency.

Container provider backed by Azure Container Apps Sandboxes.

`start_container`'s `image` is an ACA sandbox source (`disk:<name>` for a
public disk image or `disk-id:<id>` for a private one), not a Docker
registry image. The provider boots an OpenEnv server inside the sandbox,
exposes it on an anonymous ACA port, and returns an `https://` URL that
`EnvClient` connects to over `wss://`.

The environment runs untrusted code, so the provider is secure by default
(proposed RFC 002 security invariants): it requires explicit `anonymous_port=True`
ingress (S2), enforces https/wss transport (S1), offers
`deny_all_egress()` to block the cloud metadata/IMDS endpoint (S3), and
never surfaces raw sandbox output unless `surface_server_logs=True` (S4).

Validated end to end against a live ACA sandbox group; the underlying
`azure-containerapps-sandbox` SDK is preview, so pin it and re-validate
after upgrades (see `tests/test_core/test_aca_provider_integration.py`).

`close()` stops the active sandbox *and* releases the underlying SDK client,
so prefer using the provider as a context manager (or call `close()`
explicitly) for deterministic cleanup. `close()`/`__enter__`/`__exit__` are
defined on `ContainerProvider` (default no-op), so a caller holding a bare
`ContainerProvider` reference can release the client polymorphically.

Only one sandbox is active per provider: calling `start_container` again
before `stop_container()`/`close()` raises `RuntimeError` rather than
orphaning the running sandbox.

```python
with ACASandboxProvider(
    image="disk:my-env",
    anonymous_port=True,
    ...,
) as provider:
    base_url = provider.start_container(cmd=...)
    ...
# sandbox deleted and SDK client closed on exit
```

Stop the active sandbox and close the underlying SDK client.

Overrides the base no-op so a caller can release the preview SDK client
deterministically (also invoked on context-manager exit). The base
`ContainerProvider` defines `close()`/`__enter__`/`__exit__`, so callers
holding a bare `ContainerProvider` can release resources polymorphically.

Build a default-deny ACA `EgressPolicy` (RFC 002 security invariant S3).

Untrusted RL/agent code should not be able to exfiltrate data or reach
the cloud metadata/IMDS endpoint. This returns an `EgressPolicy` whose
default action is `Deny`, with an optional host allowlist (for example a
model endpoint or package registry the environment legitimately needs).

```python
provider = ACASandboxProvider(
    anonymous_port=True,
    egress_policy=ACASandboxProvider.deny_all_egress(
        allow=["my-model.openai.azure.com"]
    ),
)
```

Start an OpenEnv server in an ACA sandbox and return its base URL.

`image` is an ACA sandbox *source*, not a Docker/OCI registry image: a
bare string or `disk:<name>` for a public disk image, or `disk-id:<id>`
for a private one. It may be omitted when supplied to the constructor.
A value that looks like a container image (a registry path or
`name:tag`) is rejected with guidance. Only port 8000 is supported.
`**kwargs` accepts per-start overrides (`cmd`, `labels`,
`egress_policy`, `anonymous_port`); unknown options raise `ValueError`
so typos cannot silently change sandbox behavior.

Delete the active ACA sandbox.

Wait for the ACA-hosted OpenEnv server to answer on `/health`.

A `200` on `/health` proves HTTP reachability but not that the exposed
ACA port proxies the `/ws` WebSocket upgrade `EnvClient` needs; the
integration test covers the full `wss://` round-trip. If the captured
server process dies during startup this raises `RuntimeError` (sandbox
output is withheld unless `surface_server_logs=True`; see RFC 002 S4).

Container provider that runs environments in Modal sandboxes.

`start_container`'s `image` is either a registry tag
(`"echo-env:latest"`) or a `"dockerfile:&amp;lt;path>"` reference returned by
`image_from_dockerfile`. The server is exposed on an encrypted Modal
tunnel and the returned `https://` URL is what `EnvClient` connects to
over `wss://`.

The environment runs untrusted code, so the provider is secure by default:
it enforces https/wss transport, treats the tunnel URL as a bearer secret
(never interpolated into errors), and never surfaces raw sandbox output
unless `surface_server_logs=True`.

Only one sandbox is active per provider: calling `start_container` again
before `stop_container()`/``close()` raises `RuntimeError` rather than orphaning the running sandbox. `close()`` (and context-manager exit) stops
the active sandbox.

Example:
```python
with ModalProvider(app_name="openenv", cpu=2.0, memory=4096) as provider:
    image = ModalProvider.image_from_dockerfile(
        "envs/echo_env/server/Dockerfile"
    )
    base_url = provider.start_container(image)
    provider.wait_for_ready(base_url)
# sandbox terminated on exit
```

Sandbox v2 (beta) is opt-in:
```python
provider = ModalProvider(app_name="openenv", use_sandbox_v2=True)
```

Stop the active sandbox.

Overrides the base no-op so a caller holding a bare `ContainerProvider`
reference can release the sandbox polymorphically (also invoked on
context-manager exit). `ModalProvider` holds no separate SDK client, so
this is equivalent to `stop_container()`.

- **dockerfile_path** (*str*) --
  Path to the Dockerfile on disk.
- **context_dir** (*str*, *optional*) --
  Build context directory.  Defaults to the Dockerfile's
  grandparent directory, matching the `openenv init`
  convention where Dockerfiles live in
  `&amp;lt;env>/server/Dockerfile` and the build context is
  `&amp;lt;env>/`.  Pass explicitly for non-standard layouts
  (e.g. `context_dir="."` for repo-root contexts).*str*A `"dockerfile:&amp;lt;abs_path>"` string to pass to
`start_container`.- `FileNotFoundError` -- If *dockerfile_path* does not exist.
- `ValueError` -- If *context_dir* is given but does not exist,
  or if COPY sources in the Dockerfile cannot be found
  under the resolved context directory.`FileNotFoundError` or `ValueError`
Validate a Dockerfile and return a `dockerfile:` URI for
`start_container`.

Eagerly validates the Dockerfile (existence, COPY sources) and stores
its content in an internal registry.  The actual `modal.Image` is
created later inside `start_container`.

- **image** (*str*, *optional*) --
  Registry image tag (e.g. `"echo-env:latest"`) or
  `"dockerfile:&amp;lt;path>"` returned by
  `image_from_dockerfile`. May be omitted when supplied to
  the constructor.
- **port** (*int*, *optional*) --
  Must be `None` or `8000`. Modal exposes port 8000 via an
  encrypted tunnel; other ports raise `ValueError`.
- **env_vars** (*dict*, *optional*) --
  Environment variables forwarded to the sandbox.
- ****kwargs** --
  `cmd` (*str*) to override the server command; any remaining
  keyword arguments are forwarded to `modal.Sandbox.create`.*str*HTTPS tunnel URL for the sandbox (base_url).

Create a Modal sandbox from a Docker image or Dockerfile.

The sandbox is started with a keep-alive process and the server
command is launched via `exec` afterwards, mirroring the discovery
flow used by other cloud providers. The server command is resolved in
order:

1. Explicit `cmd` passed to the constructor.
2. `cmd` key in `**kwargs` (popped before forwarding).
3. Auto-discovered from `openenv.yaml` inside the sandbox.
4. `CMD` parsed from the Dockerfile (when *image* came from
   `image_from_dockerfile`).

Terminate the Modal sandbox.

- **base_url** (*str*) --
  Tunnel URL returned by `start_container()`.
- **timeout_s** (*float*, *optional*, defaults to *120.0*) --
  Maximum seconds to wait.- `TimeoutError` -- If the sandbox doesn't become ready in time.
- `RuntimeError` -- If the server process died (detected via PID check).`TimeoutError` or `RuntimeError`

Poll the /health endpoint until the sandbox is ready.

Uses a longer default timeout (120s) than local Docker providers
because Modal sandboxes may have cold-start latency.
