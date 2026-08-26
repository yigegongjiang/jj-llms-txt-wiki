# Dia

    
        
        
    

## Overview

[Dia](https://github.com/nari-labs/dia) is an open-source text-to-speech (TTS) model (1.6B parameters) developed by [Nari Labs](https://huggingface.co/nari-labs).
It can generate highly realistic dialogue from transcript including non-verbal communications such as laughter and coughing.
Furthermore, emotion and tone control is also possible via audio conditioning (voice cloning).

**Model Architecture:**
Dia is an encoder-decoder transformer based on the original transformer architecture. However, some more modern features such as
rotational positional embeddings (RoPE) are also included. For its text portion (encoder), a byte tokenizer is utilized while
for the audio portion (decoder), a pretrained codec model [DAC](./dac) is used - DAC encodes speech into discrete codebook
tokens and decodes them back into audio.

## Usage Tips

### Generation with Text

```python
from transformers import AutoProcessor, DiaForConditionalGeneration

model_checkpoint = "nari-labs/Dia-1.6B-0626"

text = ["[S1] Dia is an open weights text to dialogue model."]
model = DiaForConditionalGeneration.from_pretrained(model_checkpoint, device_map="auto")
processor = AutoProcessor.from_pretrained(model_checkpoint)

inputs = processor(text=text, padding=True, return_tensors="pt").to(model.device)
outputs = model.generate(**inputs, max_new_tokens=256)  # corresponds to around ~2s

# save audio to a file
outputs = processor.batch_decode(outputs)
processor.save_audio(outputs, "example.wav")
```

### Generation with Text and Audio (Voice Cloning)

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, DiaForConditionalGeneration

model_checkpoint = "nari-labs/Dia-1.6B-0626"

ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
ds = ds.cast_column("audio", Audio(sampling_rate=44100))
audio = ds[-1]["audio"]["array"]
# text is a transcript of the audio + additional text you want as new audio
text = ["[S1] I know. It's going to save me a lot of money, I hope. [S2] I sure hope so for you."]

processor = AutoProcessor.from_pretrained(model_checkpoint)
model = DiaForConditionalGeneration.from_pretrained(model_checkpoint, device_map="auto")

inputs = processor(text=text, audio=audio, padding=True, return_tensors="pt").to(model.device)
prompt_len = processor.get_audio_prompt_len(inputs["decoder_attention_mask"])
outputs = model.generate(**inputs, max_new_tokens=256)  # corresponds to around ~2s

# retrieve actually generated audio and save to a file
outputs = processor.batch_decode(outputs, audio_prompt_len=prompt_len)
processor.save_audio(outputs, "example_with_audio.wav")
```

### Training

```python
from datasets import Audio, load_dataset

from transformers import AutoProcessor, DiaForConditionalGeneration

model_checkpoint = "nari-labs/Dia-1.6B-0626"

ds = load_dataset("hf-internal-testing/dailytalk-dummy", split="train")
ds = ds.cast_column("audio", Audio(sampling_rate=44100))
audio = ds[-1]["audio"]["array"]
# text is a transcript of the audio
text = ["[S1] I know. It's going to save me a lot of money, I hope."]

model = DiaForConditionalGeneration.from_pretrained(model_checkpoint, device_map="auto")
processor = AutoProcessor.from_pretrained(model_checkpoint)

inputs = processor(
    text=text,
    audio=audio,
    generation=False,
    output_labels=True,
    padding=True,
    return_tensors="pt"
).to(model.device)

out = model(**inputs)
out.loss.backward()
```

This model was contributed by [Jaeyong Sung](https://huggingface.co/buttercrab), [Arthur Zucker](https://huggingface.co/ArthurZ),
and [Anton Vlasjuk](https://huggingface.co/AntonV). The original code can be found [here](https://github.com/nari-labs/dia/).

## DiaConfig[[transformers.DiaConfig]]

#### transformers.DiaConfig[[transformers.DiaConfig]]

```python
transformers.DiaConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, encoder_config: transformers.models.dia.configuration_dia.DiaEncoderConfig | dict | None = None, decoder_config: transformers.models.dia.configuration_dia.DiaDecoderConfig | dict | None = None, norm_eps: float = 1e-05, pad_token_id: int | None = None, eos_token_id: int | list[int] | None = None, bos_token_id: int | None = None, delay_pattern: list[int] | None = None, initializer_range: float = 0.02, use_cache: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/configuration_dia.py#L87)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

encoder_config (`Union[~models.dia.configuration_dia.DiaEncoderConfig, dict]`, *optional*) : The config object or dictionary of the encoder backbone.

decoder_config (`Union[~models.dia.configuration_dia.DiaDecoderConfig, dict]`, *optional*) : The config object or dictionary of the decoder backbone.

norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

delay_pattern (`list[int]`, *optional*, defaults to `[0, 8, 9, 10, 11, 12, 13, 14, 15]`) : The delay pattern for the decoder. The length of this list must match `decoder_config.num_channels`.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

This is the configuration class to store the configuration of a DiaModel. It is used to instantiate a Dia
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nari-labs/Dia-1.6B](https://huggingface.co/nari-labs/Dia-1.6B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DiaConfig, DiaModel

>>> # Initializing a DiaConfig with default values
>>> configuration = DiaConfig()

>>> # Initializing a DiaModel (with random weights) from the configuration
>>> model = DiaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

#### get_text_config[[transformers.DiaConfig.get_text_config]]

```python
get_text_config(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/configuration_dia.py#L164)

Defaulting to audio config as it's the decoder in this case which is usually the text backbone

## DiaDecoderConfig[[transformers.DiaDecoderConfig]]

#### transformers.DiaDecoderConfig[[transformers.DiaDecoderConfig]]

```python
transformers.DiaDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, max_position_embeddings: int = 3072, num_hidden_layers: int = 18, hidden_size: int = 2048, intermediate_size: int = 8192, num_attention_heads: int = 16, num_key_value_heads: int = 4, head_dim: int = 128, cross_num_attention_heads: int = 16, cross_head_dim: int = 128, cross_num_key_value_heads: int = 16, cross_hidden_size: int = 1024, norm_eps: float = 1e-05, vocab_size: int = 1028, hidden_act: str = 'silu', num_channels: int = 9, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, initializer_range: float = 0.02, use_cache: bool = True, pad_token_id: int | None = 1025, eos_token_id: int | list[int] | None = 1024, bos_token_id: int | None = 1026)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/configuration_dia.py#L47)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

