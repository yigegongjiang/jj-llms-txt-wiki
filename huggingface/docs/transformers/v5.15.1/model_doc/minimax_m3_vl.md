# MiniMax-M3-VL

## Overview

MiniMax-M3-VL is the vision-language member of the MiniMax-M3 family. It pairs a CLIP-style vision tower (Conv3d patch embedding with 3D rotary position embeddings) with the MiniMax-M3 text backbone, a mixed dense/sparse Mixture-of-Experts decoder that uses SwiGLU-OAI gated experts and a lightning indexer for block-sparse attention.

## Architecture
### Block-sparse attention (Lightning Indexer)

Every layer is GQA (`num_key_value_heads = 4`) with per-head QK-norm and **partial RoPE** on the first
`rotary_dim`. `config.layer_types[i]` then picks `"full_attention"` (dense causal) or
`"minimax_m3_sparse"`, where a `MiniMaxM3VLIndexer` decides, per query, which block of keys the main attention may see.

The indexer scores every key, then **max-pools those per-key scores into blocks of `index_block_size` keys**, so selection happens at the granularity of a *block
of keys*: per query it keeps the top-`index_topk_blocks` key blocks plus the always-on `index_local_blocks`
local-window block (under block-level causality), broadcasts the per-block `0`/`-inf` choice back onto every key in
the block. The result is a `[B, 1, S_q, S_k]` additive bias summed onto the causal mask. 
Theoretically this means that the attention is only computed over the selected blocks of keys, but `transformers` does not support the kernels that compute this efficiently! 
We are adding it to `kernels` asap!

### Vision tower

A [MiniMaxM3VLVisionModel](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionModel): a `Conv3d` patch embedding over flattened `[N_patches, C·T·P·P]` input, a stack of
CLIP-style encoder layers carrying a **3D rotary** position embedding (time / height / width bands). A `MiniMaxM3VLPatchMerger` groups
`spatial_merge_size²` patches into the channel dim before the 2-layer GELU `MiniMaxM3VLMultiModalProjector` maps vision features into the text hidden size.

## Usage examples

The example below runs the model on a real image loaded with `load_image()`.

```python
import torch
from transformers import AutoModelForImageTextToText, AutoProcessor
from transformers.image_utils import load_image

model = AutoModelForImageTextToText.from_pretrained(
    "MiniMaxAI/MiniMax-M3-preview", dtype=torch.bfloat16, device_map="auto",
)
processor = AutoProcessor.from_pretrained("MiniMaxAI/MiniMax-M3-preview")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": "Describe this image briefly."},
        ],
    }
]
text = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=False)
inputs = processor(images=[image], text=text, return_tensors="pt").to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=32, do_sample=False)
print(processor.batch_decode(generated_ids, skip_special_tokens=True)[0])
```

### Apple example

This example asks the model about an image of apples, again loading a real image with
`load_image()`.

```python
import torch
from transformers import AutoModelForImageTextToText, AutoProcessor
from transformers.image_utils import load_image

model = AutoModelForImageTextToText.from_pretrained(
    "MiniMaxAI/MiniMax-M3-preview", dtype=torch.bfloat16, device_map="auto",
)
processor = AutoProcessor.from_pretrained("MiniMaxAI/MiniMax-M3-preview")

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": "How many apples are in this image, and what color are they?"},
        ],
    }
]
text = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=False)
inputs = processor(images=[image], text=text, return_tensors="pt").to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=32, do_sample=False)
print(processor.batch_decode(generated_ids, skip_special_tokens=True)[0])
```

## Fastest inference configuration

| ctx | SDPA decode | MSA decode | MSA decode adv. | SDPA prefill | MSA prefill | MSA prefill adv. |
| --: | ----------: | ---------: | --------------: | -----------: | ----------: | ---------------: |
|  2K |  27.8 tok/s |       31.0 |            +12% |       303 ms |      257 ms |            1.18× |
|  4K |  23.4 tok/s |       30.5 |            +30% |       684 ms |      460 ms |            1.49× |
|  8K |  17.8 tok/s |       29.6 |            +66% |      1906 ms |      976 ms |            1.95× |
| 16K |  12.0 tok/s |       27.6 |           +130% |      6110 ms |     2344 ms |            2.61× |

