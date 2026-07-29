# Deploy Gemma 7B with TGI DLC on Vertex AI

Written by Alvaro Bartolome

Gemma is a family of lightweight, state-of-the-art open models built from the same research and technology used to create the Gemini models, developed by Google DeepMind and other teams across Google. Text Generation Inference (TGI) is a toolkit developed by Hugging Face for deploying and serving LLMs, with high performance text generation. And, Google Vertex AI is a Machine Learning (ML) platform that lets you train and deploy ML models and AI applications, and customize large language models (LLMs) for use in your AI-powered applications.

This example showcases how to deploy any supported text-generation model, in this case [`google/gemma-7b-it`](https://huggingface.co/google/gemma-7b-it), from the Hugging Face Hub on Vertex AI using the TGI DLC available in Google Cloud Platform (GCP).

!['google/gemma-7b-it' in the Hugging Face Hub](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/model-in-hf-hub.png)

## Setup / Configuration

First, you need to install `gcloud` in your local machine, which is the command-line tool for Google Cloud, following the instructions at [Cloud SDK Documentation - Install the gcloud CLI](https://cloud.google.com/sdk/docs/install).

Then, you also need to install the `google-cloud-aiplatform` Python SDK, required to programmatically create the Vertex AI model, register it, acreate the endpoint, and deploy it on Vertex AI.

```python
!pip install --upgrade --quiet google-cloud-aiplatform
```

Optionally, to ease the usage of the commands within this tutorial, you need to set the following environment variables for GCP:

```python
%env PROJECT_ID=your-project-id
%env LOCATION=your-location
%env CONTAINER_URI=us-docker.pkg.dev/deeplearning-platform-release/gcr.io/huggingface-text-generation-inference-cu124.2-3.ubuntu2204.py311
```

Then you need to login into your GCP account and set the project ID to the one you want to use to register and deploy the models on Vertex AI.

```python
!gcloud auth login
!gcloud auth application-default login  # For local development
!gcloud config set project $PROJECT_ID
```

Once you are logged in, you need to enable the necessary service APIs in GCP, such as the Vertex AI API, the Compute Engine API, and Google Container Registry related APIs.

```python
!gcloud services enable aiplatform.googleapis.com
!gcloud services enable compute.googleapis.com
!gcloud services enable container.googleapis.com
!gcloud services enable containerregistry.googleapis.com
!gcloud services enable containerfilesystem.googleapis.com
```

## Register model on Vertex AI

Once everything is set up, you can already initialize the Vertex AI session via the `google-cloud-aiplatform` Python SDK as follows:

```python
import os
from google.cloud import aiplatform

aiplatform.init(
    project=os.getenv("PROJECT_ID"),
    location=os.getenv("LOCATION"),
)
```

As [`google/gemma-7b-it`](https://huggingface.co/google/gemma-7b-it) is a gated model, you need to login into your Hugging Face Hub account with a read-access token either fine-grained with access to the gated model, or just overall read-access to your account. More information on how to generate a read-only access token for the Hugging Face Hub in the instructions at .

```python
!pip install --upgrade --quiet huggingface_hub
```

```python
from huggingface_hub import interpreter_login

interpreter_login()
```

Then you can already "upload" the model i.e. register the model on Vertex AI. It is not an upload per se, since the model will be automatically downloaded from the Hugging Face Hub in the Hugging Face DLC for TGI on startup via the `MODEL_ID` environment variable, so what is uploaded is only the configuration, not the model weights.

Before going into the code, let's quickly review the arguments provided to the `upload` method:

* **`display_name`** is the name that will be shown in the Vertex AI Model Registry.

* **`serving_container_image_uri`** is the location of the Hugging Face DLC for TGI that will be used for serving the model.

* **`serving_container_environment_variables`** are the environment variables that will be used during the container runtime, so these are aligned with the environment variables defined by `text-generation-inference`, which are analog to the [`text-generation-launcher` arguments](https://huggingface.co/docs/text-generation-inference/en/basic_tutorials/launcher). Additionally, the Hugging Face DLCs for TGI also capture the `AIP_` environment variables from Vertex AI as in [Vertex AI Documentation - Custom container requirements for prediction](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements).

    * `MODEL_ID` is the identifier of the model in the Hugging Face Hub. To explore all the supported models you can check .
    * `NUM_SHARD` is the number of shards to use if you don't want to use all GPUs on a given machine e.g. if you have two GPUs but you just want to use one for TGI then `NUM_SHARD=1`, otherwise it matches the `CUDA_VISIBLE_DEVICES`.
    * `MAX_INPUT_TOKENS` is the maximum allowed input length (expressed in number of tokens), the larger it is, the larger the prompt can be, but also more memory will be consumed.
    * `MAX_TOTAL_TOKENS` is the most important value to set as it defines the "memory budget" of running clients requests, the larger this value, the larger amount each request will be in your RAM and the less effective batching can be.
    * `MAX_BATCH_PREFILL_TOKENS` limits the number of tokens for the prefill operation, as it takes the most memory and is compute bound, it is interesting to limit the number of requests that can be sent.
    * `HUGGING_FACE_HUB_TOKEN` is the Hugging Face Hub token, required as [`google/gemma-7b-it`](https://huggingface.co/google/gemma-7b-it) is a gated model.

* (optional) **`serving_container_ports`** is the port where the Vertex AI endpoint will be exposed, by default 8080.

For more information on the supported `aiplatform.Model.upload` arguments, check its Python reference at https://cloud.google.com/python/docs/reference/aiplatform/latest/google.cloud.aiplatform.Model#google_cloud_aiplatform_Model_upload.

Starting from TGI 2.3 DLC i.e. `us-docker.pkg.dev/deeplearning-platform-release/gcr.io/huggingface-text-generation-inference-cu124.2-3.ubuntu2204.py311`, and onwards, you can set the environment variable value `MESSAGES_API_ENABLED="true"` to deploy the [Messages API](https://huggingface.co/docs/text-generation-inference/main/en/messages_api) on Vertex AI, otherwise, the [Generate API](https://huggingface.co/docs/text-generation-inference/main/en/quicktour#consuming-tgi) will be deployed instead.

```python
from huggingface_hub import get_token

model = aiplatform.Model.upload(
    display_name="google--gemma-7b-it",
    serving_container_image_uri=os.getenv("CONTAINER_URI"),
    serving_container_environment_variables={
        "MODEL_ID": "google/gemma-7b-it",
        "NUM_SHARD": "1",
        "MAX_INPUT_TOKENS": "512",
        "MAX_TOTAL_TOKENS": "1024",
        "MAX_BATCH_PREFILL_TOKENS": "1512",
        "HUGGING_FACE_HUB_TOKEN": get_token(),
    },
    serving_container_ports=[8080],
)
model.wait()
```

![Model on Vertex AI Model Registry](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-model.png)

## Deploy model on Vertex AI

After the model is registered on Vertex AI, you need to define the endpoint that you want to deploy the model to, and then link the model deployment to that endpoint resource.

To do so, you need to call the method `aiplatform.Endpoint.create` to create a new Vertex AI endpoint resource (which is not linked to a model or anything usable yet).

```python
endpoint = aiplatform.Endpoint.create(display_name="google--gemma-7b-it-endpoint")
```

![Vertex AI Endpoint created](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-endpoint.png)

Now you can deploy the registered model in an endpoint on Vertex AI.

The `deploy` method will link the previously created endpoint resource with the model that contains the configuration of the serving container, and then, it will deploy the model on Vertex AI in the specified instance.

Before going into the code, let's quickly review the arguments provided to the `deploy` method:

- **`endpoint`** is the endpoint to deploy the model to, which is optional, and by default will be set to the model display name with the `_endpoint` suffix.
- **`machine_type`**, **`accelerator_type`** and **`accelerator_count`** are arguments that define which instance to use, and additionally, the accelerator to use and the number of accelerators, respectively. The `machine_type` and the `accelerator_type` are tied together, so you will need to select an instance that supports the accelerator that you are using and vice-versa. More information about the different instances at [Compute Engine Documentation - GPU machine types](https://cloud.google.com/compute/docs/gpus), and about the `accelerator_type` naming at [Vertex AI Documentation - MachineSpec](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec).

For more information on the supported `aiplatform.Model.deploy` arguments, you can check its Python reference at https://cloud.google.com/python/docs/reference/aiplatform/latest/google.cloud.aiplatform.Model#google_cloud_aiplatform_Model_deploy.

```python
deployed_model = model.deploy(
    endpoint=endpoint,
    machine_type="g2-standard-4",
    accelerator_type="NVIDIA_L4",
    accelerator_count=1,
)
```

**WARNING**: _The Vertex AI endpoint deployment via the `deploy` method may take from 15 to 25 minutes._

![Vertex AI Endpoint running the model](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-endpoint-run.png)

![Vertex AI Endpoint logs in Cloud Logging](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-endpoint-logs.png)

## Online predictions on Vertex AI

Finally, you can run the online predictions on Vertex AI using the `predict` method, which will send the requests to the running endpoint in the `/predict` route specified within the container following Vertex AI I/O payload formatting.

As you are serving a `text-generation` model, you will need to make sure that the chat template, if any, is applied correctly to the input conversation; meaning that `transformers` need to be installed so as to instantiate the `tokenizer` for [`google/gemma-7b-it`](https://huggingface.co/google/gemma-7b-it) and run the `apply_chat_template` method over the input conversation before sending the input within the payload to the Vertex AI endpoint.

```python
!pip install --upgrade --quiet transformers
```

After the installation is complete, the following snippet will apply the chat template to the conversation:

```python
from huggingface_hub import get_token
from transformers import AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("google/gemma-7b-it", token=get_token())

messages = [
    {"role": "user", "content": "What's Deep Learning?"},
]

inputs = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True,
)
# user\nWhat's Deep Learning?\nmodel\n
```

Which is what you will be sending within the payload to the deployed Vertex AI Endpoint, as well as the generation parameters as in .

### Via Python

#### Within the same session

If you are willing to run the online prediction within the current session, you can send requests programmatically via the `aiplatform.Endpoint` (returned by the `aiplatform.Model.deploy` method) as in the following snippet:

```python
output = deployed_model.predict(
    instances=[
        {
            "inputs": "user\nWhat's Deep Learning?\nmodel\n",
            "parameters": {
                "max_new_tokens": 256, "do_sample": True,
                "top_p": 0.95, "temperature": 1.0,
            },
        },
    ]
)
print(output.predictions[0])
```

Producing the following `output`:

```
Prediction(predictions=['\n\nDeep learning is a type of machine learning that uses artificial neural networks to learn from large amounts of data, making it a powerful tool for various tasks, including image recognition, natural language processing, and speech recognition.\n\n**Key Concepts:**\n\n* **Artificial Neural Networks (ANNs):** Structures that mimic the interconnected neurons in the brain.\n* **Deep Learning Architectures:** Multi-layered ANNs that learn hierarchical features from data.\n* **Transfer Learning:** Reusing learned features from one task to improve performance on another.\n\n**Types of Deep Learning:**\n\n* **Supervised Learning:** Models are trained on labeled data, where inputs are paired with corresponding outputs.\n* **Unsupervised Learning:** Models learn patterns from unlabeled data, such as clustering or dimensionality reduction.\n* **Reinforcement Learning:** Models learn through trial-and-error by interacting with an environment to optimize a task.\n\n**Benefits:**\n\n* **High Accuracy:** Deep learning models can achieve high accuracy on complex tasks.\n* **Adaptability:** Deep learning models can adapt to new data and tasks.\n* **Scalability:** Deep learning models can handle large amounts of data.\n\n**Applications:**\n\n* Image recognition\n* Natural language processing (NLP)\n'], deployed_model_id='***', metadata=None, model_version_id='1', model_resource_name='projects/***/locations/us-central1/models/***', explanations=None)
```

![Vertex AI Endpoint logs in Cloud Logging after predict](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-endpoint-logs-predict.png)

#### From a different session

If the Vertex AI Endpoint was deployed in a different session and you want to use it but don't have access to the `deployed_model` variable returned by the `aiplatform.Model.deploy` method as in the previous section; you can also run the following snippet to instantiate the deployed `aiplatform.Endpoint` via its resource name as `projects/{PROJECT_ID}/locations/{LOCATION}/endpoints/{ENDPOINT_ID}`.

You will need to either retrieve the resource name i.e. the `projects/{PROJECT_ID}/locations/{LOCATION}/endpoints/{ENDPOINT_ID}` URL yourself via the Google Cloud Console, or just replace the `ENDPOINT_ID` below that can either be found via the previously instantiated `endpoint` as `endpoint.id` or via the Google Cloud Console under the Online predictions where the endpoint is listed.

```python
import os
from google.cloud import aiplatform

aiplatform.init(project=os.getenv("PROJECT_ID"), location=os.getenv("LOCATION"))

endpoint_display_name = "google--gemma-7b-it-endpoint"  # TODO: change to your endpoint display name

# Iterates over all the Vertex AI Endpoints within the current project and keeps the first match (if any), otherwise set to None
ENDPOINT_ID = next(
    (endpoint.name for endpoint in aiplatform.Endpoint.list() 
     if endpoint.display_name == endpoint_display_name), 
    None
)
assert ENDPOINT_ID, (
    "`ENDPOINT_ID` is not set, please make sure that the `endpoint_display_name` is correct at "\
    f"https://console.cloud.google.com/vertex-ai/online-prediction/endpoints?project={os.getenv('PROJECT_ID')}"
)

endpoint = aiplatform.Endpoint(f"projects/{os.getenv('PROJECT_ID')}/locations/{os.getenv('LOCATION')}/endpoints/{ENDPOINT_ID}")
output = endpoint.predict(
    instances=[
        {
            "inputs": "user\nWhat's Deep Learning?\nmodel\n",
            "parameters": {
                "max_new_tokens": 128,
                "do_sample": True,
                "top_p": 0.95,
                "temperature": 0.7,
            },
        },
    ],
)
print(output.predictions[0])
```

### Via the Vertex AI Online Prediction UI

Alternatively, for testing purposes you can also use the Vertex AI Online Prediction UI, that provides a field that expects the JSON payload formatted according to the Vertex AI specification (as in the examples above) being:

```json
{
    "instances": [
        {
            "inputs": "user\nWhat's Deep Learning?\nmodel\n",
            "parameters": {
                "max_new_tokens": 128,
                "do_sample": true,
                "top_p": 0.95,
                "temperature": 0.7
            }
        }
    ]
}
```

![Vertex AI Endpoint online inference](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-gemma-on-vertex-ai/assets/vertex-ai-online-prediction.png)

## Resource clean-up

Finally, you can already release the resources that you've created as follows, to avoid unnecessary costs:

* `deployed_model.undeploy_all` to undeploy the model from all the endpoints.
* `deployed_model.delete` to delete the endpoint/s where the model was deployed gracefully, after the `undeploy_all` method.
* `model.delete` to delete the model from the registry.

```python
deployed_model.undeploy_all()
deployed_model.delete()
model.delete()
```

Alternatively, you can also remove those from the Google Cloud Console following the steps:

* Go to Vertex AI in Google Cloud
* Go to Deploy and use -> Online prediction
* Click on the endpoint and then on the deployed model/s to "Undeploy model from endpoint"
* Then go back to the endpoint list and remove the endpoint
* Finally, go to Deploy and use -> Model Registry, and remove the model
