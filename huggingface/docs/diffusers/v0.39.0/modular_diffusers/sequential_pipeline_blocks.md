# SequentialPipelineBlocks

[SequentialPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks) are a multi-block type that composes other [ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks) together in a sequence. Data flows linearly from one block to the next using `inputs` and `intermediate_outputs`. Each block in [SequentialPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks) usually represents a step in the pipeline, and by combining them, you gradually build a pipeline.

This guide shows you how to connect two blocks into a [SequentialPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks).

Create two [ModularPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.ModularPipelineBlocks). The first block, `InputBlock`, outputs a `batch_size` value and the second block, `ImageEncoderBlock` uses `batch_size` as `inputs`.

```py
from diffusers.modular_pipelines import ModularPipelineBlocks, InputParam, OutputParam

class InputBlock(ModularPipelineBlocks):

    @property
    def inputs(self):
        return [
            InputParam(name="prompt", type_hint=list, description="list of text prompts"),
            InputParam(name="num_images_per_prompt", type_hint=int, description="number of images per prompt"),
        ]

    @property
    def intermediate_outputs(self):
        return [
            OutputParam(name="batch_size", description="calculated batch size"),
        ]

    @property
    def description(self):
        return "A block that determines batch_size based on the number of prompts and num_images_per_prompt argument."

    def __call__(self, components, state):
        block_state = self.get_block_state(state)
        batch_size = len(block_state.prompt)
        block_state.batch_size = batch_size * block_state.num_images_per_prompt
        self.set_block_state(state, block_state)
        return components, state
```

```py
import torch
from diffusers.modular_pipelines import ModularPipelineBlocks, InputParam, OutputParam

class ImageEncoderBlock(ModularPipelineBlocks):

    @property
    def inputs(self):
        return [
            InputParam(name="image", type_hint="PIL.Image", description="raw input image to process"),
            InputParam(name="batch_size", type_hint=int),
        ]

    @property
    def intermediate_outputs(self):
        return [
            OutputParam(name="image_latents", description="latents representing the image"),
        ]

    @property
    def description(self):
        return "Encode raw image into its latent presentation"

    def __call__(self, components, state):
        block_state = self.get_block_state(state)
        # Simulate processing the image
        # This will change the state of the image from a PIL image to a tensor for all blocks
        block_state.image = torch.randn(1, 3, 512, 512)
        block_state.batch_size = block_state.batch_size * 2
        block_state.image_latents = torch.randn(1, 4, 64, 64)
        self.set_block_state(state, block_state)
        return components, state
```

Connect the two blocks by defining a [SequentialPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks). List the block instances in `block_classes` and their corresponding names in `block_names`. The blocks are executed in the order they appear in `block_classes`, and data flows from one block to the next through [PipelineState](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_states#diffusers.modular_pipelines.PipelineState).

```py
class ImageProcessingStep(SequentialPipelineBlocks):
    """
    # auto_docstring
    """
    model_name = "my_model"
    block_classes = [InputBlock(), ImageEncoderBlock()]
    block_names = ["input", "image_encoder"]

    @property
    def description(self):
        return (
            "Process text prompts and images for the pipeline. It:\n"
            " - Determines the batch size from the prompts.\n"
            " - Encodes the image into latent space."
        )
```

When you create a [SequentialPipelineBlocks](/docs/diffusers/v0.39.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks), properties like `inputs`, `intermediate_outputs`, and `expected_components` are automatically aggregated from the sub-blocks, so there is no need to define them again.

There are a few properties you should set:

- `description`: We recommend adding a description for the assembled block to explain what the combined step does.
- `model_name`: This is automatically derived from the sub-blocks but isn't always correct, so you may need to override it.
- `outputs`: By default this is the same as `intermediate_outputs`, but you can manually set it to control which values appear in the doc. This is useful for showing only the final outputs instead of all intermediate values.

These properties, together with the aggregated `inputs`, `intermediate_outputs`, and `expected_components`, are used to automatically generate the `doc` property.

Print the `ImageProcessingStep` block to inspect its sub-blocks, and use `doc` for a full summary of the block's inputs, outputs, and components.

```py
blocks = ImageProcessingStep()
print(blocks)
print(blocks.doc)
```
