# Tools

Smolagents is an experimental API which is subject to change at any time. Results returned by the agents
can vary as the APIs or underlying models are prone to change.

To learn more about agents and tools make sure to read the [introductory guide](../index). This page
contains the API docs for the underlying classes.

## Tool Base Classes

### load_tool[[smolagents.load_tool]]

#### smolagents.load_tool[[smolagents.load_tool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L840)

Main function to quickly load a tool from the Hub.

Loading a tool means that you'll download the tool and execute it locally.
ALWAYS inspect the tool you're downloading before loading it within your runtime, as you would do when
installing a package using pip/npm/apt.

**Parameters:**

repo_id (`str`) : Space repo ID of a tool on the Hub.

model_repo_id (`str`, *optional*) : Use this argument to use a different model than the default one for the tool you selected.

token (`str`, *optional*) : The token to identify you on hf.co. If unset, will use the token generated when running `huggingface-cli login` (stored in `~/.huggingface`).

trust_remote_code (`bool`, *optional*, defaults to False) : This needs to be accepted in order to load a tool from Hub.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments that will be split in two: all arguments relevant to the Hub (such as `cache_dir`, `revision`, `subfolder`) will be used when downloading the files for your tool, and the others will be passed along to its init.

### tool[[smolagents.tool]]

#### smolagents.tool[[smolagents.tool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L1061)

Convert a function into an instance of a dynamically created Tool subclass.

**Parameters:**

tool_function (`Callable`) : Function to convert into a Tool subclass. Should have type hints for each input and a type hint for the output. Should also have a docstring including the description of the function and an 'Args:' part where each argument is described.

### Tool[[smolagents.Tool]]

#### smolagents.Tool[[smolagents.Tool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L106)

A base class for the functions used by the agent. Subclass this and implement the `forward` method as well as the
following class attributes:

- **description** (`str`) -- A short description of what your tool does, the inputs it expects and the output(s) it
  will return. For instance 'This is a tool that downloads a file from a `url`. It takes the `url` as input, and
  returns the text contained in the file'.
- **name** (`str`) -- A performative name that will be used for your tool in the prompt to the agent. For instance
  `"text-classifier"` or `"image_generator"`.
- **inputs** (`Dict[str, Dict[str, Union[str, type, bool]]]`) -- The dict of modalities expected for the inputs.
  It has one `type`key and a `description`key.
  This is used by `launch_gradio_demo` or to make a nice space from your tool, and also can be used in the generated
  description for your tool.
- **output_type** (`type`) -- The type of the tool output. This is used by `launch_gradio_demo`
  or to make a nice space from your tool, and also can be used in the generated description for your tool.
- **output_schema** (`Dict[str, Any]`, *optional*) -- The JSON schema defining the expected structure of the tool output.
  This can be included in system prompts to help agents understand the expected output format. Note: This is currently
  used for informational purposes only and does not perform actual output validation.

You can also override the method [setup()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool.setup) if your tool has an expensive operation to perform before being
usable (such as loading a model). [setup()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool.setup) will be called the first time you use your tool, but not at
instantiation.

from_dictsmolagents.Tool.from_dicthttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L367[{"name": "tool_dict", "val": ": dict[str, Any]"}, {"name": "**kwargs", "val": ""}]- **tool_dict** (`dict[str, Any]`) -- Dictionary representation of the tool.
- ****kwargs** -- Additional keyword arguments to pass to the tool's constructor.0`Tool`Tool object.

Create tool from a dictionary representation.

**Parameters:**

tool_dict (`dict[str, Any]`) : Dictionary representation of the tool.

- ****kwargs** : Additional keyword arguments to pass to the tool's constructor.

**Returns:**

``Tool``

Tool object.
#### from_gradio[[smolagents.Tool.from_gradio]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L741)

Creates a [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool) from a gradio tool.
#### from_hub[[smolagents.Tool.from_hub]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L516)

Loads a tool defined on the Hub.

Loading a tool from the Hub means that you'll download the tool and execute it locally.
ALWAYS inspect the tool you're downloading before loading it within your runtime, as you would do when
installing a package using pip/npm/apt.

**Parameters:**

repo_id (`str`) : The name of the Space repo on the Hub where your tool is defined.

token (`str`, *optional*) : The token to identify you on hf.co. If unset, will use the token generated when running `huggingface-cli login` (stored in `~/.huggingface`).

