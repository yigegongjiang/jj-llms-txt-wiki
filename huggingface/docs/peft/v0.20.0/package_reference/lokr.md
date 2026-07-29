# LoKr

    

Navigating Text-To-Image Customization: From LyCORIS Fine-Tuning to Model Evaluation

Low-Rank Kronecker Product ([LoKr](https://hf.co/papers/2309.14859)), is a LoRA-variant method that approximates the large weight matrix with two low-rank matrices and combines them with the [Kronecker product](https://en.wikipedia.org/wiki/Kronecker_product). LoKr also provides an optional third low-rank matrix to provide better control during fine-tuning. By expresseing the weight update matrix as a decomposition of a Kronecker product, creating a block matrix, LoKr is able to preserve the rank of the original weight matrix. The size of the smaller matrices are determined by its *rank* or `r`. Another benefit of the Kronecker product is that it can be vectorized by stacking the matrix columns. This can speed up the process because you're avoiding fully reconstructing ∆W.

The abstract from the paper is:

*Text-to-image generative models have garnered immense attention for their ability to produce high-fidelity images from text prompts. Among these, Stable Diffusion distinguishes itself as a leading open-source model in this fast-growing field. However, the intricacies of fine-tuning these models pose multiple challenges from new methodology integration to systematic evaluation. Addressing these issues, this paper introduces LyCORIS [Lora beYond Conventional methods, Other Rank adaptation Implementations for Stable diffusion](https://github.com/KohakuBlueleaf/LyCORIS), an open-source library that offers a wide selection of fine-tuning methodologies for Stable Diffusion. Furthermore, we present a thorough framework for the systematic assessment of varied fine-tuning techniques. This framework employs a diverse suite of metrics and delves into multiple facets of fine-tuning, including hyperparameter adjustments and the evaluation with different prompt types across various concept categories. Through this comprehensive approach, our work provides essential insights into the nuanced effects of fine-tuning parameters, bridging the gap between state-of-the-art research and practical application.*

## Usage

```py
from peft import LoKrConfig, get_peft_model

config = LoKrConfig(
    r=16,
    alpha=16,
    target_modules=["query", "value"],
    module_dropout=0.1,
    modules_to_save=["classifier"],
)
model = get_peft_model(model, config)
model.print_trainable_parameters()
"trainable params: 116,069 || all params: 87,172,042 || trainable%: 0.13314934162033282"
```

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=LOKR"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## LoKrConfig[[peft.LoKrConfig]]

"}, {"name": "alpha_pattern", "val": ": Optional[dict] = "}, {"name": "r", "val": ": int = 8"}, {"name": "alpha", "val": ": int = 8"}, {"name": "rank_dropout", "val": ": float = 0.0"}, {"name": "module_dropout", "val": ": float = 0.0"}, {"name": "use_effective_conv2d", "val": ": bool = False"}, {"name": "decompose_both", "val": ": bool = False"}, {"name": "decompose_factor", "val": ": int = -1"}, {"name": "rank_dropout_scale", "val": ": bool = False"}, {"name": "target_modules", "val": ": Optional[Union[list[str], str]] = None"}, {"name": "exclude_modules", "val": ": Optional[Union[list[str], str]] = None"}, {"name": "init_weights", "val": ": Union[bool, Literal['lycoris']] = True"}, {"name": "layers_to_transform", "val": ": Optional[Union[list[int], int]] = None"}, {"name": "layers_pattern", "val": ": Optional[Union[list[str], str]] = None"}, {"name": "modules_to_save", "val": ": Optional[list[str]] = None"}]}>
- **r** (`int`) --
  LoKr rank.
- **alpha** (`int`) --
  The alpha parameter for LoKr scaling.
- **rank_dropout** (`float`) --
  The dropout probability for rank dimension during training.
- **module_dropout** (`float`) --
  The dropout probability for disabling LoKr modules during training.
- **use_effective_conv2d** (`bool`) --
  Use parameter effective decomposition for Conv2d (and Conv1d) with ksize > 1 ("Proposition 3" from FedPara
  paper).
- **decompose_both** (`bool`) --
  Perform rank decomposition of left kronecker product matrix.
- **decompose_factor** (`int`) --
  Kronecker product decomposition factor.
- **rank_dropout_scale** ('bool) --
  Whether to scale the rank dropout while training, defaults to `False`.
- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen,
  excluding the output layer. If this is not specified, modules will be chosen according to the model
  architecture. If the architecture is not known, an error will be raised -- in this case, you should specify
  the target modules manually.
- **exclude_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to not apply the adapter. When passing a string, a regex match will be performed.
  When passing a list of strings, either an exact match will be performed or it is checked if the name of the
  module ends with any of the passed strings.
- **init_weights** (`bool`) --
  Whether to perform initialization of adapter weights. This defaults to `True`. Use "lycoris" to initialize
  weights in the style of the LYCORIS repository. Passing `False` is discouraged.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices
  that are specified in this list. If a single integer is passed, it will apply the transformations on the
  layer at this index.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the
  `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.
- **rank_pattern** (`dict`) --
  The mapping from layer names or regexp expression to ranks which are different from the default rank
  specified by `r`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.
- **alpha_pattern** (`dict`) --
  The mapping from layer names or regexp expression to alphas which are different from the default alpha
  specified by `alpha`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.
- **modules_to_save** (`Optional[List[str]]`) --
  List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

Configuration class of [LoKrModel](/docs/peft/v0.20.0/en/package_reference/lokr#peft.LoKrModel).

## LoKrModel[[peft.LoKrModel]]

- **model** (`torch.nn.Module`) -- The model to which the adapter tuner layers will be attached.
- **config** ([LoKrConfig](/docs/peft/v0.20.0/en/package_reference/lokr#peft.LoKrConfig)) -- The configuration of the LoKr model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The LoKr model.

Creates Low-Rank Kronecker Product model from a pretrained model. The original method is partially described in
https://huggingface.co/papers/2108.06098 and in https://huggingface.co/papers/2309.14859 Current implementation
heavily borrows from
https://github.com/KohakuBlueleaf/LyCORIS/blob/eb460098187f752a5d66406d3affade6f0a07ece/lycoris/modules/lokr.py

Example:
```py
>>> from diffusers import StableDiffusionPipeline
>>> from peft import LoKrModel, LoKrConfig

>>> config_te = LoKrConfig(
...     r=8,
...     lora_alpha=32,
...     target_modules=["k_proj", "q_proj", "v_proj", "out_proj", "fc1", "fc2"],
...     rank_dropout=0.0,
...     module_dropout=0.0,
...     init_weights=True,
... )
>>> config_unet = LoKrConfig(
...     r=8,
...     lora_alpha=32,
...     target_modules=[
...         "proj_in",
...         "proj_out",
...         "to_k",
...         "to_q",
...         "to_v",
...         "to_out.0",
...         "ff.net.0.proj",
...         "ff.net.2",
...     ],
...     rank_dropout=0.0,
...     module_dropout=0.0,
...     init_weights=True,
...     use_effective_conv2d=True,
... )

>>> model = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5")
>>> model.text_encoder = LoKrModel(model.text_encoder, config_te, "default")
>>> model.unet = LoKrModel(model.unet, config_unet, "default")
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([LoKrConfig](/docs/peft/v0.20.0/en/package_reference/lokr#peft.LoKrConfig)): The configuration of the LoKr model.
