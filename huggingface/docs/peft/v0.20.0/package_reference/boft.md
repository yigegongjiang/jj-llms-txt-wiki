# BOFT

[Orthogonal Butterfly (BOFT)](https://hf.co/papers/2311.06243) is a generic method designed for finetuning foundation models. It improves the parameter efficiency of the finetuning paradigm -- Orthogonal Finetuning (OFT), by taking inspiration from Cooley-Tukey fast Fourier transform, showing favorable results across finetuning different foundation models, including large vision transformers, large language models and text-to-image diffusion models.

The abstract from the paper is:

*Large foundation models are becoming ubiquitous, but training them from scratch is prohibitively expensive. Thus, efficiently adapting these powerful models to downstream tasks is increasingly important. In this paper, we study a principled finetuning paradigm -- Orthogonal Finetuning (OFT) -- for downstream task adaptation. Despite demonstrating good generalizability, OFT still uses a fairly large number of trainable parameters due to the high dimensionality of orthogonal matrices. To address this, we start by examining OFT from an information transmission perspective, and then identify a few key desiderata that enable better parameter-efficiency. Inspired by how the Cooley-Tukey fast Fourier transform algorithm enables efficient information transmission, we propose an efficient orthogonal parameterization using butterfly structures. We apply this parameterization to OFT, creating a novel parameter-efficient finetuning method, called Orthogonal Butterfly (BOFT). By subsuming OFT as a special case, BOFT introduces a generalized orthogonal finetuning framework. Finally, we conduct an extensive empirical study of adapting large vision transformers, large language models, and text-to-image diffusion models to various downstream tasks in vision and language*.

BOFT focuses on preserving a pretrained model's generative capabilities while being significantly more parameter-efficient than standard [OFT](./oft). Like OFT, BOFT maintains the same cosine similarity ([hyperspherical energy](https://huggingface.co/papers/1805.09298)) between all pairwise neurons in a layer by applying an orthogonal transformation to the pretrained weight matrix, ensuring the semantic relationships among neurons are preserved.

Instead of using a block-diagonal orthogonal matrix, BOFT factorizes the orthogonal transformation into a product of **sparse butterfly matrices** (originally introduced in the [Cooley–Tukey FFT](https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm)). Unlike OFT's block-diagonal rotations, which only mix inputs within each block, the butterfly structure guarantees that every input can influence every output, producing a **dense connectivity** with just `O(d log d)` parameters. This factorization preserves expressivity while drastically reducing the parameter count compared to OFT (at the expense of computation time).

In practice, BOFT multiplies each pretrained weight matrix by a sequence of butterfly-structured orthogonal factors, enabling efficient and expressive neuron rotations. This makes BOFT well-suited for controllable generation and tasks where maintaining the pretrained model's subject representation is critical, while also scaling to larger models with lower memory and compute overhead.

BOFT can be applied to any subset of weight matrices in a neural network to reduce the number of trainable parameters. Given the target layers for injecting BOFT parameters, the number of trainable parameters can be determined based on the size of the weight matrices.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=BOFT"
	frameborder="0"
	width="850"
	height="1000"
>

## Merge BOFT weights into the base model

Similar to LoRA, the weights learned by BOFT can be integrated into the pretrained weight matrices using the [`~BOFTModel.merge_and_unload()` function. This function merges the adapter weights with the base model which allows you to effectively use the newly merged model as a standalone model.

    

This works because during training, the orthogonal weight matrix (R in the diagram above) and the pretrained weight matrices are separate. But once training is complete, these weights can actually be merged (multiplied) into a new weight matrix that is equivalent.

## BOFT Example Usage

For an example of the BOFT method application to various downstream tasks, please refer to the following guides:

Take a look at the following step-by-step guides on how to finetune a model with BOFT:
- [Dreambooth finetuning with BOFT](https://github.com/huggingface/peft/blob/main/examples/boft_dreambooth/boft_dreambooth.md)
- [Controllable generation finetuning with BOFT (ControlNet)](https://github.com/huggingface/peft/blob/main/examples/boft_controlnet/boft_controlnet.md)

For the task of image classification, one can initialize the BOFT config for a DinoV2 model as follows:

```py
import transformers
from transformers import AutoModelForSeq2SeqLM, BOFTConfig
from peft import BOFTConfig, get_peft_model

config = BOFTConfig(
    boft_block_size=4,
    boft_n_butterfly_factor=2,
    target_modules=["query", "value", "key", "output.dense", "mlp.fc1", "mlp.fc2"],
    boft_dropout=0.1,
    bias="boft_only",
    modules_to_save=["classifier"],
)

model = transformers.Dinov2ForImageClassification.from_pretrained(
    "facebook/dinov2-large",
    num_labels=100,
)

boft_model = get_peft_model(model, config)
```

# API

## BOFTConfig[[peft.BOFTConfig]]

#### peft.BOFTConfig[[peft.BOFTConfig]]

```python
peft.BOFTConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, boft_block_size: int = 4, boft_block_num: int = 0, boft_n_butterfly_factor: int = 1, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, boft_dropout: float = 0.0, fan_in_fan_out: bool = False, bias: str = 'none', modules_to_save: Optional[list[str]] = None, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/boft/config.py#L28)

**Parameters:**

boft_block_size (`int`) : BOFT matrix block size across different layers, expressed in `int`. Bigger block sizes results in more dense update matrices with more trainable parameters. Choose `boft_block_size` to be divisible by most layer's input dimension (`in_features`), e.g., 4, 8, 16. Also, please only specify either `boft_block_size` or `boft_block_num`, but not both simultaneously or leaving both to 0, because `boft_block_size` x `boft_block_num` must equal the layer's input dimension.

boft_block_num (`int`) : Number of BOFT blocks per injected layer. Bigger `boft_block_num` result in sparser update matrices with **fewer** trainable parameters. **Note**, please choose `boft_block_num` to be divisible by most layer's input dimension (`in_features`), e.g., 4, 8, 16. Only specify either `boft_block_size` or `boft_block_num`, but not both simultaneously or leaving both to 0, because `boft_block_size` x `boft_block_num` must equal the layer's input dimension.

boft_n_butterfly_factor (`int`) : Number of butterfly factors across different layers. For `boft_n_butterfly_factor=1`, BOFT is the same as vanilla OFT, for `boft_n_butterfly_factor=2`, the effective block size of OFT becomes twice as big and the number of blocks become half.

target_modules (`Union[List[str],str]`) : The names of the modules to apply the adapter to.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

boft_dropout (`float`) : The multiplicative dropout probability, by setting OFT blocks to identity during training, similar to the dropout layer in LoRA.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

bias (`str`) : Bias type for BOFT. Can be 'none', 'all' or 'boft_only'. If 'all' or 'boft_only', the corresponding biases will be updated during training. Be aware that this means that, even when disabling the adapters, the model will not produce the same output as the base model would have without adaptation.

modules_to_save (`List[str]`) --List of modules apart from BOFT layers to be set as trainable and saved in the final checkpoint.

layers_to_transform (`Union[List[int],int]`) : The layer indexes to transform, if this argument is specified, it will apply the BOFT transformations on the layer indexes that are specified in this list. If a single integer is passed, it will apply the BOFT transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None` and if the layer pattern is not in the common layers pattern. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

This is the configuration class to store the configuration of a [BOFTModel](/docs/peft/v0.20.0/en/package_reference/boft#peft.BOFTModel).

## BOFTModel[[peft.BOFTModel]]

#### peft.BOFTModel[[peft.BOFTModel]]

```python
peft.BOFTModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/boft/model.py#L47)

**Parameters:**

model ([transformers.PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([BOFTConfig](/docs/peft/v0.20.0/en/package_reference/boft#peft.BOFTConfig)) : The configuration of the BOFT model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The BOFT model.

Creates BOFT and OFT model from a pretrained transformers model. Paper: https://huggingface.co/papers/2311.06243
https://huggingface.co/papers/2306.07280

Example:
```py
>>> import transformers
>>> from peft import BOFTConfig, get_peft_model

>>> config = BOFTConfig(
...     boft_block_size=8,
...     boft_n_butterfly_factor=1,
...     target_modules=["query", "value", "key", "output.dense", "mlp.fc1", "mlp.fc2"],
...     boft_dropout=0.1,
...     bias="boft_only",
...     modules_to_save=["classifier"],
... )

>>> model = transformers.Dinov2ForImageClassification.from_pretrained(
...     "facebook/dinov2-large",
...     num_labels=100,
... )
>>> boft_model = get_peft_model(model, config)
```

**Attributes**:
- **model** ([transformers.PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([BOFTConfig](/docs/peft/v0.20.0/en/package_reference/boft#peft.BOFTConfig)): The configuration of the BOFT model.
