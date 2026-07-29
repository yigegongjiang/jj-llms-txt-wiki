# Models[[timm.create_model]]

- **model_name** -- Name of model to instantiate.
- **pretrained** -- If set to *True*, load pretrained ImageNet-1k weights.
- **pretrained_cfg** -- Pass in an external pretrained_cfg for model.
- **pretrained_cfg_overlay** -- Replace key-values in base pretrained_cfg with these.
- **checkpoint_path** -- Path of checkpoint to load _after_ the model is initialized.
- **cache_dir** -- Override model cache dir for Hugging Face Hub and Torch checkpoints.
- **scriptable** -- Set layer config so that model is jit scriptable (not working for all models yet).
- **exportable** -- Set layer config so that model is traceable / ONNX exportable (not fully impl/obeyed yet).
- **no_jit** -- Set layer config so that model doesn't utilize jit scripted layers (so far activations only).
Create a model.

Lookup model's entrypoint function and pass relevant args to create a new model.

Tip:
**kwargs will be passed through entrypoint fn to `timm.models.build_model_with_cfg()`
and then the model class __init__(). kwargs values set to None are pruned before passing.

Keyword Args:
drop_rate (float): Classifier dropout rate for training.
drop_path_rate (float): Stochastic depth drop rate for training.
global_pool (str): Classifier global pooling type.

Example:

```py
>>> from timm import create_model

>>> # Create a MobileNetV3-Large model with no pretrained weights.
>>> model = create_model('mobilenetv3_large_100')

>>> # Create a MobileNetV3-Large model with pretrained weights.
>>> model = create_model('mobilenetv3_large_100', pretrained=True)
>>> model.num_classes
1000

>>> # Create a MobileNetV3-Large model with pretrained weights and a new head with 10 classes.
>>> model = create_model('mobilenetv3_large_100', pretrained=True, num_classes=10)
>>> model.num_classes
10

>>> # Create a Dinov2 small model with pretrained weights and save weights in a custom directory.
>>> model = create_model('vit_small_patch14_dinov2.lvd142m', pretrained=True, cache_dir="/data/my-models")
>>> # Data will be stored at */data/my-models/models--timm--vit_small_patch14_dinov2.lvd142m/*
```

- **filter** - Wildcard filter string that works with fnmatch --
- **module** - Limit model selection to a specific submodule (ie 'vision_transformer') --
- **pretrained** - Include only models with valid pretrained weights if True --
- **exclude_filters** - Wildcard filters to exclude models after including them with filter --
- **name_matches_cfg** - Include only models w/ model_name matching default_cfg name (excludes some aliases) --
- **include_tags** - Include pretrained tags in model names (model.tag). If None, defaults --
  set to True when pretrained=True else False (default: None)models - The sorted list of models
Return list of available model names, sorted alphabetically

Example:
model_list('gluon_resnet*') -- returns all models starting with 'gluon_resnet'
model_list('*resnext*, 'resnet') -- returns all models with 'resnext' in 'resnet' module
