# Pipeline parallelism

Accelerate supports pipeline parallelism for large-scale training with the PyTorch [torch.distributed.pipelining](https://pytorch.org/docs/stable/distributed.pipelining.html) API.

## prepare_pippy[[accelerate.prepare_pippy]]

#### accelerate.prepare_pippy[[accelerate.prepare_pippy]]

[Source](https://github.com/huggingface/accelerate/blob/v1.14.0/src/accelerate/inference.py#L126)

Wraps `model` for pipeline parallel inference.

**Parameters:**

model (`torch.nn.Module`) : A model we want to split for pipeline-parallel inference

split_points (`str` or `List[str]`, defaults to 'auto') : How to generate the split points and chunk the model across each GPU. 'auto' will find the best balanced split given any model. Should be a list of layer names in the model to split by otherwise.

no_split_module_classes (`List[str]`) : A list of class names for layers we don't want to be split.

example_args (tuple of model inputs) : The expected inputs for the model that uses order-based inputs for a *single process*. Recommended to use this method if possible.

example_kwargs (dict of model inputs) : The expected inputs for the model that uses dictionary-based inputs for a *single process*. This is a *highly* limiting structure that requires the same keys be present at *all* inference calls. Not recommended unless the prior condition is true for all cases.

num_chunks (`int`, defaults to the number of available GPUs) : The number of different stages the Pipeline will have. By default it will assign one chunk per GPU, but this can be tuned and played with. In general one should have num_chunks >= num_gpus.

gather_output (`bool`, defaults to `False`) : If `True`, the output from the last GPU (which holds the true outputs) is sent across to all GPUs.
