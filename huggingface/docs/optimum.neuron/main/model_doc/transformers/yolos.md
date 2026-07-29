# YOLOS

## Overview

The YOLOS model was proposed in [You Only Look at One Sequence: Rethinking Transformer in Vision through Object Detection](https://arxiv.org/abs/2106.00666) by Yuxin Fang, Bencheng Liao, Xinggang Wang, Jiemin Fang, Jiyang Qi, Rui Wu, Jianwei Niu, Wenyu Liu.
YOLOS proposes to just leverage the plain [Vision Transformer (ViT)](vit) for object detection, inspired by DETR. It turns out that a base-sized encoder-only Transformer can also achieve 42 AP on COCO, similar to DETR and much more complex frameworks such as Faster R-CNN.

## Export to Neuron

To deploy 🤗 [Transformers](https://huggingface.co/docs/transformers/index) models on Neuron devices, you first need to compile the models and export them to a serialized format for inference. Below are two approaches to compile the model, you can choose the one that best suits your needs. Here we take the `feature-extraction` as an example:

### Option 1: CLI

You can export the model using the Optimum command-line interface as follows:

```bash
optimum-cli export neuron --model hustvl/yolos-tiny --task object-detection --batch_size 1 yolos_object_detection_neuronx/
```

> [!TIP]
> Execute `optimum-cli export neuron --help` to display all command line options and their description.

### Option 2: Python API

```python
from optimum.neuron import NeuronModelForObjectDetection
from transformers import AutoImageProcessor

preprocessor = AutoImageProcessor.from_pretrained("hustvl/yolos-tiny")
neuron_model = NeuronModelForObjectDetection.from_pretrained("hustvl/yolos-tiny", export=True, batch_size=1)

neuron_model.save_pretrained("yolos_object_detection_neuronx")
neuron_model.push_to_hub(
    "yolos_object_detection_neuronx", repository_id="optimum/yolos-tiny-neuronx-bs1"  # Replace with your HF Hub repo id
)
```

## NeuronYolosForObjectDetection[[optimum.neuron.NeuronYolosForObjectDetection]]

#### optimum.neuron.NeuronYolosForObjectDetection[[optimum.neuron.NeuronYolosForObjectDetection]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/yolos/modeling_yolos.py#L43)

Neuron Model with object detection heads on top, for tasks such as COCO detection.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronYolosForObjectDetection.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/yolos/modeling_yolos.py#L53[{"name": "pixel_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`, defaults to `None`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronYolosForObjectDetection` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import requests
>>> from PIL import Image
>>> from optimum.neuron import NeuronYolosForObjectDetection
>>> from transformers import AutoImageProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> preprocessor = AutoImageProcessor.from_pretrained("optimum/yolos-tiny-neuronx-bs1")
>>> model = NeuronYolosForObjectDetection.from_pretrained("optimum/yolos-tiny-neuronx-bs1")

>>> inputs = preprocessor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.9, target_sizes=target_sizes)[0]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.
