# TimesFM 2.5

## Overview

TimesFM 2.5 (Time Series Foundation Model) is a pretrained time-series foundation model proposed in [A decoder-only foundation model for time-series forecasting](https://huggingface.co/papers/2310.10688) by Abhimanyu Das, Weihao Kong, Rajat Sen, and Yichen Zhou. It builds on the original TimesFM architecture with rotary attention, QK normalization, per-dimension attention scaling, and continuous quantile prediction.

The abstract from the paper is the following:

*Motivated by recent advances in large language models for Natural Language Processing (NLP), we design a time-series foundation model for forecasting whose out-of-the-box zero-shot performance on a variety of public datasets comes close to the accuracy of state-of-the-art supervised forecasting models for each individual dataset. Our model is based on pretraining a decoder style attention model with input patching, using a large time-series corpus comprising both real-world and synthetic datasets. Experiments on a diverse set of previously unseen forecasting datasets suggests that the model can yield accurate zero-shot forecasts across different domains, forecasting horizons and temporal granularities.*

This model was contributed by [kashif](https://huggingface.co/kashif). The original code can be found [here](https://github.com/google-research/timesfm).

You can find the checkpoint at [`google/timesfm-2.5-200m-transformers`](https://huggingface.co/google/timesfm-2.5-200m-transformers).

## Usage example

```python
import numpy as np
import torch

from transformers import TimesFm2_5ModelForPrediction

model = TimesFm2_5ModelForPrediction.from_pretrained(
    "google/timesfm-2.5-200m-transformers",
    device_map="auto",
)

forecast_input = [
    np.sin(np.linspace(0, 20, 100)),
    np.sin(np.linspace(0, 20, 200)),
    np.sin(np.linspace(0, 20, 400)),
]
forecast_input_tensor = [torch.tensor(ts, dtype=torch.float32, device=model.device) for ts in forecast_input]

with torch.no_grad():
    outputs = model(past_values=forecast_input_tensor, return_dict=True)
    point_forecast = outputs.mean_predictions
    quantile_forecast = outputs.full_predictions
```

## TimesFm2_5Config[[transformers.TimesFm2_5Config]]

#### transformers.TimesFm2_5Config[[transformers.TimesFm2_5Config]]

```python
transformers.TimesFm2_5Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, patch_length: int = 32, context_length: int = 16384, horizon_length: int = 128, num_hidden_layers: int = 20, hidden_size: int = 1280, intermediate_size: int = 1280, head_dim: int = 80, num_attention_heads: int = 16, rms_norm_eps: float = 1e-06, quantiles: list[float] | tuple[float, ...] = (0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9), attention_dropout: float | int = 0.0, initializer_range: float = 0.02, num_key_value_heads: int = 16, attention_bias: bool = False, output_quantile_len: int = 1024, decode_index: int = 5, use_bias: bool = False, activation: str = 'swish', use_continuous_quantile_head: bool = True, force_flip_invariance: bool = True, infer_is_positive: bool = True, max_position_embeddings: int = 16384, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timesfm2_5/configuration_timesfm2_5.py#L30)

**Parameters:**

patch_length (`int`, *optional*, defaults to 32) : The length of one patch in the input sequence.

context_length (`int`, *optional*, defaults to 16384) : The length of the input context.

horizon_length (`int`, *optional*, defaults to 128) : The length of the prediction horizon.

num_hidden_layers (`int`, *optional*, defaults to `20`) : Number of hidden layers in the Transformer decoder.

hidden_size (`int`, *optional*, defaults to `1280`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `1280`) : Dimension of the MLP representations.

head_dim (`int`, *optional*, defaults to `80`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

quantiles (`list[float]`, *optional*, defaults to `[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]`) : The quantiles to predict.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

num_key_value_heads (`int`, *optional*, defaults to `16`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

output_quantile_len (`int`, *optional*, defaults to 1024) : Length of the quantile output projection dimension.

decode_index (`int`, *optional*, defaults to 5) : Index into the quantile dimension used to extract the point (median) forecast.

use_bias (`bool`, *optional*, defaults to `False`) : Whether to use bias in MLP and transformer linear layers.

activation (`str`, *optional*, defaults to `swish`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

use_continuous_quantile_head (`bool`, *optional*, defaults to `True`) : Whether to use the continuous quantile head for non-median quantile predictions.

force_flip_invariance (`bool`, *optional*, defaults to `True`) : Whether to apply flip-invariance averaging during forecasting.

infer_is_positive (`bool`, *optional*, defaults to `True`) : Whether to clamp forecasts to non-negative values when the input minimum is non-negative.

max_position_embeddings (`int`, *optional*, defaults to `16384`) : The maximum sequence length that this model might ever be used with.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a TimesFm2_5Model. It is used to instantiate a Timesfm2 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/timesfm-2.5-200m-transformers](https://huggingface.co/google/timesfm-2.5-200m-transformers)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import TimesFm2_5Config, TimesFm2_5ModelForPrediction

>>> configuration = TimesFm2_5Config()
>>> model = TimesFm2_5ModelForPrediction(configuration)
>>> configuration = model.config
```

## TimesFm2_5Model[[transformers.TimesFm2_5Model]]

#### transformers.TimesFm2_5Model[[transformers.TimesFm2_5Model]]

```python
transformers.TimesFm2_5Model(config: TimesFm2_5Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timesfm2_5/modeling_timesfm2_5.py#L465)

#### forward[[transformers.TimesFm2_5Model.forward]]

```python
forward(past_values: Tensor, past_values_padding: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timesfm2_5/modeling_timesfm2_5.py#L561)

**Parameters:**

past_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Past values of the time series used as input to the model.

past_values_padding (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Padding mask for the input. `1` indicates padded (masked) time steps, `0` indicates valid values.

**Returns:** `TimesFm2_5Output` or `tuple(torch.FloatTensor)`

A `TimesFm2_5Output` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimesFm2_5Config](/docs/transformers/v5.15.1/en/model_doc/timesfm2_5#transformers.TimesFm2_5Config)) and inputs.

The [TimesFm2_5Model](/docs/transformers/v5.15.1/en/model_doc/timesfm2_5#transformers.TimesFm2_5Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **loc** (`torch.Tensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) -- Shift values of each time series' context window which is used to give the model inputs of the same
  magnitude and then used to shift back to the original magnitude.
- **scale** (`torch.Tensor` of shape `(batch_size,)` or `(batch_size, input_size)`, *optional*) -- Scaling values of each time series' context window which is used to give the model inputs of the same
  magnitude and then used to rescale back to the original magnitude.
- **context_mu** (`torch.Tensor` of shape `(batch_size, num_patches)`) -- Running means computed per input patch during normalization.
- **context_sigma** (`torch.Tensor` of shape `(batch_size, num_patches)`) -- Running standard deviations computed per input patch during normalization.

## TimesFm2_5ModelForPrediction[[transformers.TimesFm2_5ModelForPrediction]]

#### transformers.TimesFm2_5ModelForPrediction[[transformers.TimesFm2_5ModelForPrediction]]

```python
transformers.TimesFm2_5ModelForPrediction(config: TimesFm2_5Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timesfm2_5/modeling_timesfm2_5.py#L647)

TimesFm2_5 model for quantile and mean prediction.

#### forward[[transformers.TimesFm2_5ModelForPrediction.forward]]

```python
forward(past_values: Sequence, window_size: int | None = None, future_values: typing.Optional[torch.Tensor] = None, forecast_context_len: int | None = None, truncate_negative: bool | None = None, force_flip_invariance: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/timesfm2_5/modeling_timesfm2_5.py#L735)

**Parameters:**

past_values (`collections.abc.Sequence[torch.Tensor]`) : Past values of the time series that serves as input to the model. Each tensor is a 1D time series.

window_size (`int`, *optional*) : Window size of trend + residual decomposition. If `None`, decomposition is not applied.

future_values (`torch.Tensor`, *optional*) : Optional future values used to compute the loss.

forecast_context_len (`int`, *optional*) : Optional context length override used during forecasting.

truncate_negative (`bool`, *optional*) : Whether to clamp outputs to non-negative values. If `None`, defaults to `config.infer_is_positive`.

force_flip_invariance (`bool`, *optional*) : Whether to apply the flip-invariance combination. If `None`, defaults to `config.force_flip_invariance`.

**Returns:** `TimesFm2_5OutputForPrediction` or `tuple(torch.FloatTensor)`

A `TimesFm2_5OutputForPrediction` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimesFm2_5Config](/docs/transformers/v5.15.1/en/model_doc/timesfm2_5#transformers.TimesFm2_5Config)) and inputs.

The [TimesFm2_5ModelForPrediction](/docs/transformers/v5.15.1/en/model_doc/timesfm2_5#transformers.TimesFm2_5ModelForPrediction) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **mean_predictions** (`torch.Tensor` of shape `(batch_size, horizon_length)`) -- Deterministic forecasts after denormalization.
- **full_predictions** (`torch.Tensor` of shape `(batch_size, horizon_length, quantiles)`) -- Quantile forecasts including the median after denormalization.
- **loss** (`torch.Tensor` of shape `(1,)`, *optional*, returned when `future_values` is provided) -- Training loss combining MSE and quantile losses when targets are supplied.
