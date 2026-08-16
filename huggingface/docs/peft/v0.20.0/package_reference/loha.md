# LoHa

    

Navigating Text-To-Image Customization: From LyCORIS Fine-Tuning to Model Evaluation

Low-Rank Hadamard Product ([LoHa](https://huggingface.co/papers/2108.06098)), is similar to LoRA except it approximates the large weight matrix with more low-rank matrices and combines them with the Hadamard product. This method is even more parameter-efficient than LoRA and achieves comparable performance. LoHa was originally proposed for federated learning (FedPara) but works well as a general-purpose PEFT method, and is especially popular for fine-tuning image generation models such as Stable Diffusion.

> **Note:** LoHa is part of the [LyCORIS](./adapter_utils) family of adapters. Its close relative [LoKr](./lokr) uses the Kronecker product instead of the Hadamard product.

The abstract from the paper is:

*In this work, we propose a communication-efficient parameterization, FedPara, for federated learning (FL) to overcome the burdens on frequent model uploads and downloads. Our method re-parameterizes weight parameters of layers using low-rank weights followed by the Hadamard product. Compared to the conventional low-rank parameterization, our FedPara method is not restricted to low-rank constraints, and thereby it has a far larger capacity. This property enables to achieve comparable performance while requiring 3 to 10 times lower communication costs than the model with the original layers, which is not achievable by the traditional low-rank methods. The efficiency of our method can be further improved by combining with other efficient FL optimizers. In addition, we extend our method to a personalized FL application, pFedPara, which separates parameters into global and local ones. We show that pFedPara outperforms competing personalized FL methods with more than three times fewer parameters.*

Low-rank decomposition can impact performance because the weight updates are limited to the low-rank space, which can constrain a model's expressiveness. However, you don't necessarily want to use a larger rank because it increases the number of trainable parameters. To address this, LoHa was applied to diffusion models where the ability to generate diverse images is an important consideration. LoHa should also work with general model types, but support for embedding layers isn't currently implemented in PEFT.

LoHa uses the [Hadamard product](https://en.wikipedia.org/wiki/Hadamard_product_(matrices)) (element-wise product) instead of the matrix product. $\Delta W$ is represented by four smaller matrices instead of two - like in LoRA - and each pair of these low-rank matrices are combined with the Hadamard product. As a result, $\Delta W$ can have the same number of trainable parameters but a higher rank and expressivity.

## When to use LoHa

LoHa is a good choice when:

- You are fine-tuning **image generation models** (Stable Diffusion UNet or text encoder), where it is most widely used.
- You want **higher effective rank** than LoRA for the same number of trainable parameters, since the Hadamard product of two low-rank matrices spans a larger subspace than a single low-rank product.
- You want to **combine different PEFT methods** at inference time using [`PeftMixedModel`](./peft_model#peft.PeftMixedModel), for example LoHa together with LoKr.

LoHa supports linear and Conv2d layers. For tasks that additionally require embedding layer adaptation, consider [LoRA](./lora) instead.

## Usage

```python
from diffusers import StableDiffusionPipeline
from peft import LoHaConfig, get_peft_model

config_unet = LoHaConfig(
    r=8,
    alpha=8,
    target_modules=[
        "to_k",
        "to_q",
        "to_v",
        "to_out.0",
        "proj_in",
        "proj_out",
    ],
    rank_dropout=0.0,
    module_dropout=0.0,
    use_effective_conv2d=True,
)

pipeline = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5")
pipeline.unet = get_peft_model(pipeline.unet, config_unet)
pipeline.unet.print_trainable_parameters()
```

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=LOHA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## LoHaConfig[[peft.LoHaConfig]]

#### peft.LoHaConfig[[peft.LoHaConfig]]

```python
peft.LoHaConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, rank_pattern: Optional[dict] = <factory>, alpha_pattern: Optional[dict] = <factory>, r: int = 8, alpha: int = 8, rank_dropout: float = 0.0, module_dropout: float = 0.0, use_effective_conv2d: bool = False, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, modules_to_save: Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/loha/config.py#L24)

**Parameters:**

r (`int`) : LoHa rank.

alpha (`int`) : The alpha parameter for LoHa scaling.

rank_dropout (`float`) : The dropout probability for rank dimension during training.

module_dropout (`float`) : The dropout probability for disabling LoHa modules during training.

use_effective_conv2d (`bool`) : Use parameter effective decomposition for Conv2d (and Conv1d) with ksize > 1 ("Proposition 3" from FedPara paper).

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen, excluding the output layer. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

init_weights (`bool`) : Whether to perform initialization of adapter weights. This defaults to `True`, passing `False` is discouraged.

layers_to_transform (`Union[List[int], int]`) : The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices that are specified in this list. If a single integer is passed, it will apply the transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

rank_pattern (`dict`) : The mapping from layer names or regexp expression to ranks which are different from the default rank specified by `r`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.

alpha_pattern (`dict`) : The mapping from layer names or regexp expression to alphas which are different from the default alpha specified by `alpha`. For example, `{'^model.decoder.layers.0.encoder_attn.k_proj': 16}`.

modules_to_save (`Optional[List[str]]`) : List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [LoHaModel](/docs/peft/v0.20.0/en/package_reference/loha#peft.LoHaModel).

## LoHaModel[[peft.LoHaModel]]

#### peft.LoHaModel[[peft.LoHaModel]]

```python
peft.LoHaModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/loha/model.py#L27)

**Parameters:**

model (`torch.nn.Module`) : The model to which the adapter tuner layers will be attached.

config ([LoHaConfig](/docs/peft/v0.20.0/en/package_reference/loha#peft.LoHaConfig)) : The configuration of the LoHa model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The LoHa model.

Creates Low-Rank Hadamard Product model from a pretrained model. The method is partially described in
https://huggingface.co/papers/2108.06098 Current implementation heavily borrows from
https://github.com/KohakuBlueleaf/LyCORIS/blob/eb460098187f752a5d66406d3affade6f0a07ece/lycoris/modules/loha.py

Example:
```py
>>> from diffusers import StableDiffusionPipeline
>>> from peft import LoHaModel, LoHaConfig

>>> config_te = LoHaConfig(
...     r=8,
...     lora_alpha=32,
...     target_modules=["k_proj", "q_proj", "v_proj", "out_proj", "fc1", "fc2"],
...     rank_dropout=0.0,
...     module_dropout=0.0,
...     init_weights=True,
... )
>>> config_unet = LoHaConfig(
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
>>> model.text_encoder = LoHaModel(model.text_encoder, config_te, "default")
>>> model.unet = LoHaModel(model.unet, config_unet, "default")
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([LoHaConfig](/docs/peft/v0.20.0/en/package_reference/loha#peft.LoHaConfig)): The configuration of the LoHa model.
