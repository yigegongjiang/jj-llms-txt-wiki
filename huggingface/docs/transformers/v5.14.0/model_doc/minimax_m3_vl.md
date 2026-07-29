# MiniMax-M3-VL

## Overview

MiniMax-M3-VL is the vision-language member of the MiniMax-M3 family. It pairs a CLIP-style vision tower (Conv3d patch embedding with 3D rotary position embeddings) with the MiniMax-M3 text backbone, a mixed dense/sparse Mixture-of-Experts decoder that uses SwiGLU-OAI gated experts and a lightning indexer for block-sparse attention.

## Architecture
### Block-sparse attention (Lightning Indexer)

Every layer is GQA (`num_key_value_heads = 4`) with per-head QK-norm and **partial RoPE** on the first
`rotary_dim`. `config.layer_types[i]` then picks `"full_attention"` (dense causal) or
`"minimax_m3_sparse"`, where a `MiniMaxM3VLIndexer` decides, per query, which block of keys the main attention may see.

The indexer scores every key, then **max-poolsthose per-key scores into blocks of `index_block_size` keys**, so selection happens at the granularity of a *block
of keys*: per query it keeps the top-`index_topk_blocks` key blocks plus the always-on `index_local_blocks`
local-window block (under block-level causality), broadcasts the per-block `0`/`-inf` choice back onto every key in
the block. The result is a `[B, 1, S_q, S_k]` additive bias summed onto the causal mask. 
Theoretically this means that the attention is only computed over the selected blocks of keys, but `transformers` does not support the kernels that compute this efficiently! 
We are adding it to `kernels` asap!

### Vision tower

A [MiniMaxM3VLVisionModel](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionModel): a `Conv3d` patch embedding over flattened `[N_patches, C·T·P·P]` input, a stack of
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
    tp_plan="auto",
    distributed_config=DistributedConfig(enable_expert_parallel=True),
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

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `200025`) --
  The image token index used as a placeholder for input images.
- **video_token_index** (`int`, *optional*, defaults to `200026`) --
  The video token index used as a placeholder for input videos.
