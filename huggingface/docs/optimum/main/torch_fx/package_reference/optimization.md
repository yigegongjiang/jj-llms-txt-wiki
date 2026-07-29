# Optimization

## Transformation[[optimum.fx.optimization.Transformation]]

#### optimum.fx.optimization.Transformation[[optimum.fx.optimization.Transformation]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L85)

A torch.fx graph transformation.

It  must implement the [transform()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation.transform) method, and be used as a
callable.

__call__optimum.fx.optimization.Transformation.__call__https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L108[{"name": "graph_module", "val": ": GraphModule"}, {"name": "lint_and_recompile", "val": ": bool = True"}]- **graph_module** (`torch.fx.GraphModule`) --
  The module to transform.
- **lint_and_recompile** (`bool`, defaults to `True`) --
  Whether the transformed module should be linted and recompiled.
  This can be set to `False` when chaining transformations together to perform this operation only once.0`torch.fx.GraphModule`The transformed module.

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

**Returns:**

``torch.fx.GraphModule``

The transformed module.
#### get_transformed_nodes[[optimum.fx.optimization.Transformation.get_transformed_nodes]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L161)

**Parameters:**

graph_module (`torch.fx.GraphModule`) : The graph_module to get the nodes from.

**Returns:**

``List[torch.fx.Node]``

Gives the list of nodes that were transformed by the transformation.
#### mark_as_transformed[[optimum.fx.optimization.Transformation.mark_as_transformed]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L137)

Marks a node as transformed by this transformation.

**Parameters:**

node (`torch.fx.Node`) : The node to mark as transformed.
#### transform[[optimum.fx.optimization.Transformation.transform]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L95)

**Parameters:**

graph_module (`torch.fx.GraphModule`) : The module to transform.

**Returns:**

``torch.fx.GraphModule``

The transformed module.
#### transformed[[optimum.fx.optimization.Transformation.transformed]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L149)

**Parameters:**

node (`torch.fx.Node`) : The node to check.

**Returns:**

``bool``

Specifies whether the node was transformed by this transformation or not.

## Reversible transformation[[optimum.fx.optimization.ReversibleTransformation]]

#### optimum.fx.optimization.ReversibleTransformation[[optimum.fx.optimization.ReversibleTransformation]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L176)

A torch.fx graph transformation that is reversible.

It must implement the [transform()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation.transform) and
[reverse()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.ReversibleTransformation.reverse) methods, and be used as a callable.

__call__optimum.fx.optimization.ReversibleTransformation.__call__https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L197[{"name": "graph_module", "val": ": GraphModule"}, {"name": "lint_and_recompile", "val": ": bool = True"}, {"name": "reverse", "val": ": bool = False"}]- **graph_module** (`torch.fx.GraphModule`) --
  The module to transform.
- **lint_and_recompile** (`bool`, defaults to `True`) --
  Whether the transformed module should be linted and recompiled.
  This can be set to `False` when chaining transformations together to perform this operation only once.
- **reverse** (`bool`, defaults to `False`) --
  If `True`, the reverse transformation is performed.0`torch.fx.GraphModule`The transformed module.

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

**Returns:**

``torch.fx.GraphModule``

The transformed module.
#### mark_as_restored[[optimum.fx.optimization.ReversibleTransformation.mark_as_restored]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L222)

Marks a node as restored back to its original state.

**Parameters:**

node (`torch.fx.Node`) : The node to mark as restored.
#### reverse[[optimum.fx.optimization.ReversibleTransformation.reverse]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L184)

**Parameters:**

graph_module (`torch.fx.GraphModule`) : The module to transform.

**Returns:**

``torch.fx.GraphModule``

The reverse transformed module.

#### optimum.fx.optimization.compose[[optimum.fx.optimization.compose]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L721)

Composes a list of transformations together.

Example:

```python
>>> from transformers import BertModel
>>> from transformers.utils.fx import symbolic_trace
>>> from optimum.fx.optimization import ChangeTrueDivToMulByInverse, MergeLinears, compose

>>> model = BertModel.from_pretrained("bert-base-uncased")
>>> traced = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "token_type_ids"],
... )
>>> composition = compose(ChangeTrueDivToMulByInverse(), MergeLinears())
>>> transformed_model = composition(traced)
```

**Parameters:**

args ([Transformation](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation)) : The transformations to compose together.

inplace (`bool`, defaults to `True`) : Whether the resulting transformation should be inplace, or create a new graph module.

