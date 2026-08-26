# LLaVa-NeXT-Video

## Overview

The LLaVa-NeXT-Video model was proposed in [LLaVA-NeXT: A Strong Zero-shot Video Understanding Model](https://llava-vl.github.io/blog/2024-04-30-llava-next-video/) by Yuanhan Zhang, Bo Li, Haotian Liu, Yong Jae Lee, Liangke Gui, Di Fu, Jiashi Feng, Ziwei Liu, Chunyuan Li. LLaVa-NeXT-Video improves upon [LLaVa-NeXT](llava_next) by fine-tuning on a mix if video and image dataset thus increasing the model's performance on videos.

[LLaVA-NeXT](llava_next) surprisingly has strong performance in understanding video content in zero-shot fashion with the AnyRes technique that it uses. The AnyRes technique naturally represents a high-resolution image into multiple images. This technique is naturally generalizable to represent videos because videos can be considered as a set of frames (similar to a set of images in LLaVa-NeXT). The current version of LLaVA-NeXT makes use of AnyRes and trains with supervised fine-tuning (SFT) on top of LLaVA-Next on video data to achieves better video understanding capabilities.The model is a current SOTA among open-source models on [VideoMME bench](https://huggingface.co/papers/2405.21075).

The introduction from the blog is the following:

On January 30, 2024, we released LLaVA-NeXT, an open-source Large Multimodal Model (LMM) that has been trained exclusively on text-image data. With the proposed AnyRes technique, it boosts capabilities in reasoning, OCR, and world knowledge, demonstrating remarkable performance across a spectrum of image-based multimodal understanding tasks, and even exceeding Gemini-Pro on several image benchmarks, e.g. MMMU and MathVista.

**In today's exploration, we delve into the performance of LLaVA-NeXT within the realm of video understanding tasks. We reveal that LLaVA-NeXT surprisingly has strong performance in understanding video content. The current version of LLaVA-NeXT for videos has several improvements:

- Zero-shot video representation capabilities with AnyRes: The AnyRes technique naturally represents a high-resolution image into multiple images that a pre-trained VIT is able to digest, and forms them into a concatenated sequence. This technique is naturally generalizable to represent videos (consisting of multiple frames), allowing the image-only-trained LLaVA-Next model to perform surprisingly well on video tasks. Notably, this is the first time that LMMs show strong zero-shot modality transfer ability.
- Inference with length generalization improves on longer videos. The linear scaling technique enables length generalization, allowing LLaVA-NeXT to effectively handle long-video beyond the limitation of the "max_token_length" of the LLM.
- Strong video understanding ability. (1) LLaVA-Next-Image, which combines the above two techniques, yields superior zero-shot performance than open-source LMMs tuned on videos. (2) LLaVA-Next-Video, further supervised fine-tuning (SFT) LLaVA-Next-Image on video data, achieves better video understanding capabilities compared to LLaVA-Next-Image. (3) LLaVA-Next-Video-DPO, which aligns the model response with AI feedback using direct preference optimization (DPO), showing significant performance boost.
- Efficient deployment and inference with SGLang. It allows 5x faster inference on video tasks, allowing more scalable serving such as million-level video re-captioning. See instructions in our repo.**

This model was contributed by [RaushanTurganbay](https://huggingface.co/RaushanTurganbay).
The original code can be found [here](https://github.com/LLaVA-VL/LLaVA-NeXT/tree/inference).

## Usage tips

- We advise users to use `padding_side="left"` when computing batched generation as it leads to more accurate results. Simply make sure to call `processor.tokenizer.padding_side = "left"` before generating.

- Llava-Next uses different number of patches for images and thus has to pad the inputs inside modeling code, aside from the padding done when processing the inputs. The default setting is "left-padding" if model is in `eval()` mode, otherwise "right-padding".

> [!NOTE]
> LLaVA models after release v4.46 will raise warnings about adding `processor.patch_size = {{patch_size}}`, `processor.num_additional_image_tokens = {{num_additional_image_tokens}}` and `processor.vision_feature_select_strategy = {{vision_feature_select_strategy}}`. It is strongly recommended to add the attributes to the processor if you own the model checkpoint, or open a PR if it is not owned by you.
Adding these attributes means that LLaVA will try to infer the number of image tokens required per image and expand the text with as many `<image>` placeholders as there will be tokens. Usually it is around 500 tokens per image, so make sure that the text is not truncated as otherwise there will be failure when merging the embeddings.
The attributes can be obtained from model config, as `model.config.vision_config.patch_size` or `model.config.vision_feature_select_strategy`. The `num_additional_image_tokens` should be `1` if the vision backbone adds a CLS token or `0` if nothing extra is added to the vision patches.

### Formatting Prompts with Chat Templates  

Each **checkpoint** is trained with a specific prompt format, depending on the underlying large language model backbone. To ensure correct formatting, use the processor's `apply_chat_template` method.  

**Important:**  

- You must construct a conversation history — passing a plain string won't work.  
- Each message should be a dictionary with `"role"` and `"content"` keys.  
- The `"content"` should be a list of dictionaries for different modalities like `"text"` and `"image"`.  

Here's an example of how to structure your input. We will use [LLaVA-NeXT-Video-7B-hf](https://huggingface.co/llava-hf/LLaVA-NeXT-Video-7B-hf) and a conversation history of videos and images.

```python
from transformers import LlavaNextVideoProcessor

processor = LlavaNextVideoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")

conversation = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "A chat between a curious human and an artificial intelligence assistant. The assistant gives helpful, detailed, and polite answers to the human's questions."},
            ],
    },
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "What’s shown in this image?"},
            {"type": "image"},
            ],
    },
    {
        "role": "assistant",
        "content": [{"type": "text", "text": "This image shows a red stop sign."},]
    },
    {

        "role": "user",
        "content": [
            {"type": "text", "text": "Why is this video funny?"},
            {"type": "video"},
            ],
    },
]

text_prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)

# Note that the template simply formats your prompt, you still have to tokenize it and obtain pixel values for your visuals
print(text_prompt)
```

🚀 **Bonus:** If you're using `transformers>=4.49.0`, you can also get a vectorized output from `apply_chat_template`. See the **Usage Examples** below for more details on how to use it.

## Usage example

### Single Media Mode

The model can accept both images and videos as input. Here's an example code for inference in half-precision (`torch.float16`):

```python
from huggingface_hub import hf_hub_download

from transformers import LlavaNextVideoForConditionalGeneration, LlavaNextVideoProcessor

# Load the model in half-precision
model = LlavaNextVideoForConditionalGeneration.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf", device_map="auto")
processor = LlavaNextVideoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")

# Load the video as an np.array, sampling uniformly 8 frames (can sample more for longer videos)
video_path = hf_hub_download(repo_id="raushan-testing-hf/videos-test", filename="sample_demo_1.mp4", repo_type="dataset")

conversation = [
    {

        "role": "user",
        "content": [
            {"type": "text", "text": "Why is this video funny?"},
            {"type": "video", "path": video_path},
            ],
    },
]

inputs = processor.apply_chat_template(conversation, num_frames=8, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

out = model.generate(**inputs, max_new_tokens=60)
processor.batch_decode(out, skip_special_tokens=True, clean_up_tokenization_spaces=True)
```

### Mixed Media Mode

The model can also generate from an interleaved image-video inputs. However note, that it was not trained in interleaved image-video setting which might affect the performance. Below is an example usage for mixed media input, add the following lines to the above code snippet:

```python

# Generate from image and video mixed inputs
conversation = [
    {

        "role": "user",
        "content": [
            {"type": "text", "text": "How many cats are there in the image?"},
            {"type": "image", "url": "http://images.cocodataset.org/val2017/000000039769.jpg"},
            ],
    },
    {

        "role": "assistant",
        "content": [{"type": "text", "text": "There are two cats"}],
    },
    {

        "role": "user",
        "content": [
            {"type": "text", "text": "Why is this video funny?"},
            {"type": "video", "path": video_path},
            ],
    },
]
inputs = processor.apply_chat_template(conversation, num_frames=8, add_generation_prompt=True, tokenize=True, return_dict=True, padding=True, return_tensors="pt").to(model.device)

# Generate
generate_ids = model.generate(**inputs, max_length=50)
processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=True)

```

## Model optimization

### Quantization using Bitsandbytes for memory efficiency

The model can be loaded in lower bits, significantly reducing memory burden while maintaining the performance of the original model. This allows for efficient deployment on resource-constrained cases.

First, make sure to install bitsandbytes by running `pip install bitsandbytes` and to have access to a GPU/accelerator that is supported by the library.

bitsandbytes is being refactored to support multiple backends beyond CUDA. Currently, ROCm (AMD GPU) and Intel CPU implementations are mature, with Intel XPU in progress and Apple Silicon support expected by Q4/Q1. For installation instructions and the latest backend updates, visit [this link](https://huggingface.co/docs/bitsandbytes/main/en/installation#multi-backend).

We value your feedback to help identify bugs before the full release! Check out [these docs](https://huggingface.co/docs/bitsandbytes/main/en/non_cuda_backends) for more details and feedback links.

Then simply load the quantized model by adding [BitsAndBytesConfig](/docs/transformers/v5.15.1/en/main_classes/quantization#transformers.BitsAndBytesConfig) as shown below:

```python
from transformers import LlavaNextVideoForConditionalGeneration

# specify how to quantize the model
quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.float16,
)

model = LlavaNextVideoForConditionalGeneration.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf", quantization_config=quantization_config, device_map="auto")
```

### Flash-Attention 2 to speed-up generation

Additionally, we can greatly speed-up model inference by using [Flash Attention](../perf_train_gpu_one#flash-attention-2), which is a faster implementation of the attention mechanism used inside the model.

First, make sure to install the latest version of Flash Attention 2:

```bash
pip install -U flash-attn --no-build-isolation
```

Also, you should have a hardware that is compatible with Flash-Attention 2. Read more about it in the official documentation of the [flash attention repository](https://github.com/Dao-AILab/flash-attention). FlashAttention-2 can only be used when a model is loaded in `torch.float16` or `torch.bfloat16`.

To load and run a model using Flash Attention-2, simply add `attn_implementation="flash_attention_2"` when loading the model as follows:

```python
from transformers import LlavaNextVideoForConditionalGeneration

model = LlavaNextVideoForConditionalGeneration.from_pretrained(
    "llava-hf/LLaVA-NeXT-Video-7B-hf", , 
    attn_implementation="flash_attention_2",
).to(0, device_map="auto")
```

## LlavaNextVideoConfig[[transformers.LlavaNextVideoConfig]]

#### transformers.LlavaNextVideoConfig[[transformers.LlavaNextVideoConfig]]

```python
transformers.LlavaNextVideoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_index: int = 32001, video_token_index: int = 32000, projector_hidden_act: str = 'gelu', vision_feature_select_strategy: typing.Literal['default', 'full'] = 'default', vision_feature_layer: int | list[int] = -2, multimodal_projector_bias: bool = True, tie_word_embeddings: bool = False, image_grid_pinpoints: list | None = None, spatial_pool_mode: str = 'average', spatial_pool_stride: int = 2, image_seq_length: int = 576, video_seq_length: int = 288)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/configuration_llava_next_video.py#L32)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

image_token_index (`int`, *optional*, defaults to `32001`) : The image token index used as a placeholder for input images.

video_token_index (`int`, *optional*, defaults to `32000`) : The video token index used as a placeholder for input videos.

projector_hidden_act (`str`, *optional*, defaults to `gelu`) : The activation function used by the multimodal projector.

vision_feature_select_strategy (`Literal[default, full]`, *optional*, defaults to `default`) : The feature selection strategy used to select the vision feature from the vision backbone.

vision_feature_layer (`Union[int, list[int]]`, *optional*, defaults to `-2`) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

multimodal_projector_bias (`bool`, *optional*, defaults to `True`) : Whether to use bias in the multimodal projector.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

image_grid_pinpoints (`List`, *optional*, defaults to `[[336, 672], [672, 336], [672, 672], [1008, 336], [336, 1008]]`) : A list of possible resolutions to use for processing high resolution images. Each item in the list should be a tuple or list of the form `(height, width)`.

spatial_pool_mode (`str`, *optional*, defaults to `"average"`) : Pooling mode to use for videos. Can be "average", "max" or "conv".

spatial_pool_stride (`int`, *optional*, defaults to 2) : Stride used in the pooling layer for videos.

image_seq_length (`int`, *optional*, defaults to `576`) : Sequence length of one image embedding.

video_seq_length (`int`, *optional*, defaults to `288`) : Sequence length of one video embedding.

This is the configuration class to store the configuration of a LlavaNextVideoModel. It is used to instantiate a Llava Next Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [llava-hf/LLaVA-NeXT-Video-7B-hf](https://huggingface.co/llava-hf/LLaVA-NeXT-Video-7B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LlavaNextVideoForConditionalGeneration, LlavaNextVideoConfig, CLIPVisionConfig, LlamaConfig

>>> # Initializing a CLIP-vision config
>>> vision_config = CLIPVisionConfig()

>>> # Initializing a Llama config
>>> text_config = LlamaConfig()

>>> configuration = LlavaNextVideoConfig(vision_config, text_config)

>>> model = LlavaNextVideoForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LlavaNextVideoProcessor[[transformers.LlavaNextVideoProcessor]]

#### transformers.LlavaNextVideoProcessor[[transformers.LlavaNextVideoProcessor]]

```python
transformers.LlavaNextVideoProcessor(video_processor = None, image_processor = None, tokenizer = None, chat_template = None, patch_size = None, vision_feature_select_strategy = None, video_token = '<video>', image_token = '<image>', num_additional_image_tokens = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/processing_llava_next_video.py#L42)

**Parameters:**

video_processor (`LlavaNextVideoVideoProcessor`) : The video processor is a required input.

image_processor (`LlavaNextImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

patch_size (`int`, *optional*) : Patch size from the vision tower.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Should be same as in model's config

video_token (`str`, *optional*, defaults to `"<video>"`) : Special token used to denote video location.

image_token (`str`, *optional*, defaults to `"<image>"`) : Special token used to denote image location.

num_additional_image_tokens (`int`, *optional*, defaults to 0) : Number of additional tokens added to the image embeddings, such as CLS (+1). If the backbone has no CLS or other extra tokens appended, no need to set this arg.

Constructs a LlavaNextVideoProcessor which wraps a video processor, a image processor, and a tokenizer into a single processor.

[LlavaNextVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoProcessor) offers all the functionalities of [LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor), [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor), and `tokenizer_class`. See the
[~LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor), [~LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor), and `~tokenizer_class` for more information.

## LlavaNextVideoVideoProcessor[[transformers.LlavaNextVideoVideoProcessor]]

#### transformers.LlavaNextVideoVideoProcessor[[transformers.LlavaNextVideoVideoProcessor]]

```python
transformers.LlavaNextVideoVideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/video_processing_llava_next_video.py#L20)

## LlavaNextVideoModel[[transformers.LlavaNextVideoModel]]

#### transformers.LlavaNextVideoModel[[transformers.LlavaNextVideoModel]]

```python
transformers.LlavaNextVideoModel(config: LlavaNextVideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L302)

**Parameters:**

config ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Llava-Next model which consists of a vision backbone and a language model without language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LlavaNextVideoModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_sizes: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L502)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([LlavaNextVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoProcessor) uses [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor). See `LlavaNextVideoVideoProcessor.__call__()` for details ([LlavaNextVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoProcessor) uses [LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor) for processing videos).

image_sizes (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) : The sizes of the images in the batch, being (height, width) for each image.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*, defaults to `"default"`) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features. If `"full"`, the full vision features are used.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `LlavaNextVideoModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `LlavaNextVideoModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

The [LlavaNextVideoModel](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoModel) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor`  of size `(batch_size * num_frames, num_videos, sequence_length, hidden_size)`.
  video_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

#### get_image_features[[transformers.LlavaNextVideoModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_sizes: Tensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, output_hidden_states: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L388)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_patches, channels, height, width)`) : The tensors corresponding to the input images.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

image_sizes (`torch.Tensor` of shape `(batch_size, 2)`) : The sizes of the images in the batch, being (height, width) for each image.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

Obtains image last hidden states from the vision tower and apply multimodal projection.

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

#### get_placeholder_mask[[transformers.LlavaNextVideoModel.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, image_features: typing.Optional[torch.FloatTensor] = None, video_features: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L461)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

#### get_video_features[[transformers.LlavaNextVideoModel.get_video_features]]

```python
get_video_features(pixel_values: FloatTensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, output_hidden_states: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L584)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_frames, channels, height, width)`) : The tensors corresponding to the input video.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

Obtains video last hidden states from the vision tower and apply multimodal projection.

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

#### pack_image_features[[transformers.LlavaNextVideoModel.pack_image_features]]

```python
pack_image_features(image_features, image_sizes, vision_feature_select_strategy, image_newline = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L321)

**Parameters:**

image_features (`list[torch.Tensor]` of length num_images, each of shape `(num_patches, image_length, embed_dim)`) : List of image feature tensor, each contains all the visual feature of all patches.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_select_strategy (`str`) : The feature selection strategy used to select the vision feature from the vision backbone.

image_newline (`torch.Tensor` of shape `(embed_dim)`) : New line embedding vector.

**Returns:**

image_features (`torch.Tensor` of shape `(all_feat_len, embed_dim)`)
feature_lens (`list[int]`)
token length of each image in image_features

Reshape, unpad and then pack each image_feature into a single image_features tensor containing all visual vectors.

## LlavaNextVideoForConditionalGeneration[[transformers.LlavaNextVideoForConditionalGeneration]]

#### transformers.LlavaNextVideoForConditionalGeneration[[transformers.LlavaNextVideoForConditionalGeneration]]

```python
transformers.LlavaNextVideoForConditionalGeneration(config: LlavaNextVideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L642)

**Parameters:**

config ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LLAVA-NeXT model which consists of a vision backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LlavaNextVideoForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_sizes: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L694)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([LlavaNextVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoProcessor) uses [LlavaNextImageProcessor](/docs/transformers/v5.15.1/en/model_doc/granitevision#transformers.LlavaNextImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor). See `LlavaNextVideoVideoProcessor.__call__()` for details ([LlavaNextVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoProcessor) uses [LlavaNextVideoVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoVideoProcessor) for processing videos).

image_sizes (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) : The sizes of the images in the batch, being (height, width) for each image.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `LlavaNextVideoCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `LlavaNextVideoCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

The [LlavaNextVideoForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size (batch_size * num_patches, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor`  of size `(batch_size * num_frames, num_videos, sequence_length, hidden_size)`.
  video_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import av
>>> from transformers import AutoProcessor, LlavaNextVideoForConditionalGeneration

>>> def read_video_pyav(container, indices):
...     '''
...     Decode the video with PyAV decoder.
...     Args:
...         container (`av.container.input.InputContainer`): PyAV container.
...         indices (`list[int]`): List of frame indices to decode.
...     Returns:
...         result (np.ndarray): np array of decoded frames of shape (num_frames, height, width, 3).
...     '''
...     frames = []
...     container.seek(0)
...     start_index = indices[0]
...     end_index = indices[-1]
...     for i, frame in enumerate(container.decode(video=0)):
...         if i > end_index:
...             break
...         if i >= start_index and i in indices:
...             frames.append(frame)
...     return np.stack([x.to_ndarray(format="rgb24") for x in frames])

>>> model = LlavaNextVideoForConditionalGeneration.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf", device_map="auto")
>>> processor = AutoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")

>>> prompt = "USER: <video>\nWhy is this video funny? ASSISTANT:"
>>> video_path = hf_hub_download(repo_id="raushan-testing-hf/videos-test", filename="sample_demo_1.mp4", repo_type="dataset")
>>> container = av.open(video_path)

>>> # sample uniformly 8 frames from the video (model was trained with 32 frames per video, but this video is short)
>>> total_frames = container.streams.video[0].frames
>>> indices = np.arange(0, total_frames, total_frames / 8).astype(int)
>>> clip = read_video_pyav(container, indices)
>>> inputs_video = processor(text=prompt, videos=clip, return_tensors="pt").to(model.device)

>>> # load an image to generate from an image
>>> prompt = "USER:<image>\nWhat is shown in this image? ASSISTANT:"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs_image = processor(text=prompt, images=image, return_tensors="pt").to(model.device)

>>> # Generate from video
>>> generate_ids = model.generate(**inputs_video, max_length=50)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"USER:\nWhy is this video funny? ASSISTANT: The humor in this video comes from the unexpected and endearing sight of a baby wearing glasses and (...)"

>>> # Generate from image
>>> generate_ids = model.generate(**inputs_image, max_length=30)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"USER: \nWhat's the content of the image? ASSISTANT: The image shows a red stop sign on a pole, with a traditional Chinese archway (...)"
```

#### get_image_features[[transformers.LlavaNextVideoForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, image_sizes: Tensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L662)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_patches, channels, height, width)`) : The tensors corresponding to the input images.

image_sizes (`torch.Tensor` of shape `(num_images, 2)`) : Actual image size of each images (H, W).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

image_sizes (`torch.Tensor` of shape `(batch_size, 2)`) : The sizes of the images in the batch, being (height, width) for each image.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, LlavaNextVideoForConditionalGeneration

>>> model = LlavaNextVideoForConditionalGeneration.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")

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

#### get_video_features[[transformers.LlavaNextVideoForConditionalGeneration.get_video_features]]

```python
get_video_features(pixel_values: FloatTensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llava_next_video/modeling_llava_next_video.py#L815)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_frames, channels, height, width)`) : The tensors corresponding to the input video.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LlavaNextVideoConfig](/docs/transformers/v5.15.1/en/model_doc/llava_next_video#transformers.LlavaNextVideoConfig)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, LlavaNextVideoForConditionalGeneration

>>> model = LlavaNextVideoForConditionalGeneration.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/LLaVA-NeXT-Video-7B-hf")

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