The checkpoint ships in native MXFP8. For **decode throughput**, the fastest validated configuration is
**bf16 (dequantized at load) + the MSA block-sparse attention kernel + tensor & expert parallelism + a
`reduce-overhead` cudagraph compile** — roughly **31 tok/s** decode on 8×B200 at a 2048-token prefill.

Keeping the weights in **native FP8 is a memory-footprint option only — it is never faster on this setup**.
The FP8 Triton experts/linear kernels lower as opaque inductor fallback kernels that cudagraph cannot
capture on the hot expert path, so native-FP8 decode measured ~4.2 tok/s (≈7× slower than the bf16 path)
even under `torch.compile(fullgraph=True)`. Use FP8 only when the bf16 weights do not fit.

| config (sdpa baseline, TP+EP, 2048-token prefill, 8×B200) | decode |
|---|---|
| bf16 dequantize-at-load + **MSA** + compile/cudagraph | **~31 tok/s** |
| bf16 dequantize-at-load + sdpa + compile/cudagraph | ~28 tok/s |
| native FP8 + compile/cudagraph | ~4 tok/s (memory-only, not for speed) |

Dequantizing to bf16 only fits with even sharding across GPUs (TP/EP), not with `device_map="auto"`
(pipeline placement OOMs at load). Launch one process per GPU with `torchrun`:

```bash
torchrun --nproc_per_node=8 fastest_m3_vl.py
```

```python
# fastest_m3_vl.py
import os, sys
import torch
import torch.distributed as dist
from transformers import (
    AutoModelForImageTextToText,
    AutoTokenizer,
    CompileConfig,
    FineGrainedFP8Config,
)
from transformers.distributed import DistributedConfig

# The indexer feeds SDPA an additive float mask; the cuDNN SDP backend segfaults on it (B200).
torch.backends.cuda.enable_cudnn_sdp(False)

model = AutoModelForImageTextToText.from_pretrained(
    "MiniMaxAI/MiniMax-M3-preview",
    dtype=torch.bfloat16,
    # Dequantize the native MXFP8 weights to bf16 at load (the speed win); needs even TP/EP sharding.
    quantization_config=FineGrainedFP8Config(dequantize=True),
    distributed_config=DistributedConfig(
        tp_size=int(os.environ["WORLD_SIZE"]),
        enable_expert_parallel=True,
    ),
    attn_implementation="kernels-staging/msa@v0",  # MSA block-sparse attention kernel
)
model.eval()

tokenizer = AutoTokenizer.from_pretrained("MiniMaxAI/MiniMax-M3-preview")
messages = [{"role": "user", "content": "Summarize the history of computing."}]
inputs = tokenizer.apply_chat_template(
    messages, add_generation_prompt=True, return_tensors="pt", return_dict=True
).to(f"cuda:{os.environ.get('LOCAL_RANK', '0')}")

generated_ids = model.generate(
    **inputs,
    max_new_tokens=128,
    do_sample=False,
    # Static cache + reduce-overhead cudagraph capture is what pushes decode to ~31 tok/s.
    cache_implementation="static",
    compile_config=CompileConfig(mode="reduce-overhead", fullgraph=True),
)
if int(os.environ.get("RANK", "0")) == 0:
    print(tokenizer.decode(generated_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True))

# cudagraph-captured NCCL collectives deadlock the NCCL/CUDA destructors at teardown; the output is
# already produced, so hard-exit to skip the hanging cleanup.
if dist.is_initialized():
    sys.stdout.flush()
    os._exit(0)
```

## MiniMaxM3VLConfig[[transformers.MiniMaxM3VLConfig]]