- **projector_hidden_size** (`int`, *optional*, defaults to `6144`) --
  Dimensionality of text and vision projection layers.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLTextConfig[[transformers.MiniMaxM3VLTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `200064`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `6144`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `60`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `64`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `4`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **head_dim** (`int`, *optional*, defaults to `128`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `524288`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `200034`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `200020`) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_experts_per_tok** (`int`, *optional*, defaults to `4`) --
  Number of experts to route each token to. This is the top-k value for the token-choice routing.
- **num_local_experts** (`int`, *optional*, defaults to `128`) --
  Number of local experts on each device. `num_experts` should be divisible by `num_local_experts`.
- **output_router_logits** (`bool`, *optional*, defaults to `False`) --
  Whether or not the router logits should be returned by the model. Enabling this will also allow the model
  to output the auxiliary loss, including load balancing loss and router z-loss.
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.
- **router_jitter_noise** (`float`, *optional*, defaults to `0.0`) --
  Amount of noise to add to the router logits during training for better load balancing.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **dense_intermediate_size** (`int`, *optional*, defaults to 12288) --
  Intermediate size of the dense MLP used on layers whose `mlp_layer_types` entry is `"dense"`.
- **shared_intermediate_size** (`int`, *optional*, defaults to 3072) --
  Intermediate size of a single shared expert in the MoE layers.
- **routed_scaling_factor** (`float`, *optional*, defaults to `2.0`) --
  Scaling factor or routed experts.
- **rotary_dim** (`int`, *optional*, defaults to 64) --
  Number of head channels rotated by RoPE; the remaining channels are passed through unchanged.
- **swiglu_alpha** (`float`, *optional*, defaults to 1.702) --
  Sigmoid gain of the SwiGLU-OAI activation.
- **swiglu_limit** (`float`, *optional*, defaults to 7.0) --
  Clamp bound applied to the gate and up projections of the SwiGLU-OAI activation.
- **mlp_layer_types** (`list[str]`, *optional*) --
  Per-layer MLP selector: `"sparse"` for a MoE block, `"dense"` for a dense MLP.
- **index_n_heads** (`int`, *optional*, defaults to 4) --
  Number of heads in the lightning indexer's dot-product scoring branch.
- **index_head_dim** (`int`, *optional*, defaults to 128) --
  Per-head channel dimension of the lightning indexer.
- **index_block_size** (`int`, *optional*, defaults to 128) --
  Number of key tokens pooled into a single scored block.
- **index_topk_blocks** (`int`, *optional*, defaults to 16) --
  Number of top-scoring key blocks each query may attend to.
- **index_local_blocks** (`int`, *optional*, defaults to 1) --
  Number of key blocks immediately preceding the query always kept visible / attended to.
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLVisionConfig[[transformers.MiniMaxM3VLVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `5120`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`int`, *optional*, defaults to `2016`) --
  The size (resolution) of each image.
- **patch_size** (`int`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **temporal_patch_size** (`int`, *optional*, defaults to `2`) --
  Temporal patch size used in the 3D patch embedding for video inputs.
- **spatial_merge_size** (`int`, *optional*, defaults to `2`) --
  The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`float`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **rope_parameters** (`RopeParameters`, *optional*) --
  Standard RoPE configuration for the vision tower's 3D rotary position embedding.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a MiniMaxM3VLModel. It is used to instantiate a Minimax M3 Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [MiniMaxAI/MiniMax-M3-preview](https://huggingface.co/MiniMaxAI/MiniMax-M3-preview)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniMaxM3VLProcessor[[transformers.MiniMaxM3VLProcessor]]

- **image_processor** (`MiniMaxM3VLImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **video_processor** (`MiniMaxM3VLVideoProcessor`) --
  The video processor is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a MiniMaxM3VLProcessor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[MiniMaxM3VLProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) offers all the functionalities of [MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor), `tokenizer_class`, and [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See the
[~MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor), `~tokenizer_class`, and [~MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for more information.

- **generated_outputs** (`torch.Tensor` or `np.ndarray`) --
  The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)`
  or `(sequence_length,)`.
- **skip_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.
- **clean_up_tokenization_spaces** (`bool`, *optional*, defaults to `False`) --
  Whether or not to clean up the tokenization spaces. Argument passed to the tokenizer's `batch_decode` method.
- ****kwargs** --
  Additional arguments to be passed to the tokenizer's `batch_decode method`.`list[str]`The decoded text.

Post-process the output of the model to decode the text.

## MiniMaxM3VLImageProcessor[[transformers.MiniMaxM3VLImageProcessor]]

This is a standalone (non-modular) image processor: it shares the patch-flattening idea of [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor)
but does not inherit from it because the two diverge in ways that touch most of the class. The resize budget is driven by
a `max_pixels` attribute and a `{"height", "width"}` `size` rather than Qwen's `shortest_edge`/`longest_edge` scheme; the
`smart_resize` helper clamps the initial rounding with `max(factor, ...)`; and `_preprocess` performs real temporal
handling (5D patches, last-frame repeat to fill `temporal_patch_size`, and a `grid_t` dimension) instead of Qwen's
`grid_t = 1` + expand. Mapping to or subclassing Qwen would therefore change behavior or require overriding nearly
everything, so the processor is kept on its own.

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a MiniMaxM3VLImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **max_pixels** (`int`, *kwargs*, *optional*, defaults to 451584) --
  The max pixels of the image to resize the image.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## MiniMaxM3VLVideoProcessor[[transformers.MiniMaxM3VLVideoProcessor]]

## MiniMaxM3VLVisionModel[[transformers.MiniMaxM3VLVisionModel]]

- **config** ([MiniMaxM3VLVisionConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Minimax M3 Vl Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "grid_thw", "val": ": )>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **grid_thw** (`torch.Tensor` of shape `(num_images, 3)`) --
  The temporal, height and width of feature shape of each image.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [MiniMaxM3VLVisionModel](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVisionModel) forward method, overrides the `__call__` special method.

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

- **config** ([MiniMaxM3VLTextConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Minimax M3 Vl Text Model outputting raw hidden-states without any specific head on to.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`MoeModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.
The [MiniMaxM3VLTextModel](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.

## MiniMaxM3VLModel[[transformers.MiniMaxM3VLModel]]

- **config** ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
MiniMax M3 VL backbone (vision + projector + text), without LM head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor). See `MiniMaxM3VLImageProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses
  [MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See `MiniMaxM3VLVideoProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses
  [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for processing videos).
- **image_grid_thw** (`torch.Tensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of each image's feature grid, used to build the vision 3D RoPE
  and to merge patch features.
- **video_grid_thw** (`torch.Tensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of each video's feature grid, used to build the vision 3D RoPE
  and to merge patch features.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.`MiniMaxM3VLModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MiniMaxM3VLModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.
The [MiniMaxM3VLModel](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

- **config** ([MiniMaxM3VLTextConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Minimax M3 Vl Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_router_logits** (`bool`, *optional*) --
  Whether or not to return the logits of all the routers. They are useful for computing the router loss, and
  should not be returned during inference.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`MoeCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `MoeCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.
The [MiniMaxM3VLForCausalLM](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **aux_loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- aux_loss for the sparse modules.
- **router_logits** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_router_probs=True` and `config.add_router_probs=True` is passed or when `config.output_router_probs=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, sequence_length, num_experts)`.

  Raw router logtis (post-softmax) that are computed by MoE routers, these terms are used to compute the auxiliary
  loss for Mixture of Experts models.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

- **config** ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
MiniMax M3 VL full model with LM head (text + vision).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor). See `MiniMaxM3VLImageProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses
  [MiniMaxM3VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor). See `MiniMaxM3VLVideoProcessor.__call__()` for details ([MiniMaxM3VLProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLProcessor) uses
  [MiniMaxM3VLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLVideoProcessor) for processing videos).
- **image_grid_thw** (`torch.Tensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of each image's feature grid, used to build the vision 3D RoPE
  and to merge patch features.
- **video_grid_thw** (`torch.Tensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of each video's feature grid, used to build the vision 3D RoPE
  and to merge patch features.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`MiniMaxM3VLCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `MiniMaxM3VLCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniMaxM3VLConfig](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3VLConfig)) and inputs.
The [MiniMaxM3SparseForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/minimax_m3_vl#transformers.MiniMaxM3SparseForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoProcessor, MiniMaxM3SparseForConditionalGeneration

>>> model = MiniMaxM3SparseForConditionalGeneration.from_pretrained("MiniMaxAI/MiniMax-M3-preview")
>>> processor = AutoProcessor.from_pretrained("MiniMaxAI/MiniMax-M3-preview")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```
