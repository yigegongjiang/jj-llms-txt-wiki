# Sentence Transformers on AWS Inferentia with Optimum Neuron

This guide explains how to convert, load, and use [Sentence Transformers (SBERT)](https://www.sbert.net/) models on AWS Inferentia2 using Optimum Neuron, enabling efficient calculation of embeddings. Sentence Transformers are powerful models for generating sentence embeddings. You can use this Sentence Transformers to compute sentence / text embeddings for more than 100 languages. These embeddings can then be compared e.g. with cosine-similarity to find sentences with a similar meaning. This can be useful for semantic textual similarities, semantic search, or paraphrase mining.

_Note: Currently only text models are supported, we are working on vision support for CLIP._

## Compile Sentence Transformers models for AWS Inferentia2  

First, you need to convert your Sentence Transformers model to a format compatible with AWS Inferentia2. You can compile Sentence Transformers models with Optimum Neuron using the `optimum-cli` or `NeuronModelForSentenceTransformers` class. Below you will find an example for both approaches. We have to make sure `sentence-transformers` is installed. That's only needed for exporting the model. 

```python
!pip install sentence-transformers
```

Here we will use the `NeuronModelForSentenceTransformers`, which can convert any Sntence Transformers model to a format compatible with AWS Inferentia2 or load already converted models. 
 When exporting models with the `NeuronModelForSentenceTransformers` you need to set `export=True` and define the input shape and batch size. The input shape is defined by the `sequence_length` and the batch size by `batch_size`. 

```python
from optimum.neuron import NeuronModelForSentenceTransformers

# Sentence Transformers model from HuggingFace
model_id = "BAAI/bge-small-en-v1.5"
input_shapes = {"batch_size": 1, "sequence_length": 384}  # mandatory shapes

# Load Transformers model and export it to AWS Inferentia2
model = NeuronModelForSentenceTransformers.from_pretrained(model_id, export=True, **input_shapes)

# Save model to disk
model.save_pretrained("bge_emb_inf2/")
```

Here we will use the `optimum-cli` to convert the model. Similar to the `NeuronModelForSentenceTransformers` we need to define our input shape and batch size. The input shape is defined by the `sequence_length` and the batch size by `batch_size`. The `optimum-cli` will automatically convert the model to a format compatible with AWS Inferentia2 and save it to the specified output directory.

```python
!optimum-cli export neuron -m BAAI/bge-small-en-v1.5 --sequence_length 384 --batch_size 1 --task feature-extraction bge_emb_inf2/
```

## Load compiled Sentence Transformers model and run inference

Once we have a compiled Sentence Transformers model, which we either exported ourselves or is available on the Hugging Face Hub, we can load it and run inference. For loading the model we can use the `NeuronModelForSentenceTransformers` class, which is an abstraction layer for the `SentenceTransformer` class. The `NeuronModelForSentenceTransformers` class will automatically pad the input to the specified `sequence_length` and run inference on AWS Inferentia2.

```python
from transformers import AutoTokenizer

from optimum.neuron import NeuronModelForSentenceTransformers

model_id_or_path = "bge_emb_inf2/"
tokenizer_id = "BAAI/bge-small-en-v1.5"

# Load model and tokenizer
model = NeuronModelForSentenceTransformers.from_pretrained(model_id_or_path)
tokenizer = AutoTokenizer.from_pretrained(tokenizer_id)

# Run inference
prompt = "I like to eat apples"
encoded_input = tokenizer(prompt, return_tensors='pt')
outputs = model(**encoded_input)

token_embeddings = outputs.token_embeddings
sentence_embedding = outputs.sentence_embedding

print(f"token embeddings: {token_embeddings.shape}") # torch.Size([1, 7, 384])
print(f"sentence_embedding: {sentence_embedding.shape}") # torch.Size([1, 384])
```

```python

```

## Production Usage

For deploying these models in a production environment, refer to the [Amazon SageMaker Blog](https://www.philschmid.de/inferentia2-embeddings).
