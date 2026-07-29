# Run `codellama/CodeLlama-7b-hf` from Hugging Face on Inf2 & Trn1

In this example we compile and deploy the [codellama/CodeLlama-7b-hf](https://huggingface.co/codellama/CodeLlama-7b-hf) model from Hugging Face on Neuron core devices using the `transformers-neuronx` package.

The example has the following main sections:
1. Deploy your instance
1. Set up the Jupyter Notebook
1. Inference from a pre-compiled model
1. Compiling a model
1. Save to disk

This Jupyter Notebook can be run on a Inf2 instance (`inf2.xlarge`) or larger or also a Trn1 instance (`trn1.32xlarge`).

## Deploy your instance

1. Raise your limits
    The default limit for Inferentia and Trainium instances are 0, so request an increase before you start!  EC2 instances are limited by vCPUs, but SageMaker limits are based on number of instances and instance size.  Start with a vCPU limit request of 32 so you can deploy an inf2.xlarge (4 vCPUs) or an inf2.8xlarge (32 vCPUs).  Go to [Service Quotas](https://us-west-2.console.aws.amazon.com/servicequotas/home/services/ec2/quotas) and search for `inf`.

    Request increases for both On-Demand and Spot (yes, there are usually Spot instances available!).  IAD (us-east-1) and PDX (us-west-2) are the best regions to start in for the US.  Limits are region specific!
1. Go to EC2 to launch an instance.  Make sure you choose the region to match your limit increases.
1. Launch an inf2.xlarge instance with the [Hugging Face DLAMI(Deep learning AMI)](https://aws.amazon.com/marketplace/pp/prodview-gr3e6yiscria2).  This image comes with all the software preinstalled.  Alternatively, you can follow the instructions at the [Neuron Setup Guide](https://awsdocs-neuron.readthedocs-hosted.com/en/latest/general/setup/torch-neuronx.html#setup-torch-neuronx)
1. 100GB disk size is fine for this example, but larger models such as 70B make take over 500GB

## Set up the Jupyter Notebook

(You could also just copy and paste most of this code into an SSH prompt)

The following steps set up Jupyter Notebook and launch this tutorial:
1. Clone the [Hugging Face Optimum Neuron](https://github.com/huggingface/optimum-neuron) repo to your instance using
```
git clone https://github.com/huggingface/optimum-neuron.git
```
2. Navigate to the `text-generation` notebooks folder
```
cd optimum-neuron/notebooks/text-generation
```
3. Follow the instructions in [Jupyter Notebook QuickStart](https://awsdocs-neuron.readthedocs-hosted.com/en/latest/general/setup/notebook/setup-jupyter-notebook-steps-troubleshooting.html) to run Jupyter Notebook on your instance.
4. Locate this tutorial in your Jupyter Notebook session (`CodeLlama-7B-Compilation.ipynb`) and launch it. Follow the rest of the instructions in this tutorial. 

## Inference from a pre-compiled model

The model needs to be exported(compiled) to a Neuron specific format.  Some configuration changes (such as batch size and number of Neuron cores used) will require a re-compile.  We have pre-compiled a model for you to use as a test using the Hugging Face Optimum Neuron library.  

In this case, the model is pre-compiled for 2 Neuron cores, so it will run on an inf2.xlarge.  You can read more about the model at [aws-neuron/CodeLlama-7b-hf-neuron-8xlarge](https://huggingface.co/aws-neuron/CodeLlama-7b-hf-neuron-8xlarge)

Instructions on how to compile a model are in the next section.  

**Keep in mind that this is a model for code generation, so the example prompt is code, and the response will probably be code**

```python
from optimum.neuron import pipeline

p = pipeline('text-generation', 'aws-neuron/CodeLlama-7b-hf-neuron-8xlarge')
p("import socket\n\ndef ping_exponential_backoff(host: str):",
    do_sample=True,
    top_k=10,
    temperature=0.1,
    top_p=0.95,
    num_return_sequences=1,
    max_length=200,
)
```

If you are going to compile in the same session, you need to release the Neuron cores by deleting the pipeline object.  Otherwise, you may get load errors.

```python
del p
```

Example output:

[{'generated_text': 'import socket\n\ndef ping_exponential_backoff(host: str):\n    """\n    Ping a host with exponential backoff.\n\n    :param host: Host to ping\n    :return: True if host is reachable, False otherwise\n    """\n    for i in range(1, 10):\n        try:\n            socket.create_connection((host, 80), 1).close()\n            return True\n        except OSError:\n            time.sleep(2 ** i)\n    return False\n\n\ndef ping_exponential_backoff_with_timeout(host: str, timeout: int):\n    """\n    Ping a host with exponential backoff and timeout.\n\n    :param host: Host to ping\n    :param timeout: Timeout in seconds\n    :return: True if host is reachable, False otherwise\n    """\n    for'}]

## Troubleshooting
If you see the error

```
FileNotFoundError: Could not find a matching NEFF for your HLO in this directory. Ensure that the model you are trying to load is the same type and has the same parameters as the one you saved or call "save" on this model to reserialize it.
```
then you may be using a different version of the Neuron SDK than we used in this example.  

You can fix that by compiling your own following the instructions below!

## Compiling a model

Before you run the code below consider what machine you want to run the final result on.  You will need to change the num_cores to match the number of cores on your target machine (at a maximum -- you could run a model compiled for 2 cores on an inf2.24xlarge, but you will only use 2 of the 12 available cores).  

Using this method, the machine you are compiling on needs to also have at least that number of cores.  

        -inf2.xlarge, inf2.8xlarge:  num_cores=2

        -inf2.24xlarge: num_cores=12

        -inf2.48xlarge: num_cores=24

**The speed of this process depends on whether this is the first time you are running it (and the model needs to be downloaded to the local system) and whether the process is able to automatically download pre-compiled files from the online cache [aws-neuron/optimum-neuron-cache](https://huggingface.co/aws-neuron/optimum-neuron-cache)**

The optimum.neuron library will download the original model directly from Hugging Face.  In this example, that is [codellama/CodeLlama-7b-hf](https://huggingface.co/codellama/CodeLlama-7b-hf).  This is different from the code above where we pointed to a model in the aws-neuron group.

```python
from optimum.neuron import NeuronModelForCausalLM

#num_cores should be changed based on the instance.  inf2.24xlarge has 6 neuron processors (they have two cores each) so 12 total
compiler_args = {"num_cores": 2, "auto_cast_type": 'fp16'}
input_shapes = {"batch_size": 1, "sequence_length": 2048}
model = NeuronModelForCausalLM.from_pretrained("codellama/CodeLlama-7b-hf", export=True, **compiler_args, **input_shapes)
```

## Save to disk

If you want to save your compiled model out to a local directory:

```python
model.save_pretrained("CodeLlama-7b-hf-neuron-8xlarge")
```

Once you have the model saved locally,you can just give the local directory name in the pipeline code above instead of the path on Hugging Face.  You should delete the model object before loading the pipeline code above.

```python
del model
```

If you want to save your model back up to Hugging Face:
1. Create a WRITE token for you to use for command line access https://huggingface.co/settings/tokens
1. While you are there, create a "New Model" for your system to reside in.  Our example uses jburtoft/CodeLlama-7b-hf-neuron-24xlarge

```python
from huggingface_hub.hf_api import HfFolder

HfFolder.save_token('MY_HUGGINGFACE_TOKEN_HERE')
```

```python
from huggingface_hub import HfApi, login

api = HfApi()
login()

api.upload_folder(
    folder_path="CodeLlama-7b-hf-neuron-8xlarge",
    repo_id="jburtoft/CodeLlama-7b-hf-neuron-8xlarge",
    repo_type="model",
    multi_commits=True,
    multi_commits_verbose=True,
)
```