trust_remote_code(`str`, *optional*, defaults to False) : This flags marks that you understand the risk of running remote code and that you trust this tool. If not setting this to True, loading the tool from Hub will fail.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments that will be split in two: all arguments relevant to the Hub (such as `cache_dir`, `revision`, `subfolder`) will be used when downloading the files for your tool, and the others will be passed along to its init.
#### from_langchain[[smolagents.Tool.from_langchain]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L762)

Creates a [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool) from a langchain tool.
#### from_space[[smolagents.Tool.from_space]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L599)

Creates a [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool) from a Space given its id on the Hub.

Examples:
```py
>>> image_generator = Tool.from_space(
...     space_id="black-forest-labs/FLUX.1-schnell",
...     name="image-generator",
...     description="Generate an image from a prompt"
... )
>>> image = image_generator("Generate an image of a cool surfer in Tahiti")
```

```py
>>> face_swapper = Tool.from_space(
...     "tuan2308/face-swap",
...     "face_swapper",
...     "Tool that puts the face shown on the first image on the second image. You can give it paths to images.",
... )
>>> image = face_swapper('./aymeric.jpeg', './ruth.jpg')
```

**Parameters:**

space_id (`str`) : The id of the Space on the Hub.

name (`str`) : The name of the tool.

description (`str`) : The description of the tool.

api_name (`str`, *optional*) : The specific api_name to use, if the space has several tabs. If not precised, will default to the first available api.

token (`str`, *optional*) : Add your token to access private spaces or increase your GPU quotas.

**Returns:**

`[Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool)`

The Space, as a tool.
#### push_to_hub[[smolagents.Tool.push_to_hub]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L421)

Upload the tool to the Hub.

**Parameters:**

repo_id (`str`) : The name of the repository you want to push your tool to. It should contain your organization name when pushing to a given organization.

commit_message (`str`, *optional*, defaults to `"Upload tool"`) : Message to commit while pushing.

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`bool` or `str`, *optional*) : The token to use as HTTP bearer authorization for remote files. If unset, will use the token generated when running `huggingface-cli login` (stored in `~/.huggingface`).

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.
#### save[[smolagents.Tool.save]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L390)

Saves the relevant code files for your tool so it can be pushed to the Hub. This will copy the code of your
tool in `output_dir` as well as autogenerate:

- a `{tool_file_name}.py` file containing the logic for your tool.
If you pass `make_gradio_app=True`, this will also write:
- an `app.py` file providing a UI for your tool when it is exported to a Space with `tool.push_to_hub()`
- a `requirements.txt` containing the names of the modules used by your tool (as detected when inspecting its
  code)

**Parameters:**

output_dir (`str` or `Path`) : The folder in which you want to save your tool.

tool_file_name (`str`, *optional*) : The file name in which you want to save your tool.

make_gradio_app (`bool`, *optional*, defaults to True) : Whether to also export a `requirements.txt` file and Gradio UI.
#### setup[[smolagents.Tool.setup]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L251)

Overwrite this method here for any operation that is expensive and needs to be executed before you start using
your tool. Such as loading a big model.
#### to_dict[[smolagents.Tool.to_dict]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L292)

Returns a dictionary representing the tool

### launch_gradio_demo[[smolagents.launch_gradio_demo]]

#### smolagents.launch_gradio_demo[[smolagents.launch_gradio_demo]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L794)

Launches a gradio demo for a tool. The corresponding tool class needs to properly implement the class attributes
`inputs` and `output_type`.

**Parameters:**

tool (`Tool`) : The tool for which to launch the demo.

## ToolCollection[[smolagents.ToolCollection]]

#### smolagents.ToolCollection[[smolagents.ToolCollection]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L895)

Tool collections enable loading a collection of tools in the agent's toolbox.

Collections can be loaded from a collection in the Hub or from an MCP server, see:
- [ToolCollection.from_hub()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.ToolCollection.from_hub)
- [ToolCollection.from_mcp()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.ToolCollection.from_mcp)

For example and usage, see: [ToolCollection.from_hub()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.ToolCollection.from_hub) and [ToolCollection.from_mcp()](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.ToolCollection.from_mcp)

