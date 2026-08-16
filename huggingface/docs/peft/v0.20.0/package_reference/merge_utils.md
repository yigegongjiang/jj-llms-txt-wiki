# Model merge[[peft.utils.merge_utils.prune]]

PEFT provides several internal utilities for [merging LoRA adapters](../developer_guides/model_merging) with the TIES and DARE methods.

#### peft.utils.merge_utils.prune[[peft.utils.merge_utils.prune]]

```python
peft.utils.merge_utils.prune(tensor: Tensor, density: float, method: typing.Literal['magnitude', 'random'], rescale: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L75)

**Parameters:**

tensor (`torch.Tensor`) --The tensor to prune.

density (`float`) --The fraction of values to preserve. Should be in [0,1].

method (`str`) --The method to use to prune. Should be one of ["magnitude", "random"].

rescale (`bool`) --Whether to rescale the result to preserve the expected value of the original tensor.

**Returns:** `torch.Tensor`

The pruned tensor.

Prune the values of task tensors based on the `method`.

#### peft.utils.merge_utils.calculate_majority_sign_mask[[peft.utils.merge_utils.calculate_majority_sign_mask]]

```python
peft.utils.merge_utils.calculate_majority_sign_mask(tensor: Tensor, method: typing.Literal['total', 'frequency'] = 'total')
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L103)

**Parameters:**

tensor (`torch.Tensor`) --The tensor to get the mask from.

method (`str`) --The method to use to get the mask. Should be one of ["total", "frequency"].

**Returns:** `torch.Tensor`

The majority sign mask.

Get the mask of the majority sign across the task tensors. Task tensors are stacked on dimension 0.

#### peft.utils.merge_utils.disjoint_merge[[peft.utils.merge_utils.disjoint_merge]]

```python
peft.utils.merge_utils.disjoint_merge(task_tensors: Tensor, majority_sign_mask: Tensor)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L128)

**Parameters:**

task_tensors (`torch.Tensor`) --The task tensors to merge.

majority_sign_mask (`torch.Tensor`) --The mask of the majority sign across the task tensors.

**Returns:** `torch.Tensor`

The merged tensor.

Merge the task tensors using disjoint merge.

#### peft.utils.merge_utils.task_arithmetic[[peft.utils.merge_utils.task_arithmetic]]

```python
peft.utils.merge_utils.task_arithmetic(task_tensors: list, weights: Tensor)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L144)

**Parameters:**

task_tensors(`List[torch.Tensor]`) --The task tensors to merge.

weights (`torch.Tensor`) --The weights of the task tensors.

**Returns:** `torch.Tensor`

The merged tensor.

Merge the task tensors using `task arithmetic`.

#### peft.utils.merge_utils.ties[[peft.utils.merge_utils.ties]]

```python
peft.utils.merge_utils.ties(task_tensors: list, weights: Tensor, density: float, majority_sign_method: typing.Literal['total', 'frequency'] = 'total')
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L185)

**Parameters:**

task_tensors(`List[torch.Tensor]`) --The task tensors to merge.

weights (`torch.Tensor`) --The weights of the task tensors.

density (`float`) --The fraction of values to preserve. Should be in [0,1].

majority_sign_method (`str`) : The method to use to get the majority sign mask. Should be one of ["total", "frequency"].

**Returns:** `torch.Tensor`

The merged tensor.

Merge the task tensors using `ties`.

#### peft.utils.merge_utils.dare_linear[[peft.utils.merge_utils.dare_linear]]

```python
peft.utils.merge_utils.dare_linear(task_tensors: list, weights: Tensor, density: float)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L217)

**Parameters:**

task_tensors(`List[torch.Tensor]`) --The task tensors to merge.

weights (`torch.Tensor`) --The weights of the task tensors.

density (`float`) --The fraction of values to preserve. Should be in [0,1].

**Returns:** `torch.Tensor`

The merged tensor.

Merge the task tensors using `dare linear`.

#### peft.utils.merge_utils.dare_ties[[peft.utils.merge_utils.dare_ties]]

```python
peft.utils.merge_utils.dare_ties(task_tensors: list, weights: Tensor, density: float, majority_sign_method: typing.Literal['total', 'frequency'] = 'total')
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/utils/merge_utils.py#L239)

**Parameters:**

task_tensors(`List[torch.Tensor]`) --The task tensors to merge.

weights (`torch.Tensor`) --The weights of the task tensors.

density (`float`) --The fraction of values to preserve. Should be in [0,1].

majority_sign_method (`str`) : The method to use to get the majority sign mask. Should be one of ["total", "frequency"].

**Returns:** `torch.Tensor`

The merged tensor.

Merge the task tensors using `dare ties`.
