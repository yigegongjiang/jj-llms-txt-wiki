# IA3

> [!TIP]
> IA3 is an excellent choice when you need the smallest possible adapter size — it uses learned vectors instead of low-rank matrices, making it one of the most parameter-efficient methods available. If your priority is minimal disk/memory footprint and fast training, IA3 is a strong candidate.

    

IA3 introduces three vectors, lv, lk and lff to scale value, key and feed-forward activations (image source).

Infused Adapter by Inhibiting and Amplifying Inner Activations, or [IA3](https://hf.co/papers/2205.05638), is a method that adds three learned vectors to rescale the keys and values of the self-attention and encoder-decoder attention layers, and the intermediate activation of the position-wise feed-forward network.

The abstract from the paper is:

*Few-shot in-context learning (ICL) enables pre-trained language models to perform a previously-unseen task without any gradient-based training by feeding a small number of training examples as part of the input. ICL incurs substantial computational, memory, and storage costs because it involves processing all of the training examples every time a prediction is made. Parameter-efficient fine-tuning (PEFT) (e.g. adapter modules, prompt tuning, sparse update methods, etc.) offers an alternative paradigm where a small set of parameters are trained to enable a model to perform the new task. In this paper, we rigorously compare few-shot ICL and PEFT and demonstrate that the latter offers better accuracy as well as dramatically lower computational costs. Along the way, we introduce a new PEFT method called (IA)^3 that scales activations by learned vectors, attaining stronger performance while only introducing a relatively tiny amount of new parameters. We also propose a simple recipe based on the T0 model called T-Few that can be applied to new tasks without task-specific tuning or modifications. We validate the effectiveness of T-Few on completely unseen tasks by applying it to the RAFT benchmark, attaining super-human performance for the first time and outperforming the state-of-the-art by 6% absolute. All of the code used in our experiments is publicly available*.

To make fine-tuning more efficient, IA3 (Infused Adapter by Inhibiting and Amplifying Inner Activations)
rescales inner activations with learned vectors. These learned vectors are injected in the attention and feedforward modules
in a typical transformer-based architecture. These learned vectors are the only trainable parameters during fine-tuning, and thus the original
weights remain frozen. Dealing with learned vectors (as opposed to learned low-rank updates to a weight matrix like LoRA)
keeps the number of trainable parameters much smaller.

Being similar to [LoRA](./lora), IA3 carries many of the same advantages:

* IA3 makes fine-tuning more efficient by drastically reducing the number of trainable parameters. (For T0, an IA3 model only has about 0.01% trainable parameters, while even LoRA has > 0.1%)
* The original pre-trained weights are kept frozen, which means you can have multiple lightweight and portable IA3 models for various downstream tasks built on top of them.
* Performance of models fine-tuned using IA3 is comparable to the performance of fully fine-tuned models.
* IA3 does not add any inference latency because adapter weights can be merged with the base model.

In principle, IA3 can be applied to any subset of weight matrices in a neural network to reduce the number of trainable
parameters. Following the authors' implementation, IA3 weights are added to the key, value and feedforward layers
of a Transformer model. To be specific, for transformer models, IA3 weights are added to the outputs of key and value layers, and to the input of the second feedforward layer
in each transformer block.

Given the target layers for injecting IA3 parameters, the number of trainable parameters
can be determined based on the size of the weight matrices.

## Usage

> [!TIP]
> For autoregressive models, target `k_proj`, `v_proj`, and `down_proj` — these are the layers the authors found to be most effective. Set `feedforward_modules=["down_proj"]` so IA3 knows which modules are feedforward layers and can apply the correct scaling.

For the task of sequence classification, one can initialize the IA3 config for a Llama model as follows:

```py
from peft import IA3Config, TaskType, get_peft_model
from transformers import AutoModelForCausalLM

model = AutoModelForCausalLM.from_pretrained("HuggingFaceTB/SmolLM-135M")
peft_config = IA3Config(
    task_type=TaskType.CAUSAL_LM,
    target_modules=["k_proj", "v_proj", "down_proj"],
    feedforward_modules=["down_proj"],
)
model = get_peft_model(model, peft_config)
model.print_trainable_parameters()
```

### When to use IA3 vs LoRA

IA3 is particularly effective when you need the smallest possible adapters for storage-constrained deployments, or when you want fast convergence with fewer trainable parameters. LoRA may be preferable when you need the broader ecosystem of LoRA variants (DoRA, QLoRA, etc.) or want the most widely-tested approach with extensive community support.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=IA3"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## IA3Config[[peft.IA3Config]]

#### peft.IA3Config[[peft.IA3Config]]

```python
peft.IA3Config(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, feedforward_modules: Optional[Union[list[str], str]] = None, fan_in_fan_out: bool = False, modules_to_save: Optional[list[str]] = None, init_ia3_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/ia3/config.py#L25)

**Parameters:**

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen, excluding the output layer. If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

feedforward_modules (`Optional[Union[List[str], str]]`) : The names of the modules to be treated as feedforward modules, as in the original paper. These modules will have (IA)³ vectors multiplied to the input, instead of the output. `feedforward_modules` must be a name or a subset of names present in `target_modules`.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

modules_to_save (`Optional[List[str]]`) : List of modules apart from (IA)³ layers to be set as trainable and saved in the final checkpoint.

init_ia3_weights (`bool`) : Whether to initialize the vectors in the (IA)³ layers, defaults to `True`. Setting this to `False` is discouraged.

This is the configuration class to store the configuration of a [IA3Model](/docs/peft/v0.20.0/en/package_reference/ia3#peft.IA3Model).

## IA3Model[[peft.IA3Model]]

#### peft.IA3Model[[peft.IA3Model]]

```python
peft.IA3Model(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/ia3/model.py#L36)

**Parameters:**

model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([IA3Config](/docs/peft/v0.20.0/en/package_reference/ia3#peft.IA3Config)) : The configuration of the (IA)^3 model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The (IA)^3 model.

Creates a Infused Adapter by Inhibiting and Amplifying Inner Activations ((IA)^3) model from a pretrained
transformers model. The method is described in detail in https://huggingface.co/papers/2205.05638

Example:

```py
>>> from transformers import AutoModelForSeq2SeqLM
>>> from peft import IA3Config, get_peft_model

>>> config = IA3Config(
...     peft_type="IA3",
...     task_type="SEQ_2_SEQ_LM",
...     target_modules=["k", "v", "w0"],
...     feedforward_modules=["w0"],
... )

>>> model = AutoModelForSeq2SeqLM.from_pretrained("t5-base")
>>> ia3_model = get_peft_model(model, config)
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([IA3Config](/docs/peft/v0.20.0/en/package_reference/ia3#peft.IA3Config)): The configuration of the (IA)^3 model.

#### add_weighted_adapter[[peft.IA3Model.add_weighted_adapter]]

```python
add_weighted_adapter(adapters: list[str], weights: list[float], adapter_name: str)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/ia3/model.py#L273)

**Parameters:**

adapters (`list`) : List of adapter names to be merged.

weights (`list`) : List of weights for each adapter.

adapter_name (`str`) : Name of the new adapter.

This method adds a new adapter by merging the given adapters with the given weights.
