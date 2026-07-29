# Agents

Smolagents is an experimental API which is subject to change at any time. Results returned by the agents
can vary as the APIs or underlying models are prone to change.

To learn more about agents and tools make sure to read the [introductory guide](../index). This page
contains the API docs for the underlying classes.

## Agents

Our agents inherit from [MultiStepAgent](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.MultiStepAgent), which means they can act in multiple steps, each step consisting of one thought, then one tool call and execution. Read more in [this conceptual guide](../conceptual_guides/react).

We provide two types of agents, based on the main `Agent` class.
  - [CodeAgent](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.CodeAgent) writes its tool calls in Python code (this is the default).
  - [ToolCallingAgent](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.ToolCallingAgent) writes its tool calls in JSON.

Both require arguments `model` and list of tools `tools` at initialization.

### Classes of agents[[smolagents.MultiStepAgent]]

#### smolagents.MultiStepAgent[[smolagents.MultiStepAgent]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L268)

Agent class that solves the given task step by step, using the ReAct framework:
While the objective is not reached, the agent will perform a cycle of action (given by the LLM) and observation (obtained from the environment).

extract_actionsmolagents.MultiStepAgent.extract_actionhttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L789[{"name": "model_output", "val": ": str"}, {"name": "split_token", "val": ": str"}]- **model_output** (`str`) -- Output of the LLM
- **split_token** (`str`) -- Separator for the action. Should match the example in the system prompt.0

Parse action from the LLM output

**Parameters:**

tools (`list[Tool]`) : [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool)s that the agent can use.

model (`Callable[[list[dict[str, str]]], ChatMessage]`) : Model that will generate the agent's actions.

prompt_templates ([PromptTemplates](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.PromptTemplates), *optional*) : Prompt templates.

instructions (`str`, *optional*) : Custom instructions for the agent, will be inserted in the system prompt.

max_steps (`int`, default `20`) : Maximum number of steps the agent can take to solve the task.

add_base_tools (`bool`, default `False`) : Whether to add the base tools to the agent's tools.

verbosity_level (`LogLevel`, default `LogLevel.INFO`) : Level of verbosity of the agent's logs.

managed_agents (`list`, *optional*) : Managed agents that the agent can call.

step_callbacks (`list[Callable]` | `dict[Type[MemoryStep], Callable | list[Callable]]`, *optional*) : Callbacks that will be called at each step.

planning_interval (`int`, *optional*) : Interval at which the agent will run a planning step.

name (`str`, *optional*) : Necessary for a managed agent only - the name by which this agent can be called.

description (`str`, *optional*) : Necessary for a managed agent only - the description of this agent.

provide_run_summary (`bool`, *optional*) : Whether to provide a run summary when called as a managed agent.

final_answer_checks (`list[Callable]`, *optional*) : List of validation functions to run before accepting a final answer. Each function should: - Take the final answer, the agent's memory, and the agent itself as arguments. - Return a boolean indicating whether the final answer is valid.

return_full_result (`bool`, default `False`) : Whether to return the full `RunResult` object or just the final answer output from the agent run.
#### from_dict[[smolagents.MultiStepAgent.from_dict]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1010)

Create agent from a dictionary representation.

**Parameters:**

agent_dict (`dict[str, Any]`) : Dictionary representation of the agent.

- ****kwargs** : Additional keyword arguments that will override agent_dict values.

**Returns:**

``MultiStepAgent``

Instance of the agent class.
#### from_folder[[smolagents.MultiStepAgent.from_folder]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1118)

Loads an agent from a local folder.

**Parameters:**

folder (`str` or `Path`) : The folder where the agent is saved.

- ****kwargs** : Additional keyword arguments that will be passed to the agent's init.
#### from_hub[[smolagents.MultiStepAgent.from_hub]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1064)

Loads an agent defined on the Hub.

Loading a tool from the Hub means that you'll download the tool and execute it locally.
ALWAYS inspect the tool you're downloading before loading it within your runtime, as you would do when
installing a package using pip/npm/apt.

**Parameters:**

repo_id (`str`) : The name of the repo on the Hub where your tool is defined.

token (`str`, *optional*) : The token to identify you on hf.co. If unset, will use the token generated when running `huggingface-cli login` (stored in `~/.huggingface`).

trust_remote_code(`bool`, *optional*, defaults to False) : This flags marks that you understand the risk of running remote code and that you trust this tool. If not setting this to True, loading the tool from Hub will fail.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments that will be split in two: all arguments relevant to the Hub (such as `cache_dir`, `revision`, `subfolder`) will be used when downloading the files for your agent, and the others will be passed along to its init.
#### initialize_system_prompt[[smolagents.MultiStepAgent.initialize_system_prompt]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L749)

