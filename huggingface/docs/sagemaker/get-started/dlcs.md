# Deep Learning Containers

Hugging Face, together with Amazon Web Services, builds and maintains Deep Learning Containers (DLCs) so you can run your machine learning workloads in an optimized environment with no configuration or maintenance on your part. These are Docker images pre-installed with popular frameworks and libraries such as 🤗 Transformers, 🤗 Datasets, and 🤗 Tokenizers, alongside high-performance serving engines. The DLCs let you serve and train models directly, skipping the complex process of building and optimizing your own environments from scratch.

The containers are publicly maintained, updated, and released periodically by Hugging Face and the AWS team, and are available to all AWS customers in the [Amazon Elastic Container Registry (ECR)](https://aws.github.io/deep-learning-containers/reference/available_images/#huggingface-vllm-inference). You can use them in **Amazon SageMaker AI**: a fully managed platform to build, train, and deploy ML models into a production-ready hosted environment.

Hugging Face DLCs are open source and licensed under Apache 2.0. Browse the full list of images and versions in the [Available DLCs](#available-dlcs) section below, and feel free to reach out on our [community forum](https://discuss.huggingface.co/c/sagemaker/17) if you have any questions.

  
    vLLM
    LLM serving on GPU and Neuron
  
  
    SGLang
    Fast serving on GPU
  
  
    llama.cpp
    Lightweight GGUF serving
  
  
    TEI
    Embeddings and reranking
  
  
    Transformers
    Training and general inference
  

## Features & benefits

  
    One command to train
    The training DLCs ship with everything needed to run a single command — for example the TRL CLI — to fine-tune LLMs from single-GPU to multi-node multi-GPU.
  
  
    Production serving engines
    Dedicated DLCs built around vLLM, SGLang, and llama.cpp serve most Hub text-generation architectures with OpenAI-compatible APIs and direct Amazon S3 model loading.
  
  
    Embeddings and reranking
    The Text Embeddings Inference (TEI) DLC serves embedding, re-ranking, and sequence-classification models on CPU and GPU, including the thousands of supported models on the Hub.
  
  
    Built-in performance
    Tested, optimized environments with production-ready endpoints that scale with your AWS environment, so you can pick infrastructure by price/performance target.
  

## Available DLCs

Below you can find a listing of our latest Deep Learning Containers (DLCs) available on AWS.

For each supported combination of use-case (training, inference), accelerator type (CPU, GPU, Neuron), and framework (PyTorch, vLLM, SGLang, llama.cpp, TEI) containers are created. The URIs below use `us-east-1` or `us-west-2`; replace the region as needed, or [retrieve the URI programmatically](#faq).

Neuron DLCs for training and inference on AWS Trainium and AWS Inferentia instances can be found in the [Optimum Neuron documentation](https://huggingface.co/docs/optimum-neuron/en/containers). To keep track of all our available DLCs, check the [AWS Deep Learning Containers releases](https://aws.github.io/deep-learning-containers/reference/available_images#huggingface-pytorch-training) page.

### Transformers

#### Training

For training, the DLCs are available for PyTorch via Transformers. They include GPUs and AWS AI chips support, with libraries such as TRL, Sentence Transformers, or Diffusers. You can also keep track of the latest PyTorch Training DLC releases [here](https://github.com/aws/deep-learning-containers/releases?q=huggingface-training+AND+NOT+neuronx&expanded=true).

AcceleratorContainer URI

GPU763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-pytorch-training:2.11.0-transformers5.13.1-cu130-amzn2023
Neuron763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-pytorch-training-neuronx:2.8.0-transformers4.55.4-neuronx-py310-sdk2.26.0-ubuntu22.04

#### Inference

For inference, the general-purpose PyTorch inference DLC serves models trained with any of those frameworks on CPU, GPU, and AWS AI chips.

AcceleratorContainer URI

CPU763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-pytorch-inference:2.6.0-transformers4.51.3-cpu-py312-ubuntu22.04
GPU763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-pytorch-inference:2.6.0-transformers4.51.3-gpu-py312-cu124-ubuntu22.04
Neuron763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-pytorch-inference-neuronx:2.8.0-transformers4.55.4-neuronx-py310-sdk2.26.0-ubuntu22.04

### vLLM

For serving text generation models with [vLLM](https://docs.vllm.ai/), there are specific DLCs available for GPU and AWS AI chips.

AcceleratorVersionContainer URI

GPU0.29.0763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-vllm:0.29-gpu-py312-cu130-ubuntu24.04
Neuron0.11.0763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-vllm-inference-neuronx:0.11.0-optimum0.4.5-neuronx-py310-sdk2.26.1-ubuntu22.04

#### vLLM Omni

You can also use vLLM Omni for serving multimodal models with vLLM on GPUs.

AcceleratorVersionContainer URI

GPU0.20.0763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-vllm-omni:0.20.0-transformers5.8.1-gpu-py312-cu130-amzn2023

### SGLang

There is also a specific DLC for serving models with [SGLang](https://docs.sglang.ai/) on GPU.

AcceleratorVersionContainer URI

GPU0.5.12763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-sglang:0.5.12-transformers5.6.0-gpu-py312-cu130-ubuntu24.04

### Llama.cpp

For lightweight inference serving, there is a specific DLC for serving models with [llama.cpp](https://github.com/ggml-org/llama.cpp) on both CPU and GPU.

AcceleratorVersionContainer URI

GPUb9522763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-llama.cpp:b9522-gpu-cu130-ubuntu24.04
CPUb9522763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-llama.cpp:b9522-cpu-ubuntu24.04

### Text Embeddings Inference

Finally, the [Text Embeddings Inference (TEI)](https://huggingface.co/docs/text-embeddings-inference) DLC provides high-performance serving of embedding models on CPU and GPU.

AcceleratorContainer URI

CPU683313688378.dkr.ecr.us-east-1.amazonaws.com/tei-cpu:2.0.1-tei1.9.3-cpu-py310-ubuntu24.04
GPU683313688378.dkr.ecr.us-east-1.amazonaws.com/tei:2.0.1-tei1.9.3-gpu-py310-cu129-ubuntu24.04

## FAQ

How do I find the URI of my container?

The SageMaker SDK provides a utility function to get the URI of a container programmatically:

```python
from sagemaker.core import image_uris

AVAILABLE_FRAMEWORKS = [
    "huggingface",
    "huggingface-tei",
    "huggingface-llamacpp",
    "huggingface-vllm",
    "huggingface-vllm-omni",
    "huggingface-sglang",
]

# use image_scope="training" for training containers
image_uris.retrieve(
    "huggingface-vllm",
    region="us-east-1",
    image_scope="inference",
    instance_type="ml.g5.2xlarge",
)
```

Can the SDK choose the container for me?

If you just want the default container for a given model, you can rely on the SageMaker SDK `ModelBuilder`, which automatically chooses the container for you:

```python
from sagemaker.serve import ModelBuilder

builder = ModelBuilder(
    model="google/gemma-4-E2B-it",
    instance_type="ml.g5.2xlarge",
    role_arn=role,
)
```

> [!NOTE]
> The SDK may not always be up to date or may choose the wrong container for your use case. When in doubt, compare the container URI returned by the SDK with the ones listed on this page.
