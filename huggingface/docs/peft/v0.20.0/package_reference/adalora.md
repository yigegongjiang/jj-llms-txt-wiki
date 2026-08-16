# AdaLoRA

[AdaLoRA](https://hf.co/papers/2303.10512) (Adaptive LoRA) is a method for optimizing the number of trainable parameters to assign to weight matrices and layers, unlike LoRA, which distributes parameters evenly across all modules. More parameters are budgeted for important weight matrices and layers while less important ones receive fewer parameters. You can control the average desired *rank* or `r` of the matrices, and which modules to apply AdaLoRA to with `target_modules`. Other important parameters to set are `lora_alpha` (scaling factor), and `modules_to_save` (the modules apart from the AdaLoRA layers to be trained and saved). All of these parameters - and more - are found in the [AdaLoraConfig](/docs/peft/v0.20.0/en/package_reference/adalora#peft.AdaLoraConfig).

The abstract from the paper is:

*Fine-tuning large pre-trained language models on downstream tasks has become an important paradigm in NLP. However, common practice fine-tunes all of the parameters in a pre-trained model, which becomes prohibitive when a large number of downstream tasks are present. Therefore, many fine-tuning methods are proposed to learn incremental updates of pre-trained weights in a parameter efficient way, e.g., low-rank increments. These methods often evenly distribute the budget of incremental updates across all pre-trained weight matrices, and overlook the varying importance of different weight parameters. As a consequence, the fine-tuning performance is suboptimal. To bridge this gap, we propose AdaLoRA, which adaptively allocates the parameter budget among weight matrices according to their importance score. In particular, AdaLoRA parameterizes the incremental updates in the form of singular value decomposition. Such a novel approach allows us to effectively prune the singular values of unimportant updates, which is essentially to reduce their parameter budget but circumvent intensive exact SVD computations. We conduct extensive experiments with several pre-trained models on natural language processing, question answering, and natural language generation to validate the effectiveness of AdaLoRA. Results demonstrate that AdaLoRA manifests notable improvement over baselines, especially in the low budget settings. Our code is publicly available at https://github.com/QingruZhang/AdaLoRA*.

> [!WARNING]
> AdaLoRA has an [update_and_allocate()](/docs/peft/v0.20.0/en/package_reference/adalora#peft.AdaLoraModel.update_and_allocate) method that should be called at each training step to update the parameter budget and mask, otherwise the adaptation step is not performed. This requires writing a custom training loop or subclassing the [Trainer](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.Trainer) to incorporate this method. As an example, take a look at this [custom training loop](https://github.com/huggingface/peft/blob/912ad41e96e03652cabf47522cd876076f7a0c4f/examples/conditional_generation/peft_adalora_seq2seq.py#L120).

AdaLoRA manages the parameter budget introduced from LoRA by allocating more parameters - in other words, a higher rank `r` - for important weight matrices that are better adapted for a task and pruning less important ones. The rank is controlled by a method similar to singular value decomposition (SVD). The $\Delta W$ is parameterized with two orthogonal matrices and a diagonal matrix which contains singular values. This parametrization method avoids iteratively applying SVD which is computationally expensive. Based on this method, the rank of $\Delta W$ is adjusted according to an importance score. $\Delta W$ is divided into triplets and each triplet is scored according to its contribution to model performance. Triplets with low importance scores are pruned and triplets with high importance scores are kept for finetuning.

Training with AdaLoRA has three phases: the init phase, the budgeting phase and the final phase. In the initial phase, no budgeting is applied, therefore the ranks are not touched. During the budgeting phase the process described above is applied and the rank is redistributed according to a budget, aiming to give more important adapters more rank and less important layers less. When reaching the final phase, budgeting has ended, the ranks are redistributed but we may continue training for a while with the redistributed ranks to further improve performance.

> [!NOTE]
> **Contributions welcome**: This section needs clarification.
>
> It is unclear how importance is measured. The explanations are also a bit redundant and could benefit from consolidation.
> See [here](../developer_guides/contributing#documentation-improvements) on how to contribute.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=ADALORA"
	frameborder="0"
	width="850"
	height="1000"
>

## Usage

```py
from peft import AdaLoraConfig, get_peft_model

config = AdaLoraConfig(
    r=8,
    init_r=12,
    tinit=200,
    tfinal=1000,
    deltaT=10,
    target_modules=["query", "value"],
    modules_to_save=["classifier"],
)
model = get_peft_model(model, config)
model.print_trainable_parameters()
"trainable params: 520,325 || all params: 87,614,722 || trainable%: 0.5938785036606062"

[... training code ...]

model.update_and_allocate(step_idx)
```

# API

## AdaLoraConfig[[peft.AdaLoraConfig]]

#### peft.AdaLoraConfig[[peft.AdaLoraConfig]]

```python
peft.AdaLoraConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 8, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, lora_alpha: int = 8, lora_dropout: float = 0.0, fan_in_fan_out: bool = False, bias: Literal['none', 'all', 'lora_only'] = 'none', use_rslora: bool = False, modules_to_save: Optional[list[str]] = None, init_lora_weights: bool | Literal['gaussian', 'eva', 'olora', 'pissa', 'pissa_niter_[number of iters]', 'corda', 'loftq', 'orthogonal', 'mica'] = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None, rank_pattern: typing.Optional[dict] = None, alpha_pattern: Optional[dict] = <factory>, megatron_config: Optional[dict] = None, megatron_core: Optional[str] = 'megatron.core', trainable_token_indices: Optional[Union[list[int], dict[str, list[int]]]] = None, loftq_config: Union[LoftQConfig, dict] = <factory>, eva_config: Optional[EvaConfig] = None, corda_config: Optional[CordaConfig] = None, lora_ga_config: Optional[LoraGAConfig] = None, use_dora: bool = False, velora_config: Optional[Union[VeloraConfig, dict]] = None, alora_invocation_tokens: Optional[list[int]] = None, use_qalora: bool = False, qalora_group_size: int = 16, monteclora_config: Optional[MontecloraConfig] = None, layer_replication: Optional[list[tuple[int, int]]] = None, runtime_config: LoraRuntimeConfig = <factory>, lora_bias: bool = False, target_parameters: Optional[list[str]] = None, use_bdlora: Optional[BdLoraConfig] = None, arrow_config: Optional[ArrowConfig] = None, ensure_weight_tying: bool = False, target_r: int = 8, init_r: int = 12, tinit: int = 0, tfinal: int = 0, deltaT: int = 1, beta1: float = 0.85, beta2: float = 0.85, orth_reg_weight: float = 0.5, total_step: typing.Optional[int] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adalora/config.py#L24)

**Parameters:**

target_r (`int`) : The target average rank of incremental matrix.

init_r (`int`) : The initial rank for each incremental matrix.

tinit (`int`) : The steps of initial fine-tuning warmup.

tfinal (`int`) : The number of steps of final fine-tuning.

deltaT (`int`) : The time internval between two budget allocations.

beta1 (`float`) : The hyperparameter of EMA for sensitivity smoothing.

beta2 (`float`) : The hyperparameter of EMA for undertainty quantification.

orth_reg_weight (`float`) : The coefficient of orthogonal regularization.

total_step (`int`) : The total training steps that should be specified before training.

rank_pattern (`list`) : The allocated rank for each weight matrix by RankAllocator.

This is the configuration class to store the configuration of a [AdaLoraModel](/docs/peft/v0.20.0/en/package_reference/adalora#peft.AdaLoraModel).

AdaLoRA has three phases defined by `tinit`, `tfinal` and `total_step`.

The initial phase can be understood as a step for pre-training the adapters so that when reducing their rank, there
is already some information encoded that can be reduced instead of random matrices. This phase is defined by
supplying `tinit`.

After the initial phase is over (`tinit` steps have passed) and the final phase has not begun, AdaLoRA reduces the
budget of how much rank each layer is allowed to have with each step. This is where the reduction of rank is
happening. This goes on until `total_step - tfinal` steps are reached.

The last phase, beginning once `total_step - tfinal` steps are reached, does not change the layer ranks anymore but
fine-tunes the reduced-rank layers that resulted from the previous phase.

A practical example: `tinit` is 10, `tfinal` is 20, `total_step` is 100. We spend 10 steps doing pre-training
without rank reduction because our budget is constant (init phase), then we spend 80 (100-20) steps in the
reduction phase where our budget decreases step-wise and, finally, 20 steps in the final fine-tuning stage without
reduction.

## AdaLoraModel[[peft.AdaLoraModel]]

#### peft.AdaLoraModel[[peft.AdaLoraModel]]

```python
peft.AdaLoraModel(model, config, adapter_name, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adalora/model.py#L36)

**Parameters:**

model ([transformers.PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to be adapted.

config ([AdaLoraConfig](/docs/peft/v0.20.0/en/package_reference/adalora#peft.AdaLoraConfig)) : The configuration of the AdaLora model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

**Returns:** `torch.nn.Module`

The AdaLora model.

Creates AdaLoRA (Adaptive LoRA) model from a pretrained transformers model. Paper:
https://openreview.net/forum?id=lq62uWRJjiY

Example:
```py
>>> from transformers import AutoModelForSeq2SeqLM
>>> from peft import AdaLoraConfig, get_peft_model

>>> config = AdaLoraConfig(
...     peft_type="ADALORA",
...     task_type="SEQ_2_SEQ_LM",
...     init_r=12,
...     lora_alpha=32,
...     target_modules=["q", "v"],
...     lora_dropout=0.01,
...     total_step=1000,
... )
>>> model = AutoModelForSeq2SeqLM.from_pretrained("t5-base")
>>> adalora_model = get_peft_model(model, config)
```

**Attributes**:
- **model** ([transformers.PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([AdaLoraConfig](/docs/peft/v0.20.0/en/package_reference/adalora#peft.AdaLoraConfig)): The configuration of the AdaLora model.

#### add_weighted_adapter[[peft.AdaLoraModel.add_weighted_adapter]]

```python
add_weighted_adapter(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adalora/model.py#L347)

This method is not supported for AdaLoRA, use LoRA instead.

#### update_and_allocate[[peft.AdaLoraModel.update_and_allocate]]

```python
update_and_allocate(global_step)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adalora/model.py#L305)

**Parameters:**

global_step (`int`) : The current training step, it is used to calculate adalora budget.

This method updates Adalora budget and mask.

This should be called in every training step after `loss.backward()` and before `zero_grad()`.

`tinit`, `tfinal` and `deltaT` are handled with in the method.

Example:

```python
>>> loss = model(**input).loss
>>> loss.backward()
>>> optimizer.step()
>>> model.base_model.update_and_allocate(i_step)
>>> optimizer.zero_grad()
```
