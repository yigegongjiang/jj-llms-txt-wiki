# Accelerating Inference

Intel Gaudi offers several possibilities to make inference faster.

## Lazy Mode

The following execution modes are supported:
- *Lazy mode*, where operations are accumulated in a graph whose execution is triggered in a lazy manner.
  This allows the graph compiler to optimize the device execution for these operations.
- *Eager mode*, where one operation at a time is executed.
- *Eager mode* with *torch.compile*, where a model (or part of a model) is enclosed into a graph.

Not all models are yet supported with Eager mode and Eager mode with torch.compile (still in development).
Lazy mode is the default mode.

In lazy mode, the graph compiler generates optimized binary code that implements the given model topology on Gaudi.
It performs operator fusion, data layout management, parallelization, pipelining and memory management, as well as graph-level optimizations.

To execute inference in lazy mode, you must provide the following arguments:
```python
args = GaudiTrainingArguments(
    # same arguments as in Transformers,
    use_habana=True,
    use_lazy_mode=True,
)
```

In lazy mode, the last batch may trigger an extra compilation because it could be smaller than previous batches.
To avoid this, you can discard the last batch with `dataloader_drop_last=True`.

## HPU Graphs

Gaudi provides a way to run fast inference with HPU Graphs.
It consists in capturing a series of operations (i.e. graphs) in an HPU stream and then replaying them in an optimized way (more information [here](https://docs.habana.ai/en/latest/PyTorch/Inference_on_Gaudi/Inference_using_HPU_Graphs/Inference_using_HPU_Graphs.html)).
Thus, you can apply this to the `forward` method of your model to run it efficiently at inference.

HPU Graphs are integrated into the `GaudiTrainer` and the `GaudiStableDiffusionPipeline` so that one can use them very easily:
- `GaudiTrainer` needs the training argument `use_hpu_graphs_for_inference` to be set to `True` as follows:
```python
from optimum.habana import GaudiTrainer, GaudiTrainingArguments

# define the training arguments
training_args = GaudiTrainingArguments(
    use_habana=True,
    use_lazy_mode=True,
    use_hpu_graphs_for_inference=True,
    gaudi_config_name=gaudi_config_name,
    ...
)

# Initialize our Trainer
trainer = GaudiTrainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset
    ... # other arguments
)
```
- `GaudiStableDiffusionPipeline` needs its argument `use_hpu_graphs` to be set to `True` such as:
```python
from optimum.habana.diffusers import GaudiDDIMScheduler, GaudiStableDiffusionPipeline

model_name = "CompVis/stable-diffusion-v1-4"

scheduler = GaudiDDIMScheduler.from_pretrained(model_name, subfolder="scheduler")

pipeline = GaudiStableDiffusionPipeline.from_pretrained(
    model_name,
    scheduler=scheduler,
    use_habana=True,
    use_hpu_graphs=True,
    gaudi_config="Habana/stable-diffusion",
)

outputs = generator(
    ["An image of a squirrel in Picasso style"],
    num_images_per_prompt=16,
    batch_size=4,
)
```

With HPU Graphs and in lazy mode, the *first couple of iterations* may be slower due to graph compilations.

## Custom Operators

Intel Gaudi provides a few custom operators that achieve better performance than their PyTorch counterparts.
You can also define your own custom operator for Gaudi as described [here](https://docs.habana.ai/en/latest/PyTorch/PyTorch_CustomOp_API/page_index.html).

### Gaudi Optimized Flash Attention

Flash attention algorithm with additional Intel Gaudi AI Accelerator optimizations is supported for both Lazy and Eager mode.
See [Using Fused Scaled Dot Product Attention (FusedSDPA)](https://docs.habana.ai/en/latest/PyTorch/Model_Optimization_PyTorch/Optimization_in_PyTorch_Models.html#using-fused-scaled-dot-product-attention-fusedsdpa).
