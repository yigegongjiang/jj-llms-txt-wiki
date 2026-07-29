# Bridging The Gap between Low-rank and Orthogonal Adaptation via Householder Reflection Adaptation (HRA)

    

Bridging The Gap between Low-rank and Orthogonal Adaptation via Householder Reflection Adaptation

[HRA](https://huggingface.co/papers/2405.17484) provides a new perspective connecting LoRA to OFT, which means it can harness the advantages of both strategies, by leveraging [Householder reflections](https://en.wikipedia.org/wiki/Householder_transformation) to reduce parameters and computation costs while penalizing the loss of pre-training knowledge. It consistently achieves better performance with fewer trainable parameters and outperforms state-of-the-art adapters across different models, including large language models (LLMs) and conditional image generators.

HRA constructs a chain of `r` trainable Householder reflections (HRs). Because the Householder reflection matrix is an orthogonal matrix and the product of orthogonal matrices is also an orthogonal matrix, HRA satisfies the theoretical guarantee of Orthogonal Finetuning (OFT). Meanwhile, HRA can also be viewed as a low-rank fine-tuning adapter. The higher `r`, the more trainable parameters, resulting in a larger model capacity and better performance. Besides, due to the chain structure, the orthogonality of HR planes impacts the capacity and regularity of HRA. To achieve a trade-off between the model capacity and regularity, an orthogonality regularizer of the HR planes is added to the loss function. The weight \\(\lambda\\) can control the strength of the regularizer.

The abstract from the paper is:

> While following different technical routes, both low-rank and orthogonal adaptation techniques can efficiently adapt large-scale pre-training models in specific tasks or domains based on a small piece of trainable parameters. In this study, we bridge the gap between these two techniques, proposing a simple but effective adaptation method based on Householder reflections. Given a pre-trained model, our method fine-tunes its layers by multiplying each frozen weight matrix with an orthogonal matrix constructed by a chain of learnable Householder reflections (HRs). This HR-based orthogonal fine-tuning is equivalent to an adaptive low-rank adaptation. Moreover, we show that the orthogonality of the reflection planes corresponding to the HRs impacts the model capacity and regularity. The analysis motivates us to regularize the orthogonality of the HRs, leading to different implementations of the proposed Householder reflection adaptation (HRA) method. Compared with state-of-the-art methods, HRA achieves superior performance with fewer learnable parameters when adapting large language models and conditional image generators. The code is available at [peft](https://github.com/huggingface/peft/tree/main/src/peft/tuners/hra) and [HRA](https://github.com/DaShenZi721/HRA).

# API

## HRAConfig[[peft.HRAConfig]]

- **r** (`int`) --
  The rank of HRA across different layers. It is best to set 'r' to an even number; otherwise, the default
  initialization method will not work.
- **apply_GS** (`bool`) --
  Whether to apply Gram-Schmidt orthogonalization.
- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear modules are chosen, excluding
  the output layer. If this is not specified, modules will be chosen according to the model architecture. If
  the architecture is not known, an error will be raised -- in this case, you should specify the target
  modules manually.
- **exclude_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to not apply the adapter. When passing a string, a regex match will be performed.
  When passing a list of strings, either an exact match will be performed or it is checked if the name of the
  module ends with any of the passed strings.
- **init_weights** (`bool`) --
  Whether to perform initialization of HRA weights.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices
  that are specified in this list. If a single integer is passed, it will apply the transformations on the
  layer at this index.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the
  `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.
- **bias** (`str`) --
  Bias type for HRA. Can be `'none'`, `'all'` or `'hra_only'`.
- **modules_to_save** (`List[str]`) --
  List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

This is the configuration class to store the configuration of a [HRAModel](/docs/peft/v0.20.0/en/package_reference/hra#peft.HRAModel).

## HRAModel[[peft.HRAModel]]

- **model** (`torch.nn.Module`) -- The model to which the adapter tuner layers will be attached.
- **config** ([HRAConfig](/docs/peft/v0.20.0/en/package_reference/hra#peft.HRAConfig)) -- The configuration of the HRA model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The HRA model.

Creates Householder reflection adaptation (HRA) model from a pretrained model. The method is described in
https://huggingface.co/papers/2405.17484

Example:
```py
>>> from diffusers import StableDiffusionPipeline
>>> from peft import HRAModel, HRAConfig

>>> config_te = HRAConfig(
...     r=8,
...     target_modules=["k_proj", "q_proj", "v_proj", "out_proj", "fc1", "fc2"],
...     init_weights=True,
... )
>>> config_unet = HRAConfig(
...     r=8,
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
...     init_weights=True,
... )

>>> model = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5")
>>> model.text_encoder = HRAModel(model.text_encoder, config_te, "default")
>>> model.unet = HRAModel(model.unet, config_unet, "default")
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([HRAConfig](/docs/peft/v0.20.0/en/package_reference/hra#peft.HRAConfig)): The configuration of the HRA model.
