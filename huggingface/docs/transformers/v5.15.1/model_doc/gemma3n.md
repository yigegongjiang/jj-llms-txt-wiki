# Gemma3n

## Overview

[Gemma3n](https://developers.googleblog.com/en/introducing-gemma-3n/) is a multimodal model with pretrained and instruction-tuned variants, available in E4B and E2B sizes. While
large portions of the language model architecture are shared with prior Gemma releases, there are many new additions in
this model, including [Alternating Updates][altup] (AltUp), [Learned Augmented Residual Layer][laurel] (LAuReL),
[MatFormer][matformer], Per-Layer Embeddings (PLE), [Activation Sparsity with Statistical Top-k][spark-transformer], and KV cache sharing. The language model uses
a similar attention pattern to [Gemma 3](./gemma3) with alternating 4 local sliding window self-attention layers for
every global self-attention layer with a maximum context length of 32k tokens. Gemma 3n introduces
MobileNet v5 as the vision encoder, using a default resolution of 768x768 pixels, and adds a newly
trained audio encoder based on the [Universal Speech Model][usm] (USM) architecture.

The instruction-tuned variant was post-trained with knowledge distillation and reinforcement learning.

You can find all the original Gemma 3n checkpoints under the [Gemma 3n][gemma3n-collection] release.

> [!TIP]
> Click on the Gemma 3n models in the right sidebar for more examples of how to apply Gemma to different vision, audio,
> and language tasks.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-text-to-text",
    model="google/gemma-3n-e4b",
    device=0,
)
pipeline(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
    text="<start_of_image> What is shown in this image?"
)
```

```python
from transformers import AutoProcessor, Gemma3nForConditionalGeneration

model = Gemma3nForConditionalGeneration.from_pretrained(
    "google/gemma-3n-e4b-it",
    device_map="auto",
    attn_implementation="sdpa"
)
processor = AutoProcessor.from_pretrained(
    "google/gemma-3n-e4b-it",
    padding_side="left"
)

messages = [
    {
        "role": "system",
        "content": [
            {"type": "text", "text": "You are a helpful assistant."}
        ]
    },
    {
        "role": "user", "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "What is shown in this image?"},
        ]
    },
]
inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    add_generation_prompt=True,
).to(model.device)

