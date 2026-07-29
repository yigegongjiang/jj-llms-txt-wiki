# Deploy Embedding Models with TEI DLC on Vertex AI

Written by Alvaro Bartolome

BGE, standing for BAAI General Embedding, is a collection of embedding models released by BAAI, which is an English base model for general embedding tasks ranked in the MTEB Leaderboard. Text Embeddings Inference (TEI) is a toolkit developed by Hugging Face for deploying and serving open source text embeddings and sequence classification models; enabling high-performance extraction for the most popular models, including FlagEmbedding, Ember, GTE and E5. And, Google Vertex AI is a Machine Learning (ML) platform that lets you train and deploy ML models and AI applications, and customize large language models (LLMs) for use in your AI-powered applications.

This example showcases how to deploy any supported embedding model, in this case [`BAAI/bge-large-en-v1.5`](https://huggingface.co/BAAI/bge-large-en-v1.5), from the Hugging Face Hub on Vertex AI using the TEI DLC available in Google Cloud Platform (GCP) in both CPU and GPU instances.

!['BAAI/bge-large-en-v1.5' in the Hugging Face Hub](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/model-in-hf-hub.png)

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
%env CONTAINER_URI=us-docker.pkg.dev/deeplearning-platform-release/gcr.io/huggingface-text-embeddings-inference-cu122.1-4.ubuntu2204
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

Then you can already "upload" the model i.e. register the model on Vertex AI. It is not an upload per se, since the model will be automatically downloaded from the Hugging Face Hub in the Hugging Face DLC for TEI on startup via the `MODEL_ID` environment variable, so what is uploaded is only the configuration, not the model weights.

Before going into the code, let's quickly review the arguments provided to the `upload` method:

* **`display_name`** is the name that will be shown in the Vertex AI Model Registry.

* **`serving_container_image_uri`** is the location of the Hugging Face DLC for TEI that will be used for serving the model.

* **`serving_container_environment_variables`** are the environment variables that will be used during the container runtime, so these are aligned with the environment variables defined by `text-embeddings-inference`, which are analog to the [`text-embeddings-router` arguments](https://huggingface.co/docs/text-embeddings-inference/en/cli_arguments). Additionally, the Hugging Face DLCs for TEI also capture the `AIP_` environment variables from Vertex AI as in [Vertex AI Documentation - Custom container requirements for prediction](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements).

    * `MODEL_ID` is the identifier of the model in the Hugging Face Hub, to see all the TEI supported models please check https://huggingface.co/models?other=text-embeddings-inference&sort=trending.

* (optional) **`serving_container_ports`** is the port where the Vertex AI endpoint will be exposed, by default 8080.

For more information on the supported `aiplatform.Model.upload` arguments, check its Python reference at https://cloud.google.com/python/docs/reference/aiplatform/latest/google.cloud.aiplatform.Model#google_cloud_aiplatform_Model_upload.

```python
model = aiplatform.Model.upload(
    display_name="BAAI--bge-large-en-v1.5",
    serving_container_image_uri=os.getenv("CONTAINER_URI"),
    serving_container_environment_variables={
        "MODEL_ID": "BAAI/bge-large-en-v1.5",
    },
    serving_container_ports=[8080],
)
model.wait()
```

![Model on Vertex AI Model Registry](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/vertex-ai-model.png)

## Deploy model on Vertex AI

After the model is registered on Vertex AI, you need to define the endpoint that you want to deploy the model to, and then link the model deployment to that endpoint resource.

To do so, you need to call the method `aiplatform.Endpoint.create` to create a new Vertex AI endpoint resource (which is not linked to a model or anything usable yet).

```python
endpoint = aiplatform.Endpoint.create(display_name="BAAI--bge-large-en-v1.5-endpoint")
```

![Vertex AI Endpoint created](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/vertex-ai-endpoint.png)

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
    sync=True,
)
```

The Vertex AI endpoint deployment via the `deploy` method may take from 15 to 25 minutes.

![Vertex AI Endpoint running the model](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/vertex-ai-endpoint-run.png)

![Vertex AI Endpoint logs in Cloud Logging](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/vertex-ai-endpoint-logs.png)

## Online predictions on Vertex AI

Finally, you can run the online predictions on Vertex AI using the `predict` method, which will send the requests to the running endpoint in the `/predict` route specified within the container following Vertex AI I/O payload formatting.

```python
output = deployed_model.predict(instances=[{"inputs": "What is Deep Learning?"}])
print(output.predictions[0][0][:5], len(output.predictions[0][0])
```

Which produces the following output (truncated for brevity, but original tensor length is 1024, which is the embedding dimension of [`BAAI/bge-large-en-v1.5`](https://huggingface.co/BAAI/bge-large-en-v1.5)):

```
([0.018108826, 0.0029993146, -0.04984767, -0.035081815, 0.014210845], 1024)
```

![Vertex AI Endpoint logs in Cloud Logging after predict](https://raw.githubusercontent.com/huggingface/Google-Cloud-Containers/main/examples/vertex-ai/notebooks/deploy-embedding-on-vertex-ai/assets/vertex-ai-endpoint-logs-predict.png)

## Resource clean-up

Finally, you can release the resources programmatically within the same Python session as follows:

- `deployed_model.undeploy_all` to undeploy the model from all the endpoints.
- `deployed_model.delete` to delete the endpoint/s where the model was deployed gracefully after the `undeploy_all`.
- `model.delete` to delete the model from the registry.

```python
deployed_model.undeploy_all()
deployed_model.delete()
model.delete()
```