To be implemented in child classes
#### interrupt[[smolagents.MultiStepAgent.interrupt]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L754)

Interrupts the agent execution.
#### provide_final_answer[[smolagents.MultiStepAgent.provide_final_answer]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L810)

Provide the final answer to the task, based on the logs of the agent's interactions.

**Parameters:**

task (`str`) : Task to perform.

images (`list[PIL.Image.Image]`, *optional*) : Image(s) objects.

**Returns:**

``str``

Final answer to the task.
#### push_to_hub[[smolagents.MultiStepAgent.push_to_hub]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1160)

Upload the agent to the Hub.

**Parameters:**

repo_id (`str`) : The name of the repository you want to push to. It should contain your organization name when pushing to a given organization.

commit_message (`str`, *optional*, defaults to `"Upload agent"`) : Message to commit while pushing.

private (`bool`, *optional*, defaults to `None`) : Whether to make the repo private. If `None`, the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`bool` or `str`, *optional*) : The token to use as HTTP bearer authorization for remote files. If unset, will use the token generated when running `huggingface-cli login` (stored in `~/.huggingface`).

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.
#### replay[[smolagents.MultiStepAgent.replay]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L859)

Prints a pretty replay of the agent's steps.

**Parameters:**

detailed (bool, optional) : If True, also displays the memory at each step. Defaults to False. Careful: will increase log length exponentially. Use only for debugging.
#### run[[smolagents.MultiStepAgent.run]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L436)

Run the agent for the given task.

Example:
```py
from smolagents import CodeAgent
agent = CodeAgent(tools=[])
agent.run("What is the result of 2 power 3.7384?")
```

**Parameters:**

task (`str`) : Task to perform.

stream (`bool`) : Whether to run in streaming mode. If `True`, returns a generator that yields each step as it is executed. You must iterate over this generator to process the individual steps (e.g., using a for loop or `next()`). If `False`, executes all steps internally and returns only the final answer after completion.

reset (`bool`) : Whether to reset the conversation or keep it going from previous run.

images (`list[PIL.Image.Image]`, *optional*) : Image(s) objects.

additional_args (`dict`, *optional*) : Any other variables that you want to pass to the agent run, for instance images or dataframes. Give them clear names!

max_steps (`int`, *optional*) : Maximum number of steps the agent can take to solve the task. if not provided, will use the agent's default value.

return_full_result (`bool`, *optional*) : Whether to return the full `RunResult` object or just the final answer output. If `None` (default), the agent's `self.return_full_result` setting is used.
#### save[[smolagents.MultiStepAgent.save]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L892)

Saves the relevant code files for your agent. This will copy the code of your agent in `output_dir` as well as autogenerate:

- a `tools` folder containing the logic for each of the tools under `tools/{tool_name}.py`.
- a `managed_agents` folder containing the logic for each of the managed agents.
- an `agent.json` file containing a dictionary representing your agent.
- a `prompt.yaml` file containing the prompt templates used by your agent.
- an `app.py` file providing a UI for your agent when it is exported to a Space with `agent.push_to_hub()`
- a `requirements.txt` containing the names of the modules used by your tool (as detected when inspecting its
  code)

**Parameters:**

output_dir (`str` or `Path`) : The folder in which you want to save your agent.
#### step[[smolagents.MultiStepAgent.step]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L782)

Perform one step in the ReAct framework: the agent thinks, acts, and observes the result.
Returns either None if the step is not final, or the final answer.
#### to_dict[[smolagents.MultiStepAgent.to_dict]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L970)

Convert the agent to a dictionary representation.

**Returns:**

``dict``

Dictionary representation of the agent.
#### visualize[[smolagents.MultiStepAgent.visualize]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L855)

Creates a rich tree visualization of the agent's structure.
#### write_memory_to_messages[[smolagents.MultiStepAgent.write_memory_to_messages]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L758)

Reads past llm_outputs, actions, and observations or errors from the memory into a series of messages
that can be used as input to the LLM. Adds a number of keywords (such as PLAN, error, etc) to help
the LLM.

#### smolagents.CodeAgent[[smolagents.CodeAgent]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1505)

In this agent, the tool calls will be formulated by the LLM in code format, then parsed and executed.

cleanupsmolagents.CodeAgent.cleanuphttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1593[]
Clean up resources used by the agent, such as the remote Python executor.

**Parameters:**

tools (`list[Tool]`) : [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool)s that the agent can use.

model (`Model`) : Model that will generate the agent's actions.