#### transformers.MiniMaxM3VLConfig[[transformers.MiniMaxM3VLConfig]]

```python
transformers.MiniMaxM3VLConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_index: int = 200025, video_token_index: int = 200026, projector_hidden_size: int = 6144, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/configuration_minimax_m3_vl.py#L187)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

image_token_index (`int`, *optional*, defaults to `200025`) : The image token index used as a placeholder for input images.

video_token_index (`int`, *optional*, defaults to `200026`) : The video token index used as a placeholder for input videos.

projector_hidden_size (`int`, *optional*, defaults to `6144`) : Dimensionality of text and vision projection layers.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLTextConfig[[transformers.MiniMaxM3VLTextConfig]]

#### transformers.MiniMaxM3VLTextConfig[[transformers.MiniMaxM3VLTextConfig]]

```python
transformers.MiniMaxM3VLTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 200064, hidden_size: int = 6144, intermediate_size: int = 3072, num_hidden_layers: int = 60, num_attention_heads: int = 64, num_key_value_heads: int = 4, head_dim: int = 128, hidden_act: str = 'silu', max_position_embeddings: int = 524288, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = 200034, eos_token_id: int | list[int] | None = 200020, tie_word_embeddings: bool = False, attention_dropout: float | int = 0.0, num_experts_per_tok: int = 4, num_local_experts: int = 128, output_router_logits: bool = False, router_aux_loss_coef: float = 0.001, router_jitter_noise: float = 0.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, dense_intermediate_size: int = 12288, shared_intermediate_size: int = 3072, routed_scaling_factor: float = 2.0, rotary_dim: int = 64, swiglu_alpha: float = 1.702, swiglu_limit: float = 7.0, mlp_layer_types: list[str] | None = None, index_n_heads: int = 4, index_head_dim: int = 128, index_block_size: int = 128, index_topk_blocks: int = 16, index_local_blocks: int = 1, layer_types: list[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/configuration_minimax_m3_vl.py#L30)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `200064`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `6144`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `60`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `64`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `4`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `524288`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `200034`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `200020`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

num_experts_per_tok (`int`, *optional*, defaults to `4`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

num_local_experts (`int`, *optional*, defaults to `128`) : Number of local experts on each device. `num_experts` should be divisible by `num_local_experts`.

output_router_logits (`bool`, *optional*, defaults to `False`) : Whether or not the router logits should be returned by the model. Enabling this will also allow the model to output the auxiliary loss, including load balancing loss and router z-loss.

router_aux_loss_coef (`float`, *optional*, defaults to `0.001`) : Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

router_jitter_noise (`float`, *optional*, defaults to `0.0`) : Amount of noise to add to the router logits during training for better load balancing.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

dense_intermediate_size (`int`, *optional*, defaults to 12288) : Intermediate size of the dense MLP used on layers whose `mlp_layer_types` entry is `"dense"`.

shared_intermediate_size (`int`, *optional*, defaults to 3072) : Intermediate size of a single shared expert in the MoE layers.

routed_scaling_factor (`float`, *optional*, defaults to `2.0`) : Scaling factor or routed experts.

rotary_dim (`int`, *optional*, defaults to 64) : Number of head channels rotated by RoPE; the remaining channels are passed through unchanged.

swiglu_alpha (`float`, *optional*, defaults to 1.702) : Sigmoid gain of the SwiGLU-OAI activation.

swiglu_limit (`float`, *optional*, defaults to 7.0) : Clamp bound applied to the gate and up projections of the SwiGLU-OAI activation.

mlp_layer_types (`list[str]`, *optional*) : Per-layer MLP selector: `"sparse"` for a MoE block, `"dense"` for a dense MLP.

index_n_heads (`int`, *optional*, defaults to 4) : Number of heads in the lightning indexer's dot-product scoring branch.

index_head_dim (`int`, *optional*, defaults to 128) : Per-head channel dimension of the lightning indexer.

index_block_size (`int`, *optional*, defaults to 128) : Number of key tokens pooled into a single scored block.

index_topk_blocks (`int`, *optional*, defaults to 16) : Number of top-scoring key blocks each query may attend to.

index_local_blocks (`int`, *optional*, defaults to 1) : Number of key blocks immediately preceding the query always kept visible / attended to.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLVisionConfig[[transformers.MiniMaxM3VLVisionConfig]]

#### transformers.MiniMaxM3VLVisionConfig[[transformers.MiniMaxM3VLVisionConfig]]

```python
transformers.MiniMaxM3VLVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1280, intermediate_size: int = 5120, num_hidden_layers: int = 32, num_attention_heads: int = 16, num_channels: int = 3, image_size: int = 2016, patch_size: int = 14, temporal_patch_size: int = 2, spatial_merge_size: int = 2, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-05, attention_dropout: float = 0.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/configuration_minimax_m3_vl.py#L159)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1280`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `5120`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`int`, *optional*, defaults to `2016`) : The size (resolution) of each image.

patch_size (`int`, *optional*, defaults to `14`) : The size (resolution) of each patch.

temporal_patch_size (`int`, *optional*, defaults to `2`) : Temporal patch size used in the 3D patch embedding for video inputs.

spatial_merge_size (`int`, *optional*, defaults to `2`) : The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

rope_parameters (`RopeParameters`, *optional*) : Standard RoPE configuration for the vision tower's 3D rotary position embedding.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLProcessor[[transformers.MiniMaxM3VLProcessor]]

#### transformers.MiniMaxM3VLProcessor[[transformers.MiniMaxM3VLProcessor]]

```python
transformers.MiniMaxM3VLProcessor(image_processor = None, tokenizer = None, video_processor = None, chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/processing_minimax_m3_vl.py#L31)

**Parameters:**

image_processor (`MiniMaxM3VLImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

video_processor (`MiniMaxM3VLVideoProcessor`) : The video processor is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a MiniMaxM3VLProcessor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[MiniMaxM3VLProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) offers all the functionalities of [MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor), `tokenizer_class`, and [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See the
[~MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor), `~tokenizer_class`, and [~MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for more information.

#### post_process_image_text_to_text[[transformers.MiniMaxM3VLProcessor.post_process_image_text_to_text]]

```python
post_process_image_text_to_text(generated_outputs, skip_special_tokens = True, clean_up_tokenization_spaces = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/processing_minimax_m3_vl.py#L121)

**Parameters:**

generated_outputs (`torch.Tensor` or `np.ndarray`) : The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)` or `(sequence_length,)`.

skip_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.

clean_up_tokenization_spaces (`bool`, *optional*, defaults to `False`) : Whether or not to clean up the tokenization spaces. Argument passed to the tokenizer's `batch_decode` method.

- ****kwargs** : Additional arguments to be passed to the tokenizer's `batch_decode method`.

**Returns:** `list[str]`

The decoded text.

Post-process the output of the model to decode the text.

## MiniMaxM3VLImageProcessor[[transformers.MiniMaxM3VLImageProcessor]]

This is a standalone (non-modular) image processor: it shares the patch-flattening idea of [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor)
but does not inherit from it because the two diverge in ways that touch most of the class. The resize budget is driven by
a `max_pixels` attribute and a `{"height", "width"}` `size` rather than Qwen's `shortest_edge`/`longest_edge` scheme; the
`smart_resize` helper clamps the initial rounding with `max(factor, ...)`; and `_preprocess` performs real temporal
handling (5D patches, last-frame repeat to fill `temporal_patch_size`, and a `grid_t` dimension) instead of Qwen's
`grid_t = 1` + expand. Mapping to or subclassing Qwen would therefore change behavior or require overriding nearly
everything, so the processor is kept on its own.

#### transformers.MiniMaxM3VLImageProcessor[[transformers.MiniMaxM3VLImageProcessor]]

```python
transformers.MiniMaxM3VLImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/image_processing_minimax_m3_vl.py#L86)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'shortest_edge' : 3136, 'longest_edge': 451584}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

min_pixels (`int`, *kwargs*, *optional*, defaults to `56 * 56`) : The min pixels of the image to resize the image.

max_pixels (`int`, *kwargs*, *optional*, defaults to `28 * 28 * 1280`) : The max pixels of the image to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 14) : The spatial patch size of the vision encoder.

temporal_patch_size (`int`, *kwargs*, *optional*, defaults to 2) : The temporal patch size of the vision encoder.

merge_size (`int`, *kwargs*, *optional*, defaults to 2) : The merge size of the vision encoder to llm encoder.

Constructs a MiniMaxM3VLImageProcessor image processor.

#### get_number_of_image_patches[[transformers.MiniMaxM3VLImageProcessor.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/image_processing_minimax_m3_vl.py#L255)

**Parameters:**

height (`int`) : Height of the input image.

width (`int`) : Width of the input image.

images_kwargs (`dict`, *optional*) : Any kwargs to override defaults of the image processor.

**Returns:** `int`

Number of image patches per image.

A utility that returns number of image patches for a given image size.

Note: Do not remove this method! It is used by vLLM to infer the number of patches and placeholders
without an image input.

#### patchify[[transformers.MiniMaxM3VLImageProcessor.patchify]]

```python
patchify(images: torch.Tensor, patch_size: int, merge_size: int, temporal_patch_size: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/image_processing_minimax_m3_vl.py#L163)

Patchifies each image into flat layout of shape (`seq_len`, `patch_dim`) so we can concat dynamically shaped pixels.

#### preprocess[[transformers.MiniMaxM3VLImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/image_processing_minimax_m3_vl.py#L129)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

min_pixels (`int`, *kwargs*, *optional*, defaults to `56 * 56`) : The min pixels of the image to resize the image.

max_pixels (`int`, *kwargs*, *optional*, defaults to `28 * 28 * 1280`) : The max pixels of the image to resize the image.

patch_size (`int`, *kwargs*, *optional*, defaults to 14) : The spatial patch size of the vision encoder.

temporal_patch_size (`int`, *kwargs*, *optional*, defaults to 2) : The temporal patch size of the vision encoder.

merge_size (`int`, *kwargs*, *optional*, defaults to 2) : The merge size of the vision encoder to llm encoder.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### resize[[transformers.MiniMaxM3VLImageProcessor.resize]]

```python
resize(images: torch.Tensor, size: SizeDict, resample: PILImageResampling | tvF.InterpolationMode | int | None, factor: int, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/image_processing_minimax_m3_vl.py#L137)

Resize dynamically based on input image aspect ratio.

## MiniMaxM3VLVideoProcessor[[transformers.MiniMaxM3VLVideoProcessor]]

#### transformers.MiniMaxM3VLVideoProcessor[[transformers.MiniMaxM3VLVideoProcessor]]

```python
transformers.MiniMaxM3VLVideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/video_processing_minimax_m3_vl.py#L35)

## MiniMaxM3VLVisionModel[[transformers.MiniMaxM3VLVisionModel]]

#### transformers.MiniMaxM3VLVisionModel[[transformers.MiniMaxM3VLVisionModel]]

```python
transformers.MiniMaxM3VLVisionModel(config: MiniMaxM3VLVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1173)

**Parameters:**

config ([MiniMaxM3VLVisionConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Minimax M3 Vl Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniMaxM3VLVisionModel.forward]]

```python
forward(pixel_values: Tensor, grid_thw: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1194)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

grid_thw (`torch.Tensor` of shape `(num_images, 3)`) : The temporal, height and width of feature shape of each image.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [MiniMaxM3VLVisionModel](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## MiniMaxM3VLTextModel[[transformers.MiniMaxM3VLTextModel]]

#### transformers.MiniMaxM3VLTextModel[[transformers.MiniMaxM3VLTextModel]]

```python
transformers.MiniMaxM3VLTextModel(config: MiniMaxM3VLTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L716)

**Parameters:**

config ([MiniMaxM3VLTextConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Minimax M3 Vl Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniMaxM3VLTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L733)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.

The [MiniMaxM3VLTextModel](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logits (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## MiniMaxM3VLModel[[transformers.MiniMaxM3VLModel]]

#### transformers.MiniMaxM3VLModel[[transformers.MiniMaxM3VLModel]]

```python
transformers.MiniMaxM3VLModel(config: MiniMaxM3VLConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1297)

**Parameters:**

config ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MiniMax M3 VL backbone (vision + projector + text), without LM head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniMaxM3VLModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.Tensor] = None, video_grid_thw: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1366)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor). See `MiniMaxM3VLImageProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses [MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See `MiniMaxM3VLVideoProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for processing videos).

image_grid_thw (`torch.Tensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.Tensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** `MiniMaxM3VLModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `MiniMaxM3VLModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.

The [MiniMaxM3VLModel](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(num_image_patches, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(num_video_patches, hidden_size)`.
  video_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

## MiniMaxM3VLForCausalLM[[transformers.MiniMaxM3VLForCausalLM]]

#### transformers.MiniMaxM3VLForCausalLM[[transformers.MiniMaxM3VLForCausalLM]]

```python
transformers.MiniMaxM3VLForCausalLM(config: MiniMaxM3VLTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L878)

**Parameters:**

config ([MiniMaxM3VLTextConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Minimax M3 Vl Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniMaxM3VLForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_router_logits: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L897)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_router_logits (`bool`, *optional*) : Whether or not to return the logits of all the routers. They are useful for computing the router loss, and should not be returned during inference.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `MoeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `MoeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.

The [MiniMaxM3VLForCausalLM](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logits (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, MiniMaxM3VLForCausalLM

>>> model = MiniMaxM3VLForCausalLM.from_pretrained("mistralai/MiniMaxM3VL-8x7B-v0.1")
>>> tokenizer = AutoTokenizer.from_pretrained("mistralai/MiniMaxM3VL-8x7B-v0.1")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```

## MiniMaxM3SparseForConditionalGeneration[[transformers.MiniMaxM3SparseForConditionalGeneration]]

#### transformers.MiniMaxM3SparseForConditionalGeneration[[transformers.MiniMaxM3SparseForConditionalGeneration]]

```python
transformers.MiniMaxM3SparseForConditionalGeneration(config: MiniMaxM3VLConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1443)

**Parameters:**

config ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MiniMax M3 VL full model with LM head (text + vision).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniMaxM3SparseForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.Tensor] = None, video_grid_thw: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minimax_m3_vl/modeling_minimax_m3_vl.py#L1460)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor). See `MiniMaxM3VLImageProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses [MiniMaxM3VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See `MiniMaxM3VLVideoProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for processing videos).

image_grid_thw (`torch.Tensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.Tensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `MiniMaxM3VLCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `MiniMaxM3VLCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.

The [MiniMaxM3SparseForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3SparseForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(num_image_patches, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(num_video_patches, hidden_size)`.
  video_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, MiniMaxM3SparseForConditionalGeneration

>>> model = MiniMaxM3SparseForConditionalGeneration.from_pretrained("mini_max_m3_sparse-hf/mini_max_m3_sparse-1.5-7b-hf")
>>> processor = AutoProcessor.from_pretrained("mini_max_m3_sparse-hf/mini_max_m3_sparse-1.5-7b-hf")

>>> prompt = "USER: <image>\nWhat's the content of the image? ASSISTANT:"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_new_tokens=15)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"USER:  \nWhat's the content of the image? ASSISTANT: The image features a busy city street with a stop sign prominently displayed"
```
