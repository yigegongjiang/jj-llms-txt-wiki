# Optimum Neuron Container

We provide pre-built Optimum Neuron containers for Amazon SageMaker. These containers come with all of the Hugging Face libraries and dependencies pre-installed, so you can start using them right away.
We have containers for training and inference on EC2, and optimized text generation serving containers for SageMaker. The table is up to date and only includes the latest versions of each container. You can find older versions in the [Deep Learning Container Release Notes](https://github.com/aws/deep-learning-containers/releases?q=hf-neuronx&expanded=true)

If you have the Optimum Neuron package installed, you can use the function `image_uri` to retrieve the image URI for the container you want to use. The result is the same as the one retrieved by the `sagemaker` Python SDK, but the image URI retrieved can be newer than the one reported by the `sagemaker` Python SDK.
```python
from optimum.neuron.utils import ecr

# retrieve the image uri
image = ecr.image_uri("inference", region="us-west-2")

print(f"image uri: {image}")

```

## Available Optimum Neuron Containers

| Type                       | Optimum Neuron Version | Image URI                                   |
|-----------------------------|-------------------------|---------------------------------------------|
| Training  | 0.4.1 | `763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-pytorch-training-neuronx:2.8.0-transformers4.55.4-neuronx-py310-sdk2.26.0-ubuntu22.04`   |
| Inference | 0.4.1 | `763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-pytorch-inference-neuronx:2.8.0-transformers4.55.4-neuronx-py310-sdk2.26.0-ubuntu22.04`      |
| vLLM      | 0.4.4 | `763104351884.dkr.ecr.us-west-2.amazonaws.com/huggingface-vllm-inference-neuronx:0.11.0-optimum0.4.4-neuronx-py310-sdk2.26.1-ubuntu22.04` |

Please replace `763104351884` with the correct [AWS account ID](https://github.com/aws/sagemaker-python-sdk/blob/master/sagemaker-core/src/sagemaker/core/image_uri_config/huggingface-neuronx.json) and `region` with the AWS region you are working in.