prompt_templates ([PromptTemplates](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.PromptTemplates), *optional*) : Prompt templates.

additional_authorized_imports (`list[str]`, *optional*) : Additional authorized imports for the agent.

planning_interval (`int`, *optional*) : Interval at which the agent will run a planning step.

executor ([PythonExecutor](/docs/smolagents/v1.26.0/en/reference/python_executors#smolagents.PythonExecutor), *optional*) : Custom Python code executor. If not provided, a default executor will be created based on `executor_type`.

executor_type (`Literal["local", "blaxel", "e2b", "modal", "docker"]`, default `"local"`) : Type of code executor.

executor_kwargs (`dict`, *optional*) : Additional arguments to pass to initialize the executor.

max_print_outputs_length (`int`, *optional*) : Maximum length of the print outputs.

stream_outputs (`bool`, *optional*, default `False`) : Whether to stream outputs during execution.

use_structured_outputs_internally (`bool`, default `False`) : Whether to use structured generation at each action step: improves performance for many models.  

code_block_tags (`tuple[str, str]` | `Literal["markdown"]`, *optional*) : Opening and closing tags for code blocks (regex strings). Pass a custom tuple, or pass 'markdown' to use ("```(?:python|py)", "\n```"), leave empty to use ("", "").

- ****kwargs** : Additional keyword arguments.
#### from_dict[[smolagents.CodeAgent.from_dict]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1779)

Create CodeAgent from a dictionary representation.

**Parameters:**

agent_dict (`dict[str, Any]`) : Dictionary representation of the agent.

- ****kwargs** : Additional keyword arguments that will override agent_dict values.

**Returns:**

``CodeAgent``

Instance of the CodeAgent class.

#### smolagents.ToolCallingAgent[[smolagents.ToolCallingAgent]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1215)

This agent uses JSON-like tool calls, using method `model.get_tool_call` to leverage the LLM engine's tool calling capabilities.

execute_tool_callsmolagents.ToolCallingAgent.execute_tool_callhttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1453[{"name": "tool_name", "val": ": str"}, {"name": "arguments", "val": ": dict[str, str] | str"}]- **tool_name** (`str`) -- Name of the tool or managed agent to execute.
- **arguments** (dict[str, str] | str) -- Arguments passed to the tool call.0

Execute a tool or managed agent with the provided arguments.

The arguments are replaced with the actual values from the state if they refer to state variables.

**Parameters:**

tools (`list[Tool]`) : [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool)s that the agent can use.

model (`Model`) : Model that will generate the agent's actions.

prompt_templates ([PromptTemplates](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.PromptTemplates), *optional*) : Prompt templates.

planning_interval (`int`, *optional*) : Interval at which the agent will run a planning step.

stream_outputs (`bool`, *optional*, default `False`) : Whether to stream outputs during execution.

max_tool_threads (`int`, *optional*) : Maximum number of threads for parallel tool calls. Higher values increase concurrency but resource usage as well. Defaults to `ThreadPoolExecutor`'s default.

- ****kwargs** : Additional keyword arguments.
#### process_tool_calls[[smolagents.ToolCallingAgent.process_tool_calls]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L1361)

Process tool calls from the model output and update agent memory.

**Parameters:**

chat_message (`ChatMessage`) : Chat message containing tool calls from the model.

memory_step (`ActionStep)` : Memory ActionStep to update with results.

### stream_to_gradio[[smolagents.stream_to_gradio]]

#### smolagents.stream_to_gradio[[smolagents.stream_to_gradio]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/gradio_ui.py#L248)

Runs an agent with the given task and streams the messages from the agent as gradio ChatMessages.

### GradioUI[[smolagents.GradioUI]]

> [!TIP]
> You must have `gradio` installed to use the UI. Please run `pip install 'smolagents[gradio]'` if it's not the case.

#### smolagents.GradioUI[[smolagents.GradioUI]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/gradio_ui.py#L279)

Gradio interface for interacting with a [MultiStepAgent](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.MultiStepAgent).

This class provides a web interface to interact with the agent in real-time, allowing users to submit prompts, upload files, and receive responses in a chat-like format.
It uses the modern `gradio.ChatInterface` component for a native chatbot experience.
It can reset the agent's memory at the start of each interaction if desired.
It supports file uploads via multimodal input.
This class requires the `gradio` extra to be installed: `pip install 'smolagents[gradio]'`.

Example:
```python
from smolagents import CodeAgent, GradioUI, InferenceClientModel

model = InferenceClientModel(model_id="meta-llama/Meta-Llama-3.1-8B-Instruct")
agent = CodeAgent(tools=[], model=model)
gradio_ui = GradioUI(agent, file_upload_folder="uploads", reset_agent_memory=True)
gradio_ui.launch()
```

launchsmolagents.GradioUI.launchhttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/gradio_ui.py#L422[{"name": "share", "val": ": bool = True"}, {"name": "**kwargs", "val": ""}]- **share** (`bool`, defaults to `True`) -- Whether to share the app publicly.
- ****kwargs** -- Additional keyword arguments to pass to the Gradio launch method.0

Launch the Gradio app with the agent interface.

**Parameters:**

agent ([MultiStepAgent](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.MultiStepAgent)) : The agent to interact with.

file_upload_folder (`str`, *optional*) : The folder where uploaded files will be saved. If not provided, file uploads are disabled.

reset_agent_memory (`bool`, *optional*, defaults to `False`) : Whether to reset the agent's memory at the start of each interaction. If `True`, the agent will not remember previous interactions.
#### upload_file[[smolagents.GradioUI.upload_file]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/gradio_ui.py#L335)

Handle file upload with validation.

**Parameters:**

file : The uploaded file object.

file_uploads_log : List to track uploaded files.

allowed_file_types : List of allowed extensions. Defaults to [".pdf", ".docx", ".txt"].

**Returns:**

Tuple of (status textbox, updated file log).

## Prompts[[smolagents.PromptTemplates]]

#### smolagents.PromptTemplates[[smolagents.PromptTemplates]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L166)

Prompt templates for the agent.

**Parameters:**

system_prompt (`str`) : System prompt.

planning ([PlanningPromptTemplate](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.PlanningPromptTemplate)) : Planning prompt templates.

managed_agent ([ManagedAgentPromptTemplate](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.ManagedAgentPromptTemplate)) : Managed agent prompt templates.

final_answer ([FinalAnswerPromptTemplate](/docs/smolagents/v1.26.0/en/reference/agents#smolagents.FinalAnswerPromptTemplate)) : Final answer prompt templates.

#### smolagents.PlanningPromptTemplate[[smolagents.PlanningPromptTemplate]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L125)

Prompt templates for the planning step.

**Parameters:**

plan (`str`) : Initial plan prompt.

update_plan_pre_messages (`str`) : Update plan pre-messages prompt.

update_plan_post_messages (`str`) : Update plan post-messages prompt.

#### smolagents.ManagedAgentPromptTemplate[[smolagents.ManagedAgentPromptTemplate]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L140)

Prompt templates for the managed agent.

**Parameters:**

task (`str`) : Task prompt.

report (`str`) : Report prompt.

#### smolagents.FinalAnswerPromptTemplate[[smolagents.FinalAnswerPromptTemplate]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/agents.py#L153)

Prompt templates for the final answer.

**Parameters:**

pre_messages (`str`) : Pre-messages prompt.

post_messages (`str`) : Post-messages prompt.

## Memory[[smolagents.AgentMemory]]

Smolagents use memory to store information across multiple steps.

#### smolagents.AgentMemory[[smolagents.AgentMemory]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L214)

Memory for the agent, containing the system prompt and all steps taken by the agent.

This class is used to store the agent's steps, including tasks, actions, and planning steps.
It allows for resetting the memory, retrieving succinct or full step information, and replaying the agent's steps.

**Attributes**:
- **system_prompt** (`SystemPromptStep`) -- System prompt step for the agent.
- **steps** (`list[TaskStep | ActionStep | PlanningStep]`) -- List of steps taken by the agent, which can include tasks, actions, and planning steps.

get_full_stepssmolagents.AgentMemory.get_full_stepshttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L242[]
Return a full representation of the agent's steps, including model input messages.

**Parameters:**

system_prompt (`str`) : System prompt for the agent, which sets the context and instructions for the agent's behavior.
#### get_succinct_steps[[smolagents.AgentMemory.get_succinct_steps]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L236)

Return a succinct representation of the agent's steps, excluding model input messages.
#### replay[[smolagents.AgentMemory.replay]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L248)

Prints a pretty replay of the agent's steps.

**Parameters:**

logger (`AgentLogger`) : The logger to print replay logs to.

detailed (`bool`, default `False`) : If True, also displays the memory at each step. Defaults to False. Careful: will increase log length exponentially. Use only for debugging.
#### reset[[smolagents.AgentMemory.reset]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L232)

Reset the agent's memory, clearing all steps and keeping the system prompt.
#### return_full_code[[smolagents.AgentMemory.return_full_code]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/memory.py#L273)

Returns all code actions from the agent's steps, concatenated as a single script.
