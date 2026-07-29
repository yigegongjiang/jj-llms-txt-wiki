# Wide ResNet

**Wide Residual Networks** are a variant on [ResNets](https://paperswithcode.com/method/resnet) where we decrease depth and increase the width of residual networks. This is achieved through the use of [wide residual blocks](https://paperswithcode.com/method/wide-residual-block).

## How do I use this model on an image?

To load a pretrained model:

```py
>>> import timm
>>> model = timm.create_model('wide_resnet101_2', pretrained=True)
>>> model.eval()
```

To load and preprocess the image:

```py 
>>> import urllib
>>> from PIL import Image
>>> from timm.data import resolve_data_config
>>> from timm.data.transforms_factory import create_transform

>>> config = resolve_data_config({}, model=model)
>>> transform = create_transform(**config)

>>> url, filename = ("https://github.com/pytorch/hub/raw/master/images/dog.jpg", "dog.jpg")
>>> urllib.request.urlretrieve(url, filename)
>>> img = Image.open(filename).convert('RGB')
>>> tensor = transform(img).unsqueeze(0) # transform and add batch dimension
```

To get the model predictions:

```py
>>> import torch
>>> with torch.inference_mode():
...     out = model(tensor)
>>> probabilities = torch.nn.functional.softmax(out[0], dim=0)
>>> print(probabilities.shape)
>>> # prints: torch.Size([1000])
```

To get the top-5 predictions class names:

```py
>>> # Get imagenet class mappings
>>> url, filename = ("https://raw.githubusercontent.com/pytorch/hub/master/imagenet_classes.txt", "imagenet_classes.txt")
>>> urllib.request.urlretrieve(url, filename) 
>>> with open("imagenet_classes.txt", "r") as f:
...     categories = [s.strip() for s in f.readlines()]

>>> # Print top categories per image
>>> top5_prob, top5_catid = torch.topk(probabilities, 5)
>>> for i in range(top5_prob.size(0)):
...     print(categories[top5_catid[i]], top5_prob[i].item())
>>> # prints class names and probabilities like:
>>> # [('Samoyed', 0.6425196528434753), ('Pomeranian', 0.04062102362513542), ('keeshond', 0.03186424449086189), ('white wolf', 0.01739676296710968), ('Eskimo dog', 0.011717947199940681)]
```

Replace the model name with the variant you want to use, e.g. `wide_resnet101_2`. You can find the IDs in the model summaries at the top of this page.

To extract image features with this model, follow the [timm feature extraction examples](../feature_extraction), just change the name of the model you want to use.

## How do I finetune this model?

You can finetune any of the pre-trained models just by changing the classifier (the last layer).

```py
>>> model = timm.create_model('wide_resnet101_2', pretrained=True, num_classes=NUM_FINETUNE_CLASSES)
```
To finetune on your own dataset, you have to write a training loop or adapt [timm's training
script](https://github.com/rwightman/pytorch-image-models/blob/master/train.py) to use your dataset.

## How do I train this model?

You can follow the [timm recipe scripts](../training_script) for training a new model afresh.

## Citation

```BibTeX
@article{DBLP:journals/corr/ZagoruykoK16,
  author    = {Sergey Zagoruyko and
               Nikos Komodakis},
  title     = {Wide Residual Networks},
  journal   = {CoRR},
  volume    = {abs/1605.07146},
  year      = {2016},
  url       = {http://arxiv.org/abs/1605.07146},
  archivePrefix = {arXiv},
  eprint    = {1605.07146},
  timestamp = {Mon, 13 Aug 2018 16:46:42 +0200},
  biburl    = {https://dblp.org/rec/journals/corr/ZagoruykoK16.bib},
  bibsource = {dblp computer science bibliography, https://dblp.org}
}
```
