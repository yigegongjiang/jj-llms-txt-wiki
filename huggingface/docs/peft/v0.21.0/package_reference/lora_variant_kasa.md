### KaSA

> [!NOTE]
> This is a variant of LoRA and therefore everything that is possible with LoRA is valid for this method except otherwise stated on this page.

[KaSA](https://huggingface.co/papers/2412.06071) (Knowledge-aware Singular-value Adaptation) is a LoRA variant that uses the singular value decomposition of the base weight to filter out task-irrelevant knowledge and parametrizes the update with learnable singular values. It changes vanilla LoRA in two ways:

1. **Knowledge-based SVD truncation of the frozen base weight.** At initialization, the base weight `W` is SVD-factored and its `r` smallest ("noisy"/long-tail) singular components are discarded, leaving the rank-`(k - r)` approximation as the new frozen base (`k = min(in_features, out_features)`). The trainable branch then re-learns in the discarded residual subspace.
2. **Knowledge-aware singular-value adaptation.** The trainable update is parametrized in SVD form with a learnable diagonal of singular values inserted between the LoRA factors: `ΔW = scaling * B @ diag(ΔΣ) @ A`, where `ΔΣ` (`lora_diag`) is a learnable `r`-vector and the only new parameter per layer.

In PEFT, KaSA is configured as a LoRA variant through the `kasa_config` argument on [LoraConfig](/docs/peft/v0.21.0/en/package_reference/lora#peft.LoraConfig):

```py
from peft import KasaConfig, LoraConfig

config = LoraConfig(
    target_modules=["q_proj", "v_proj"],
    kasa_config=KasaConfig(beta=1e-4, gamma=1e-3),
)
```

The paper additionally trains with two auxiliary regularizers: an L2 penalty on the learnable singular values (weighted by `beta`) and an orthogonal regularization on the adapter factors (weighted by `gamma`), which softly enforces the semi-orthogonality assumed by the SVD parametrization. These cannot be injected automatically by PEFT, so during training you must add them to the task loss by calling `LoraModel._get_kasa_loss()` on the underlying `LoraModel`:

```py
task_loss = ...  # standard loss returned by your model
kasa_loss = model._get_kasa_loss()  # 0.0 if KaSA is not used
total_loss = task_loss + kasa_loss
```

For detailed usage, see [these instructions](https://github.com/huggingface/peft/tree/main/examples/kasa_finetuning).

#### Caveats

- KaSA is currently supported on standard LoRA linear layers only, and not with `fan_in_fan_out=True` layers (e.g. transformers `Conv1D`).
- KaSA adapters cannot be combined with non-KaSA adapters on the same model, since the base-weight truncation would change the base weights under the other adapters' feet. Multiple KaSA adapters are allowed.
- `convert_to_lora` is not supported: the KaSA update depends on `lora_diag` and on the truncated base weight, neither of which is representable in a vanilla LoRA adapter.
- The SVD truncation of the base weight is **destructive**: adding a KaSA adapter permanently changes the layer's frozen weight. Disabling or unloading the adapter does not restore the original base weight, and `merge` followed by `unmerge` round-trips to the truncated weight, not the original one. This is inherent to the method. Keep the original checkpoint if you need to recover the unmodified base model.
- Loading a trained KaSA adapter with `PeftModel.from_pretrained` re-applies the same truncation to the freshly loaded base weight, so saving and reloading is consistent.
