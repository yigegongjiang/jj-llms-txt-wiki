# Overview

Modular Diffusers is a unified pipeline system that simplifies your workflow with *pipeline blocks*.

- Blocks are reusable and you only need to create new blocks that are unique to your pipeline.
- Blocks can be mixed and matched to adapt to or create a pipeline for a specific workflow or multiple workflows.

The Modular Diffusers docs are organized as shown below.

## Quickstart

- The [quickstart](./quickstart) shows you how to run a modular pipeline, understand its structure, and customize it by modifying the blocks that compose it.

## ModularPipelineBlocks

- [States](./modular_diffusers_states) explains how data is shared and communicated between blocks and [ModularPipeline](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline#diffusers.ModularPipeline).
- [ModularPipelineBlocks](./pipeline_block) is the most basic unit of a [ModularPipeline](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline#diffusers.ModularPipeline) and this guide shows you how to create one.
- [SequentialPipelineBlocks](./sequential_pipeline_blocks) is a type of block that chains multiple blocks so they run one after another, passing data along the chain. This guide shows you how to create [SequentialPipelineBlocks](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline_blocks#diffusers.SequentialPipelineBlocks) and how they connect and work together.
- [LoopSequentialPipelineBlocks](./loop_sequential_pipeline_blocks) is a type of block that runs a series of blocks in a loop. This guide shows you how to create [LoopSequentialPipelineBlocks](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline_blocks#diffusers.LoopSequentialPipelineBlocks).
- [AutoPipelineBlocks](./auto_pipeline_blocks) is a type of block that automatically chooses which blocks to run based on the input. This guide shows you how to create [AutoPipelineBlocks](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline_blocks#diffusers.AutoPipelineBlocks).
- [Building Custom Blocks](./custom_blocks) shows you how to create your own custom blocks and share them on the Hub.

## ModularPipeline

- [ModularPipeline](./modular_pipeline) shows you how to create and convert pipeline blocks into an executable [ModularPipeline](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline#diffusers.ModularPipeline).
- [ComponentsManager](./components_manager) shows you how to manage and reuse components across multiple pipelines.
- [Guiders](../using-diffusers/guiders) shows you how to use different guidance methods in the pipeline.

## Mellon Integration

- [Using Custom Blocks with Mellon](./mellon) shows you how to make your custom blocks work with [Mellon](https://github.com/cubiq/Mellon), a visual node-based interface for building workflows.
