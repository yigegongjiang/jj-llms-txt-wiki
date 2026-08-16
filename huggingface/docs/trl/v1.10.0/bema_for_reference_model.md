# BEMA for Reference Model

This feature implements the BEMA algorithm to update the reference model during DPO training.

## Usage

```python
from trl.experimental.bema_for_ref_model import BEMACallback, DPOTrainer
from datasets import load_dataset

dataset = load_dataset("trl-internal-testing/zen", "standard_preference", split="train")

bema_callback = BEMACallback(update_ref_model=True)

trainer = DPOTrainer(
    model="trl-internal-testing/tiny-Qwen2ForCausalLM-2.5",
    train_dataset=dataset,
    callbacks=[bema_callback],
)
trainer.train()
```

## DPOTrainer[[trl.DPOTrainer]]

#### trl.DPOTrainer[[trl.DPOTrainer]]

```python
trl.DPOTrainer(*args, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/bema_for_ref_model/dpo_trainer.py#L19)

#### train[[trl.DPOTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L1347)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.DPOTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L3794)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.DPOTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/transformers/trainer.py#L4041)

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

## BEMACallback[[trl.BEMACallback]]

#### trl.BEMACallback[[trl.BEMACallback]]

```python
trl.BEMACallback(update_freq: int = 400, ema_power: float = 0.5, bias_power: float = 0.2, lag: int = 10, update_after: int = 0, multiplier: float = 1.0, min_ema_multiplier: float = 0.0, device: str = 'cpu', update_ref_model: bool = False, ref_model_update_freq: int = 400, ref_model_update_after: int = 0)
```

[Source](https://github.com/huggingface/trl/blob/v1.10.0/trl/experimental/bema_for_ref_model/callback.py#L59)

**Parameters:**

update_freq (`int`, *optional*, defaults to `400`) : Update the BEMA weights every X steps. Denoted this as  \\( \phi \\) in the paper.

ema_power (`float`, *optional*, defaults to `0.5`) : Power for the EMA decay factor. Denoted  \\( \kappa \\) in the paper. To disable EMA, set this to `0.0`.

bias_power (`float`, *optional*, defaults to `0.2`) : Power for the BEMA scaling factor. Denoted  \\( \eta \\) in the paper. A large value (e.g. `8.0`) makes  \\( \alpha_t \\) decay to `0`, approximating disabled bias-correction; `0.0` instead pins  \\( \alpha_t \\) at `1` for every step (maximum, undecayed correction).

lag (`int`, *optional*, defaults to `10`) : Initial offset in the weight decay schedule that controls early-stage smoothness by acting as a virtual starting age for the updates. Denoted as  \\( \rho \\) in the paper.

update_after (`int`, *optional*, defaults to `0`) : Burn-in time before starting to update the BEMA weights. Denoted  \\( \tau \\) in the paper.

multiplier (`float`, *optional*, defaults to `1.0`) : Initial value for the EMA decay factor. Denoted as  \\( \gamma \\) in the paper.

min_ema_multiplier (`float`, *optional*, defaults to `0.0`) : Minimum value for the EMA decay factor.

device (`str`, *optional*, defaults to `"cpu"`) : Device to use for the BEMA buffers, e.g. `"cpu"` or `"cuda"`. Note that in most cases, this device SHOULD BE DIFFERENT from the device used for training in order to avoid OOM.

update_ref_model (`bool`, *optional*, defaults to `False`) : Whether to update the reference model with BEMA weights. This creates a lagged, smoothed version of the main model as the reference model.

ref_model_update_freq (`int`, *optional*, defaults to `400`) : Update the reference model with BEMA weights every this many steps.

ref_model_update_after (`int`, *optional*, defaults to `0`) : Number of steps to wait before starting to update the reference model.

A [TrainerCallback](https://huggingface.co/docs/transformers/v5.15.0/en/main_classes/callback#transformers.TrainerCallback) that implements [BEMA](https://huggingface.co/papers/2508.00180)
(Bias-Corrected Exponential Moving Average) by [Adam Block](https://huggingface.co/abblock) and [Cyril
Zhang](https://huggingface.co/cyrilzhang). Code from https://github.com/abblock/bema under MIT license.

BEMA computes model weights that scale like:

$$
\theta_t' = \alpha_t \cdot (\theta_t - \theta_0) + \text{EMA}_t
$$

where  \\( \theta_t \\) is the current model weights,  \\( \theta_0 \\) is a snapshot of the model weights at the
first `update_after` step,  \\( \text{EMA}_t  \\) is the exponential moving average of the model weights, and
\\( \alpha_t \\) is a scaling factor that decays with the number of steps  \\( t \\) as

$$
\alpha_t = (\rho + \gamma \cdot t)^{-\eta}.
$$

The EMA is computed as:

$$
\text{EMA}_t = (1 - \beta_t) \cdot \text{EMA}_{t-1} + \beta_t \cdot \theta_t
$$

where  \\( \beta_t \\) is a decay factor that decays with the number of steps  \\( t \\) as

$$
\beta_t = (\rho + \gamma \cdot t)^{-\kappa}.
$$

Example:

```python
>>> from trl import BEMACallback

>>> trainer = Trainer(..., callbacks=[BEMACallback()])
```