max_position_embeddings (`int`, *optional*, defaults to `3072`) : The maximum sequence length that this model might ever be used with.

num_hidden_layers (`int`, *optional*, defaults to `18`) : Number of hidden layers in the Transformer decoder.

hidden_size (`int`, *optional*, defaults to `2048`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `8192`) : Dimension of the MLP representations.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `4`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

cross_num_attention_heads (`int`, *optional*, defaults to 16) : Number of attention heads for each cross-attention layer in the Transformer decoder.

cross_head_dim (`int`, *optional*, defaults to 128) : Dimensionality of the cross-attention head.

cross_num_key_value_heads (`int`, *optional*, defaults to 16) : Number of key and value heads for each cross-attention layer in the Transformer decoder.

cross_hidden_size (`int`, *optional*, defaults to 1024) : Dimensionality of the cross-attention layers.

norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

vocab_size (`int`, *optional*, defaults to `1028`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

num_channels (`int`, *optional*, defaults to `9`) : The number of input channels.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*, defaults to `1025`) : Token id used for padding in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `1024`) : Token id used for end-of-stream in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1026`) : Token id used for beginning-of-stream in the vocabulary.

This is the configuration class to store the configuration of a DiaModel. It is used to instantiate a Dia
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nari-labs/Dia-1.6B](https://huggingface.co/nari-labs/Dia-1.6B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DiaEncoderConfig[[transformers.DiaEncoderConfig]]

#### transformers.DiaEncoderConfig[[transformers.DiaEncoderConfig]]

```python
transformers.DiaEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, max_position_embeddings: int = 1024, num_hidden_layers: int = 12, hidden_size: int = 1024, num_attention_heads: int = 16, num_key_value_heads: int = 16, head_dim: int = 128, intermediate_size: int = 4096, norm_eps: float = 1e-05, vocab_size: int = 256, hidden_act: str = 'silu', rope_parameters: dict | None = None, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/configuration_dia.py#L28)

