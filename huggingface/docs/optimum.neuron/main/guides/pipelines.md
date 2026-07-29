# Inference pipelines with AWS Neuron (Inf2/Trn1)

The `pipeline()` function makes it simple to use models from the [Model Hub](https://huggingface.co/models)
for accelerated inference on a variety of tasks such as text classification, question answering and image classification.

You can also use the
[pipeline()](https://huggingface.co/docs/transformers/main/en/main_classes/pipelines#pipelines) function from
Transformers and provide your NeuronModel model class.

Currently the supported tasks are:

* `feature-extraction`
* `fill-mask`
* `text-classification`
* `token-classification`
* `question-answering`
* `text-generation`
* `image-classification`
* `image-segmentation`
* `object-detection`
* `automatic-speech-recognition`
* `audio-classification`

## Optimum pipeline usage

While each task has an associated pipeline class, it is simpler to use the general `pipeline()` function which wraps all the task-specific pipelines in one object.
The `pipeline()` function automatically loads a default model and tokenizer/feature-extractor capable of performing inference for your task.

1. Start by creating a pipeline by specifying an inference task:

```python
>>> from optimum.neuron.pipelines import pipeline

>>> classifier = pipeline(task="text-classification")
```

2. Pass your input text/image to the `pipeline()` function:

```python
>>> classifier("I like you. I love you.")
[{'label': 'POSITIVE', 'score': 0.9998838901519775}]
```

_Note: The default models used in the `pipeline()` function are not optimized for inference or quantized, so there won't be a performance improvement compared to their PyTorch counterparts._

### Using vanilla Transformers model and converting to AWS Neuron

The `pipeline()` function accepts any supported model from the [Hugging Face Hub](https://huggingface.co/models).
There are tags on the Model Hub that allow you to filter for a model you'd like to use for your task.

To be able to load the model with the Neuron Runtime, the export to neuron needs
to be supported for the considered architecture.

You can check the list of supported architectures
[here](../package_reference/configuration#supported-architectures).

Once you have picked an appropriate model, you can create the `pipeline()` by specifying the model repo:

```python
>>> from optimum.neuron.pipelines import pipeline

# The model will be loaded to an NeuronModelForQuestionAnswering.
>>> neuron_qa = pipeline("question-answering", model="deepset/roberta-base-squad2", export=True)
>>> question = "What's my name?"
>>> context = "My name is Philipp and I live in Nuremberg."

>>> pred = neuron_qa(question=question, context=context)
```

It is also possible to load it with the `from_pretrained(model_name_or_path, export=True)`
method associated with the `NeuronModelForXXX` class.

For example, here is how you can load the `~neuron.NeuronModelForQuestionAnswering` class for question answering:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForQuestionAnswering, pipeline

>>> tokenizer = AutoTokenizer.from_pretrained("deepset/roberta-base-squad2")

>>> # Loading the PyTorch checkpoint and converting to the neuron format by providing export=True
>>> model = NeuronModelForQuestionAnswering.from_pretrained(
...     "deepset/roberta-base-squad2",
...     export=True
... )

>>> neuron_qa = pipeline("question-answering", model=model, tokenizer=tokenizer)
>>> question = "What's my name?"
>>> context = "My name is Philipp and I live in Nuremberg."

>>> pred = neuron_qa(question=question, context=context)
```

### Defining Input Shapes

NeuronModels currently require static `input_shapes` to run inference. The default input shapes will be used if you are not providing input shapes when providing the `export=True` parameter.
Below is an example of how to specify the input shapes for the sequence length and batch size.

```python
>>> from optimum.neuron.pipelines import pipeline

>>> input_shapes = {"batch_size": 1, "sequence_length": 64}
>>> clt = pipeline("token-classification", model="dslim/bert-base-NER", export=True,input_shapes=input_shapes)
>>> context = "My name is Philipp and I live in Nuremberg."

>>> pred = clt(context)
```