**Returns:**

The composition transformation object.

### Transformations[[optimum.fx.optimization.MergeLinears]]

#### optimum.fx.optimization.MergeLinears[[optimum.fx.optimization.MergeLinears]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L237)

Transformation that merges linear layers that take the same input into one big linear layer.

Example:

```python
>>> from transformers import BertModel
>>> from transformers.utils.fx import symbolic_trace
>>> from optimum.fx.optimization import MergeLinears

>>> model = BertModel.from_pretrained("bert-base-uncased")
>>> traced = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "token_type_ids"],
... )
>>> transformation = MergeLinears()
>>> transformed_model = transformation(traced)
>>> restored_model = transformation(transformed_model, reverse=True)
```

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

#### optimum.fx.optimization.FuseBiasInLinear[[optimum.fx.optimization.FuseBiasInLinear]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L393)

Transformation that fuses the bias to the weight in torch.nn.Linear.

Example:

```python
>>> from transformers import BertModel
>>> from transformers.utils.fx import symbolic_trace
>>> from optimum.fx.optimization import FuseBiasInLinear

>>> model = BertModel.from_pretrained("bert-base-uncased")
>>> traced = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "token_type_ids"],
... )
>>> transformation = FuseBiasInLinear()
>>> transformed_model = transformation(traced)
>>> restored_model = transformation(transformed_model, reverse=True)
```

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

#### optimum.fx.optimization.ChangeTrueDivToMulByInverse[[optimum.fx.optimization.ChangeTrueDivToMulByInverse]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L447)

Transformation that changes truediv nodes to multiplication by the inverse nodes when the denominator is static.
For example, that is sometimes the case for the scaling factor in attention layers.

Example:

```python
>>> from transformers import BertModel
>>> from transformers.utils.fx import symbolic_trace
>>> from optimum.fx.optimization import ChangeTrueDivToMulByInverse

>>> model = BertModel.from_pretrained("bert-base-uncased")
>>> traced = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "token_type_ids"],
... )
>>> transformation = ChangeTrueDivToMulByInverse()
>>> transformed_model = transformation(traced)
>>> restored_model = transformation(transformed_model, reverse=True)
```

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

#### optimum.fx.optimization.FuseBatchNorm2dInConv2d[[optimum.fx.optimization.FuseBatchNorm2dInConv2d]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L478)

Transformation that fuses `nn.BatchNorm2d` following `nn.Conv2d` into a single `nn.Conv2d`.
The fusion will be done only if the convolution has the batch normalization as sole following node.

For example, fusion will not be done in the case

```
     Conv2d
     /   \
    /     \
ReLU   BatchNorm2d
```

Example:
```python
>>> from transformers.utils.fx import symbolic_trace
>>> from transformers import AutoModelForImageClassification

>>> from optimum.fx.optimization import FuseBatchNorm2dInConv2d

>>> model = AutoModelForImageClassification.from_pretrained("microsoft/resnet-50")
>>> model.eval()
>>> traced_model = symbolic_trace(
...     model,
...     input_names=["pixel_values"],
...     disable_check=True
... )

>>> transformation = FuseBatchNorm2dInConv2d()
>>> transformed_model = transformation(traced_model)
```

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.

#### optimum.fx.optimization.FuseBatchNorm1dInLinear[[optimum.fx.optimization.FuseBatchNorm1dInLinear]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/fx/optimization/transformations.py#L561)

Transformation that fuses `nn.BatchNorm1d` following or preceding `nn.Linear` into a single `nn.Linear`.
The fusion will be done only if the linear layer has the batch normalization as sole following node, or the batch normalization
has the linear layer as sole following node.

For example, fusion will not be done in the case

```
     Linear
     /   \
    /     \
ReLU   BatchNorm1d
```

Example:
```python
>>> from transformers.utils.fx import symbolic_trace
>>> from transformers import AutoModel

>>> from optimum.fx.optimization import FuseBatchNorm1dInLinear

>>> model = AutoModel.from_pretrained("nvidia/groupvit-gcc-yfcc")
>>> model.eval()
>>> traced_model = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "pixel_values"],
...     disable_check=True
... )

>>> transformation = FuseBatchNorm1dInLinear()
>>> transformed_model = transformation(traced_model)
```

**Parameters:**

preserves_computation (`bool`, defaults to `False`) : Whether the transformation preserves the graph computation or not. If `True`, the original and the transformed graph should produce the same outputs.
