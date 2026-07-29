# Using BERTopic at Hugging Face

[BERTopic](https://github.com/MaartenGr/BERTopic) is a topic modeling framework that leverages 🤗 transformers and c-TF-IDF to create dense clusters allowing for easily interpretable topics whilst keeping important words in the topic descriptions. 

BERTopic supports all kinds of topic modeling techniques:  

  
    Guided
    Supervised
    Semi-supervised
 
   
    Manual
    Multi-topic distributions
    Hierarchical
 
 
    Class-based
    Dynamic
    Online/Incremental
 
 
    Multimodal
    Multi-aspect
    Text Generation/LLM
 
 
    Zero-shot (new!)
    Merge Models (new!)
    Seed Words (new!)
 

## Exploring BERTopic on the Hub

You can find BERTopic models by filtering at the left of the [models page](https://huggingface.co/models?library=bertopic&sort=trending).

BERTopic models hosted on the Hub have a model card with useful information about the models. Thanks to BERTopic Hugging Face Hub integration, you can load BERTopic models with a few lines of code. You can also deploy these models using [Inference Endpoints](https://huggingface.co/inference-endpoints).

## Installation

To get started, you can follow the [BERTopic installation guide](https://github.com/MaartenGr/BERTopic#installation).
You can also use the following one-line install through pip:

```bash
pip install bertopic
```

## Using Existing Models

All BERTopic models can easily be loaded from the Hub:

```py
from bertopic import BERTopic
topic_model = BERTopic.load("MaartenGr/BERTopic_Wikipedia")
```

Once loaded, you can use BERTopic's features to predict the topics for new instances:

```py
topic, prob = topic_model.transform("This is an incredible movie!")
topic_model.topic_labels_[topic]
```

Which gives us the following topic:

```text
64_rating_rated_cinematography_film
```

## Sharing Models

When you have created a BERTopic model, you can easily share it with others through the Hugging Face Hub. To do so, we can make use of the `push_to_hf_hub` function that allows us to directly push the model to the Hugging Face Hub:

```python
from bertopic import BERTopic

# Train model
topic_model = BERTopic().fit(my_docs)

# Push to HuggingFace Hub
topic_model.push_to_hf_hub(
    repo_id="MaartenGr/BERTopic_ArXiv",
    save_ctfidf=True
)
```

Note that the saved model does not include the dimensionality reduction and clustering algorithms. Those are removed since they are only necessary to train the model and find relevant topics. Inference is done through a straightforward cosine similarity between the topic and document embeddings. This not only speeds up the model but allows us to have a tiny BERTopic model that we can work with.

## Additional Resources

* [BERTopic repository](https://github.com/MaartenGr/BERTopic)
* [BERTopic docs](https://maartengr.github.io/BERTopic/)
* [BERTopic models in the Hub](https://huggingface.co/models?library=bertopic&sort=trending)
