# ESMFold2

## Overview

ESMFold2 is an all-atom protein structure prediction model. It predicts 3D coordinates and per-residue confidence
(pLDDT, PAE, PDE) directly from an amino-acid sequence, using the [ESMC](./esmc) protein language model as its
backbone. The architecture combines a sliding-window atom encoder with 3D rotary position embeddings, a pairwise
folding trunk applied iteratively, a diffusion-based structure head, and a confidence head.

The model checkpoint is available on the Hugging Face Hub at [`biohub/ESMFold2-hf`](https://huggingface.co/biohub/ESMFold2-hf).

## Usage example

```python
import torch

from transformers import EsmFold2Model

# The ESMC backbone is bundled in the checkpoint and loaded with the model.
# bf16 is the recommended inference precision.
model = EsmFold2Model.from_pretrained("biohub/ESMFold2-hf", dtype=torch.bfloat16, device_map="auto")

pdb_string = model.infer_protein_as_pdb("MKTAYIAKQRQISFVKSHFSRQLEERLGLIEVQ")
print(pdb_string)
```

`infer_protein` returns the raw outputs (atom coordinates, distogram logits and confidence metrics) as an
[EsmFold2Output](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output) if you need them instead of a PDB string. You may get
slightly different predictions if you run the same sequence multiple times. Set a manual seed if you want exactly
reproducible structures.

ESMFold2 draws `config.structure_head.num_diffusion_samples` structures per fold. `infer_protein_as_pdb` renders the best-ranked
one (highest pTM); pass `sample_idx` to pick a specific sample instead. The PDB carries per-residue pLDDT in the
b-factor column, on the same 0-1 scale as the `plddt` output.

### `forward` vs `fold`

A structure prediction has two halves. `EsmFold2Model.forward` is the first: it runs the folding trunk over the
featurized inputs and returns the refined pair representation plus the distogram, as an
[EsmFold2TrunkOutput](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput). It does not produce 3D coordinates — ESMFold2 gets those
by iterative denoising, and that sampling loop (the noise schedule, Kabsch alignment and the ODE/SDE update) lives in
`EsmFold2FoldingMixin` along with the confidence head call:

| Method | Use it for |
| --- | --- |
| `infer_protein_as_pdb(sequence)` | a PDB string, straight from an amino-acid sequence |
| `infer_protein(sequence)` | the raw [EsmFold2Output](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output) |
| `fold(**features)` | pre-featurized inputs (what `infer_protein` calls) |
| `forward(**features)` | the trunk alone — a distogram and pair representation, no sampling |

Call `fold` or `infer_protein` for an actual structure. Reach for `forward` when you only need the distogram, or when
you want to drive the diffusion sampler yourself: `fold` calls `forward` once and then hands its output to
`EsmFold2DiffusionModule`, whose own `forward` is the single denoising step.

## Faster inference with a fused kernel

The folding trunk's dominant cost is the triangle-multiplication update. Passing `use_kernels=True` to
[from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) swaps it for a fused Triton kernel loaded from the Hub via the
[`kernels`](https://github.com/huggingface/kernels) library, leaving the prediction unchanged. It is inference-only and
CUDA-only; on CPU or without the kernel installed the model transparently falls back to the pure-PyTorch implementation.
Make sure the model is on a CUDA device when kernelization happens (e.g. with `device_map`).

```python
import torch

from transformers import EsmFold2Model

model = EsmFold2Model.from_pretrained(
    "biohub/ESMFold2-hf", dtype=torch.bfloat16, device_map="cuda", use_kernels=True
)

pdb_string = model.infer_protein_as_pdb("MKTAYIAKQRQISFVKSHFSRQLEERLGLIEVQ")
```

## EsmFold2Config[[transformers.EsmFold2Config]]

#### transformers.EsmFold2Config[[transformers.EsmFold2Config]]

```python
transformers.EsmFold2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int | None = 384, pairwise_hidden_size: int | None = 256, single_inputs_size: int | None = 451, pair_transition_intermediate_size: int | None = 1024, sliding_window: int | None = 128, chunk_size: int | None = 64, num_relative_residx_bins: int | None = 32, num_relative_chain_bins: int | None = 2, num_loops: int | None = 10, num_res_types: int | None = 33, max_atomic_number: int | None = 128, char_vocab_size: int | None = 64, max_chars: int | None = 4, max_atoms_per_token: int | None = 23, atom_feature_dim: int | None = None, folding_trunk_num_hidden_layers: int | None = 24, parcae_num_coda_layers: int | None = 2, atom_encoder: dict | transformers.models.esmfold2.configuration_esmfold2.EsmFold2AtomEncoderConfig | None = None, structure_head: dict | transformers.models.esmfold2.configuration_esmfold2.EsmFold2StructureHeadConfig | None = None, confidence_head: dict | transformers.models.esmfold2.configuration_esmfold2.EsmFold2ConfidenceHeadConfig | None = None, msa_encoder: dict | transformers.models.esmfold2.configuration_esmfold2.EsmFold2MsaEncoderConfig | None = None, lm_encoder: dict | transformers.models.esmfold2.configuration_esmfold2.EsmFold2LmEncoderConfig | None = None, esmc_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/configuration_esmfold2.py#L297)

**Parameters:**

hidden_size (`int`, *optional*, defaults to 384) : Single-representation width.

pairwise_hidden_size (`int`, *optional*, defaults to 256) : Pair-representation width.

single_inputs_size (`int`, *optional*, defaults to 451) : Width of the concatenated single-input features fed to the trunk and diffusion conditioning: `atom_encoder.output_dim` plus two residue-type one-hots and the profile scalar.

pair_transition_intermediate_size (`int`, *optional*, defaults to 1024) : SwiGLU width of the pair-stream transitions.

sliding_window (`int`, *optional*, defaults to 128) : Sliding-window size (token-index distance) for the atom-stack attention, as the total window width rather than the radius.

chunk_size (`int`, *optional*, defaults to 64) : Chunk size for the memory-heavy pair-/MSA-stream ops. `None` disables chunking.

num_relative_residx_bins (`int`, *optional*, defaults to 32) : Number of relative residue-index bins in the relative-position encoding.

num_relative_chain_bins (`int`, *optional*, defaults to 2) : Number of relative chain-index bins in the relative-position encoding.

num_loops (`int`, *optional*, defaults to 10) : Number of trunk refinement loops.

num_res_types (`int`, *optional*, defaults to 33) : Number of residue types.

max_atomic_number (`int`, *optional*, defaults to 128) : Size of the element one-hot in the atom features.

char_vocab_size (`int`, *optional*, defaults to 64) : Character-vocabulary size for the encoded atom names.

max_chars (`int`, *optional*, defaults to 4) : Number of characters per encoded atom name.

max_atoms_per_token (`int`, *optional*, defaults to 23) : Maximum number of atoms per token.

atom_feature_dim (`int`, *optional*) : Atom feature width: xyz, charge and mask, plus the element and atom-name-char one-hots. Derived from `max_atomic_number`, `char_vocab_size` and `max_chars` if unset.

folding_trunk_num_hidden_layers (`int`, *optional*, defaults to 24) : Number of pair-update blocks in the folding trunk.

parcae_num_coda_layers (`int`, *optional*, defaults to 2) : Number of pair-update blocks in the parcae coda.

atom_encoder (`EsmFold2AtomEncoderConfig`, *optional*) : Configuration for the inputs-embedder SWA atom encoder.

structure_head (`EsmFold2StructureHeadConfig`, *optional*) : Configuration for the diffusion structure-prediction head (holds the `diffusion_module` sub-config).

confidence_head (`EsmFold2ConfidenceHeadConfig`, *optional*) : Configuration for the confidence head.

msa_encoder (`EsmFold2MsaEncoderConfig`, *optional*) : Configuration for the MSA encoder.

lm_encoder (`EsmFold2LmEncoderConfig`, *optional*) : Configuration for the language-model hidden-state encoder.

esmc_config ([EsmcConfig](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcConfig), *optional*) : Configuration for the bundled ESMC backbone.

This is the configuration class to store the configuration of a Esmfold2Model. It is used to instantiate a Esmfold2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [biohub/ESMFold2-hf](https://huggingface.co/biohub/ESMFold2-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

#### validate_architecture[[transformers.EsmFold2Config.validate_architecture]]

```python
validate_architecture()
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/configuration_esmfold2.py#L406)

Checks the width relations that span sub-configs; each sub-config checks its own.

## EsmFold2PreTrainedModel[[transformers.EsmFold2PreTrainedModel]]

#### transformers.EsmFold2PreTrainedModel[[transformers.EsmFold2PreTrainedModel]]

```python
transformers.EsmFold2PreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L1848)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## EsmFold2Model[[transformers.EsmFold2Model]]

#### transformers.EsmFold2Model[[transformers.EsmFold2Model]]

```python
transformers.EsmFold2Model(config: EsmFold2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L1903)

**Parameters:**

config ([EsmFold2Config](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.EsmFold2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ESMFold2 all-atom protein structure predictor with a bundled ESMC protein-language-model backbone. This is the
standard released ESMFold2 architecture, whose trunk is a linear-recurrent stack (internally referred to as
"parcae").

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EsmFold2Model.forward]]

```python
forward(token_index: Tensor, residue_index: Tensor, asym_id: Tensor, sym_id: Tensor, entity_id: Tensor, mol_type: Tensor, res_type: Tensor, token_bonds: Tensor, attention_mask: Tensor, atom_inputs: EsmFold2AtomInputs, deletion_mean: typing.Optional[torch.Tensor] = None, msa: typing.Optional[torch.Tensor] = None, has_deletion: typing.Optional[torch.Tensor] = None, deletion_value: typing.Optional[torch.Tensor] = None, msa_attention_mask: typing.Optional[torch.Tensor] = None, input_ids: typing.Optional[torch.Tensor] = None, lm_hidden_states: typing.Optional[torch.Tensor] = None, num_loops: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L2187)

**Parameters:**

token_index (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Per-token positional index within the full complex; feeds the relative-position encoding.

residue_index (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Residue index within each chain; feeds the relative-position encoding.

asym_id (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Asymmetric-unit (chain) ID for each token.

sym_id (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Symmetry-copy ID distinguishing identical chains of a homomer.

entity_id (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Entity ID grouping tokens that belong to the same molecular entity.

mol_type (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Molecule-type code for each token (`0` = protein).

res_type (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Residue-type (amino-acid identity) index for each token.

token_bonds (*torch.Tensor* of shape *(batch_size, num_tokens, num_tokens, 1)*) : Pairwise inter-token covalent-bond feature.

attention_mask (*torch.Tensor* of shape *(batch_size, num_tokens)*) : Mask marking valid tokens (`1`) versus padding (`0`). Inputs must be right-padded.

atom_inputs (*~models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs*) : The raw reference-conformer atom tensors, bundled: positions, atomic numbers, formal charges, encoded atom names, per-atom group IDs, the atom padding mask and the atom->token map.

deletion_mean (*torch.Tensor* of shape *(batch_size, num_tokens)*, *optional*) : Mean MSA deletion count per column. Defaults to zeros (no MSA).

msa (*torch.Tensor* of shape *(batch_size, msa_depth, num_tokens)*, *optional*) : MSA residue-type tokens (row 0 is the query sequence). Defaults to a single-sequence MSA.

has_deletion (*torch.Tensor* of shape *(batch_size, msa_depth, num_tokens)*, *optional*) : Boolean flag marking MSA positions preceded by a deletion.

deletion_value (*torch.Tensor* of shape *(batch_size, msa_depth, num_tokens)*, *optional*) : Per-position MSA deletion counts.

msa_attention_mask (*torch.Tensor* of shape *(batch_size, msa_depth, num_tokens)*, *optional*) : Validity mask for the MSA rows/columns.

input_ids (*torch.Tensor* of shape *(batch_size, num_tokens)*, *optional*) : ESMC-vocabulary token ids for the sequence. Fed to the bundled ESMC backbone to produce *lm_hidden_states* when those are not passed directly; ignored when *lm_hidden_states* is given.

lm_hidden_states (*torch.Tensor* of shape *(batch_size, num_tokens, num_esmc_layers + 1, esmc_hidden_size)*, *optional*) : Precomputed ESMC backbone hidden states, one per backbone layer plus the embeddings. When provided, the backbone is not run and *input_ids* is unused.

num_loops (*int*, *optional*) : Number of trunk refinement loops. Defaults to *config.num_loops*.

**Returns:** [*~models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput*] or *tuple(torch.FloatTensor)*

A [*~models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput*] or a tuple of
*torch.FloatTensor* (if *return_dict=False* is passed or when *config.return_dict=False*) comprising various
elements depending on the configuration ([*EsmFold2Config*]) and inputs.

Run the folding trunk: featurize the inputs, embed them into a pair representation, refine it over
*num_loops* recycling iterations, and read off the distogram. This is the deterministic half of a
structure prediction; the diffusion sampler that turns the returned pair representation into 3D
coordinates lives in *EsmFold2FoldingMixin* — call [*~EsmFold2Model.fold*] or
[*~EsmFold2Model.infer_protein*] for an end-to-end prediction.

- **distogram_logits** (*torch.FloatTensor* of shape *(batch_size, num_tokens, num_tokens, distogram_bins)*) -- Predicted distance-distribution logits over residue pairs.
- **pair_states** (*torch.FloatTensor* of shape *(batch_size, num_tokens, num_tokens, pairwise_hidden_size)*) -- The trunk's final pair representation, in fp32.
- **single_inputs** (*torch.FloatTensor* of shape *(batch_size, num_tokens, single_inputs_size)*) -- Concatenated single-input features built by the inputs embedder.
- **relative_position_encoding** (*torch.FloatTensor* of shape *(batch_size, num_tokens, num_tokens, pairwise_hidden_size)*) -- Relative-position pair encoding.
- **token_bonds_encoding** (*torch.FloatTensor* of shape *(batch_size, num_tokens, num_tokens, pairwise_hidden_size)*) -- Embedded inter-token covalent-bond feature.
- **atom_inputs** (*~models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs*, *optional*) -- The featurized reference-conformer atom inputs, reused by the diffusion atom stack.

#### fold[[transformers.EsmFold2Model.fold]]

```python
fold(attention_mask: Tensor, asym_id: Tensor, mol_type: Tensor, distogram_atom_idx: Tensor, num_diffusion_samples: int | None = None, num_sampling_steps: int | None = None, **trunk_kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/generation_esmfold2.py#L103)

Predict a structure end-to-end from featurized inputs: run the trunk
([*EsmFold2Model.forward*], which documents the feature arguments), sample coordinates from the
diffusion structure head, and score them with the confidence head.

Only the arguments the sampler and the confidence head need are named here; the rest are
forwarded to the trunk untouched, so `fold` takes the same arguments as `forward`.

attention_mask (*torch.Tensor* of shape *(batch_size, num_tokens)*):
Mask marking valid tokens (`1`) versus padding (`0`). Also forwarded to the trunk.
asym_id (*torch.Tensor* of shape *(batch_size, num_tokens)*):
Asymmetric-unit (chain) ID for each token. Also forwarded to the trunk.
mol_type (*torch.Tensor* of shape *(batch_size, num_tokens)*):
Molecule-type code for each token (`0` = protein). Also forwarded to the trunk.
distogram_atom_idx (*torch.Tensor* of shape *(batch_size, num_tokens)*):
Index of the representative atom (Cβ, or Cα for glycine) of each token. Used by the
confidence head; the trunk does not need it.
num_diffusion_samples (*int*, *optional*):
Number of parallel structure samples to draw; the confidence head re-runs once per sample.
Defaults to *config.structure_head.num_diffusion_samples*.
num_sampling_steps (*int*, *optional*):
Number of diffusion sampling steps. Defaults to *config.structure_head.inference_num_steps*.
trunk_kwargs:
The remaining featurized inputs (*num_loops* included, plus the raw per-atom tensors bundled
below into *atom_inputs*), forwarded to [*EsmFold2Model.forward*], which documents them.

#### infer_protein[[transformers.EsmFold2Model.infer_protein]]

```python
infer_protein(sequence: str, **forward_kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/generation_esmfold2.py#L307)

Featurize `sequence` and fold it. `forward_kwargs` go to [*~EsmFold2Model.fold*].

#### infer_protein_as_pdb[[transformers.EsmFold2Model.infer_protein_as_pdb]]

```python
infer_protein_as_pdb(sequence: str, sample_idx: int | None = None, **forward_kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/generation_esmfold2.py#L314)

Fold `sequence` and render the prediction as a PDB string.

`sample_idx` picks which diffusion sample to render; by default the best-ranked one.

## EsmFold2Output[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output]]

#### transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output]]

```python
transformers.models.esmfold2.modeling_esmfold2.EsmFold2Output(distogram_logits: typing.Optional[torch.Tensor] = None, sample_atom_coords: typing.Optional[torch.Tensor] = None, plddt_logits: typing.Optional[torch.Tensor] = None, plddt: typing.Optional[torch.Tensor] = None, plddt_per_atom: typing.Optional[torch.Tensor] = None, plddt_ca: typing.Optional[torch.Tensor] = None, complex_plddt: typing.Optional[torch.Tensor] = None, complex_iplddt: typing.Optional[torch.Tensor] = None, pae_logits: typing.Optional[torch.Tensor] = None, pae: typing.Optional[torch.Tensor] = None, pde_logits: typing.Optional[torch.Tensor] = None, pde: typing.Optional[torch.Tensor] = None, resolved_logits: typing.Optional[torch.Tensor] = None, ptm: typing.Optional[torch.Tensor] = None, iptm: typing.Optional[torch.Tensor] = None, pair_chains_iptm: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L1227)

**Parameters:**

distogram_logits (`torch.FloatTensor` of shape `(batch_size, num_tokens, num_tokens, distogram_bins)`) : Predicted distance-distribution logits over residue pairs (RNG-independent; no diffusion sampling).

sample_atom_coords (`torch.FloatTensor` of shape `(num_diffusion_samples, num_atoms, 3)`) : Predicted all-atom Cartesian coordinates for each diffusion sample.

plddt_logits (`torch.FloatTensor` of shape `(num_diffusion_samples, num_atoms, num_plddt_bins)`) : Per-atom pLDDT bin logits.

plddt (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens)`) : Per-residue predicted lDDT confidence.

plddt_per_atom (`torch.FloatTensor` of shape `(num_diffusion_samples, num_atoms)`) : Per-atom predicted lDDT confidence.

plddt_ca (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens)`) : Predicted lDDT at the representative (Cα) atom of each token.

complex_plddt (`torch.FloatTensor` of shape `(num_diffusion_samples,)`) : Mean pLDDT over all atoms of the complex.

complex_iplddt (`torch.FloatTensor` of shape `(num_diffusion_samples,)`) : Interface-weighted complex pLDDT.

pae_logits (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens, num_tokens, num_pae_bins)`) : Predicted-aligned-error bin logits.

pae (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens, num_tokens)`) : Expected predicted aligned error (Å) for each residue pair.

pde_logits (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens, num_tokens, num_pde_bins)`) : Predicted-distance-error bin logits.

pde (`torch.FloatTensor` of shape `(num_diffusion_samples, num_tokens, num_tokens)`) : Expected predicted distance error (Å) for each residue pair.

resolved_logits (`torch.FloatTensor` of shape `(num_diffusion_samples, num_atoms, 2)`) : Per-atom resolved/unresolved logits.

ptm (`torch.FloatTensor` of shape `(num_diffusion_samples,)`) : Predicted TM-score for each sample.

iptm (`torch.FloatTensor` of shape `(num_diffusion_samples,)`) : Predicted interface TM-score for each sample.

pair_chains_iptm (`torch.FloatTensor` of shape `(num_diffusion_samples, num_chains, num_chains)`) : Predicted interface TM-score for each ordered chain pair.

Output of [EsmFold2Model](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.EsmFold2Model). All confidence scores are on a 0-1 scale; per-sample tensors
have a leading `num_diffusion_samples` axis.

## EsmFold2TrunkOutput[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput]]

#### transformers.models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput]]

```python
transformers.models.esmfold2.modeling_esmfold2.EsmFold2TrunkOutput(distogram_logits: typing.Optional[torch.Tensor] = None, pair_states: typing.Optional[torch.Tensor] = None, single_inputs: typing.Optional[torch.Tensor] = None, relative_position_encoding: typing.Optional[torch.Tensor] = None, token_bonds_encoding: typing.Optional[torch.Tensor] = None, atom_inputs: transformers.models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L1289)

**Parameters:**

distogram_logits (`torch.FloatTensor` of shape `(batch_size, num_tokens, num_tokens, distogram_bins)`) : Predicted distance-distribution logits over residue pairs.

pair_states (`torch.FloatTensor` of shape `(batch_size, num_tokens, num_tokens, pairwise_hidden_size)`) : The trunk's final pair representation, in fp32.

single_inputs (`torch.FloatTensor` of shape `(batch_size, num_tokens, single_inputs_size)`) : Concatenated single-input features built by the inputs embedder.

relative_position_encoding (`torch.FloatTensor` of shape `(batch_size, num_tokens, num_tokens, pairwise_hidden_size)`) : Relative-position pair encoding.

token_bonds_encoding (`torch.FloatTensor` of shape `(batch_size, num_tokens, num_tokens, pairwise_hidden_size)`) : Embedded inter-token covalent-bond feature.

atom_inputs (`~models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs`, *optional*) : The featurized reference-conformer atom inputs, reused by the diffusion atom stack.

Output of [EsmFold2Model.forward()](/docs/transformers/v5.17.0/en/model_doc/esmfold2#transformers.EsmFold2Model.forward): the folding trunk's pair representation, the distogram read
off it, and the conditioning tensors that the structure and confidence heads consume. Everything
here is deterministic given the inputs apart from the trunk's random initial pair state.

## EsmFold2AtomInputs[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs]]

#### transformers.models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs[[transformers.models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs]]

```python
transformers.models.esmfold2.modeling_esmfold2.EsmFold2AtomInputs(ref_pos: Tensor, ref_charge: Tensor, atom_attention_mask: Tensor, ref_element: Tensor, ref_atom_name_chars: Tensor, ref_space_uid: Tensor, atom_to_token: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmfold2/modeling_esmfold2.py#L45)

**Parameters:**

ref_pos : *(batch_size, num_atoms, 3)* reference-conformer Cartesian coordinates.

ref_charge : *(batch_size, num_atoms)* formal charge of each atom.

atom_attention_mask : *(batch_size, num_atoms)* valid atoms (*1*) versus padding (*0*).

ref_element : *(batch_size, num_atoms, max_atomic_number)* one-hot atomic number.

ref_atom_name_chars : *(batch_size, num_atoms, max_chars, char_vocab_size)* one-hot atom name.

ref_space_uid : *(batch_size, num_atoms)* per-atom group ID (the atom's token index).

atom_to_token : *(batch_size, num_atoms)* index of the token each atom belongs to.

Featurized reference-conformer atom inputs, bundled so the atom stack takes one argument.

Taken as one argument by [*EsmFold2Model.forward*], holding the raw featurizer tensors.
`EsmFold2Model._prepare_features` then one-hot encodes the categorical fields and zeroes the
padding, and `forward` swaps those in with *dataclasses.replace*, so every consumer downstream of
that point sees the encoded forms documented below. Returned on [*EsmFold2TrunkOutput*] so the
diffusion sampler and the confidence head reuse the trunk's featurization rather than redoing it.

A plain dataclass rather than a [*~utils.ModelOutput*]: every field here is required and is read
unconditionally downstream, and `ModelOutput` permits at most one required field (the rest must
default to `None`), which would turn a missing tensor from a constructor error into a `None`
surfacing deep in the atom encoder.
