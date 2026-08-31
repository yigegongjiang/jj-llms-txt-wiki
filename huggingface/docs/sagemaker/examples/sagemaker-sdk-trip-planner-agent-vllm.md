# Build a reasoning trip-planning agent on Amazon SageMaker AI with Hugging Face vLLM

Written by Dario SalvatiLast updated 2026-08-31

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/trip-planner-agent-vllm/cover.png)

In this notebook, we'll deploy [`Qwen/Qwen3-8B`](https://huggingface.co/Qwen/Qwen3-8B) with the Hugging Face **vLLM** Deep Learning Container (DLC) on Amazon SageMaker, and use it to build a **trip-planning agent**: an assistant that reasons through multi-constraint travel requests and calls tools to check the weather, convert currencies, and find places to visit.

We'll walk through the following steps:

- Select a reasoning and tool-calling model and a SageMaker DLC for your use case
- Deploy the model to SageMaker with the SageMaker Python SDK `ModelBuilder`
- Configure vLLM for reasoning and tool calling
- Build an agent with [Strands Agents](https://strandsagents.com/) and give it custom tools
- Hold a multi-turn conversation where the agent reasons, calls tools, and remembers context
- Optionally connect a Hugging Face MCP server and launch a Gradio app
- Clean up the endpoint resources to avoid ongoing charges

For this example, you'll need AWS credentials and a SageMaker execution role. Qwen3-8B is a public model, so a Hugging Face token is optional here. Authenticated requests get more generous rate limits and faster downloads, and a token is required if you switch to a gated model.

## How an agent works

An agent is a loop around a language model. The model does not just answer in one shot; it decides, step by step, what information it still needs and how to get it.

Two capabilities make this possible:

- **Reasoning**: the model thinks through the problem before answering, which helps with multi-step requests such as planning a trip under several constraints.
- **Tool calling**: instead of guessing a fact, the model can call a function you provide (check the weather, convert a currency, look up places) and continue with the real result.

Put together, the loop looks like this: the user asks a question, the model reasons about it and requests a tool, your code runs the tool and returns the result, and the model reasons again with that new information. This repeats until the model has everything it needs to give a final answer.

[`Qwen/Qwen3-8B`](https://huggingface.co/Qwen/Qwen3-8B) supports both a thinking mode and native tool calling, so it can drive this loop on its own. We'll deploy it, then let [Strands Agents](https://strandsagents.com/) run the loop for us.

## Hugging Face vLLM and Strands Agents

[vLLM](https://github.com/vllm-project/vllm) is a high-throughput inference engine for large language models. The Hugging Face **vLLM** DLC packages it for SageMaker with current `transformers` and `huggingface_hub`, and exposes an OpenAI-compatible API (including tool calling and reasoning parsing) through simple environment variables. Its multimodal sibling, vLLM-Omni, serves tasks like text-to-speech or image generation; here we serve a text model, so we use the plain vLLM image.

[Strands Agents](https://strandsagents.com/) is a lightweight agent framework with a native SageMaker integration. Instead of hand-writing the tool-calling loop, we point Strands at our endpoint and it orchestrates the reasoning, tool calls, and results for us.

## Setup

To run this example, we'll install the SageMaker Python SDK for model deployment, `huggingface_hub` for authentication, `strands-agents` (with its SageMaker integration) for the agent, and `gradio` for the optional interactive app.

```python
%pip install -q "sagemaker>=3" huggingface_hub "strands-agents[sagemaker]>=1.48.0" "gradio>=5"
```

We are going to need:

- An `HF_TOKEN`: used to download the model from Hugging Face. Optional for the public Qwen3-8B; authenticated requests get more generous rate limits and faster downloads (and a token is required for gated models).
- A SageMaker execution role: used to pull the DLC from ECR and deploy the model to SageMaker.

Let's start by setting up the token and the execution role.

```python
from huggingface_hub import get_token

HF_TOKEN = get_token()

if HF_TOKEN:
    print("HF_TOKEN loaded")
else:
    print(
        "No HF_TOKEN found. Public Qwen3-8B downloads still work. Run "
        "huggingface_hub.notebook_login() to authenticate for higher rate "
        "limits or to access gated models."
    )
```

**Production tip:** avoid pasting long-lived tokens into notebooks or committing them. For production, keep the token in a vault such as [AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html) and load it at deploy time:

```python
import json
import boto3

secret = boto3.client("secretsmanager").get_secret_value(SecretId="hf-token")
HF_TOKEN = json.loads(secret["SecretString"])["HF_TOKEN"]
```

To avoid handing the token to the endpoint at all, pre-stage the model weights in S3 and point the container at them, so no Hub token is needed at runtime.

```python
import os

import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

REGION = boto3.Session().region_name or os.environ.get("AWS_REGION", "us-east-1")
boto_sess = boto3.Session(region_name=REGION)
sess = Session(boto_session=boto_sess)

try:
    role = get_execution_role(sagemaker_session=sess)
    print(f"Role extracted from execution role: {role}")
except Exception:
    role_name = "sagemaker_execution_role"
    iam_client = boto_sess.client("iam")
    role = iam_client.get_role(RoleName=role_name)["Role"]["Arn"]
    print(f"Role extracted from iam client: {role}")
```

## Choosing a model and a DLC

There are many open LLMs on the Hugging Face Hub. For an agent we want strong instruction-following, reasoning, and tool-calling, while staying small enough to run on a single GPU.

We'll use [`Qwen/Qwen3-8B`](https://huggingface.co/Qwen/Qwen3-8B) and serve it with the Hugging Face **vLLM** DLC. Rather than hardcoding the container URI, we resolve it with the SageMaker SDK's `image_uris.retrieve` helper, which returns the right image for the chosen region and instance. You can also browse the available images on the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page.

We'll target an `ml.g5.xlarge` instance (a single NVIDIA A10G 24GB GPU). For higher concurrency or larger models, consider larger GPU instance types such as `ml.g5.2xlarge` or `ml.g6e.*`.

```python
from time import strftime

from sagemaker.core.image_uris import retrieve

MODEL_ID = "Qwen/Qwen3-8B"
INSTANCE_TYPE = "ml.g5.xlarge"
IMAGE_URI = retrieve(
    "huggingface-vllm",
    region=REGION,
    image_scope="inference",
    instance_type=INSTANCE_TYPE,
)

RESOURCE_SUFFIX = strftime("%Y%m%d-%H%M%S")
MODEL_NAME = f"trip-planner-agent-model-{RESOURCE_SUFFIX}"
ENDPOINT_NAME = f"trip-planner-agent-endpoint-{RESOURCE_SUFFIX}"

print(IMAGE_URI)
```

## Configuring vLLM for reasoning and tool calling

The vLLM container is configured entirely through environment variables. Any vLLM server flag can be passed by uppercasing it, replacing dashes with underscores, and prefixing it with `SM_VLLM_` (for example `--max-model-len` becomes `SM_VLLM_MAX_MODEL_LEN`).

Three of these variables are what turn a plain text model into an agent backend:

- `SM_VLLM_ENABLE_AUTO_TOOL_CHOICE=true` lets the model decide when to call a tool.
- `SM_VLLM_TOOL_CALL_PARSER=hermes` parses Qwen3's tool calls into the OpenAI format.
- `SM_VLLM_REASONING_PARSER=qwen3` separates the model's thinking from its final answer.

We also set `SM_VLLM_HOST=0.0.0.0`, which is required so the container passes the SageMaker health check. `ModelBuilder` sets `HF_MODEL_ID` for us, so we don't repeat the model id here.

```python
env_vars = {
    "SM_VLLM_HOST": "0.0.0.0",  # Bind to all interfaces so the health check passes
    "SM_VLLM_MAX_MODEL_LEN": "16384",  # Context length; bounds KV cache on a single A10G
    "SM_VLLM_GPU_MEMORY_UTILIZATION": "0.9",
    "SM_VLLM_ENABLE_AUTO_TOOL_CHOICE": "true",  # Let the model call tools
    "SM_VLLM_TOOL_CALL_PARSER": "hermes",  # Parse Qwen3 tool calls
    "SM_VLLM_REASONING_PARSER": "qwen3",  # Separate reasoning from the answer
}

# Only pass the token when present; SageMaker rejects non-string env values.
if HF_TOKEN:
    env_vars["HF_TOKEN"] = HF_TOKEN
```

## Deploy the model

The SageMaker Python SDK v3 deploys models with `ModelBuilder`: we describe the model, the serving container, and the instance, then call `build` and `deploy`. There is no need to assemble model, endpoint-config, and endpoint resources by hand.

One detail worth calling out: this image is built on CUDA 13 (`cu130` in the tag). vLLM images with CUDA 13 or newer require the `al2-ami-sagemaker-inference-gpu-3-1` inference AMI, which we pass to `deploy`. Without it the container fails to start before any logs are produced.

```python
from sagemaker.serve import ModelBuilder, ModelServer

model_builder = ModelBuilder(
    model=MODEL_ID,
    role_arn=role,
    sagemaker_session=sess,
    instance_type=INSTANCE_TYPE,
    image_uri=IMAGE_URI,
    model_server=ModelServer.VLLM,
    env_vars=env_vars,
)

built_model = model_builder.build(model_name=MODEL_NAME)

model_builder.deploy(
    endpoint_name=ENDPOINT_NAME,
    initial_instance_count=1,
    instance_type=INSTANCE_TYPE,
    inference_ami_version="al2-ami-sagemaker-inference-gpu-3-1",
    container_timeout_in_seconds=900,
    wait=True,
)
```

## A first look: reasoning and tool calls

Before bringing in the agent framework, let's send a single request straight to the endpoint to see what the model returns. SageMaker requests go through the `/invocations` route; we use `CustomAttributes` to forward them to vLLM's OpenAI-compatible Chat Completions API.

We pass a `get_weather` tool definition and let the model decide whether to call it (`tool_choice="auto"`). The response separates the model's reasoning (`reasoning_content`) from any tool calls it wants to make (`tool_calls`).

```python
import json

runtime = boto_sess.client("sagemaker-runtime")

weather_tool = {
    "type": "function",
    "function": {
        "name": "get_weather",
        "description": "Get the weather forecast for a city on a given date.",
        "parameters": {
            "type": "object",
            "properties": {
                "city": {"type": "string", "description": "City name."},
                "date": {"type": "string", "description": "Date in YYYY-MM-DD format."},
            },
            "required": ["city", "date"],
        },
    },
}

response = runtime.invoke_endpoint(
    EndpointName=ENDPOINT_NAME,
    ContentType="application/json",
    Body=json.dumps(
        {
            "model": MODEL_ID,
            "messages": [{"role": "user", "content": "What should I pack for Lisbon on 2026-10-12?"}],
            "tools": [weather_tool],
            "tool_choice": "auto",
        }
    ),
    CustomAttributes="route=/v1/chat/completions",
)

message = json.loads(response["Body"].read())["choices"][0]["message"]
print("Reasoning:\n", message.get("reasoning_content"), "\n")
print("Tool calls:\n", json.dumps(message.get("tool_calls"), indent=2))
```

## Building the agent

We describe the endpoint once with `SageMakerAIModel`, hand the agent our tools, and from then on a single call runs the full reason-and-act loop until the model produces an answer.

Two `payload_config` choices matter here:

- `stream=True`: a reasoning model can generate for a while. Streaming returns tokens as they are produced instead of waiting for the whole answer, which keeps the response flowing and avoids the real-time endpoint timing out on longer generations.
- `chat_template_kwargs={"enable_thinking": True}`: keeps Qwen3 in thinking mode.

The provider logs the full request and response (and every streamed chunk) at `INFO`, so we first raise the log level to `WARNING` to keep the agent's output readable.

```python
import logging

# The SageMaker model provider logs the full request, every streamed chunk, and
# the final response at INFO level. Raise the level to keep the output readable.
logging.getLogger("strands").setLevel(logging.WARNING)

from strands.models.sagemaker import SageMakerAIModel

sagemaker_model = SageMakerAIModel(
    endpoint_config={"endpoint_name": ENDPOINT_NAME, "region_name": REGION},
    payload_config={
        "max_tokens": 8192,
        "temperature": 0.7,
        "top_p": 0.95,
        "stream": True,
        "additional_args": {"chat_template_kwargs": {"enable_thinking": True}},
    },
)
```

### Giving the agent tools

Strands turns any Python function into a tool with the `@tool` decorator: it reads the type hints and docstring to build the schema the model sees. Our concierge gets three tools it can combine to plan a trip.

To keep the notebook self-contained and reproducible, these are deterministic mocks. Swap in real APIs (or a built-in like `strands_tools.http_request`) when you adapt this to production; the agent code does not change.

```python
from strands import tool

@tool
def get_weather(city: str, date: str) -> str:
    """Get the weather forecast for a city on a given date.

    Args:
        city: City to look up.
        date: Date in YYYY-MM-DD format.
    """
    return f"{city} on {date}: sunny, 22°C during the day and 14°C at night."

@tool
def convert_currency(amount: float, from_currency: str, to_currency: str) -> str:
    """Convert an amount between two currencies.

    Args:
        amount: Amount to convert.
        from_currency: ISO code to convert from, e.g. EUR.
        to_currency: ISO code to convert to, e.g. USD.
    """
    per_eur = {"EUR": 1.0, "USD": 1.08, "GBP": 0.85, "JPY": 170.0}
    in_eur = amount / per_eur[from_currency.upper()]
    converted = in_eur * per_eur[to_currency.upper()]
    return f"{amount:.2f} {from_currency.upper()} = {converted:.2f} {to_currency.upper()}"

@tool
def search_places(city: str, category: str) -> str:
    """Find points of interest in a city by category.

    Args:
        city: City to search in.
        category: Kind of place, e.g. "museum", "restaurant", or "park".
    """
    catalog = {
        "museum": ["National Tile Museum", "Berardo Collection", "MAAT"],
        "restaurant": ["Time Out Market", "Cervejaria Ramiro", "A Cevicheria"],
        "park": ["Eduardo VII Park", "Monsanto Forest Park", "Jardim da Estrela"],
    }
    found = catalog.get(category.lower(), ["City-center walking tour"])
    return f"{category.title()} options in {city}: " + ", ".join(found)
```

### Running the agent

Now we send a request that no single tool can answer on its own. The agent has to reason about what it needs, call several tools, and combine the results into an itinerary.

The `run_agent` helper below sends a prompt and then prints two things: the tool calls the agent chose to make (so we can see it acting), followed by the final answer. It reads them from `agent.messages`, the running conversation history, so the printout stays clean while the verbose provider logs remain off.

```python
import json

from strands import Agent

agent = Agent(
    model=sagemaker_model,
    tools=[get_weather, convert_currency, search_places],
    callback_handler=None,
)

def run_agent(prompt: str):
    """Run the agent, print the tool calls it made, then the final answer."""
    start = len(agent.messages)
    result = agent(prompt)

    for message in agent.messages[start:]:
        for block in message.get("content", []):
            if "toolUse" in block:
                call = block["toolUse"]
                print(f"[tool call] {call['name']}({json.dumps(call['input'])})")

    print()
    print(result)
    return result

result = run_agent(
    "Plan a 3-day trip to Lisbon starting 2026-10-12. My budget is 800 EUR, "
    "so tell me what that is in USD. Suggest a few museums and restaurants, "
    "and tell me what to pack based on the weather."
)
```

### Following up in the same conversation

A Strands `Agent` keeps the conversation history, so it behaves like a real assistant across turns. We can ask a follow-up and it reuses everything it already worked out, calling tools again only where the change requires it.

```python
follow_up = run_agent(
    "Actually, change the destination to Porto but keep the same budget and dates. "
    "Update the packing tips if the weather is different."
)
```

## Optional: connect the Hugging Face MCP server

[Model Context Protocol (MCP)](https://huggingface.co/docs/hub/en/hf-mcp-server) servers expose ready-made tools an agent can call. Strands connects to them with `MCPClient`, so you can, for example, give the concierge access to the Hugging Face Hub without writing any new tools.

This section is optional. Set `ENABLE_MCP = True` to try it. The Hugging Face MCP server requires authentication, so make sure `HF_TOKEN` is set first. MCP tools must be used inside the client's context manager.

```python
ENABLE_MCP = False

if ENABLE_MCP:
    if not HF_TOKEN:
        raise ValueError(
            "The Hugging Face MCP server requires authentication. Set a token with "
            "huggingface_hub.notebook_login() before enabling MCP."
        )

    from mcp.client.streamable_http import streamablehttp_client
    from strands.tools.mcp import MCPClient

    hf_mcp = MCPClient(
        lambda: streamablehttp_client(
            "https://huggingface.co/mcp",
            headers={"Authorization": f"Bearer {HF_TOKEN}"},
        )
    )
    with hf_mcp:
        mcp_agent = Agent(
            model=sagemaker_model,
            tools=[get_weather, convert_currency, search_places, *hf_mcp.list_tools_sync()],
            callback_handler=None,
        )
        print(mcp_agent("Find a small Qwen model with tool-calling support and include the Hub links."))
```

## Optional: an interactive chat app

For a nicer experience than a notebook cell, we can wrap the agent in a small [Gradio](https://www.gradio.app/) chat app. It streams the agent's events as they happen, rendering the reasoning, each tool call, its result, and the final answer as separate, collapsible blocks.

This section is optional. Set `ENABLE_GRADIO = True` to launch the app.

**Security and cost warning:** `share=True` creates a public Gradio URL. Anyone with that URL can chat with the app, which invokes the already-deployed billable SageMaker endpoint using your AWS credentials. Share the URL only with trusted users, and disable the app or delete the endpoint when you are finished.

```python
import gradio as gr

# A dedicated agent for the app, so the chat starts fresh instead of inheriting
# the conversation from the demo cells above.
chat_agent = Agent(
    model=sagemaker_model,
    tools=[get_weather, convert_currency, search_places],
    callback_handler=None,
)

async def chat(message, history):
    # Gradio passes an empty history at the start of a conversation (including
    # after the "Clear" button), which is our cue to reset the agent's memory.
    if not history:
        chat_agent.messages.clear()

    turn = []
    kind = None  # "reasoning" | "tool" | "text" for the current block
    tool_name = "tool"

    async for event in chat_agent.stream_async(message):
        if "event" in event:
            raw = event["event"]

            # A tool block announces its name at the start.
            tool_use = raw.get("contentBlockStart", {}).get("start", {}).get("toolUse")
            if tool_use:
                tool_name = tool_use.get("name", "tool")
                kind = None  # force a fresh block for this tool call

            delta = raw.get("contentBlockDelta", {}).get("delta")
            if not delta:
                continue

            # Classify the chunk: reasoning, tool arguments, or final answer.
            if "reasoningContent" in delta:
                piece, this, meta = delta["reasoningContent"].get("text", ""), "reasoning", {"title": "💭 Thinking"}
            elif "toolUse" in delta:
                piece, this, meta = delta["toolUse"].get("input", ""), "tool", {"title": f"🛠️ Using tool `{tool_name}`"}
            else:
                piece, this, meta = delta.get("text", ""), "text", None

            if this != kind:
                kind = this
                turn.append(gr.ChatMessage(role="assistant", content="", metadata=meta))
            turn[-1].content += piece
            yield turn

        elif "message" in event:
            msg = event["message"]
            if msg.get("role") != "user":
                continue
            for block in msg.get("content", []):
                result = block.get("toolResult")
                if not result:
                    continue
                text = "\n".join(item.get("text", "") for item in result.get("content", []) if "text" in item).strip()
                if text:
                    status = result.get("status", "success").upper()
                    turn.append(
                        gr.ChatMessage(
                            role="assistant",
                            content=text,
                            metadata={"title": f"✅ Tool result [{status}]"},
                        )
                    )
                    kind = None
                    yield turn

    yield turn

ENABLE_GRADIO = False

if ENABLE_GRADIO:
    demo = gr.ChatInterface(
        fn=chat,
        title="Trip-planning concierge",
        description="Ask for help planning a trip. The agent reasons, calls tools, and answers.",
        examples=[
            "Plan a 2-day trip to Lisbon on 2026-10-12 with a 500 EUR budget in USD.",
            "What museums should I visit in Lisbon, and what's the weather on 2026-10-13?",
        ],
    )
    demo.launch(share=True, server_name="0.0.0.0", server_port=7860, show_error=True)
```

## Cleanup

SageMaker endpoints are billed while they are `InService`, so delete the endpoint and its associated resources when you're done.

```python
sm = boto_sess.client("sagemaker")
endpoint_config_name = sm.describe_endpoint(EndpointName=ENDPOINT_NAME)["EndpointConfigName"]

# delete_endpoint is asynchronous; wait for it to finish before removing the
# config and model, which stay in use while the endpoint is still deleting.
sm.delete_endpoint(EndpointName=ENDPOINT_NAME)
sm.get_waiter("endpoint_deleted").wait(EndpointName=ENDPOINT_NAME)

sm.delete_endpoint_config(EndpointConfigName=endpoint_config_name)
sm.delete_model(ModelName=built_model.model_name)
```

## Conclusion and references

We deployed [`Qwen/Qwen3-8B`](https://huggingface.co/Qwen/Qwen3-8B) on the Hugging Face vLLM DLC with the SageMaker SDK `ModelBuilder`, configured vLLM for reasoning and tool calling, and built a trip-planning agent with Strands Agents that reasons, calls tools, and holds a multi-turn conversation. The same pattern works for other tool-calling models on the Hub: swap the model id, adjust the instance, and register your own `@tool` functions.

References:

- [Qwen/Qwen3-8B](https://huggingface.co/Qwen/Qwen3-8B)
- [Hugging Face on Amazon SageMaker](https://huggingface.co/docs/sagemaker/en/index) and the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) list
- [vLLM documentation](https://docs.vllm.ai/) and [tool calling](https://docs.vllm.ai/en/latest/features/tool_calling.html)
- [Strands Agents documentation](https://strandsagents.com/)
- [Amazon SageMaker Python SDK](https://sagemaker.readthedocs.io/)
- [Model Context Protocol on the Hugging Face Hub](https://huggingface.co/docs/hub/en/hf-mcp-server)

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/trip-planner-agent-vllm/sagemaker-notebook.ipynb)!
