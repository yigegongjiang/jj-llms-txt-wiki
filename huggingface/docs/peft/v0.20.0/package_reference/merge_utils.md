# Model merge[[peft.utils.merge_utils.prune]]

PEFT provides several internal utilities for [merging LoRA adapters](../developer_guides/model_merging) with the TIES and DARE methods.

- **tensor** (`torch.Tensor`) --The tensor to prune.
- **density** (`float`) --The fraction of values to preserve. Should be in [0,1].
- **method** (`str`) --The method to use to prune. Should be one of ["magnitude", "random"].
- **rescale** (`bool`) --Whether to rescale the result to preserve the expected value of the original tensor.`torch.Tensor`The pruned tensor.

Prune the values of task tensors based on the `method`.

- **tensor** (`torch.Tensor`) --The tensor to get the mask from.
- **method** (`str`) --The method to use to get the mask. Should be one of ["total", "frequency"].`torch.Tensor`The majority sign mask.

Get the mask of the majority sign across the task tensors. Task tensors are stacked on dimension 0.

- **task_tensors** (`torch.Tensor`) --The task tensors to merge.
- **majority_sign_mask** (`torch.Tensor`) --The mask of the majority sign across the task tensors.`torch.Tensor`The merged tensor.

Merge the task tensors using disjoint merge.

- **task_tensors(`List[torch.Tensor]`)** --The task tensors to merge.
- **weights** (`torch.Tensor`) --The weights of the task tensors.`torch.Tensor`The merged tensor.

Merge the task tensors using `task arithmetic`.

- **task_tensors(`List[torch.Tensor]`)** --The task tensors to merge.
- **weights** (`torch.Tensor`) --The weights of the task tensors.
- **density** (`float`) --The fraction of values to preserve. Should be in [0,1].
- **majority_sign_method** (`str`) --
  The method to use to get the majority sign mask. Should be one of ["total", "frequency"].`torch.Tensor`The merged tensor.

Merge the task tensors using `ties`.

- **task_tensors(`List[torch.Tensor]`)** --The task tensors to merge.
- **weights** (`torch.Tensor`) --The weights of the task tensors.
- **density** (`float`) --The fraction of values to preserve. Should be in [0,1].`torch.Tensor`The merged tensor.

Merge the task tensors using `dare linear`.

- **task_tensors(`List[torch.Tensor]`)** --The task tensors to merge.
- **weights** (`torch.Tensor`) --The weights of the task tensors.
- **density** (`float`) --The fraction of values to preserve. Should be in [0,1].
- **majority_sign_method** (`str`) --
  The method to use to get the majority sign mask. Should be one of ["total", "frequency"].`torch.Tensor`The merged tensor.

Merge the task tensors using `dare ties`.
