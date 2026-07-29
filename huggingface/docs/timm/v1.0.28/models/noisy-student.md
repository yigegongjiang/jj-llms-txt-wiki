# Noisy Student (EfficientNet)

**Noisy Student Training** is a semi-supervised learning approach. It extends the idea of self-training
and distillation with the use of equal-or-larger student models and noise added to the student during learning. It has three main steps: 

1. train a teacher model on labeled images
2. use the teacher to generate pseudo labels on unlabeled images
3. train a student model on the combination of labeled images and pseudo labeled images. 

The algorithm is iterated a few times by treating the student as a teacher to relabel the unlabeled data and training a new student.

Noisy Student Training seeks to improve on self-training and distillation in two ways. First, it makes the student larger than, or at least equal to, the teacher so the student can better learn from a larger dataset. Second, it adds noise to the student so the noised student is forced to learn harder from the pseudo labels. To noise the student, it uses input noise such as RandAugment data augmentation, and model noise such as dropout and stochastic depth during training.

## How do I use this model on an image?

To load a pretrained model:

```py
>>> import timm
>>> model = timm.create_model('tf_efficientnet_b0_ns', pretrained=True)
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

Replace the model name with the variant you want to use, e.g. `tf_efficientnet_b0_ns`. You can find the IDs in the model summaries at the top of this page.

To extract image features with this model, follow the [timm feature extraction examples](../feature_extraction), just change the name of the model you want to use.

## How do I finetune this model?

You can finetune any of the pre-trained models just by changing the classifier (the last layer).

```py
>>> model = timm.create_model('tf_efficientnet_b0_ns', pretrained=True, num_classes=NUM_FINETUNE_CLASSES)
```
To finetune on your own dataset, you have to write a training loop or adapt [timm's training
script](https://github.com/rwightman/pytorch-image-models/blob/master/train.py) to use your dataset.

## How do I train this model?

You can follow the [timm recipe scripts](../training_script) for training a new model afresh.

## Citation

```BibTeX
@misc{xie2020selftraining,
      title={Self-training with Noisy Student improves ImageNet classification}, 
      author={Qizhe Xie and Minh-Thang Luong and Eduard Hovy and Quoc V. Le},
      year={2020},
      eprint={1911.04252},
      archivePrefix={arXiv},
      primaryClass={cs.LG}
}
```
