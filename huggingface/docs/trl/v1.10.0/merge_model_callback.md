# MergeModelCallback[[trl.experimental.merge_model_callback.MergeModelCallback]]

#### trl.experimental.merge_model_callback.MergeModelCallback[[trl.experimental.merge_model_callback.MergeModelCallback]]

```python
trl.experimental.merge_model_callback.MergeModelCallback(merge_config: MergeConfig | None = None, merge_at_every_checkpoint: bool = False, push_to_hub: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/merge_model_callback.py#L294)

**Parameters:**

merge_config (`experimental.merge_model_callback.MergeConfig`, *optional*) : Configuration used for the merging process. If not provided, the default `MergeConfig` is used.

merge_at_every_checkpoint (`bool`, *optional*, defaults to `False`) : Whether to merge the model at every checkpoint.

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether to push the merged model to the Hub after merging.

A [TrainerCallback](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/callback#transformers.TrainerCallback) that merges the policy model (the model being trained) with another model based
on a merge configuration.

Example:

```python
>>> from trl.experimental.merge_model_callback import MergeConfig, MergeModelCallback

>>> config = MergeConfig()
>>> merge_callback = MergeModelCallback(config)
>>> trainer = DPOTrainer(..., callbacks=[merge_callback])
```
