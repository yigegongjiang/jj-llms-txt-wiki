# PSOFT

[PSOFT](https://hf.co/papers/2505.11235) is an Orthogonal Fine-Tuning (OFT)-based parameter-efficient fine-tuning method that preserves the geometric relationships of pre-trained weight column vectors while achieving a balanced trade-off between performance and multi-dimensional efficiency, including parameter count, memory usage, and computational cost. By restricting orthogonal transformations to a low-rank principal subspace derived from pre-trained weights, PSOFT bridges the gap between LoRA and OFT, providing both theoretical guarantees and practical adaptability. Its effectiveness is validated through extensive evaluations on diverse benchmarks, including GLUE, VTAB-1K, GSM8K, MATH, and commonsense reasoning benchmarks.

- Only `nn.Linear` layers are supported.
- Quantized layers are not supported.

The abstract from the paper is:

*Driven by the rapid growth of model parameters, parameter-efficient fine-tuning (PEFT) has become essential for adapting large models to diverse downstream tasks under constrained computational resources. Within this paradigm, orthogonal fine-tuning and its variants preserve semantic representations of pre-trained models, but struggle to achieve both expressiveness and efficiency in terms of parameter counts, memory, and computation. To overcome this limitation, we propose efficient Orthogonal Fine-Tuning with Principal Subspace adaptation (PSOFT), which confines orthogonal transformations to the principal subspace of pre-trained weights. Specifically, PSOFT constructs this subspace via matrix decomposition to enable compatible transformations, establishes a theoretical condition that strictly maintains the geometry of this subspace for essential semantic preservation, and introduces efficient tunable vectors that gradually relax orthogonality during training to enhance adaptability. Extensive experiments on 35 NLP and CV tasks across four representative models demonstrate that PSOFT offers a practical and scalable solution to simultaneously achieve semantic preservation, expressiveness, and multi-dimensional efficiency in PEFT.*

## How PSOFT Works

PSOFT decomposes each weight matrix $W_{pre}$ into $W_{pri}$ and $W_{res}$ using SVD:
$W_{\text{pre}} = U S V^\top$

The principal subspace $W_{\text{pri}} = U_r S_r V_r^\top = AB$ is constructed from the top-$r$ singular components:

$W_{\text{pre}} = W_{\text{pri}} + W_{\text{res}} = AB + W_{\text{res}},$

$W_{\text{ps-tuned}} = ARB + W_{\text{res}}.$ (PSOFT-SO: PSOFT with strict orthogonality)

$W_{\text{ps-tuned}} = A \, \mathrm{diag}(\alpha) \, R \, \mathrm{diag}(\beta) \, B + W_{\text{res}}.$ (PSOFT-RO: PSOFT with relaxed orthogonality)

During training, $A$, $B$, and $W_{\text{res}}$ are frozen, and only $R$ (or $R$ with $\alpha$ and $\beta$) is trainable.

For compatibility with the PEFT framework (which expects additive weight updates), PSOFT is implemented in the following additive form:
$W_{\text{ps-tuned}} = W_{\text{pre}} + A (R - I_r) B$

## Trainable Parameters

After applying PSOFT:

- The original model weights ($A$, $B$, and $W_{\text{res}}$) are frozen.
- Only the orthogonal matrix $R$ (and optionally $\alpha$, $\beta$) are trainable.
- No additional bias parameters are introduced.

## Basic Usage
```python
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PsoftConfig, get_peft_model

# Load base model
model_id = "facebook/opt-125m"
model = AutoModelForCausalLM.from_pretrained(model_id)

# Configure PSOFT
config = PsoftConfig(
    r=32,                                   # the dimension of trainable matrix R,
    psoft_alpha=32,                         # scaling factor (typically set to r in PSOFT),
    target_modules=["q_proj", "v_proj"],    # target attention projection layers
    ab_svd_init="psoft_init",        # principal subspace initialization
    psoft_svd="full",                       # SVD method
    psoft_orth=True,                        # enable orthogonal R (Cayley parameterization)
    psoft_mag_a=True,                       # enable tunable vector alpha
    psoft_mag_b=True,                       # enable tunable vector beta
    use_cayley_neumann=False,               # disable Cayley–Neumann approximation
    num_cayley_neumann_terms=5,             # number of Neumann series terms
    cayley_neumann_eps=None,                # improve numerical stability
)

# Apply PSOFT
model = get_peft_model(model, config)
model.train()

tokenizer = AutoTokenizer.from_pretrained(model_id)
tokenizer.pad_token = tokenizer.eos_token

# Train
inputs = tokenizer("Hello world", return_tensors="pt", padding=True)
loss = model(**inputs, labels=inputs["input_ids"]).loss
loss.backward()

trainable = [p for p in model.parameters() if p.requires_grad]
optimizer = torch.optim.AdamW(trainable, lr=5e-4)
optimizer.step()
optimizer.zero_grad(set_to_none=True)
```
## Configuration Options

### Different Mode

(PSOFT-SO: PSOFT with strict orthogonality)

```python
config = PsoftConfig(psoft_orth=True,psoft_mag_a=False,psoft_mag_b=False)
```

(PSOFT-RO: PSOFT with relaxed orthogonality)
```python
config = PsoftConfig(psoft_orth=True,psoft_mag_a=True,psoft_mag_b=True)
```

### Best Practices
1. **Rank Choice**: Smaller ranks (e.g., `32–128`) are suitable for simpler tasks, while larger ranks (e.g., `64–256`) provide greater expressiveness for more complex tasks at the cost of increased parameters and computation.
2. **Scaling Factor**: The scaling factor is typically set to $r$ in PSOFT.
3. **Learning Rate**: Use standard learning rates (e.g., `1e-4` to `5e-3`) for stable training.
4. **SVD Initialization**: The `lowrank` option is more memory- and compute-efficient than `full`, making it more suitable for large models.
5. **Cayley–Neumann Approximation**: When the rank is large, enabling the Cayley–Neumann approximation can significantly improve computational efficiency, while the benefit is less pronounced for small ranks. In practice, a small number of Neumann series terms (typically `5`) usually provides a good balance between accuracy and efficiency.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=PSOFT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## PsoftConfig[[peft.PsoftConfig]]

#### peft.PsoftConfig[[peft.PsoftConfig]]

```python
peft.PsoftConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, r: int = 32, target_modules: Optional[Union[list[str], str]] = None, exclude_modules: Optional[Union[list[str], str]] = None, psoft_alpha: int = 32, psoft_dropout: float = 0.0, fan_in_fan_out: bool = False, ab_svd_init: Literal['psoft_init', 'pissa_init'] = 'psoft_init', psoft_svd: Literal['full', 'lowrank'] = 'full', psoft_svd_lowrank_niter: int = 10, random_seed: int = 0, psoft_orth: bool = True, psoft_mag_b: bool = True, psoft_mag_a: bool = True, use_cayley_neumann: bool = False, num_cayley_neumann_terms: int = 5, cayley_neumann_eps: Optional[float] = None, modules_to_save: Optional[list[str]] = None, init_weights: bool = True, layers_to_transform: Optional[Union[list[int], int]] = None, layers_pattern: Optional[Union[list[str], str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/psoft/config.py#L26)

**Parameters:**

r (`int`) : Defaults to 32. PSOFT rank (r) controls the adapter capacity through an r*r transformation R. Smaller ranks 32-128 are typically sufficient for simple tasks, More complex tasks may benefit from 64-256, increasing expressiveness at the cost of additional parameters and computation. See the paper for empirically validated settings: https://openreview.net/forum?id=FSHrinMArK.

target_modules (`Optional[Union[List[str], str]]`) : The names of the modules to apply the adapter to. If this is specified, only the modules with the specified names will be replaced. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings. If this is specified as 'all-linear', then all linear/Conv1D modules are chosen (if the model is a PreTrainedModel, the output layer excluded). If this is not specified, modules will be chosen according to the model architecture. If the architecture is not known, an error will be raised -- in this case, you should specify the target modules manually.

exclude_modules (`Optional[Union[List[str], str]]`) : The names of the modules to not apply the adapter. When passing a string, a regex match will be performed. When passing a list of strings, either an exact match will be performed or it is checked if the name of the module ends with any of the passed strings.

psoft_alpha (`int`) : Defaults to 32. It controls PSOFT scaling factor. Same semantics as LoRA alpha.

psoft_dropout (`float`) : Defaults to 0.0. Dropout for PSOFT path. Same semantics as LoRA dropout.

fan_in_fan_out (`bool`) : Set this to True if the layer to replace stores weight like (fan_in, fan_out). For example, gpt-2 uses `Conv1D` which stores weights like (fan_in, fan_out) and hence this should be set to `True`.

ab_svd_init (`Literal["psoft_init", "pissa_init"]`) : Defaults to 'psoft_init'. Initialization strategy for A and B used to construct the principal subspace in PSOFT. 'psoft_init': SVD-based initialization with row-orthogonal A, ensuring strict orthogonality (PSOFT). 'pissa_init': SVD-based initialization with symmetric A and B (standard PiSSA).

psoft_svd (`Literal["full", "lowrank"]`) : Defaults to 'full'. SVD backend for initialization: 'full' uses torch.linalg.svd; 'lowrank' uses torch.svd_lowrank.

psoft_svd_lowrank_niter (`int`) : Only used when psoft_svd='lowrank'. Defaults to 10. Number of power iterations used by torch.svd_lowrank when psoft_svd='lowrank'.

psoft_orth (`bool`) : Defaults to 'True'. If True, constrains R to be orthogonal via Cayley parameterization, preserving the geometric relationships among column of the pre-trained weight vectors. If False, R is a free matrix without orthogonality constraints.

psoft_mag_b (`bool`) : Defaults to 'True'. If True, learns a diagonal scaling vector on the 'output' side of R. Commonly paired with psoft_mag_a to increase task adaptability, with slight distortion to the pre-trained geometry.

psoft_mag_a (`bool`) : Defaults to 'True'. If True, learns a diagonal scaling vector on the 'input' side of R. Commonly paired with psoft_mag_b to increase task adaptability, with slight distortion to the pre-trained geometry.

use_cayley_neumann (`bool`) : Defaults to 'False'. Whether to use the Cayley-Neumann formulation of PSOFT or not. Set to True to improve computational efficiency but comes at costs of bigger approximation error for orthogonality.

num_cayley_neumann_terms (`int`) : Defaults to 5. Only used when use_cayley_neumann=True. Number of Cayley-Neumann terms to use. Higher number results in less approximation error for orthogonality.

cayley_neumann_eps (`optional[float]`) : Defaults to 'None'. Only used when use_cayley_neumann=True. Optional Frobenius-norm bound for the generator matrix Q in the Cayley-Neumann approximation. If None (default), no rescaling is applied. If set to a value in (0, 1) (e.g., 0.9), Q is rescaled whenever ||Q||_F exceeds the threshold to improve numerical stability. See https://spherelab.ai/oftv2/ for details.

init_weights (`bool`) : Defaults to 'True'. Whether to initialize the weights of the PSOFT layers with their default initialization. Don't change this setting, except if you know exactly what you're doing.

modules_to_save (`List[str]`) : List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.

layers_to_transform (`Union[List[int], int]`) : The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices that are specified in this list. If a single integer is passed, it will apply the transformations on the layer at this index.

layers_pattern (`Optional[Union[List[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.

Configuration for PSOFT (Efficient Orthogonal Fine-Tuning with Principal Subspace Adaptation).

PSOFT inserts an r*r orthogonal transformation R between low-rank matrices A and B, so the low-rank update is ΔW =
B @ (R-I) @ A. Only R (and optional tunable vectors) are trained; A and B are initialized with psoft_init
(SVD-based, row-orthogonal A) and frozen.

## PsoftModel[[peft.PsoftModel]]

#### peft.PsoftModel[[peft.PsoftModel]]

```python
peft.PsoftModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/psoft/model.py#L28)

**Parameters:**

model : The model to adapt.

config : PsoftConfig.

adapter_name : Adapter name, default "default".

low_cpu_mem_usage : Create empty adapter weights on meta device.

PSOFT (Efficient Orthogonal Fine-Tuning with Principal Subspace Adaptation) model.

Inserts an r*r orthogonal (or scaled) transformation R between low-rank A and B: ΔW = B @ (R-I) @ A. Use
ab_svd_init="psoft_init" to initialize A/B from SVD and freeze them, training only R (and optional magnitude
vectors).
