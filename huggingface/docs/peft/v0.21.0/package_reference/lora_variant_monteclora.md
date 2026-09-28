# MonteCLoRA (Monte Carlo Low-Rank Adaptation)

> [!NOTE]
> This is a variant of LoRA and therefore everything that is possible with LoRA is valid for this method except otherwise stated on this page.

MonteCLoRA wraps a standard LoRA adapter with a small variational module that draws Monte Carlo samples of stochastic perturbations on top of the LoRA `A` matrix during training. Concretely, it learns variational parameters (a Wishart-based covariance, a per-sample multivariate-normal noise term, and a Dirichlet weighting over the samples) and adds the resulting averaged perturbation to `lora_A` at every forward pass. A KL-divergence + entropy term is added to the training loss to keep these variational parameters anchored to a sensible prior. At inference time the sampler is disabled and MonteCLoRA behaves exactly like a regular LoRA adapter, so there is **no extra inference cost or extra parameters to merge**. For the full method see https://huggingface.co/papers/2411.04358.

You may want to consider MonteCLoRA when:

- You are fine-tuning on a small or noisy dataset and want stronger regularization than vanilla LoRA. The Monte Carlo averaging and the KL term together act as a Bayesian-style regularizer.
- You want better uncertainty calibration / robustness from your adapter without paying extra cost at inference time (the variational machinery is training-only).
- Vanilla LoRA is overfitting and lowering `r` or increasing `lora_dropout` is not enough.

You probably do *not* need MonteCLoRA when you have a large, clean dataset and vanilla LoRA already trains stably — in that regime the extra variational parameters mostly add training overhead without much benefit.

To enable MonteCLoRA, pass a `MontecloraConfig` to `LoraConfig`:

```py
from peft import LoraConfig, MontecloraConfig

monteclora_config = MontecloraConfig(
    num_samples=8,         # number of Monte Carlo samples per forward pass
    sample_scaler=1e-4,    # magnitude of the variational perturbation
    kl_loss_weight=1e-5,   # weight of the KL term added to the training loss
)
config = LoraConfig(
    r=16,
    lora_alpha=32,
    target_modules=["q_proj", "v_proj"],
    monteclora_config=monteclora_config,
)
```

During training you must add the variational regularization loss to the task loss. The simplest way is to call `LoraModel._get_monteclora_loss()` on the underlying `LoraModel`:

```py
task_loss = ...  # standard loss returned by your model
monteclora_loss = model._get_monteclora_loss()  # 0.0 if MonteCLoRA is not used
total_loss = task_loss + monteclora_loss
total_loss.backward()
```

If you train with the HF `Trainer`, you can simply mix in `peft.helpers.MontecloraTrainerMixin` which does this for you in `compute_loss`:

```py
from transformers import Trainer
from peft.helpers import MontecloraTrainerMixin

class MontecloraTrainer(MontecloraTrainerMixin, Trainer):
    pass
```

A complete working example is available at [`examples/monteclora_finetuning`](https://github.com/huggingface/peft/tree/main/examples/monteclora_finetuning).

# API

## MonteCloraConfig[[peft.MontecloraConfig]]

#### peft.MontecloraConfig[[peft.MontecloraConfig]]

```python
peft.MontecloraConfig(num_samples: int = 8, use_entropy: bool = False, dirichlet_prior: float = 0.1, sample_scaler: float = 0.0001, kl_loss_weight: float = 1e-05, buffer_size: int = 150)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/lora/config.py#L1177)

**Parameters:**

num_samples (`int`) : Number of Monte Carlo samples to draw per forward pass. Higher values usually give smoother training and better uncertainty estimates but increase compute and memory usage. Lower this if training is too slow or memory constrained; increase it if training is stable and you want stronger Monte Carlo averaging.

use_entropy (`bool`) : Whether to add an entropy regularization term that keeps the Monte Carlo weights from collapsing to a single sample. Turn this on if you observe the sampler becoming very peaky or want stronger regularization; leave it off to mimic standard LoRA more closely.

dirichlet_prior (`float`) : Concentration parameter for the Dirichlet prior over sample/expert weights. Larger values push the weights towards being more uniform (stronger regularization, less sparsity), while smaller positive values encourage sparser, more peaked weights. Increase if the sampler overfits; decrease (but keep > 0) if it is too conservative.

sample_scaler (`float`) : Overall scaling factor for the sampled perturbations applied to the LoRA weights. Increasing this makes the Monte Carlo noise stronger (more regularization and exploration, but also more training instability); decreasing it moves the behavior closer to standard deterministic LoRA. Setting it very close to 0 largely disables the effect of Monteclora.

kl_loss_weight (`float`) : Weight of the KL-divergence term between the variational distribution and its prior. Larger values put more emphasis on matching the prior (stronger regularization, potentially underfitting); smaller values rely more on the data likelihood (weaker regularization, potentially overfitting). Tune this if you find Monteclora over- or under-regularizing the adapter.

buffer_size (`int`) : Size of the internal buffer used by the Monte Carlo sampler (e.g. for storing recent statistics). Larger values can stabilize the estimated variational parameters at the cost of additional memory; reduce this if you are memory constrained.

This is the sub-configuration class to store the configuration for Monteclora (Monte Carlo Low-Rank Adaptation).
Monteclora introduces variational inference into LoRA by adding Monte Carlo sampling to the adapter weights.

In practice you can think of Monteclora as adding stochastic, learned perturbations on top of the LoRA weights to
obtain a better-calibrated and better-regularized adapter. The arguments below let you trade off stability,
regularization strength, and compute cost.
