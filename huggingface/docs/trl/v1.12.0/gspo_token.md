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

#### trl.GRPOTrainer[[trl.GRPOTrainer]]

```python
trl.GRPOTrainer(model: str | PreTrainedModel | PeftModel, reward_funcs: str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]] | list[str | transformers.modeling_utils.PreTrainedModel | collections.abc.Callable[..., list[float | None]]] | None = None, args: trl.trainer.grpo_config.GRPOConfig | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | datasets.dataset_dict.DatasetDict | datasets.dataset_dict.IterableDatasetDict | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.processing_utils.ProcessorMixin | None = None, reward_processing_classes: transformers.tokenization_utils_base.PreTrainedTokenizerBase | list[transformers.tokenization_utils_base.PreTrainedTokenizerBase] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), quantization_config: BitsAndBytesConfig | None = None, peft_config: PeftConfig | None = None, tools: list[collections.abc.Callable] | None = None, rollout_func: collections.abc.Callable[[list[str], 'GRPOTrainer'], dict[str, typing.Any]] | None = None, environment_factory: collections.abc.Callable[[], trl.trainer.grpo_trainer._SupportsReset] | dict[str, collections.abc.Callable[[], trl.trainer.grpo_trainer._SupportsReset]] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/experimental/gspo_token/grpo_trainer.py#L21)

#### train[[trl.GRPOTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L1350)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.GRPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.GRPOTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L4052)

**Parameters:**

commit_message (`str`, *optional*, defaults to `"End of training"`) : Message to commit while pushing.

blocking (`bool`, *optional*, defaults to `True`) : Whether the function should return only when the `git push` has finished.

token (`str`, *optional*, defaults to `None`) : Token with write permission to overwrite Trainer's original args.

revision (`str`, *optional*) : The git revision to commit from. Defaults to the head of the "main" branch.

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to `~Trainer.create_model_card`.

**Returns:**

The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.
