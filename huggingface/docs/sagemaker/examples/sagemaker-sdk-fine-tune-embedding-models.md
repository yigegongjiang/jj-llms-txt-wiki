# Fine-tune and deploy embedding models with Amazon SageMaker

Last updated 2026-08-31

Embedding models are crucial for successful RAG applications, but they're often trained on general knowledge, which limits their effectiveness for company or domain specific adoption. Customizing embedding for your domain specific data can significantly boost the retrieval performance of your RAG Application. With the new release of [Sentence Transformers 3](https://huggingface.co/blog/train-sentence-transformers) and the [Hugging Face Embedding Container](https://huggingface.co/blog/sagemaker-huggingface-embedding), it's easier than ever to fine-tune and deploy embedding models.

In this example, we'll show you how to fine-tune and deploy a custom embedding model on Amazon SageMaker using the new Hugging Face Embedding Container. We'll use the [Sentence Transformers 3](https://huggingface.co/blog/train-sentence-transformers) library to fine-tune a model on a custom dataset and deploy it on Amazon SageMaker for inference. We will fine-tune [BAAI/bge-base-en-v1.5](https://huggingface.co/BAAI/bge-base-en-v1.5) for financial RAG applications using a synthetic dataset from the [2023_10 NVIDIA SEC Filing](https://stocklight.com/stocks/us/nasdaq-nvda/nvidia/annual-reports/nasdaq-nvda-2023-10K-23668751.pdf). 

1. [Setup development environment](#2-setup-development-environment)
2. [Create and prepare the dataset](#3-create-and-prepare-the-dataset)
3. [Fine-tune Embedding model on Amazon SageMaker](#3-fine-tune-embedding-model-on-amazon-sagemaker)
4. [Deploy & Test fine-tuned Embedding Model on Amazon SageMaker](#4-deploy--test-fine-tuned-embedding-model-on-amazon-sagemaker)

**What is new with Sentence Transformers 3?**

Sentence Transformers v3 introduces a new trainer that makes it easier to fine-tune and train embedding models. This update includes enhanced components like diverse datasets, updated loss functions, and a streamlined training process, improving the efficiency and flexibility of model development.

**What is the Hugging Face Embedding Container?**

The Hugging Face Embedding Container is a new purpose-built Inference Container to easily deploy Embedding Models in a secure and managed environment. The DLC is powered by [Text Embedding Inference (TEI)](https://github.com/huggingface/text-embeddings-inference) a blazing fast and memory efficient solution for deploying and serving Embedding Models. TEI enables high-performance extraction for the most popular models, including FlagEmbedding, Ember, GTE and E5. TEI implements many features such as:

_Note: This blog was created and validated on `ml.g5.xlarge` for training and `ml.c6i.2xlarge` for inference instance._

## 1. Setup Development Environment

Our first step is to install Hugging Face Libraries we need on the client to correctly prepare our dataset and start our training/evaluations jobs. 

```python
!pip install transformers "datasets[s3]==2.18.0" "sagemaker>=3.0.0" "huggingface_hub" --upgrade --quiet
```

This example uses the [SageMaker Python SDK v3](https://github.com/aws/sagemaker-python-sdk). v3 introduces a new, framework-agnostic API built around `ModelTrainer` (training) and `ModelBuilder` (inference), which replaces the v2 `HuggingFace` and `HuggingFaceModel` classes.

If you are going to use Sagemaker in a local environment. You need access to an IAM Role with the required permissions for Sagemaker. You can find [here](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html) more about it.

```python
import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

sess = Session()
# sagemaker session bucket -> used for uploading data, models and logs
# sagemaker will automatically create this bucket if it does not exist
sagemaker_session_bucket = sess.default_bucket()

try:
    role = get_execution_role()
except Exception:
    iam = boto3.client("iam")
    role = iam.get_role(RoleName="sagemaker_execution_role")["Role"]["Arn"]

print(f"sagemaker role arn: {role}")
print(f"sagemaker bucket: {sess.default_bucket()}")
print(f"sagemaker session region: {sess.boto_region_name}")
```

## 2. Create and prepare the dataset

An embedding dataset typically consists of text pairs (question, answer/context) or triplets that represent relationships or similarities between sentences. The dataset format you choose or have available will also impact the loss function you can use. Common formats for embedding datasets:

- **Positive Pair**: Text Pairs of related sentences (query, context | query, answer), suitable for tasks like similarity or semantic search, example datasets: `sentence-transformers/sentence-compression`, `sentence-transformers/natural-questions`.
- **Triplets**: Text triplets consisting of (anchor, positive, negative), example datasets `sentence-transformers/quora-duplicates`, `nirantk/triplets`.
- **Pair with Similarity Score**: Sentence pairs with a similarity score indicating how related they are, example datasets: `sentence-transformers/stsb`, `PhilipMay/stsb_multi_mt`

Learn more at [Dataset Overview](https://sbert.net/docs/sentence_transformer/dataset_overview.html).

We are going to use [philschmid/finanical-rag-embedding-dataset](https://huggingface.co/datasets/philschmid/finanical-rag-embedding-dataset), which includes 7,000 positive text pairs of questions and corresponding context from the [2023_10 NVIDIA SEC Filing](https://stocklight.com/stocks/us/nasdaq-nvda/nvidia/annual-reports/nasdaq-nvda-2023-10K-23668751.pdf).

The dataset has the following format
```json
{"question": "<question>", "context": "<relevant context to answer>"}
{"question": "<question>", "context": "<relevant context to answer>"}
{"question": "<question>", "context": "<relevant context to answer>"}
```

We are going to use the [FileSystem integration](https://huggingface.co/docs/datasets/filesystems) to upload our dataset to S3. We are using the `sess.default_bucket()`, adjust this if you want to store the dataset in a different S3 bucket. We will use the S3 path later in our training script.

```python
from datasets import load_dataset

# Load dataset from the hub
dataset = load_dataset("philschmid/finanical-rag-embedding-dataset", split="train")
input_path = f"s3://{sess.default_bucket()}/datasets/rag-embedding"

# rename columns
dataset = dataset.rename_column("question", "anchor")
dataset = dataset.rename_column("context", "positive")

# Add an id column to the dataset
dataset = dataset.add_column("id", range(len(dataset)))

# split dataset into a 10% test set
dataset = dataset.train_test_split(test_size=0.1)

# save train_dataset to s3 using our SageMaker session

# save datasets to s3
dataset["train"].to_json(f"{input_path}/train/dataset.json", orient="records")
train_dataset_s3_path = f"{input_path}/train/dataset.json"
dataset["test"].to_json(f"{input_path}/test/dataset.json", orient="records")
test_dataset_s3_path = f"{input_path}/test/dataset.json"

print(f"Training data uploaded to:")
print(train_dataset_s3_path)
print(test_dataset_s3_path)
print(
    f"https://s3.console.aws.amazon.com/s3/buckets/{sess.default_bucket()}/?region={sess.boto_region_name}&prefix={input_path.split('/', 3)[-1]}/"
)
```

## 3. Fine-tune Embedding model on Amazon SageMaker

We are now ready to fine-tune our model. We will use the [SentenceTransformerTrainer](https://www.sbert.net/docs/package_reference/sentence_transformer/trainer.html) from `sentence-transformers` to fine-tune our model. The `SentenceTransformerTrainer` makes it straightfoward to supervise fine-tune open Embedding Models, as it is a subclass of the `Trainer` from the `transformers`. We prepared a script [run_mnr.py](scripts/run_mnr.py) which will loads the dataset from disk, prepare the model, tokenizer and start the training.
The `SentenceTransformerTrainer` makes it straightfoward to supervise fine-tune open Embedding supporting:
- **Integrated Components**: Combines datasets, loss functions, and evaluators into a unified training framework.
- **Flexible Data Handling**: Supports various data formats and easy integration with Hugging Face datasets.
- **Versatile Loss Functions**: Offers multiple loss functions for different training tasks.
- **Multi-Dataset Training**: Facilitates simultaneous training with multiple datasets and different loss functions.
- **Seamless Integration**: Easy saving, loading, and sharing of models within the Hugging Face ecosystem.

In order to create a SageMaker training job we use a `ModelTrainer`. The `ModelTrainer` handles end-to-end Amazon SageMaker training. Amazon SageMaker takes care of starting and managing all the required ec2 instances for us, provides the correct Hugging Face container, uploads the provided scripts and downloads the data from our S3 bucket into the container at `/opt/ml/input/data`. We provide the container image explicitly with `image_uris.retrieve`.

Note: Make sure that you include the `requirements.txt` in the `source_dir` if you are using a custom training script. We recommend to just clone the whole repository.

Lets first define our trainings parameter. Those are passed as cli arguments to our training script. We are going to use the `BAAI/bge-base-en-v1.5` model, which is a pre-trained model on a large corpus of English text. We will use the `MultipleNegativesRankingLoss` in combination with the `MatryoshkaLoss`. This approach allows us to leverage the efficiency and flexibility of Matryoshka embeddings, enabling different embedding dimensions to be utilized without significant performance trade-offs. The `MultipleNegativesRankingLoss` is a great loss function if you only have positive pairs as it adds in batch negative samples to the loss function to have per sample n-1 negative samples.

```python
from sagemaker.train.model_trainer import ModelTrainer
from sagemaker.train.configs import SourceCode, Compute, StoppingCondition
from sagemaker.core import image_uris

# define Training Job Name
job_name = "bge-base-exp1"

# define hyperparameters, which are passed into the training job as `--key value` CLI args
training_arguments = {
    "model_id": "BAAI/bge-base-en-v1.5",  # model id from the hub
    "train_dataset_path": "/opt/ml/input/data/train/",  # path inside the container where the training data is stored
    "test_dataset_path": "/opt/ml/input/data/test/",  # path inside the container where the test data is stored
    "num_train_epochs": 3,  # number of training epochs
    "learning_rate": 2e-5,  # learning rate
}

instance_type = "ml.g5.xlarge"

# Retrieve the Hugging Face PyTorch training DLC image URI
training_image = image_uris.retrieve(
    framework="huggingface",
    region=sess.boto_region_name,
    version="4.49.0",
    base_framework_version="pytorch2.5.1",
    py_version="py311",
    image_scope="training",
    instance_type=instance_type,
)

# create the ModelTrainer
huggingface_estimator = ModelTrainer(
    sagemaker_session=sess,
    role=role,
    base_job_name=job_name,  # the name of the training job
    training_image=training_image,
    source_code=SourceCode(
        source_dir="scripts",  # directory which includes all the files needed for training
        entry_script="run_mnr.py",  # train script
        requirements="requirements.txt",  # dependencies installed before running the script
    ),
    compute=Compute(
        instance_type=instance_type,  # instances type used for the training job
        instance_count=1,  # the number of instances used for training
    ),
    stopping_condition=StoppingCondition(
        max_runtime_in_seconds=2 * 24 * 60 * 60,  # maximum runtime in seconds (days * hours * minutes * seconds)
    ),
    hyperparameters=training_arguments,
    environment={
        "HUGGINGFACE_HUB_CACHE": "/tmp/.cache",  # set env variable to cache models in /tmp
    },
)
```

We can now start our training job, with the `.train()` method passing our S3 paths as input data channels.

```python
from sagemaker.train.configs import InputData

# define the input data channels with our uploaded s3 uris
# each channel is mounted at /opt/ml/input/data/<channel_name> inside the container
data = [
    InputData(channel_name="train", data_source=train_dataset_s3_path),
    InputData(channel_name="test", data_source=test_dataset_s3_path),
]

# starting the train job with our uploaded datasets as input
huggingface_estimator.train(input_data_config=data, wait=True)
```

In our example the training BGE Base with Flash Attention 2 (SDPA) for 3 epochs with a dataset of 6,3k train samples and 700 eval samples took 645 seconds (~10minutes) on a `ml.g5.xlarge` (1.2575 $/h) or ~$5.

## 4. Deploy & Test fine-tuned Embedding Model on Amazon SageMaker

We are going to use the [Hugging Face Embedding Container](https://huggingface.co/blog/sagemaker-huggingface-embedding#what-is-the-hugging-face-embedding-container) a purpose-built Inference  Container to easily deploy Embedding Models in a secure and managed environment. The DLC is powered by Text Embedding Inference (TEI) a blazing fast and memory efficient solution for deploying and serving Embedding Models.

To retrieve the Hugging Face Embedding Container in Amazon SageMaker, we use the `image_uris.retrieve` helper from `sagemaker.core`. Important to note is that TEI has 2 different versions for cpu and gpu, so we create a helper function to retrieve the correct image uri based on the instance type.

```python
from sagemaker.core import image_uris

# retrieve the image uri based on instance type
def get_image_uri(instance_type):
    framework = "huggingface-tei" if instance_type.startswith(("ml.g", "ml.p")) else "huggingface-tei-cpu"
    return image_uris.retrieve(
        framework=framework,
        region=sess.boto_region_name,
        version="1.8.2",
        image_scope="inference",
        instance_type=instance_type,
    )
```

We can now create a `ModelBuilder` using the TEI container image and the S3 path to our fine-tuned model artifacts. We provide `image_uri` + `s3_model_data_url` (without `model`/`model_server`), which uses ModelBuilder's image pass-through mode, and set `HF_MODEL_ID` to the path where SageMaker extracts the model inside the container.

```python
from sagemaker.serve import ModelBuilder

# sagemaker config
instance_type = "ml.c6i.2xlarge"

# S3 URI of the fine-tuned model artifacts produced by the training job
model_data = huggingface_estimator._latest_training_job.model_artifacts.s3_model_artifacts

# create ModelBuilder with the TEI image and the trained model artifacts
model_builder = ModelBuilder(
    image_uri=get_image_uri(instance_type),
    s3_model_data_url=model_data,
    env_vars={"HF_MODEL_ID": "/opt/ml/model"},  # Path to the model in the container
    role_arn=role,
    sagemaker_session=sess,
    instance_type=instance_type,
)

emb_model = model_builder.build()
```

After we have built the model with `ModelBuilder` we can deploy it to Amazon SageMaker using the `deploy` method. We will deploy the model with the `ml.c6i.2xlarge` instance type.

```python
# Deploy model to an endpoint
emb = model_builder.deploy(
    initial_instance_count=1,
    instance_type=instance_type,
)
```

SageMaker will now create our endpoint and deploy the model to it. This can take ~5 minutes. After our endpoint is deployed we can run inference on it. We will use the `invoke` method of the `Endpoint` to run inference. The request and response bodies are JSON.

```python
import json

data = {
    "inputs": "the mesmerizing performances of the leads keep the film grounded and keep the audience riveted .",
}

res = emb.invoke(body=json.dumps(data), content_type="application/json")
embeddings = json.loads(res.body.read())

# print some results
print(f"length of embeddings: {len(embeddings[0])}")
print(f"first 10 elements of embeddings: {embeddings[0][:10]}")
```

We trained our model with the Matryoshka Loss means that the semantic meaning is frontloaded. To use the different mathryshoka dimension we need to manually truncate our embeddings manually. Below is an example on how you would truncate the embeddings to 256 dimension, which is 1/3 of the original size. If we check our training logs we can see that the NDCG metric for 768 is `0.823` and for 256 `0.818` meaning we preserve > 99% accuracy. 

```python
data = {
    "inputs": "the mesmerizing performances of the leads keep the film grounded and keep the audience riveted .",
}

res = emb.invoke(body=json.dumps(data), content_type="application/json")
embeddings = json.loads(res.body.read())

# truncate embeddings to matryoshka dimensions
dim = 256
truncated = embeddings[0][0:dim]

# print some results
print(f"length of embeddings: {len(truncated)}")
```

Awesome! 🚀 Now that we can generate embeddings and integrate your endpoint into your RAG application. 

To clean up, we can delete the model and endpoint.

```python
emb.delete()
emb_model.delete()
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/fine-tune-embedding-models/sagemaker-notebook.ipynb)!
