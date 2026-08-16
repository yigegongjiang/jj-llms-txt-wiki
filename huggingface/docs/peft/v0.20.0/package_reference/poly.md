# Polytropon

[Polytropon](https://hf.co/papers/2202.13914) is a multitask model with a number of different LoRA adapters in its "inventory". The model learns the correct combination of adapters from the inventory with a routing function to choose the best subset of modules for a specific task. PEFT also supports [Multi-Head Adapter Routing (MHR)](https://hf.co/papers/2211.03831) for Polytropon which builds on and improves the routing function by combining the adapter heads more granularly. The adapter heads are separated into disjoint blocks and a different routing function is learned for each one, allowing for more expressivity.

The abstract from the paper is:

*A modular design encourages neural models to disentangle and recombine different facets of knowledge to generalise more systematically to new tasks. In this work, we assume that each task is associated with a subset of latent discrete skills from a (potentially small) inventory. In turn, skills correspond to parameter-efficient (sparse / low-rank) model parameterisations. By jointly learning these and a task-skill allocation matrix, the network for each task is instantiated as the average of the parameters of active skills. To favour non-trivial soft partitions of skills across tasks, we experiment with a series of inductive biases, such as an Indian Buffet Process prior and a two-speed learning rate. We evaluate our latent-skill model on two main settings: 1) multitask reinforcement learning for grounded instruction following on 8 levels of the BabyAI platform; and 2) few-shot adaptation of pre-trained text-to-text generative models on CrossFit, a benchmark comprising 160 NLP tasks. We find that the modular design of a network significantly increases sample efficiency in reinforcement learning and few-shot generalisation in supervised learning, compared to baselines with fully shared, task-specific, or conditionally generated parameters where knowledge is entangled across tasks. In addition, we show how discrete skills help interpretability, as they yield an explicit hierarchy of tasks.*

The abstract from the paper is:

*Parameter-efficient fine-tuning (PEFT) for cross-task generalization consists in pre-training adapters on a multi-task training set before few-shot adaptation to test tasks. Polytropon [Ponti et al., 2023] (Poly) jointly learns an inventory of adapters and a routing function that selects a (variable-size) subset of adapters for each task during both pre-training and few-shot adaptation. In this paper, we investigate the role that adapter routing plays in its success and design new variants based on our findings. First, we build on the intuition that finer-grained routing provides more expressivity. Hence, we propose MHR (Multi-Head Routing), which combines subsets of adapter parameters and outperforms Poly under a comparable parameter budget; by only fine-tuning the routing function and not the adapters (MHR-z), we achieve competitive performance with extreme parameter efficiency. Second, we find that Poly/MHR performance is a result of better multi-task optimization, rather than modular inductive biases that facilitate adapter recombination and local adaptation, as previously hypothesized. In fact, we find that MHR exhibits higher gradient alignment between tasks than any other method. Since this implies that routing is only crucial during multi-task pre-training, we propose MHR-mu, which discards routing and fine-tunes the average of the pre-trained adapters during few-shot adaptation. This establishes MHR-mu as an effective method for single-adapter fine-tuning.*.

In case you want to try out routing without training first, you can check out the [Arrow LoRA variant](./lora#Arrow).

# API

## PolyConfig[[peft.PolyConfig]]

#### peft.PolyConfig[[peft.PolyConfig]]

```python
peft.PolyConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 8, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, modules_to_save: Optional[list[str]] = None, init_weights: bool = True, poly_type: Literal['poly'] = 'poly', n_tasks: int = 1, n_skills: int = 4, n_splits: int = 1)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/poly/config.py#L25)

**Parameters:**

r (`int`) : Attention dimension of each Lora in Poly.

target_modules (`Union[List[str],str]`) : The names of the modules to apply Poly to.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

modules_to_save (`List[str]`) : List of modules apart from Poly layers to be set as trainable and saved in the final checkpoint.

init_weights (bool) : Whether to perform initialization of Poly weights.

poly_type (`Literal["poly"]`) : The variant of the Poly module to use. Currently, only "poly" is supported.

n_tasks (`int`) : The number of tasks in a multitasking scenario.

n_skills (`int`) : The number of skills (LoRA) in each Poly layer.

n_splits (`int`) : The number of splits within each LoRA of a Poly layer. A value greater than 1 indicates the use of Multi-Head Routing (MHR).

This is the configuration class to store the configuration of a [PolyModel](/docs/peft/v0.20.0/en/package_reference/poly#peft.PolyModel).
- [Polytropon (Poly)](https://huggingface.co/papers/2202.13914)
- [Multi-Head Routing (MHR)](https://huggingface.co/papers/2211.03831)

## PolyModel[[peft.PolyModel]]

#### peft.PolyModel[[peft.PolyModel]]

```python
peft.PolyModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/poly/model.py#L28)