output = model.generate(**inputs, max_new_tokens=50, cache_implementation="static")
print(processor.decode(output[0], skip_special_tokens=True))
```

## Notes

- Use [Gemma3nForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nForConditionalGeneration) for image-audio-and-text, image-and-text, image-and-audio, audio-and-text,
    image-only and audio-only inputs.
- Gemma 3n supports multiple images per input, but make sure the images are correctly batched before passing them to
    the processor. Each batch should be a list of one or more images.

    ```py
    url_cow = "https://media.istockphoto.com/id/1192867753/photo/cow-in-berchida-beach-siniscola.jpg?s=612x612&w=0&k=20&c=v0hjjniwsMNfJSuKWZuIn8pssmD5h5bSN1peBd1CmH4="
    url_cat = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"

    messages =[
        {
            "role": "system",
            "content": [
                {"type": "text", "text": "You are a helpful assistant."}
            ]
        },
        {
            "role": "user",
            "content": [
                {"type": "image", "url": url_cow},
                {"type": "image", "url": url_cat},
                {"type": "text", "text": "Which image is cuter?"},
            ]
        },
    ]
    ```

- Text passed to the processor should have a `<image_soft_token>` token wherever an image should be inserted.
- Gemma 3n accepts at most one target audio clip per input, though multiple audio clips can be provided in few-shot
    prompts, for example.
- Text passed to the processor should have a `<audio_soft_token>` token wherever an audio clip should be inserted.
- The processor has its own [apply_chat_template()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) method to convert chat messages to model inputs.

## Gemma3nAudioFeatureExtractor[[transformers.Gemma3nAudioFeatureExtractor]]

#### transformers.Gemma3nAudioFeatureExtractor[[transformers.Gemma3nAudioFeatureExtractor]]

```python
transformers.Gemma3nAudioFeatureExtractor(feature_size: int = 128, sampling_rate: int = 16000, padding_value: float = 0.0, return_attention_mask: bool = True, frame_length_ms: float = 32.0, hop_length_ms: float = 10.0, min_frequency: float = 125.0, max_frequency: float = 7600.0, preemphasis: float = 0.97, preemphasis_htk_flavor: bool = True, fft_overdrive: bool = True, dither: float = 0.0, input_scale_factor: float = 1.0, mel_floor: float = 1e-05, per_bin_mean: collections.abc.Sequence[float] | None = None, per_bin_stddev: collections.abc.Sequence[float] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/feature_extraction_gemma3n.py#L108)

**Parameters:**

feature_size (`int`, *optional*, defaults to 128) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : Padding value used to pad the audio. Should correspond to silences.

return_attention_mask (`bool`, *optional*, defaults to `True`) : Whether to return the attention mask for the generated MEL spectrograms.

frame_length_ms (`float`, *optional*, defaults to 32.0) : The length of a frame in milliseconds.

hop_length_ms (`float`, *optional*, defaults to 10.0) : Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.

min_frequency (`float`, *optional*, defaults to 125.0) : The minimum frequency (in Hz) for the Mel filterbank.

max_frequency (`float`, *optional*, defaults to 7600.0) : The maximum frequency (in Hz) for the Mel filterbank.

preemphasis (`float`, *optional*, defaults to 0.97) : The preemphasis coefficient.

preemphasis_htk_flavor (`bool`, *optional*, defaults to `True`) : Whether to use HTK-style preemphasis.

fft_overdrive (`bool`, *optional*, defaults to `True`) : Whether to use FFT overdrive.

dither (`float`, *optional*, defaults to 0.0) : Adds dithering. In other words, adds a small Gaussian noise to each frame. E.g. use 0.0001 to add dithering with a normal distribution centered around 0.0 with standard deviation 0.0001 (assuming [-1,+1] range of raw_speech). The value 0.0 means no dithering. Dithering has similar effect as `spectrogram(mel_floor=...)`. It reduces the high log_mel_fbank values for signals with hard-zero sections, when VAD cutoff is present in the signal.

input_scale_factor (`float`, *optional*, defaults to 1.0) : Scaling factor applied to the input waveform.

mel_floor (`float`, *optional*, defaults to 1e-05) : Minimum value for Mel spectrograms to avoid log(0).

per_bin_mean (`Optional[Sequence[float]]`, *optional*) : Mean values for per-bin normalization.

per_bin_stddev (`Optional[Sequence[float]]`, *optional*) : Standard deviation values for per-bin normalization.

An audio feature extractor Universal Speech Models https://huggingface.co/papers/2303.01037.

## Gemma3nProcessor[[transformers.Gemma3nProcessor]]

#### transformers.Gemma3nProcessor[[transformers.Gemma3nProcessor]]

```python
transformers.Gemma3nProcessor(feature_extractor, image_processor, tokenizer, chat_template = None, audio_seq_length: int = 188, image_seq_length: int = 256, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/processing_gemma3n.py#L32)

**Parameters:**

feature_extractor (`Gemma3nAudioFeatureExtractor`) : The feature extractor is a required input.

image_processor (`SiglipImageProcessor`) : The image processor is a required input.

tokenizer (`GemmaTokenizer`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

audio_seq_length (`int`, *optional*, defaults to 188) : The number of audio soft tokens that will be added to the text prompt

image_seq_length (`int`, *optional*, defaults to 256) : The number of image soft tokens that should be added to

Constructs a Gemma3nProcessor which wraps a feature extractor, a image processor, and a tokenizer into a single processor.

[Gemma3nProcessor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nProcessor) offers all the functionalities of [Gemma3nAudioFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nAudioFeatureExtractor), [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor), and [GemmaTokenizer](/docs/transformers/v5.15.1/en/model_doc/gemma#transformers.GemmaTokenizer). See the
[~Gemma3nAudioFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nAudioFeatureExtractor), [~SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor), and [~GemmaTokenizer](/docs/transformers/v5.15.1/en/model_doc/gemma#transformers.GemmaTokenizer) for more information.

#### __call__[[transformers.Gemma3nProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] = None, audio: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/processing_gemma3n.py#L71)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, list[float], list[numpy.ndarray], list[list[float]]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~feature_extraction_utils.BatchFeature`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

## Gemma3nTextConfig[[transformers.Gemma3nTextConfig]]

#### transformers.Gemma3nTextConfig[[transformers.Gemma3nTextConfig]]

```python
transformers.Gemma3nTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 262400, hidden_size: int = 2048, intermediate_size: int | list[int] = 16384, num_hidden_layers: int = 35, num_attention_heads: int = 8, num_key_value_heads: int = 2, head_dim: int = 256, hidden_activation: str = 'gelu_pytorch_tanh', max_position_embeddings: int = 32768, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = 0, eos_token_id: int | list[int] | None = 1, bos_token_id: int | None = 2, tie_word_embeddings: bool = True, rope_parameters: dict | None = None, attention_bias: bool = False, attention_dropout: int | float | None = 0.0, sliding_window: int = 512, layer_types: list[str] | None = None, final_logit_softcapping: float = 30.0, vocab_size_per_layer_input: int = 262144, hidden_size_per_layer_input: int = 256, altup_active_idx: int = 0, altup_coef_clip: float = 120.0, altup_correct_scale: bool = True, altup_num_inputs: int = 4, num_kv_shared_layers: int = 15, laurel_rank: int = 64, activation_sparsity_pattern: float | list[float] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/configuration_gemma3n.py#L38)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `262400`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `2048`) : Dimension of the hidden representations.

intermediate_size (`Union[int, list[int]]`, *optional*, defaults to `16384`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `35`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `2`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `256`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_activation (`str`, *optional*, defaults to `gelu_pytorch_tanh`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `32768`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `1`) : Token id used for end-of-stream in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `2`) : Token id used for beginning-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[int, float]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

sliding_window (`int`, *optional*, defaults to `512`) : Sliding window attention window size. If `None`, no sliding window is applied.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

final_logit_softcapping (`float`, *optional*, defaults to `30.0`) : Soft-capping value applied to the final logits before computing the probability distribution. Logits are scaled by `tanh(logit / cap) * cap`.

vocab_size_per_layer_input (`int`, *optional*, defaults to 262144) : Vocabulary size of the per-layer text embeddings that augment the standard embeddings.

hidden_size_per_layer_input (`int`, *optional*, defaults to 256) : Dimension of the hidden representations for per-layer embeddings.

altup_active_idx (`int`, *optional*, defaults to 0) : The index of the prediction from which AltUp will compute additional predictions or correct the active prediction.

altup_coef_clip (`float`, *optional*, defaults to 120.0) : The maximum amplitude of an AltUp prediction or correction coefficient weight.

altup_correct_scale (`bool`, *optional*, defaults to `True`) : If True, apply the `AltUp.correct_output_scale` to the corrected prediction at `altup_active_idx`.

altup_num_inputs (`int`, *optional*, defaults to 4) : The number of predictions that AltUp should make given the input sequence.

num_kv_shared_layers (`int`, *optional*, defaults to 15) : The number of layers that share KV cache values. During the forward pass, the last `num_kv_shared_layers` layers in the model "share" the KV values in that each local and global layer in this range uses the KV cache values computed for the last local or global layer, respectively, before entering this range. The value should be a multiple of the attention pattern size (see `layer_types` parameter).

laurel_rank (`int`, *optional*, defaults to 64) : The intermediate size for the linear projections in the Learned Augmented Residual Layer.

activation_sparsity_pattern (`Sequence[float]`, *optional*) : The sparsity factor used to extract the top-k activations for a given layer. The provided Sequence must explicitly provide a sparsity value for each layer in the model. By default, the first 10 layers are sparse with a sparsity factor of 0.95 and the rest are dense.

This is the configuration class to store the configuration of a Gemma3nModel. It is used to instantiate a Gemma3N
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-3n-E4B](https://huggingface.co/google/gemma-3n-E4B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Gemma3nTextModel, Gemma3nTextConfig

>>> # Initializing a Gemma3nText gemma3n_text-E4B style configuration
>>> configuration = Gemma3nTextConfig()

>>> # Initializing a model from the gemma3n_text-E4B style configuration
>>> model = Gemma3nTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Gemma3nVisionConfig[[transformers.Gemma3nVisionConfig]]

#### transformers.Gemma3nVisionConfig[[transformers.Gemma3nVisionConfig]]

```python
transformers.Gemma3nVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, architecture: str = 'mobilenetv5_300m_enc', initializer_range: float = 0.02, do_pooling: bool = False, model_args: dict | None = None, hidden_size: int = 2048, vocab_size: int = 128, vocab_offset: int = 262144, rms_norm_eps: float = 1e-06)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/configuration_gemma3n.py#L302)

