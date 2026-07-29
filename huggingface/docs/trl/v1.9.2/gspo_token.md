# GSPO-token

In the paper [Group Sequence Policy Optimization](https://huggingface.co/papers/2507.18071), the authors propose a token-level objective variant to GSPO, called GSPO-token. To use GSPO-token, you can use the `GRPOTrainer` class in `trl.experimental.gspo_token`.

## Usage

```python
from trl.experimental.gspo_token import GRPOTrainer
from trl import GRPOConfig

training_args = GRPOConfig(
    importance_sampling_level="sequence_token",
    ...
)
```

> [!WARNING]
> To leverage GSPO-token, the user will need to provide the per-token advantage  \\( \hat{A_{i,t}} \\) for each token  \\( t \\) in the sequence  \\( i \\) (i.e., make  \\( \hat{A_{i,t}} \\) varies with  \\( t \\)—which isn't the case here,  \\( \hat{A_{i,t}}=\hat{A_{i}} \\)). Otherwise, GSPO-Token gradient is just equivalent to the original GSPO implementation.

## GRPOTrainer[[trl.GRPOTrainer]]

- **resume_from_checkpoint** (`str` or `bool`, *optional*) --
  If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a
  `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance
  of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.
- **trial** (`optuna.Trial` or `dict[str, Any]`, *optional*) --
  The trial run or the hyperparameter dictionary for hyperparameter search.
- **ignore_keys_for_eval** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions for evaluation during the training.`~trainer_utils.TrainOutput`Object containing the global step count, training loss, and metrics.

Main training entry point.

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

- **commit_message** (`str`, *optional*, defaults to `"End of training"`) --
  Message to commit while pushing.
- **blocking** (`bool`, *optional*, defaults to `True`) --
  Whether the function should return only when the `git push` has finished.
- **token** (`str`, *optional*, defaults to `None`) --
  Token with write permission to overwrite Trainer's original args.
- **revision** (`str`, *optional*) --
  The git revision to commit from. Defaults to the head of the "main" branch.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments passed along to `~Trainer.create_model_card`.The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.
