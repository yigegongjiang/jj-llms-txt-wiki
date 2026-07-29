# Pipeline blocks

## ModularPipelineBlocks[[diffusers.ModularPipelineBlocks]]

#### diffusers.ModularPipelineBlocks[[diffusers.ModularPipelineBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L307)

Base class for all Pipeline Blocks: ConditionalPipelineBlocks, AutoPipelineBlocks, SequentialPipelineBlocks,
LoopSequentialPipelineBlocks

[ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks) provides method to load and save the definition of pipeline blocks.

> [!WARNING] > This is an experimental feature and is likely to change in the future.

get_block_statediffusers.ModularPipelineBlocks.get_block_statehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L495[{"name": "state", "val": ": PipelineState"}]
Get all inputs and intermediates in one dictionary
#### get_execution_blocks[[diffusers.ModularPipelineBlocks.get_execution_blocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L377)

Get the block(s) that would execute given the inputs. Must be implemented by subclasses that support
conditional block selection.

**Parameters:**

- ****kwargs** : Input names and values. Only trigger inputs affect block selection.
#### get_workflow[[diffusers.ModularPipelineBlocks.get_workflow]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L395)

Get the execution blocks for a specific workflow. Must be implemented by subclasses that define
`_workflow_map`.

**Parameters:**

workflow_name : Name of the workflow to retrieve.
#### init_pipeline[[diffusers.ModularPipelineBlocks.init_pipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L473)

create a ModularPipeline, optionally accept pretrained_model_name_or_path to load from hub.

## SequentialPipelineBlocks[[diffusers.SequentialPipelineBlocks]]

#### diffusers.SequentialPipelineBlocks[[diffusers.SequentialPipelineBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L942)

A Pipeline Blocks that combines multiple pipeline block classes into one. When called, it will call each block in
sequence.

This class inherits from [ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks). Check the superclass documentation for the generic methods the
library implements for all the pipeline blocks (such as loading or saving etc.)

> [!WARNING] > This is an experimental feature and is likely to change in the future.

from_blocks_dictdiffusers.SequentialPipelineBlocks.from_blocks_dicthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1009[{"name": "blocks_dict", "val": ": dict"}, {"name": "description", "val": ": str | None = None"}]- **blocks_dict** -- Dictionary mapping block names to block classes or instances0A new SequentialPipelineBlocks instance
Creates a SequentialPipelineBlocks instance from a dictionary of blocks.

**Parameters:**

block_classes : list of block classes to be used

block_names : list of prefixes for each block

**Returns:**

A new SequentialPipelineBlocks instance
#### get_execution_blocks[[diffusers.SequentialPipelineBlocks.get_execution_blocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1152)

Get the blocks that would execute given the specified inputs.