**Parameters:**

architecture (`str`, *optional*, defaults to `"resnet50"`) : The timm architecture to load.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

do_pooling (`bool`, *optional*, defaults to `True`) : Whether to do pooling for the last_hidden_state in `TimmWrapperModel` or not.

model_args (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the `timm.create_model` function. e.g. `model_args={"depth": 3}` for `timm/vit_base_patch32_clip_448.laion2b_ft_in12k_in1k` to create a model with 3 blocks. Defaults to `None`.

hidden_size (`int`, *optional*, defaults to `2048`) : Dimension of the hidden representations.

vocab_size (`int`, *optional*, defaults to `128`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

vocab_offset (`int`, *optional*, defaults to 262144) : Offset between the tokenizer vocab index for the token ids embedded by `Gemma3nMultimodalEmbedder` and the 0-indexed `Gemma3nMultimodalEmbedder.embedding` table.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

This is the configuration class to store the configuration of a Gemma3nModel. It is used to instantiate a Gemma3N
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-3n-E4B](https://huggingface.co/google/gemma-3n-E4B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Gemma3nVisionConfig, TimmWrapper

>>> # Initializing a TimmWrapper gemma3n_vision-E4B-style configuration
>>> configuration = Gemma3nVisionConfig()

>>> # Initializing a gemma3n_vision-E4B-style TimmWrapper from the configuration
>>> model = TimmWrapper(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Gemma3nAudioConfig[[transformers.Gemma3nAudioConfig]]

#### transformers.Gemma3nAudioConfig[[transformers.Gemma3nAudioConfig]]

```python
transformers.Gemma3nAudioConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 128, vocab_offset: int = 262272, input_feat_size: int = 128, hidden_size: int = 1536, rms_norm_eps: float = 1e-06, gradient_clipping: float = 10000000000.0, conf_attention_chunk_size: int = 12, conf_attention_context_left: int = 13, conf_attention_context_right: int = 0, conf_attention_logit_cap: float = 50.0, conf_num_attention_heads: int = 8, conf_num_hidden_layers: int = 12, conf_conv_kernel_size: int = 5, conf_reduction_factor: int = 4, conf_residual_weight: float = 0.5, sscp_conv_channel_size: list[int] | tuple[int, int] = (128, 32), sscp_conv_group_norm_eps: float = 0.001, sscp_conv_kernel_size: list | tuple[tuple[int, int], tuple[int, int]] = ((3, 3), (3, 3)), sscp_conv_stride_size: list | tuple[tuple[int, int], tuple[int, int]] = ((2, 2), (2, 2)))
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/configuration_gemma3n.py#L202)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `128`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

vocab_offset (`int`, *optional*, defaults to 262272) : Offset between the tokenizer vocab index for the token ids embedded by `Gemma3nMultimodalEmbedder` and the 0-indexed `Gemma3nMultimodalEmbedder.embedding` table.

input_feat_size (`int`, *optional*, defaults to 128) : The number of channels in each mel-spectrogram frame.

hidden_size (`int`, *optional*, defaults to `1536`) : Dimension of the hidden representations.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

gradient_clipping (`float`, *optional*, defaults to 10000000000.0) : Clipping value used to stabilize extremely large gradient values.

conf_attention_chunk_size (`int`, *optional*, defaults to 12) : The sub-sequence size for local attention processing inside the Conformer ("conf") section of the Universal Speech Model.

conf_attention_context_left (`int`, *optional*, defaults to 13) : The left context size of the local attention inside the Conformer ("conf") section of the Universal Speech Model.

conf_attention_context_right (`int`, *optional*, defaults to 0) : The right context size of the local attention inside the Conformer ("conf") section of the Universal Speech Model.

conf_attention_logit_cap (`float`, *optional*, defaults to 50.0) : Logit cap applied during local attention inside the Conformer ("conf") section of the Universal Speech Model.

conf_num_attention_heads (`int`, *optional*, defaults to 8) : The number of attention heads in local attention inside the Conformer ("conf") section of the Universal Speech Model.

conf_num_hidden_layers (`int`, *optional*, defaults to 12) : The number of layers that use local attention inside the Conformer ("conf") section of the Universal Speech Model.

conf_conv_kernel_size (`int`, *optional*, defaults to 5) : Convolution kernel size for the conformer block inside the Conformer ("conf") section of the Universal Speech Model.

conf_reduction_factor (`int`, *optional*, defaults to 4) : Reduction factor used in the conformer block inside the Conformer ("conf") section of the Universal Speech Model.

conf_residual_weight (`float`, *optional*, defaults to 0.5) : Residual connection weight inside the Conformer ("conf") section of the Universal Speech Model.

sscp_conv_channel_size (`tuple(int, int)`, *optional*, defaults to `(128, 32)`) : The channel sizes for the first and second convolutional layers in the Sub-sample Convolution Projection ("sscp") section of the Universal Speech Model.

sscp_conv_group_norm_eps (`float`, *optional*, defaults to 0.001) : Epsilon used in group normalization in the subsample convolution projection in the Sub-sample Convolution Projection ("sscp") section of the Universal Speech Model.

sscp_conv_kernel_size (`tuple(tuple(int, int), tuple(int, int))`, *optional*, defaults to `((3, 3), (3, 3))`) : Kernel sizes of the two convolutional layers in the subsample convolution projection  in the Sub-sample Convolution Projection ("sscp") section of the Universal Speech Model. The kernel sizes are specified as a tuple of height and width for each layer, where the height corresponds to the time dimension and the width corresponds to the frequency dimension.

sscp_conv_stride_size (`tuple(tuple(int, int), tuple(int, int))`, *optional*, defaults to `((2, 2), (2, 2))`) : Stride sizes of the two convolutional layers in the subsample convolution projection in the Sub-sample Convolution Projection ("sscp") section of the Universal Speech Model. The stride sizes are specified as a tuple of height and width for each layer, where the height corresponds to the time dimension and the width corresponds to the frequency dimension.

This is the configuration class to store the configuration of a Gemma3nModel. It is used to instantiate a Gemma3N
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-3n-E4B](https://huggingface.co/google/gemma-3n-E4B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Gemma3nAudioConfig, Gemma3nAudioEncoder

>>> # Initializing a Gemma3nAudioEncoder gemma3n_audio-E4B-style configuration
>>> configuration = Gemma3nAudioConfig()

>>> # Initializing a model from the gemma3n_audio-E4B style configuration
>>> model = Gemma3nAudioEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Gemma3nConfig[[transformers.Gemma3nConfig]]

#### transformers.Gemma3nConfig[[transformers.Gemma3nConfig]]

```python
transformers.Gemma3nConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: transformers.models.gemma3n.configuration_gemma3n.Gemma3nTextConfig | dict[str, typing.Any] | None = None, vision_config: transformers.models.gemma3n.configuration_gemma3n.Gemma3nVisionConfig | dict[str, typing.Any] | None = None, audio_config: transformers.models.gemma3n.configuration_gemma3n.Gemma3nAudioConfig | dict[str, typing.Any] | None = None, audio_soft_tokens_per_image: int | None = 188, vision_soft_tokens_per_image: int | None = 256, boi_token_id: int | None = 255999, eoi_token_id: int | None = 262144, image_token_id: int | None = 262145, boa_token_id: int | None = 256000, eoa_token_id: int | None = 262272, audio_token_id: int | None = 262273, initializer_range: float | None = 0.02, tie_word_embeddings: bool | None = True, use_cache: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/configuration_gemma3n.py#L395)

**Parameters:**

text_config (`Union[~models.gemma3n.configuration_gemma3n.Gemma3nTextConfig, dict[str, Any]]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[~models.gemma3n.configuration_gemma3n.Gemma3nVisionConfig, dict[str, Any]]`, *optional*) : The config object or dictionary of the vision backbone.

audio_config (`Union[~models.gemma3n.configuration_gemma3n.Gemma3nAudioConfig, dict[str, Any]]`, *optional*) : The config object or dictionary of the audio backbone.

audio_soft_tokens_per_image (`int`, *optional*, defaults to 188) : The number of soft tokens per audio clip.

vision_soft_tokens_per_image (`int`, *optional*, defaults to 256) : The number of soft tokens per image.

boi_token_id (`int`, *optional*, defaults to 255999) : The begin-of-image token index to wrap the image prompt.

eoi_token_id (`int`, *optional*, defaults to 262144) : The end-of-image token index to wrap the image prompt.

image_token_id (`int`, *optional*, defaults to `262145`) : The image token index used as a placeholder for input images.

boa_token_id (`int`, *optional*, defaults to 256000) : The begin-of-audio token index to wrap the audio prompt.

eoa_token_id (`int`, *optional*, defaults to 262272) : The end-of-audio token index to wrap the audio prompt.

audio_token_id (`int`, *optional*, defaults to `262273`) : The audio token index used as a placeholder for input audio.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

This is the configuration class to store the configuration of a Gemma3nModel. It is used to instantiate a Gemma3N
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-3n-E4B](https://huggingface.co/google/gemma-3n-E4B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Gemma3nForConditionalGeneration, Gemma3nConfig, Gemma3nTextConfig

>>> # Initializing a MobileNet vision config, which is loaded from TIMM
>>> vision_config = Gemma3nVisionConfig()

>>> # Initializing a Gemma3n Audio config
>>> audio_config = Gemma3nAudioConfig()

>>> # Initializing a Gemma3n Text config
>>> text_config = Gemma3nTextConfig()

>>> # Initializing a Gemma3n gemma-3-4b style configuration
>>> configuration = Gemma3nConfig(text_config, vision_config, audio_config)

>>> # Initializing a model from the gemma-3-4b style configuration
>>> model = Gemma3nTextConfig(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Gemma3nTextModel[[transformers.Gemma3nTextModel]]

#### transformers.Gemma3nTextModel[[transformers.Gemma3nTextModel]]

```python
transformers.Gemma3nTextModel(config: Gemma3nTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1602)

**Parameters:**

config ([Gemma3nTextConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 3n language model without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Gemma3nTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, per_layer_inputs: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1663)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

per_layer_inputs (`torch.Tensor`, *optional*, defaults to None) : Pre-computed per-layer embeddings. If None, they are derived from input_ids if provided.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) and inputs.

The [Gemma3nTextModel](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
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

## Gemma3nModel[[transformers.Gemma3nModel]]

#### transformers.Gemma3nModel[[transformers.Gemma3nModel]]

```python
transformers.Gemma3nModel(config: Gemma3nConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1949)

**Parameters:**

config ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 3n model comprising a vision backbone, an audio backbone, and a language model without a
language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Gemma3nModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, token_type_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, **lm_kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L2032)

input_features_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*):
Attention mask for `input_features` where non-zero values mark valid audio frames.
labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*):
Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
config.text_config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
(masked), the loss is only computed for the tokens with labels in `[0, ..., config.text_config.vocab_size]`.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Gemma3nForConditionalGeneration

>>> model = Gemma3nForConditionalGeneration.from_pretrained("google/gemma3n2-3b-mix-224")
>>> processor = AutoProcessor.from_pretrained("google/gemma3n2-3b-mix-224")

>>> prompt = "Where is the cat standing?"
>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt,  return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs,)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Where is the cat standing?\nsnow"
```

#### get_image_features[[transformers.Gemma3nModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1966)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([Gemma3nProcessor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nProcessor) uses [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) and inputs.

Projects the last hidden state from the vision model into language model space.

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

#### get_audio_features[[transformers.Gemma3nModel.get_audio_features]]

```python
get_audio_features(input_features: Tensor, input_features_mask: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L2175)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(num_images, seq_length, num_features)`) : The tensors corresponding to the input audio.

input_features_mask (`torch.FloatTensor` of shape `(num_images, seq_length)`) : The attention mask for the input audio.

**Returns:** `Gemma3nAudioEncoderModelOutput` or `tuple(torch.FloatTensor)`

A `Gemma3nAudioEncoderModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) and inputs.

