# Deploy an AI Agent on AWS Inferentia2 with SageMaker

Last updated 2026-07-27

 This notebook demontrates how to deploy an open-ource LLM on a real-time AWS endpoint with Neuron devices using Hugging Face library `optimum-neuron`, `strands-agent` and AWS Sagemaker.

### 🎯 Goal

By the end of this notebook, you will know how to:

- **Deploy an LLM** optimized for instruction-following, reasoning, and tool-calling — specifically [`ibm-granite/granite-3.3-8b-instruct`](https://huggingface.co/ibm-granite/granite-3.3-8b-instruct).
- **Run it on an AWS Inferentia2 inference-optimized instance** (`inf2.24xlarge`) using a **SageMaker real-time endpoint**.
- **Use the deployed endpoint in an agentic setup**, including:
  - Calling external tools such as **web search**.
  - Connecting to and interacting with **MCP servers**, such as the Hugging Face MCP servers, to work with the Hugging Face Hub.

This notebook guides you from deployment to practical agent-enabled usage.

#### Configure logging and env

```python
%%writefile .env
HF_TOKEN=your_hf_token_here
```

```python
from pathlib import Path
from dotenv import load_dotenv

# Load secrets from .env if present so the notebook picks up local configuration.
dotenv_path = Path.cwd() / ".env"
if dotenv_path.exists():
    load_dotenv(dotenv_path)
    console.print(f"[green]Loaded environment from {dotenv_path}[/green]")
else:
    console.print(f"[yellow]No .env file found at {dotenv_path}; using existing environment variables.[/yellow]")
```

```python
!pip install -q rich
```

```python
import logging
from rich.console import Console
from rich.logging import RichHandler

console = Console()
logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
    datefmt="[%X]",
    handlers=[RichHandler(console=console, rich_tracebacks=True, markup=True)],
    force=True,
)
logger = logging.getLogger("neuron-agent")
```

### 1. Deploy an LLM on a SageMaker Inferentia2 real-time endpoint

***🔧 Configure SageMaker Session***

```python
!pip install sagemaker --upgrade --quiet
```

```python
>> import sagemaker
>> import boto3

>> sess = sagemaker.Session()
>> # sagemaker session bucket -> used for uploading data, models and logs
>> # sagemaker will automatically create this bucket if it not exists
>> sagemaker_session_bucket = None
>> if sagemaker_session_bucket is None and sess is not None:
...     # set to default bucket if a bucket name is not given
...     sagemaker_session_bucket = sess.default_bucket()
```

sagemaker.config INFO - Not applying SDK defaults from location: /etc/xdg/sagemaker/config.yaml
sagemaker.config INFO - Not applying SDK defaults from location: /home/ec2-user/.config/sagemaker/config.yaml

***🔑 Configure SageMaker Role***

```python
sagemaker_role_name = "sagemaker-dlcs" # TODO: change to your own role name
iam = boto3.client("iam")
role = iam.get_role(RoleName=sagemaker_role_name)["Role"]["Arn"]
```

```python
>> sess = sagemaker.Session(default_bucket=sagemaker_session_bucket)

>> print(f"sagemaker role arn: {role}")
>> print(f"sagemaker session region: {sess.boto_region_name}")
```

sagemaker role arn: arn:aws:iam::754289655784:role/sagemaker-dlcs
sagemaker session region: us-east-1

***📦 Prepare the model***

Before deploying the model, we need to **compile it for AWS Inferentia2**.

Inferentia and Trainium devices run models using the **Neuron runtime**, which requires models to be **compiled ahead of time** into an optimized Neuron executable.  
This compilation step transforms the original model (PyTorch or Transformers weights) into a hardware-optimized graph that can run efficiently on `inf2` instances.

Because we're working on a **Neuron-based instance**, compiling the model is a mandatory step before deployment.  

SageMaker will then use the **Optimum-Neuron vLLM container** to load the compiled artifacts and run them efficiently on Inferentia2.  
Instead of compiling the model at inference time, the container expects a **pre-compiled Neuron executable**, which it can serve directly through a real-time endpoint for fast and stable performance.

***🧩 Optimum Neuron Cache***

Hugging Face makes working with Neuron hardware much easier through the **Optimum Neuron Cache** ([`aws-neuron/optimum-neuron-cache`](https://huggingface.co/aws-neuron/optimum-neuron-cache)), a public repository that hosts **pre-compiled Neuron artifacts** for many popular models.  
Instead of spending time compiling models yourself—a process that can take minutes to hours—you can simply reuse these ready-made artifacts.

This means:

- **Instant startup** with no compilation step  
- **Consistent, reproducible performance** on Inferentia2 and Trainium  
- **Seamless integration** with the Optimum-Neuron vLLM containers, which can load these cached executables directly

If a matching compiled artifact is available, you can deploy your model on SageMaker immediately.

Learn more about the Optimum Neuron model cache [here](https://huggingface.co/docs/optimum-neuron/en/guides/cache_system).

***⚙️ Model compilation***

The model we selected, [`ibm-granite/granite-3.3-8b-instruct`]("https://huggingface.co/ibm-granite/granite-3.3-8b-instruct"), provides strong instruction-following, reasoning, and tool-calling abilities while remaining lightweight enough to run efficiently on a single Inferentia2 with a large input context necessary for agentic use-cases.

At the time of writing, **granite-3.3-8B-instruct** was not yet available as a pre-compiled Neuron artifact, so we compile it ourselves before deploying it on Inferentia2. We followed the steps from the official Optimum Neuron export tutorial (https://huggingface.co/docs/optimum-neuron/en/guides/export_model) to compile the model for Inferentia2.

To compile the model, we:

- Launched an **Inferentia2 EC2 instance** using the official **Hugging Face Neuron Deep Learning AMI**  
  (https://aws.amazon.com/marketplace/pp/prodview-gr3e6yiscria2).
- Connected to the instance via **SSH**.
- Used **Optimum Neuron** to compile the model with the following command:

  ```bash
  optimum-cli export neuron \
      --model ibm-granite/granite-3.3-8b-instruct \
      --sequence_length 16384 \
      --batch_size 1 \
      --num_cores 8 \
      --task text-generation \
      <folder-to-saved-compiled-artifacts>
- Pushed the compiled Neuron artifacts to the Hugging Face Hub:
[`florentgbelidji/granite-3.3-8b-instruct-neuron-bs-1-seq-16384-tp-8`](https://huggingface.co/florentgbelidji/granite-3.3-8b-instruct-neuron-bs-1-seq-16384-tp-8)

At the time of writing, AWS Inferentia2 does not support dynamic shapes for inference, which means that we need to specify our sequence length and batch size ahead of time. We chose these compilation parameters to match our agentic use case and the target hardware:

- **`--sequence_length 16384`**: Enables *large-context* inference, which is important when prompts include long tool definitions or multi-step reasoning typical in agent workflows.
- **`--batch_size 1`**: Prioritizes *low latency* over throughput, ideal for the demo where requests are sent sequentially rather than in batches.
- **`--num_cores 8`**: Maximizes hardware utilization on `inf2` instances.  
  An `inf2.24xlarge` exposes *6 Neuron devices = 12 Neuron cores* (learn more https://awsdocs-neuron.readthedocs-hosted.com/en/latest/about-neuron/arch/neuron-hardware/inf2-arch.html#aws-inf2-arch
  )

We set **`--num_cores 8`** because, at **deployment time**, the Neuron runtime requires that the model’s number of attention heads be divisible by the number of Neuron cores assigned to the model.  
Using 8 cores satisfies this constraint on Inferentia2 and ensures the model can be loaded and served correctly.

**🧱 Deployment Server: HuggingFace vLLM Inference Neuronx DLC**

For deployment, we use the **HuggingFace vLLM Inference Neuronx** Deep Learning Container from AWS ECR:

```
763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-vllm-inference-neuronx:0.11.0-optimum0.4.4-neuronx-py310-sdk2.26.1-ubuntu22.04
```

This container is built on [Optimum Neuron](https://huggingface.co/docs/optimum-neuron) and is designed to **serve Neuron-compiled models using vLLM on Inferentia2 instances**, combining Neuron runtime optimizations with vLLM's efficient inference mechanisms.

```python
llm_image = "763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-vllm-inference-neuronx:0.11.0-optimum0.4.4-neuronx-py310-sdk2.26.1-ubuntu22.04"
```

***⚙️ Configure the SageMaker Endpoint with the Hugging Face SDK***

Now that we have a Neuron-compiled model and a deployment image, we can configure a SageMaker real-time endpoint using the **Hugging Face SageMaker SDK**

```python
import os 
from sagemaker.huggingface import HuggingFaceModel

# sagemaker config
instance_type = "ml.inf2.24xlarge"
health_check_timeout = 3600  # additional time to load the model

config = {
    "SM_ON_MODEL": "florentgbelidji/granite-3.3-8b-instruct-neuron-bs-1-seq-16384-tp-8",
    "SM_ON_TENSOR_PARALLEL_SIZE": "8",# corresponds to the number of Neuron cores
    "SM_ON_BATCH_SIZE": "1",  # max batch size for the model
    "SM_ON_SEQUENCE_LENGTH": "16384",  # max length of generated text
    "SM_ON_ENABLE_AUTO_TOOL_CHOICE": "true", # enables tool choice
    "SM_ON_TOOL_CALL_PARSER": "granite", # enables tool call parsing with parser configured for granite
    "SM_ON_MAX_NUM_BATCHED_TOKENS": "8192", # max number of tokens in a batch, control prefill chunking
    "HF_TOKEN": os.environ["HF_TOKEN"],
}

endpoint_name = "granite-8b-instruct-neuron-bs-1-seq-16384-tp-8-demo-endpoint"
```

***Create Sagemaker Model***

```python
llm_model = HuggingFaceModel(
    role=role, 
    image_uri=llm_image, 
    env=config, 
    name=endpoint_name
    )
```

***🚀 Deploy***

```python
>> llm_model._is_compiled_model = True

>> llm = llm_model.deploy(
...     initial_instance_count=1,
...     instance_type=instance_type,
...     container_startup_health_check_timeout=health_check_timeout,
...     inference_ami_version="al2-ami-sagemaker-inference-neuron-2",
... )
```

<pre>
-------------------!
</pre>

Go to console.aws.amazon.com to check the status of your endpoint and the logs

```python
print(llm.endpoint_name)
```

***Reload already created endpoint***

If your endpoint is still running run the following:

```python
existing_endpoint_name = "granite-8b-instruct-neuron-bs-1-seq-163-2025-11-25-16-35-23-419" # TODO: change to your own endpoint name
```

```python
from sagemaker.predictor import Predictor
llm = Predictor(
    endpoint_name=existing_endpoint_name , 
    sagemaker_session=sess
                )
```

**Test**

```python
from sagemaker.deserializers import JSONDeserializer
from sagemaker.serializers import JSONSerializer

messages = [
    {"role": "user", "content": [{"type": "text", "text": "Hello my name is Huggy"}]},
]

parameters = {
    "max_tokens": 16,
    "temperature": 0.1,
    "stream": False,
}

llm.serializer = JSONSerializer()
llm.deserializer = JSONDeserializer()

payload = {"messages": messages, **parameters}
chat = llm.predict(payload)
```

```python
chat
```

```python
>> print(chat["choices"][0]["message"]["content"])
```

<pre>
Hello Huggy, nice to meet you! How can I assist you today
</pre>

### 2. Create an Agent with Strands Agent

[Strands Agents](https://strandsagents.com/latest/) is an open-source Python SDK for building **production-grade AI agents** with minimal code.  
It provides a clean, model-driven framework for defining system prompts, tools, multi-step reasoning, and orchestration.

A key advantage for AWS users: **Strands natively supports SageMaker real-time endpoints**, allowing you to plug in your deployed LLMs directly as agent backends with no custom wrappers.

```python
!pip install -q 'strands-agents[sagemaker]' strands-agents-tools
```

```python
from strands import Agent, tool
from strands.tools.mcp import MCPClient
from strands.models.sagemaker import SageMakerAIModel
from mcp.client.streamable_http import streamablehttp_client
```

***🔗 Link your endpoint to strands-agent***

```python
model = SageMakerAIModel(
    endpoint_config={
        "endpoint_name": endpoint_name,
        "region_name": "us-east-1",  # or your region
    },
    payload_config={
        "max_tokens": 2048,
        "temperature": 0.2,
        "stream": False,
        
    }
)
```

***🕸️ Define a web-search tool for the agent***

To enable our agent to fetch fresh information from the web, we install the `duckduckgo-search` package, which provides a simple Python API for performing real-time DuckDuckGo queries.

```python
!pip install -q ddgs
```

```python
from ddgs import DDGS

# define a tool to search the web
@tool()
def web_search(query:str, max_results=5)->dict:
    results = ""
    with DDGS() as ddgs:
        results = [r for r in ddgs.text(query, max_results=max_results)]
    return results
```

***📞 My first call to the agent***

```python
>> agent = Agent(model=model, tools=[web_search])
>> response = agent("What are AWS Neuron devices?")
```

<pre>
Tool #1: web_search
</pre>

```python
>> from pprint import pprint
>> pprint(response.message["content"][0]["text"])
```

<pre>
('AWS Neuron devices refer to the accelerated machine learning chips, '
 'specifically AWS Inferentia and AWS Trainium, which are designed to enhance '
 'AI and deep learning workloads on Amazon EC2 instances. These chips are '
 'optimized for high-density, scalable, and cost-effective inference and '
 'training respectively. \n'
 '\n'
 'AWS Neuron is a software development kit (SDK) that facilitates the '
 'optimization of AI and deep learning models for deployment on these Neuron '
 'devices. It transforms models from popular frameworks like PyTorch or '
 'TensorFlow into a specialized, low-level representation for efficient '
 'execution on AWS Inferentia and Trainium.\n'
 '\n'
 'The AWS Neuron Kernel Mode Driver establishes a communication channel (FWIO) '
 'that allows the driver and an application to send specific commands to the '
 'device. \n'
 '\n'
 "Additionally, there's a Neuron backend for Hugging Face's Text Generation "
 'Inference (TGI) that allows deployment on AWS Trainium and Inferentia family '
 'of chips.\n'
 '\n'
 'For more detailed technical information, you can refer to the provided links '
 'in the web search results.')
</pre>

Here’s a concise breakdown of what the logs show:

- **Initial payload:**  
  The first block (`payload=...`) is the request sent to the SageMaker endpoint.  
  It includes the user message *("Explain what is a Neuron device")*, the tool schema,  
  and generation parameters (`max_tokens`, `temperature`, etc.).

- **Model triggers a tool:**  
  The first response contains a `tool_call`, meaning the model decided  
  to use the `web_search` tool.  
  It sends arguments like:  
  `{"query": "what is a Neuron device", "max_results": 1}`.

- **Tool execution:**  
  The notebook executes the `web_search` function locally.  
  It runs the DuckDuckGo/Bing query and returns the results to the agent.

- **Second payload:**  
  The agent sends a new request to the endpoint that includes:  
  - the original user message  
  - the tool call  
  - the tool result  
  This allows the model to produce a final answer with the retrieved information.

- **Final model response:**  
  The last block shows the assistant's answer generated after incorporating  
  the tool's output. (In this example, the model misinterprets *Neuron device*,  
  but the flow is correct.)

Overall, this demonstrates the complete agent loop:
1. Model decides to call a tool  
2. Tool runs  
3. Result is injected back into the model  
4. Model produces the final answer  

✨ **The entire iteration—model → tool → model—completed in under 10 seconds.**

### 3. Use my Agent with HF MCP server 🤗

The **Hugging Face MCP (Model Context Protocol) Server** is a service that lets your MCP-compatible AI assistant (for example in VS Code, Cursor, Zed, or Claude Desktop) **connect directly to the Hugging Face Hub**. 

Once connected, your assistant can:
- Search and explore Hub resources (models, datasets, Spaces, papers).
- Invoke community tools (Gradio apps on Spaces) as functions, returning results—metadata, links, context.  

It effectively turns the Hugging Face Hub into a tool-server your agent can use, enhancing its tool-calling and exploration capabilities.

Learn more about the MCP server [here](https://huggingface.co/docs/hub/en/hf-mcp-server).

***🔧 Configure the MCP client***

To use the Hugging Face MCP server with your agent, you must first configure it at:  
👉 **https://huggingface.co/settings/mcp**

On this page, you can generate an MCP-compatible access token and configure which built-in tools your agent is allowed to use.

Once enabled, your agent gains access to a rich set of Hub-powered tools, including:

- **Spaces Semantic Search** – find the best AI apps on the Hub using natural language.  
- **Model Search** – query models by task, library, license, or metadata.  
- **Documentation Semantic Search** – search across all HF documentation.  
- **Hub Repository Details** – retrieve structured metadata for models, datasets, and Spaces.  
- **Papers Semantic Search** – find AI research papers semantically.  
- **Dataset Search** – search datasets by author, tags, modality, etc.  
- **Run and Manage Jobs** – schedule and monitor jobs on Hugging Face infrastructure.  
- **Include README files** – allow agents to read repository READMEs in results.

This integration lets your agent use the Hugging Face Hub as a powerful knowledge and tooling backend.

![MCP Settings](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/mcp-settings.png)

```python
from strands.tools.mcp import MCPClient
from mcp.client.streamable_http import streamablehttp_client
#MCP URL
hf_mcp_url = "https://huggingface.co/mcp"
hf_token = os.environ.get("HF_TOKEN")

mcp_client = MCPClient(
    lambda: streamablehttp_client(
        url=hf_mcp_url,
        headers={"Authorization": f"Bearer {hf_token}"} if hf_token else {},
        timeout=120,
        sse_read_timeout=600,
    )
)
```

```python
#Use the MCP client as a tool
agent = Agent(model=model, tools=[mcp_client])
```

We are now ready to use the MCP client as a tool in our agent. we will test it to:
- Search for a Space that can transcribe audio files
- Search for trending image generation models
- Search the doc to export a model to neuron with optimum

***🔉 Space search***

```python
>> audio_space_search = agent("Search for a Space that can transcribe audio files")
>> pprint(audio_space_search.message["content"][0]["text"])
```

<pre>
Tool #1: space_search
</pre>

***📈 Trending model search***

```python
>> image_model_search = agent("Search for trending image generation models")
>> pprint(image_model_search.message["content"][0]["text"])
```

<pre>
Tool #2: model_search
</pre>

***📚 Documentation search***

```python
>> doc_search = agent("How to export a model to neuron with optimum")
>> pprint(doc_search.message["content"][0]["text"])
```

<pre>
To export a Hugging Face model to Neuron using Optimum, you'll need to follow these steps:

1. **Install Optimum**: If you haven't already, install Optimum using pip:

   ```bash
   pip install optimum
   ```

2. **Choose a Model**: Select a model from the Hugging Face Model Hub that you want to export. For example, let's use `bert-base-uncased` for a text classification task.

3. **Load the Model**: Use the `transformers` library to load your chosen model.

   ```python
   from transformers import AutoModelForSequenceClassification, AutoTokenizer

   model_name = "bert-base-uncased"
   model = AutoModelForSequenceClassification.from_pretrained(model_name)
   tokenizer = AutoTokenizer.from_pretrained(model_name)
   ```

4. **Prepare for Export**: Optimum provides a `neuron_quantized` exporter. First, you need to quantize your model.

   ```python
   from optimum.neuron.quantization import NeuronQuantizer

   quantizer = NeuronQuantizer(model)
   quantized_model = quantizer.fit(model, dataset=your_dataset)
   ```

   Replace `your_dataset` with a suitable dataset for fine-tuning or evaluation.

5. **Export the Model**: Use the `export` method provided by Optimum to export the quantized model to Neuron format.

   ```python
   from optimum.neuron.export import export_neuron

   export_neuron(quantized_model, "bert-base-uncased-neuron")
   ```

   This will save the exported model in Neuron format in a directory named `bert-base-uncased-neuron`.

6. **Load on Neuron**: Finally, load the exported model on Neuron hardware using the Neuron SDK.

Please note that the exact steps might vary slightly based on the model you choose and the specific Neuron hardware you are targeting. Always refer to the [Optimum documentation](https://huggingface.co/docs/optimum/index) and the [Neuron SDK documentation](https://docs.neuron.app/) for the most accurate and up-to-date instructions.

If you encounter any specific issues or need further assistance, feel free to ask!("To export a Hugging Face model to Neuron using Optimum, you'll need to "
 'follow these steps:\n'
 '\n'
 "1. **Install Optimum**: If you haven't already, install Optimum using pip:\n"
 '\n'
 '   ```bash\n'
 '   pip install optimum\n'
 '   ```\n'
 '\n'
 '2. **Choose a Model**: Select a model from the Hugging Face Model Hub that '
 "you want to export. For example, let's use `bert-base-uncased` for a text "
 'classification task.\n'
 '\n'
 '3. **Load the Model**: Use the `transformers` library to load your chosen '
 'model.\n'
 '\n'
 '   ```python\n'
 '   from transformers import AutoModelForSequenceClassification, '
 'AutoTokenizer\n'
 '\n'
 '   model_name = "bert-base-uncased"\n'
 '   model = AutoModelForSequenceClassification.from_pretrained(model_name)\n'
 '   tokenizer = AutoTokenizer.from_pretrained(model_name)\n'
 '   ```\n'
 '\n'
 '4. **Prepare for Export**: Optimum provides a `neuron_quantized` exporter. '
 'First, you need to quantize your model.\n'
 '\n'
 '   ```python\n'
 '   from optimum.neuron.quantization import NeuronQuantizer\n'
 '\n'
 '   quantizer = NeuronQuantizer(model)\n'
 '   quantized_model = quantizer.fit(model, dataset=your_dataset)\n'
 '   ```\n'
 '\n'
 '   Replace `your_dataset` with a suitable dataset for fine-tuning or '
 'evaluation.\n'
 '\n'
 '5. **Export the Model**: Use the `export` method provided by Optimum to '
 'export the quantized model to Neuron format.\n'
 '\n'
 '   ```python\n'
 '   from optimum.neuron.export import export_neuron\n'
 '\n'
 '   export_neuron(quantized_model, "bert-base-uncased-neuron")\n'
 '   ```\n'
 '\n'
 '   This will save the exported model in Neuron format in a directory named '
 '`bert-base-uncased-neuron`.\n'
 '\n'
 '6. **Load on Neuron**: Finally, load the exported model on Neuron hardware '
 'using the Neuron SDK.\n'
 '\n'
 'Please note that the exact steps might vary slightly based on the model you '
 'choose and the specific Neuron hardware you are targeting. Always refer to '
 'the [Optimum documentation](https://huggingface.co/docs/optimum/index) and '
 'the [Neuron SDK documentation](https://docs.neuron.app/) for the most '
 'accurate and up-to-date instructions.\n'
 '\n'
 'If you encounter any specific issues or need further assistance, feel free '
 'to ask!')

### 🧩 Understanding the Agent Output

The long log you see comes from the agent’s **full reasoning loop**, not just the final answer.  
Here’s what’s happening:

- The agent keeps a **persistent conversation history**, so past queries and tool calls are always included.
- When you run `agent("…")`, the model decides whether it should call a tool (e.g., HF MCP search).
- If a tool is needed, the agent:
  1. emits a `tool_call`,
  2. executes the tool,
  3. appends the tool result back into the conversation,
  4. asks the model again to produce the final answer.
- The printed value (`doc_search.message["content"][0]["text"]`) only shows the **assistant’s final answer**, but the log shows the **full chain**:  
  user message → tool call → tool output → final completion.

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/neuron-agent-inf/sagemaker-notebook.ipynb)!