As the traversal walks through sequential blocks, intermediate outputs from resolved blocks are added to the
active inputs. This means conditional blocks that depend on intermediates (e.g., "run img2img if image_latents
is present") will resolve correctly, as long as the condition is based on presence/absence (None or not None),
not on the actual value.

**Parameters:**

- ****kwargs** : Input names and values. Only trigger inputs affect block selection.

**Returns:**

SequentialPipelineBlocks containing only the blocks that would execute

## LoopSequentialPipelineBlocks[[diffusers.LoopSequentialPipelineBlocks]]

#### diffusers.LoopSequentialPipelineBlocks[[diffusers.LoopSequentialPipelineBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1297)

A Pipeline blocks that combines multiple pipeline block classes into a For Loop. When called, it will call each
block in sequence.

This class inherits from [ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks). Check the superclass documentation for the generic methods the
library implements for all the pipeline blocks (such as loading or saving etc.)

> [!WARNING] > This is an experimental feature and is likely to change in the future.

from_blocks_dictdiffusers.LoopSequentialPipelineBlocks.from_blocks_dicthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1460[{"name": "blocks_dict", "val": ": dict"}]- **blocks_dict** -- Dictionary mapping block names to block instances0A new LoopSequentialPipelineBlocks instance

Creates a LoopSequentialPipelineBlocks instance from a dictionary of blocks.

**Parameters:**

block_classes : list of block classes to be used

block_names : list of prefixes for each block

**Returns:**

A new LoopSequentialPipelineBlocks instance

## AutoPipelineBlocks[[diffusers.AutoPipelineBlocks]]

#### diffusers.AutoPipelineBlocks[[diffusers.AutoPipelineBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L881)

A Pipeline Blocks that automatically selects a block to run based on the presence of trigger inputs.

This is a specialized version of `ConditionalPipelineBlocks` where:
- Each block has one corresponding trigger input (1:1 mapping)
- Block selection is automatic: the first block whose trigger input is present gets selected
- `block_trigger_inputs` must have the same length as `block_names` and `block_classes`
- Use `None` in `block_trigger_inputs` to specify the default block, i.e the block that will run if no trigger
  inputs are present

Example:
```python
    class MyAutoBlock(AutoPipelineBlocks):
        block_classes = [InpaintEncoderBlock, ImageEncoderBlock, TextEncoderBlock]
        block_names = ["inpaint", "img2img", "text2img"]
        block_trigger_inputs = ["mask_image", "image", None]  # text2img is the default
```

With this definition:
- As long as `mask_image` is provided, "inpaint" block runs (regardless of `image` being provided or not)
- If `mask_image` is not provided but `image` is provided, "img2img" block runs
- Otherwise, "text2img" block runs (default, trigger is `None`)

select_blockdiffusers.AutoPipelineBlocks.select_blockhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L934[{"name": "**kwargs", "val": ""}]
Select block based on which trigger input is present (not None).

**Parameters:**

block_classes : List of block classes to be used. Must have the same length as `block_names` and `block_trigger_inputs`.

block_names : List of names for each block. Must have the same length as `block_classes` and `block_trigger_inputs`.

block_trigger_inputs : List of input names where each element specifies the trigger input for the corresponding block. Use `None` to mark the default block.

## ConditionalPipelineBlocks[[diffusers.ConditionalPipelineBlocks]]

#### diffusers.ConditionalPipelineBlocks[[diffusers.ConditionalPipelineBlocks]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L580)

A Pipeline Blocks that conditionally selects a block to run based on the inputs. Subclasses must implement the
`select_block` method to define the logic for selecting the block. Currently, we only support selection logic based
on the presence or absence of inputs (i.e., whether they are `None` or not)

This class inherits from [ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks). Check the superclass documentation for the generic methods the
library implements for all the pipeline blocks (such as loading or saving etc.)

> [!WARNING] > This is an experimental feature and is likely to change in the future.

get_execution_blocksdiffusers.ConditionalPipelineBlocks.get_execution_blockshttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L772[{"name": "**kwargs", "val": ""}]- ****kwargs** -- Input names and values. Only trigger inputs affect block selection.0- `ModularPipelineBlocks`A leaf block or resolved `SequentialPipelineBlocks`
- `None`: If this block would be skipped (no trigger matched and no default)

Get the block(s) that would execute given the inputs.

Recursively resolves nested ConditionalPipelineBlocks until reaching either:
- A leaf block (no sub_blocks or LoopSequentialPipelineBlocks) → returns single `ModularPipelineBlocks`
- A `SequentialPipelineBlocks` → delegates to its `get_execution_blocks()` which returns
a `SequentialPipelineBlocks` containing the resolved execution blocks

**Parameters:**

block_classes : List of block classes to be used. Must have the same length as `block_names`.

block_names : List of names for each block. Must have the same length as `block_classes`.

block_trigger_inputs : List of input names that `select_block()` uses to determine which block to run. For `ConditionalPipelineBlocks`, this does not need to correspond to `block_names` and `block_classes`. For `AutoPipelineBlocks`, this must have the same length as `block_names` and `block_classes`, where each element specifies the trigger input for the corresponding block.

default_block_name : Name of the default block to run when no trigger inputs match. If None, this block can be skipped entirely when no trigger inputs are provided.

**Returns:**

`- `ModularPipelineBlocks``

A leaf block or resolved `SequentialPipelineBlocks`
- `None`: If this block would be skipped (no trigger matched and no default)
#### select_block[[diffusers.ConditionalPipelineBlocks.select_block]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L728)

Select the block to run based on the trigger inputs. Subclasses must implement this method to define the logic
for selecting the block.

Note: When trigger inputs include intermediate outputs from earlier blocks, the selection logic should only
depend on the presence or absence of the input (i.e., whether it is None or not), not on its actual value. This
is because `get_execution_blocks()` resolves conditions statically by propagating intermediate output names
without their runtime values.

**Parameters:**

- ****kwargs** : Trigger input names and their values from the state.

**Returns:**

`str | None`

The name of the block to run, or None to use default/skip.