Projects the last hidden state from the audio encoder into language model space.

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
- **audio_mel_mask** (`torch.BoolTensor`, *optional*) -- A torch.BoolTensor of shape `(batch_size, num_frames)`

## Gemma3nForCausalLM[[transformers.Gemma3nForCausalLM]]

#### transformers.Gemma3nForCausalLM[[transformers.Gemma3nForCausalLM]]

```python
transformers.Gemma3nForCausalLM(config: Gemma3nTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1813)

**Parameters:**

config ([Gemma3nTextConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 3n language model with a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Gemma3nForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L1829)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) and inputs.

The [Gemma3nForCausalLM](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
>>> from transformers import AutoTokenizer, Gemma3nForCausalLM

>>> model = Gemma3nForCausalLM.from_pretrained("google/gemma-2-9b")
>>> tokenizer = AutoTokenizer.from_pretrained("google/gemma-2-9b")

>>> prompt = "What is your favorite condiment?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"What is your favorite condiment?"
```

## Gemma3nForConditionalGeneration[[transformers.Gemma3nForConditionalGeneration]]

#### transformers.Gemma3nForConditionalGeneration[[transformers.Gemma3nForConditionalGeneration]]

```python
transformers.Gemma3nForConditionalGeneration(config: Gemma3nConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L2204)

**Parameters:**

config ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 3n model comprising a vision backbone, an audio backbone, a language model, and a language modeling
head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Gemma3nForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, input_features_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, token_type_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **lm_kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L2218)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([Gemma3nProcessor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nProcessor) uses [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [Gemma3nAudioFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nAudioFeatureExtractor). See `Gemma3nAudioFeatureExtractor.__call__()` for details ([Gemma3nProcessor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nProcessor) uses [Gemma3nAudioFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nAudioFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

input_features_mask (`torch.Tensor`, *optional*, defaults to None) : The attention mask for the input audio.

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.text_config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.text_config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `Gemma3nCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Gemma3nCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma3nConfig](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nConfig)) and inputs.

