# Optimization

The `optimum.fx.optimization` module provides a set of torch.fx graph transformations, along with classes and functions to write your own transformations and compose them.

## The transformation guide

In 🤗 Optimum, there are two kinds of transformations: reversible and non-reversible transformations.

### Write a non-reversible transformation

The most basic case of transformations is non-reversible transformations. Those transformations cannot be reversed, meaning that after applying them to a graph module, there is no way to get the original model back. To implement such transformations in 🤗 Optimum, it is very easy: you just need to subclass [Transformation](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation) and implement the [transform()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation.transform) method.

For instance, the following transformation changes all the multiplications to additions:

```python
>>> import operator
>>> from optimum.fx.optimization import Transformation

>>> class ChangeMulToAdd(Transformation):
...     def transform(self, graph_module):
...         for node in graph_module.graph.nodes:
...             if node.op == "call_function" and node.target == operator.mul:
...                 node.target = operator.add
...         return graph_module
```

After implementing it, your transformation can be used as a regular function:

```python
>>> from transformers import BertModel
>>> from transformers.utils.fx import symbolic_trace

>>> model = BertModel.from_pretrained("bert-base-uncased")
>>> traced = symbolic_trace(
...     model,
...     input_names=["input_ids", "attention_mask", "token_type_ids"],
... )

>>> transformation = ChangeMulToAdd()
>>> transformed_model = transformation(traced)
```

### Write a reversible transformation

A reversible transformation implements both the transformation and its reverse, allowing to retrieve the original model from the transformed one. To implement such transformation, you need to subclass [ReversibleTransformation](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.ReversibleTransformation) and implement the [transform()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.Transformation.transform) and [reverse()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.ReversibleTransformation.reverse) methods.

For instance, the following transformation is reversible:

```python
>>> import operator
>>> from optimum.fx.optimization import ReversibleTransformation

>>> class MulToMulTimesTwo(ReversibleTransformation):
...     def transform(self, graph_module):
...         for node in graph_module.graph.nodes:
...             if node.op == "call_function" and node.target == operator.mul:
...                 x, y = node.args
...                 node.args = (2 * x, y)
...         return graph_module
...
...     def reverse(self, graph_module):
...         for node in graph_module.graph.nodes:
...             if node.op == "call_function" and node.target == operator.mul:
...                 x, y = node.args
...                 node.args = (x / 2, y)
...         return graph_module
```

### Composing transformations together

As applying multiple transformations in chain is needed more often that not, [compose()](/docs/optimum/main/en/torch_fx/package_reference/optimization#optimum.fx.optimization.compose) is provided. It is an utility function that allows you to create a transformation by chaining multiple other transformations.

```python
>>> from optimum.fx.optimization import compose
>>> composition = compose(MulToMulTimesTwo(), ChangeMulToAdd())
```
