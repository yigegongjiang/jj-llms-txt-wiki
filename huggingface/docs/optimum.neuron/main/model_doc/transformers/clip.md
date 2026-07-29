# CLIP

## Overview

The CLIP model was proposed in [Learning Transferable Visual Models From Natural Language Supervision](https://arxiv.org/abs/2103.00020) by Alec Radford, Jong Wook Kim, Chris Hallacy, Aditya Ramesh, Gabriel Goh,
Sandhini Agarwal, Girish Sastry, Amanda Askell, Pamela Mishkin, Jack Clark, Gretchen Krueger, Ilya Sutskever. CLIP
(Contrastive Language-Image Pre-Training) is a neural network trained on a variety of (image, text) pairs. It can be
instructed in natural language to predict the most relevant text snippet, given an image, without directly optimizing
for the task, similarly to the zero-shot capabilities of GPT-2 and 3.

## Export to Neuron

To deploy 🤗 [Transformers](https://huggingface.co/docs/transformers/index) models on Neuron devices, you first need to compile the models and export them to a serialized format for inference. Below are two approaches to compile the model, you can choose the one that best suits your needs. Here we take the `feature-extraction` as an example:

### Option 1: CLI

You can export the model using the Optimum command-line interface as follows:

```bash
optimum-cli export neuron --model openai/clip-vit-base-patch32 --task feature-extraction --text_batch_size 2 --sequence_length 77 --image_batch_size 1 --num_channels 3 --width 224 --height 224 clip_feature_extraction_neuronx/
```

> [!TIP]
> Execute `optimum-cli export neuron --help` to display all command line options and their description.

### Option 2: Python API

```python
from optimum.neuron import NeuronCLIPModel

input_shapes = {"text_batch_size": 2, "sequence_length": 77, "image_batch_size": 1, "num_channels": 3, "width": 224, "height": 224}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}
neuron_model = NeuronCLIPModel.from_pretrained(
    "openai/clip-vit-base-patch32",
    export=True,
    **input_shapes,
    **compiler_args,
)
# Save locally
neuron_model.save_pretrained("clip_feature_extraction_neuronx/")

# Upload to the HuggingFace Hub
neuron_model.push_to_hub(
    "clip_feature_extraction_neuronx/", repository_id="optimum/clip-vit-base-patch32-neuronx"  # Replace with your HF Hub repo id
)
```

## NeuronCLIPModel[[optimum.neuron.NeuronCLIPModel]]

#### optimum.neuron.NeuronCLIPModel[[optimum.neuron.NeuronCLIPModel]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/clip/modeling_clip.py#L58)

Bare CLIP Model without any specific head on top, used for the task "feature-extraction".

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronCLIPModel.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/clip/modeling_clip.py#L61[{"name": "input_ids", "val": ": Tensor"}, {"name": "pixel_values", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronCLIPModel` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronCLIPModel

>>> processor = AutoProcessor.from_pretrained("optimum/clip-vit-base-patch32-neuronx")
>>> model = NeuronCLIPModel.from_pretrained("optimum/clip-vit-base-patch32-neuronx")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)
>>> inputs = processor(text=["a photo of a cat", "a photo of a dog"], images=image, return_tensors="pt", padding=True)

>>> outputs = model(**inputs)
>>> logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
>>> probs = logits_per_image.softmax(dim=1)
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronCLIPForImageClassification[[optimum.neuron.NeuronCLIPForImageClassification]]

#### optimum.neuron.NeuronCLIPForImageClassification[[optimum.neuron.NeuronCLIPForImageClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/clip/modeling_clip.py#L110)

CLIP vision encoder with an image classification head on top (a linear layer on top of the pooled final hidden states of the patch tokens) e.g. for ImageNet.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronCLIPForImageClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/clip/modeling_clip.py#L120[{"name": "pixel_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`, defaults to `None`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronCLIPForImageClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import requests
>>> from PIL import Image
>>> from optimum.neuron import NeuronCLIPForImageClassification
>>> from transformers import AutoImageProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> preprocessor = AutoImageProcessor.from_pretrained("optimum/clip-vit-base-patch32-image-classification-neuronx")
>>> model = NeuronCLIPForImageClassification.from_pretrained("optimum/clip-vit-base-patch32-image-classification-neuronx")

>>> inputs = preprocessor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> predicted_label = logits.argmax(-1).item()
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.