The [Gemma3nForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.text_config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder after projecting last hidden state.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  audio_hidden_states of the model produced by the audio encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Gemma3ForConditionalGeneration

>>> model = Gemma3ForConditionalGeneration.from_pretrained("google/gemma-3-4b-it")
>>> processor = AutoProcessor.from_pretrained("google/gemma-3-4b-it")

>>> messages = [
...     {
...         "role": "system",
...         "content": [
...             {"type": "text", "text": "You are a helpful assistant."}
...         ]
...     },
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenizer=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"user\nYou are a helpful assistant.\n\n\n\n\n\nWhere is the cat standing?\nmodel\nBased on the image, the cat is standing in a snowy area, likely outdoors. It appears to"
```

#### get_image_features[[transformers.Gemma3nForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/gemma3n/modeling_gemma3n.py#L2214)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([Gemma3nProcessor](/docs/transformers/v5.15.1/en/model_doc/gemma3n#transformers.Gemma3nProcessor) uses [SiglipImageProcessor](/docs/transformers/v5.15.1/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Gemma3nForConditionalGeneration

>>> model = Gemma3nForConditionalGeneration.from_pretrained("google/gemma-3n-E4B")
>>> processor = AutoProcessor.from_pretrained("google/gemma-3n-E4B")

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

[altup]: https://proceedings.neurips.cc/paper_files/paper/2023/hash/f2059277ac6ce66e7e5543001afa8bb5-Abstract-Conference.html
[attention-mask-viz]: https://github.com/huggingface/transformers/blob/beb9b5b02246b9b7ee81ddf938f93f44cfeaad19/src/transformers/utils/attention_visualizer.py#L139
[gemma3n-collection]: https://huggingface.co/collections/google/gemma-3n
[laurel]: https://huggingface.co/papers/2411.07501
[matformer]: https://huggingface.co/papers/2310.07707
[spark-transformer]: https://huggingface.co/papers/2506.06644
[usm]: https://huggingface.co/papers/2303.01037
