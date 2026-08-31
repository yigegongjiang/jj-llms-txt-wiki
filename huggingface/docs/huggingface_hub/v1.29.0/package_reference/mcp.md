# MCP Client

The `huggingface_hub` library now includes an [MCPClient](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.MCPClient), designed to empower Large Language Models (LLMs) with the ability to interact with external Tools via the [Model Context Protocol](https://modelcontextprotocol.io) (MCP). This client extends an [AsyncInferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient) to seamlessly integrate Tool usage.

The [MCPClient](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.MCPClient) connects to MCP servers (local `stdio` scripts or remote `http`/`sse` services) that expose tools. It feeds these tools to an LLM (via [AsyncInferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient)). If the LLM decides to use a tool, [MCPClient](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.MCPClient) manages the execution request to the MCP server and relays the Tool's output back to the LLM, often streaming results in real-time.

We also provide a higher-level [Agent](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.Agent) class. This 'Tiny Agent' simplifies creating conversational Agents by managing the chat loop and state, acting as a wrapper around [MCPClient](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.MCPClient).

## MCP Client[[huggingface_hub.MCPClient]]

#### huggingface_hub.MCPClient[[huggingface_hub.MCPClient]]

```python
huggingface_hub.MCPClient(model: typing.Optional[str] = None, provider: typing.Union[typing.Literal['baseten', 'cerebras', 'cohere', 'deepinfra', 'fal-ai', 'featherless-ai', 'fireworks-ai', 'groq', 'hf-inference', 'novita', 'nscale', 'openai', 'ovhcloud', 'publicai', 'replicate', 'scaleway', 'together', 'wavespeed', 'zai-org'], typing.Literal['auto'], NoneType] = None, base_url: typing.Optional[str] = None, api_key: typing.Optional[str] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/mcp_client.py#L55)

**Parameters:**

model (`str`, `optional`) : The model to run inference with. Can be a model id hosted on the Hugging Face Hub, e.g. `meta-llama/Meta-Llama-3-8B-Instruct` or a URL to a deployed Inference Endpoint or other local or remote endpoint.

provider (`str`, *optional*) : Name of the provider to use for inference. Defaults to "auto" i.e. the first of the providers available for the model, sorted by the user's order in https://hf.co/settings/inference-providers. If model is a URL or `base_url` is passed, then `provider` is not used.

base_url (`str`, *optional*) : The base URL to run inference. Defaults to None.

api_key (`str`, `optional`) : Token to use for authentication. Will default to the locally Hugging Face saved token if not provided. You can also use your own provider API key to interact directly with the provider's service.

Client for connecting to one or more MCP servers and processing chat completions with tools.

> [!WARNING]
> This class is experimental and might be subject to breaking changes in the future without prior notice.

#### add_mcp_server[[huggingface_hub.MCPClient.add_mcp_server]]

```python
add_mcp_server(type: typing.Literal['stdio', 'sse', 'http'], **params: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/mcp_client.py#L123)

**Parameters:**

type (`str`) : Type of the server to connect to. Can be one of: - "stdio": Standard input/output server (local) - "sse": Server-sent events (SSE) server - "http": StreamableHTTP server

- ****params** (`dict[str, Any]`) : Server parameters that can be either: - For stdio servers: - command (str): The command to run the MCP server - args (list[str], optional): Arguments for the command - env (dict[str, str], optional): Environment variables for the command - cwd (Union[str, Path, None], optional): Working directory for the command - allowed_tools (list[str], optional): List of tool names to allow from this server - For SSE servers: - url (str): The URL of the SSE server - headers (dict[str, Any], optional): Headers for the SSE connection - timeout (float, optional): Connection timeout - sse_read_timeout (float, optional): SSE read timeout - allowed_tools (list[str], optional): List of tool names to allow from this server - For StreamableHTTP servers: - url (str): The URL of the StreamableHTTP server - headers (dict[str, Any], optional): Headers for the StreamableHTTP connection - timeout (timedelta, optional): Connection timeout - sse_read_timeout (timedelta, optional): SSE read timeout - terminate_on_close (bool, optional): Whether to terminate on close - allowed_tools (list[str], optional): List of tool names to allow from this server

Connect to an MCP server

#### cleanup[[huggingface_hub.MCPClient.cleanup]]

```python
cleanup()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/mcp_client.py#L109)

Clean up resources

#### process_single_turn_with_tools[[huggingface_hub.MCPClient.process_single_turn_with_tools]]

```python
process_single_turn_with_tools(messages: list, exit_loop_tools: typing.Optional[list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputTool]] = None, exit_if_first_chunk_no_tool: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/mcp_client.py#L248)

**Parameters:**

messages (`list[dict]`) : List of message objects representing the conversation history

exit_loop_tools (`list[ChatCompletionInputTool]`, *optional*) : List of tools that should exit the generator when called

exit_if_first_chunk_no_tool (`bool`, *optional*) : Exit if no tool is present in the first chunks. Default to False.

**Yields:**

[ChatCompletionStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionStreamOutput) chunks or [ChatCompletionInputMessage](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputMessage) objects

Process a query using `self.model` and available tools, yielding chunks and tool outputs.

## Agent[[huggingface_hub.Agent]]

#### huggingface_hub.Agent[[huggingface_hub.Agent]]

```python
huggingface_hub.Agent(model: Optional[str] = None, servers: Iterable[ServerConfig], provider: Optional[PROVIDER_OR_POLICY_T] = None, base_url: Optional[str] = None, api_key: Optional[str] = None, prompt: Optional[str] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/agent.py#L13)

**Parameters:**

model (`str`, *optional*) : The model to run inference with. Can be a model id hosted on the Hugging Face Hub, e.g. `meta-llama/Meta-Llama-3-8B-Instruct` or a URL to a deployed Inference Endpoint or other local or remote endpoint.

servers (`Iterable[dict]`) : MCP servers to connect to. Each server is a dictionary containing a `type` key and a `config` key. The `type` key can be `"stdio"` or `"sse"`, and the `config` key is a dictionary of arguments for the server.

provider (`str`, *optional*) : Name of the provider to use for inference. Defaults to "auto" i.e. the first of the providers available for the model, sorted by the user's order in https://hf.co/settings/inference-providers. If model is a URL or `base_url` is passed, then `provider` is not used.

base_url (`str`, *optional*) : The base URL to run inference. Defaults to None.

api_key (`str`, *optional*) : Token to use for authentication. Will default to the locally Hugging Face saved token if not provided. You can also use your own provider API key to interact directly with the provider's service.

prompt (`str`, *optional*) : The system prompt to use for the agent. Defaults to the default system prompt in `constants.py`.

Implementation of a Simple Agent, which is a simple while loop built right on top of an [MCPClient](/docs/huggingface_hub/v1.29.0/en/package_reference/mcp#huggingface_hub.MCPClient).

> [!WARNING]
> This class is experimental and might be subject to breaking changes in the future without prior notice.

#### run[[huggingface_hub.Agent.run]]

```python
run(user_input: str, abort_event: Optional[asyncio.Event] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_mcp/agent.py#L57)

**Parameters:**

user_input (`str`) : The user input to run the agent with.

abort_event (`asyncio.Event`, *optional*) : An event that can be used to abort the agent. If the event is set, the agent will stop running.

Run the agent with the given user input.