from_hubsmolagents.ToolCollection.from_hubhttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L909[{"name": "collection_slug", "val": ": str"}, {"name": "token", "val": ": str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}]- **collection_slug** (str) -- The collection slug referencing the collection.
- **token** (str, *optional*) -- The authentication token if the collection is private.
- **trust_remote_code** (bool, *optional*, defaults to False) -- Whether to trust the remote code.0ToolCollectionA tool collection instance loaded with the tools.
Loads a tool collection from the Hub.

it adds a collection of tools from all Spaces in the collection to the agent's toolbox

> [!NOTE]
> Only Spaces will be fetched, so you can feel free to add models and datasets to your collection if you'd
> like for this collection to showcase them.

Example:
```py
>>> from smolagents import ToolCollection, CodeAgent

>>> image_tool_collection = ToolCollection.from_hub("huggingface-tools/diffusion-tools-6630bb19a942c2306a2cdb6f")
>>> agent = CodeAgent(tools=[*image_tool_collection.tools], add_base_tools=True)

>>> agent.run("Please draw me a picture of rivers and lakes.")
```

**Parameters:**

collection_slug (str) : The collection slug referencing the collection.

token (str, *optional*) : The authentication token if the collection is private.

trust_remote_code (bool, *optional*, defaults to False) : Whether to trust the remote code.

**Returns:**

`ToolCollection`

A tool collection instance loaded with the tools.
#### from_mcp[[smolagents.ToolCollection.from_mcp]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/tools.py#L949)

Automatically load a tool collection from an MCP server.

This method supports Stdio, Streamable HTTP, and legacy HTTP+SSE MCP servers. Look at the `server_parameters`
argument for more details on how to connect to each MCP server.

Note: a separate thread will be spawned to run an asyncio event loop handling
the MCP server.

Example with a Stdio MCP server:
```py
>>> import os
>>> from smolagents import ToolCollection, CodeAgent, InferenceClientModel
>>> from mcp import StdioServerParameters

>>> model = InferenceClientModel()

>>> server_parameters = StdioServerParameters(
>>>     command="uvx",
>>>     args=["--quiet", "pubmedmcp@0.1.3"],
>>>     env={"UV_PYTHON": "3.12", **os.environ},
>>> )

>>> with ToolCollection.from_mcp(server_parameters, trust_remote_code=True) as tool_collection:
>>>     agent = CodeAgent(tools=[*tool_collection.tools], add_base_tools=True, model=model)
>>>     agent.run("Please find a remedy for hangover.")
```

Example with structured output enabled:
```py
>>> with ToolCollection.from_mcp(server_parameters, trust_remote_code=True, structured_output=True) as tool_collection:
>>>     agent = CodeAgent(tools=[*tool_collection.tools], add_base_tools=True, model=model)
>>>     agent.run("Please find a remedy for hangover.")
```

Example with a Streamable HTTP MCP server:
```py
>>> with ToolCollection.from_mcp({"url": "http://127.0.0.1:8000/mcp", "transport": "streamable-http"}, trust_remote_code=True) as tool_collection:
>>>     agent = CodeAgent(tools=[*tool_collection.tools], add_base_tools=True, model=model)
>>>     agent.run("Please find a remedy for hangover.")
```

**Parameters:**

server_parameters (`mcp.StdioServerParameters` or `dict`) : Configuration parameters to connect to the MCP server. This can be:  - An instance of `mcp.StdioServerParameters` for connecting a Stdio MCP server via standard input/output using a subprocess.  - A `dict` with at least: - "url": URL of the server. - "transport": Transport protocol to use, one of: - "streamable-http": Streamable HTTP transport (default). - "sse": Legacy HTTP+SSE transport (deprecated).

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to trust the execution of code from tools defined on the MCP server. This option should only be set to `True` if you trust the MCP server, and understand the risks associated with running remote code on your local machine. If set to `False`, loading tools from MCP will fail.

structured_output (`bool`, *optional*, defaults to `False`) : Whether to enable structured output features for MCP tools. If True, enables: - Support for outputSchema in MCP tools - Structured content handling (structuredContent from MCP responses) - JSON parsing fallback for structured data If False, uses the original simple text-only behavior for backwards compatibility.

**Returns:**

`ToolCollection`

A tool collection instance.

## MCP Client[[smolagents.MCPClient]]

#### smolagents.MCPClient[[smolagents.MCPClient]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/mcp_client.py#L33)

Manages the connection to an MCP server and make its tools available to SmolAgents.

Note: tools can only be accessed after the connection has been started with the
`connect()` method, done during the init. If you don't use the context manager
we strongly encourage to use "try ... finally" to ensure the connection is cleaned up.

Example:
```python
# fully managed context manager + stdio
with MCPClient(...) as tools:
    # tools are now available

# context manager + Streamable HTTP transport:
with MCPClient({"url": "http://localhost:8000/mcp", "transport": "streamable-http"}) as tools:
    # tools are now available

# Enable structured output for advanced MCP tools:
with MCPClient(server_parameters, structured_output=True) as tools:
    # tools with structured output support are now available

# manually manage the connection via the mcp_client object:
try:
    mcp_client = MCPClient(...)
    tools = mcp_client.get_tools()

    # use your tools here.
finally:
    mcp_client.disconnect()
```

connectsmolagents.MCPClient.connecthttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/mcp_client.py#L124[]
Connect to the MCP server and initialize the tools.

**Parameters:**

server_parameters (StdioServerParameters | dict[str, Any] | list[StdioServerParameters | dict[str, Any]]) : Configuration parameters to connect to the MCP server. Can be a list if you want to connect multiple MCPs at once.  - An instance of `mcp.StdioServerParameters` for connecting a Stdio MCP server via standard input/output using a subprocess.  - A `dict` with at least: - "url": URL of the server. - "transport": Transport protocol to use, one of: - "streamable-http": Streamable HTTP transport (default). - "sse": Legacy HTTP+SSE transport (deprecated).

adapter_kwargs (dict[str, Any], optional) : Additional keyword arguments to be passed directly to `MCPAdapt`.

structured_output (bool, optional, defaults to False) : Whether to enable structured output features for MCP tools. If True, enables: - Support for outputSchema in MCP tools - Structured content handling (structuredContent from MCP responses) - JSON parsing fallback for structured data If False, uses the original simple text-only behavior for backwards compatibility.
#### disconnect[[smolagents.MCPClient.disconnect]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/mcp_client.py#L128)

Disconnect from the MCP server
#### get_tools[[smolagents.MCPClient.get_tools]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/mcp_client.py#L137)

The SmolAgents tools available from the MCP server.

Note: for now, this always returns the tools available at the creation of the session,
but it will in a future release return also new tools available from the MCP server if
any at call time.

**Returns:**

`list[Tool]`

The SmolAgents tools available from the MCP server.

## Agent Types

Agents can handle any type of object in-between tools; tools, being completely multimodal, can accept and return
text, image, audio, video, among other types. In order to increase compatibility between tools, as well as to
correctly render these returns in ipython (jupyter, colab, ipython notebooks, ...), we implement wrapper classes
around these types.

The wrapped objects should continue behaving as initially; a text object should still behave as a string, an image
object should still behave as a `PIL.Image`.

These types have three specific purposes:

- Calling `to_raw` on the type should return the underlying object
- Calling `to_string` on the type should return the object as a string: that can be the string in case of an `AgentText`
  but will be the path of the serialized version of the object in other instances
- Displaying it in an ipython kernel should display the object correctly

### AgentText[[smolagents.AgentText]]

#### smolagents.AgentText[[smolagents.AgentText]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L62)

Text type returned by the agent. Behaves as a string.

### AgentImage[[smolagents.AgentImage]]

#### smolagents.AgentImage[[smolagents.AgentImage]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L74)

Image type returned by the agent. Behaves as a PIL.Image.Image.

savesmolagents.AgentImage.savehttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L164[{"name": "output_bytes", "val": ""}, {"name": "format", "val": ": str = None"}, {"name": "**params", "val": ""}]- **output_bytes** (bytes) -- The output bytes to save the image to.
- **format** (str) -- The format to use for the output image. The format is the same as in PIL.Image.save.
- ****params** -- Additional parameters to pass to PIL.Image.save.0

Saves the image to a file.

**Parameters:**

output_bytes (bytes) : The output bytes to save the image to.

format (str) : The format to use for the output image. The format is the same as in PIL.Image.save.

- ****params** : Additional parameters to pass to PIL.Image.save.
#### to_raw[[smolagents.AgentImage.to_raw]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L119)

Returns the "raw" version of that object. In the case of an AgentImage, it is a PIL.Image.Image.
#### to_string[[smolagents.AgentImage.to_string]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L136)

Returns the stringified version of that object. In the case of an AgentImage, it is a path to the serialized
version of the image.

### AgentAudio[[smolagents.AgentAudio]]

#### smolagents.AgentAudio[[smolagents.AgentAudio]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L176)

Audio type returned by the agent.

to_rawsmolagents.AgentAudio.to_rawhttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L216[]

Returns the "raw" version of that object. It is a `torch.Tensor` object.
#### to_string[[smolagents.AgentAudio.to_string]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agent_types.py#L237)

Returns the stringified version of that object. In the case of an AgentAudio, it is a path to the serialized
version of the audio.