**Parameters:**

max_position_embeddings (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `16`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

intermediate_size (`int`, *optional*, defaults to `4096`) : Dimension of the MLP representations.

norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

vocab_size (`int`, *optional*, defaults to `256`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a DiaModel. It is used to instantiate a Dia
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nari-labs/Dia-1.6B](https://huggingface.co/nari-labs/Dia-1.6B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DiaTokenizer[[transformers.DiaTokenizer]]

#### transformers.DiaTokenizer[[transformers.DiaTokenizer]]

```python
transformers.DiaTokenizer(pad_token: str | None = '<pad>', unk_token: str | None = '<pad>', max_length: int | None = 1024, offset: int = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/tokenization_dia.py#L23)

**Parameters:**

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

unk_token (`str`, *optional*, defaults to `"<pad>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

max_length (`int`, *optional*, defaults to 1024) : The maximum length of the sequences when encoding. Sequences longer than this will be truncated.

offset (`int`, *optional*, defaults to 0) : The offset of the tokenizer.

Construct a Dia tokenizer. Dia simply uses raw bytes utf-8 encoding except for special tokens `[S1]` and `[S2]`.

This tokenizer inherits from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### __call__[[transformers.DiaTokenizer.__call__]]

```python
__call__(text: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, add_special_tokens: bool = True, padding: bool | str | PaddingStrategy = False, truncation: bool | str | TruncationStrategy | None = None, max_length: int | None = None, stride: int = 0, is_split_into_words: bool = False, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, tokenizer_kwargs: dict[str, Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L2417)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

tokenizer_kwargs (`dict[str, Any]`, *optional*) : Additional kwargs to pass to the tokenizer. These will be merged with the explicit parameters and other kwargs, with explicit parameters taking precedence. 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to add special tokens when encoding the sequences. This will use the underlying `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens automatically.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence is provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

is_split_into_words (`bool`, *optional*, defaults to `False`) : Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace) which it will tokenize. This is useful for NER or token classification.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

padding_side (`str`, *optional*) : The side on which the model should have padding applied. Should be selected between ['right', 'left']. Default value is picked from the class attribute of the same name.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects. 

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length  (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

- ****kwargs** : passed to the `self.tokenize()` method

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`)

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences.

## DiaFeatureExtractor[[transformers.DiaFeatureExtractor]]

#### transformers.DiaFeatureExtractor[[transformers.DiaFeatureExtractor]]

```python
transformers.DiaFeatureExtractor(feature_size: int = 1, sampling_rate: int = 16000, padding_value: float = 0.0, hop_length: int = 512, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/feature_extraction_dia.py#L26)

**Parameters:**

feature_size (`int`, *optional*, defaults to 1) : The feature dimension of the extracted features. Use 1 for mono, 2 for stereo.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio waveform should be digitalized, expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used for padding.

hop_length (`int`, *optional*, defaults to 512) : Overlap length between successive windows.

Constructs an Dia feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.DiaFeatureExtractor.__call__]]

```python
__call__(raw_audio: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], padding: bool | str | transformers.utils.generic.PaddingStrategy | None = None, truncation: bool | None = False, max_length: int | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/feature_extraction_dia.py#L57)

**Parameters:**

raw_audio (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) : The sequence or batch of sequences to be processed. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. The numpy array must be of shape `(num_samples,)` for mono audio (`feature_size = 1`), or `(2, num_samples)` for stereo audio (`feature_size = 2`).

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, *optional*, defaults to `False`) : Activates truncation to cut input sequences longer than `max_length` to `max_length`.

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*, default to 'pt') : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sampling rate at which the `audio` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors.

Main method to featurize and prepare for the model one or several sequence(s).

## DiaProcessor[[transformers.DiaProcessor]]

#### transformers.DiaProcessor[[transformers.DiaProcessor]]

```python
transformers.DiaProcessor(feature_extractor, tokenizer, audio_tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/processing_dia.py#L84)

**Parameters:**

feature_extractor (`DiaFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`DiaTokenizer`) : The tokenizer is a required input.

audio_tokenizer (`DacModel`) : An instance of [DacModel](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacModel) used to encode/decode audio into/from codebooks. It is a required input.

Constructs a DiaProcessor which wraps a feature extractor and a tokenizer into a single processor.

[DiaProcessor](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor) offers all the functionalities of [DiaFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaFeatureExtractor) and [DiaTokenizer](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaTokenizer). See the
[~DiaFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaFeatureExtractor) and [~DiaTokenizer](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaTokenizer) for more information.

#### __call__[[transformers.DiaProcessor.__call__]]

```python
__call__(text: str | list[str], audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, output_labels: bool | None = False, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/processing_dia.py#L144)

**Parameters:**

text (`Union[str, list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

output_labels (`bool`, *optional*, defaults to `False`) : Whether to return labels for training. When `True`, the processor generates labels from the decoder input sequence by shifting it by one position. Labels use special values: `-100` for tokens to ignore in loss computation (padding and BOS tokens), and `-101` for audio frames used only for the backbone model (when `depth_decoder_labels_ratio < 1.0`). Cannot be used together with `generation=True`.

bos_token_id (`int`, *kwargs*, *optional*, defaults to `1026`) : The token ID used as the beginning-of-sequence token for audio codebooks. This token is prepended to each audio sequence during encoding.

eos_token_id (`int`, *kwargs*, *optional*, defaults to `1024`) : The token ID used as the end-of-sequence token for audio codebooks. This token is appended to audio sequences during training (when `generation=False`) to mark the end of the audio.

pad_token_id (`int`, *kwargs*, *optional*, defaults to `1025`) : The token ID used for padding audio codebook sequences. This token is used to fill positions in the delay pattern where no valid audio token exists.

delay_pattern (`list[int]`, *kwargs*, *optional*, defaults to `[0, 8, 9, 10, 11, 12, 13, 14, 15]`) : A list of delay values (in frames) for each codebook channel. The delay pattern creates temporal offsets between different codebook channels, allowing the model to capture dependencies across channels. Each value represents the number of frames to delay that specific channel.

generation (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether the processor is being used for generation (text-to-speech) or training. When `True`, the processor prepares inputs for generation mode where audio is generated from text. When `False`, it prepares inputs for training where both text and audio are provided.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

#### batch_decode[[transformers.DiaProcessor.batch_decode]]

```python
batch_decode(decoder_input_ids: torch.Tensor, audio_prompt_len: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/processing_dia.py#L247)

**Parameters:**

decoder_input_ids (`torch.Tensor`) : The complete output sequence of the decoder.

audio_prompt_len (`int`) : The audio prefix length (e.g. when using voice cloning).

Decodes a batch of audio codebook sequences into their respective audio waveforms via the
`audio_tokenizer`. See [decode()](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacModel.decode) for more information.

#### decode[[transformers.DiaProcessor.decode]]

```python
decode(decoder_input_ids: torch.Tensor, audio_prompt_len: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/processing_dia.py#L318)

Decodes a single sequence of audio codebooks into the respective audio waveform via the
`audio_tokenizer`. See [decode()](/docs/transformers/v5.15.1/en/model_doc/dac#transformers.DacModel.decode) and [batch_decode()](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor.batch_decode) for more information.

## DiaModel[[transformers.DiaModel]]

#### transformers.DiaModel[[transformers.DiaModel]]

```python
transformers.DiaModel(config: DiaConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/modeling_dia.py#L649)

**Parameters:**

config ([DiaConfig](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Dia model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DiaModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_position_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: transformers.modeling_outputs.BaseModelOutput | tuple | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, use_cache: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/modeling_dia.py#L657)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size * num_codebooks, target_sequence_length) --

or (batch_size, target_sequence_length, num_codebooks)`, *optional*) : 1. (batch_size * num_codebooks, target_sequence_length): corresponds to the general use case where the audio input codebooks are flattened into the batch dimension. This also aligns with the flat- tened audio logits which are used to calculate the loss.  2. (batch_size, sequence_length, num_codebooks): corresponds to the internally used shape of Dia to calculate embeddings and subsequent steps more efficiently.  If no `decoder_input_ids` are provided, it will create a tensor of `bos_token_id` with shape `(batch_size, 1, num_codebooks)`. Indices can be obtained using the [DiaProcessor](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor). See [DiaProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor.__call__) for more details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`.  [What are position IDs?](../glossary#position-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`Union[~modeling_outputs.BaseModelOutput, tuple]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [DiaModel](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

## DiaForConditionalGeneration[[transformers.DiaForConditionalGeneration]]

#### transformers.DiaForConditionalGeneration[[transformers.DiaForConditionalGeneration]]

```python
transformers.DiaForConditionalGeneration(config: DiaConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/modeling_dia.py#L760)

**Parameters:**

config ([DiaConfig](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Dia model consisting of a (byte) text encoder and audio decoder with a prediction head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DiaForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_position_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: transformers.modeling_outputs.BaseModelOutput | tuple | None = None, past_key_values: transformers.cache_utils.EncoderDecoderCache | None = None, use_cache: bool | None = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/modeling_dia.py#L779)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size * num_codebooks, target_sequence_length) --

or (batch_size, target_sequence_length, num_codebooks)`, *optional*) : 1. (batch_size * num_codebooks, target_sequence_length): corresponds to the general use case where the audio input codebooks are flattened into the batch dimension. This also aligns with the flat- tened audio logits which are used to calculate the loss.  2. (batch_size, sequence_length, num_codebooks): corresponds to the internally used shape of Dia to calculate embeddings and subsequent steps more efficiently.  If no `decoder_input_ids` are provided, it will create a tensor of `bos_token_id` with shape `(batch_size, 1, num_codebooks)`. Indices can be obtained using the [DiaProcessor](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor). See [DiaProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaProcessor.__call__) for more details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_position_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`) : Indices of positions of each input sequence tokens in the position embeddings. Used to calculate the position embeddings up to `config.decoder_config.max_position_embeddings`.  [What are position IDs?](../glossary#position-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`Union[~modeling_outputs.BaseModelOutput, tuple]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.EncoderDecoderCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

labels (`torch.LongTensor` of shape `(batch_size * num_codebooks,)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.decoder_config.vocab_size - 1]` or -100. Tokens with indices set to `-100` are ignored (masked).

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [DiaForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/dia#transformers.DiaForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

#### generate[[transformers.DiaForConditionalGeneration.generate]]

```python
generate(inputs: typing.Optional[torch.Tensor] = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, logits_processor: transformers.generation.logits_process.LogitsProcessorList | None = None, stopping_criteria: transformers.generation.stopping_criteria.StoppingCriteriaList | None = None, prefix_allowed_tokens_fn: collections.abc.Callable[[int, torch.Tensor], list[int]] | None = None, synced_gpus: bool | None = None, assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, negative_prompt_ids: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, custom_generate: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dia/generation_dia.py#L406)
