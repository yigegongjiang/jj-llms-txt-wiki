# GraLoRA

[**Granular Low-Rank Adaptation (GraLoRA)**](https://huggingface.co/papers/2505.20355) is a PEFT method designed to enhance the **expressivity** of low-rank adaptation while improving **robustness to outlier** activations, based on insights from well-known issues in quantization.

![GraLoRA Overview](https://github.com/SqueezeBits/GraLoRA/raw/main/figure/gralora_overview.png)

Unlike standard LoRA, which applies a single low-rank adapter across the entire feature space, GraLoRA introduces a structured and fine-grained adaptation scheme. It divides the adaptation space into a grid of $𝑘^2$ smaller, independent adapter pairs, each responsible for a localized subset of the input and output dimensions. As a result, each adapter operates on a subspace that is $k$ times smaller in both dimensions than the original LoRA adapter.

This granular decomposition enables spatially localized and context-aware updates, effectively increasing representational capacity without additional parameters or computational cost. By isolating the influence of extreme activations within smaller subspaces, GraLoRA mitigates gradient distortion and preserves inter-channel balance during adaptation.

---

The abstract from the paper is:

*Low-Rank Adaptation (LoRA) is a popular method for parameter-efficient fine-
tuning (PEFT) of generative models, valued for its simplicity and effectiveness.
Despite recent enhancements, LoRA still suffers from a fundamental limitation:
overfitting when the bottleneck is widened. It performs best at ranks 32–64, yet its
accuracy stagnates or declines at higher ranks, still falling short of full fine-tuning
(FFT) performance. We identify the root cause as LoRA’s structural bottleneck,
which introduces gradient entanglement to the unrelated input channels and distorts
gradient propagation. To address this, we introduce a novel structure, Granular
Low-Rank Adaptation (GraLoRA) that partitions weight matrices into sub-blocks,
each with its own low-rank adapter. With negligible computational or storage cost,
GraLoRA overcomes LoRA’s limitations, effectively increases the representational
capacity, and more closely approximates FFT behavior. Experiments on code
generation, commonsense reasoning, mathematical reasoning, general language
understanding, and image generation benchmarks show that GraLoRA consistently
outperforms LoRA and other baselines, achieving up to +8.5% absolute gain in
Pass@1 on HumanEval+. These improvements hold across model sizes and rank
settings, making GraLoRA a scalable and robust solution for PEFT.*

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=GRALORA"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## GraloraConfig[[peft.GraloraConfig]]

- **r** (`int`) --
  GraLoRA attention dimension determines the rank of the GraLoRA adapter. The total parameter count of the
  GraLoRA adapter is same as LoRA with same rank r, while the expressivitiy is multiplied by gralora_k.
- **hybrid_r** (`int`) --
  Hybrid GraLoRA rank determines the rank allocated to vanilla LoRA method when using Hybrid GraLoRA method.
  Hybrid GraLoRA, a combination of GraLoRA and vanilla LoRA, becomes available when hybrid_r > 0. The
  parameter count of the GraLoRA adapter is r + hybrid_r.
- **target_modules** (`Union[List[str], str]`) --
  List of module names or regex expression of the module names to replace with GraLoRA. " For example, ['q',
  'v'] or '.*decoder.*(SelfAttention|EncDecAttention).*(q|v)$'. " This can also be a wildcard 'all-linear'
  which matches all linear/Conv1D " "(if the model is a PreTrainedModel, the output layer excluded). " If not
  specified, modules will be chosen according to the model architecture, If the architecture is " not known,
  an error will be raised -- in this case, you should specify the target modules manually. " To avoid
  targeting any modules (because you want to apply `target_parameters`), set " `target_modules=[]`.
- **alpha** (`int`) -- GraLoRA alpha.
  GraLoRA alpha is the scaling factor for the GraLoRA adapter. Scale becomes alpha / (r + hybrid_r).
- **gralora_dropout** (`float`) --
  GraLoRA dropout is the dropout probability for the GraLoRA adapter. It is used to prevent overfitting and
  improve the generalization of the GraLoRA adapter.
- **gralora_k** (`int`) --
  GraLoRA k determines the number of subblocks in the GraLoRA adapter. The rank r must be divisible by
  gralora_k for the GraLoRA adapter to be valid. The total parameter count is preserved regardless of
  gralora_k. The entire rank of the GraLoRA adapter is increased by gralora_k, while the rank of each
  subblock is reduced by gralora_k. gralora_k=2 is recommended for rank 32 or lower, and gralora_k=4 is
  recommended for rank 64 or higher.
- **fan_in_fan_out** (`bool`) --
  Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses
  `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.
- **bias** (`str`) --
  Bias type for gralora. Can be 'none', 'all' or 'gralora_only'. If 'all' or 'gralora_only', the
  corresponding biases will be updated during training. Be aware that this means that, even when disabling
  the adapters, the model will not produce the same output as the base model would have without adaptation.
- **modules_to_save** (`Optional[list[str]]`) --
  List of modules apart from gralora layers to be set as trainable and saved in the final checkpoint. For
  example, in Sequence Classification or Token Classification tasks, the final layer `classifier/score` are
  randomly initialized and as such need to be trainable and saved.
- **init_weights** (`bool`) --
  Whether to initialize the weights of the GraLoRA layers with their default initialization. Don't change
  this setting, except if you know exactly what you're doing.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indexes to transform, is this argument is specified, PEFT will transform only the layers indexes
  that are specified inside this list. If a single integer is passed, PEFT will transform only the layer at
  this index. This only works when target_modules is a list of str.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different to None and if the layer pattern is
  not in the common layers pattern. This only works when target_modules is a list of str. This should target
  the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

This is the configuration class to store the configuration of a [GraloraModel](/docs/peft/v0.20.0/en/package_reference/gralora#peft.GraloraModel).

## GraloraModel[[peft.GraloraModel]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **config** ([GraloraConfig](/docs/peft/v0.20.0/en/package_reference/gralora#peft.GraloraConfig)) -- The configuration of the Gralora model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.`torch.nn.Module`The Gralora model.

Creates Vector-based Random Matrix Adaptation (Gralora) model from a pretrained transformers model.

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import GraloraConfig, get_peft_model

>>> base_model = AutoModelForCausalLM.from_pretrained("facebook/opt-125m")
>>> config = GraloraConfig(r=128)
>>> model = get_peft_model(base_model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([GraloraConfig](/docs/peft/v0.20.0/en/package_reference/gralora#peft.GraloraConfig)): The configuration of the Gralora model.
