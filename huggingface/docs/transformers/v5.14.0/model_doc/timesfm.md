# TimesFM

## Overview

TimesFM (Time Series Foundation Model) is a pretrained time-series foundation model proposed in [A decoder-only foundation model for time-series forecasting](https://huggingface.co/papers/2310.10688) by Abhimanyu Das, Weihao Kong, Rajat Sen, and  Yichen Zhou. It is a decoder only model that uses non-overlapping patches of time-series data as input and outputs some output patch length prediction in an autoregressive fashion.

The abstract from the paper is the following:

*Motivated by recent advances in large language models for Natural Language Processing (NLP), we design a time-series foundation model for forecasting whose out-of-the-box zero-shot performance on a variety of public datasets comes close to the accuracy of state-of-the-art supervised forecasting models for each individual dataset. Our model is based on pretraining a patched-decoder style attention model on a large time-series corpus, and can work well across different forecasting history lengths, prediction lengths and temporal granularities.*

This model was contributed by [kashif](https://huggingface.co/kashif).
The original code can be found [here](https://github.com/google-research/timesfm).

To use the model:

```python
import numpy as np
import torch

from transformers import TimesFmModelForPrediction

model = TimesFmModelForPrediction.from_pretrained(
    "google/timesfm-2.0-500m-pytorch",
    attn_implementation="sdpa",
    device_map="auto"
)

 # Create dummy inputs
forecast_input = [
    np.sin(np.linspace(0, 20, 100)),
    np.sin(np.linspace(0, 20, 200)),
    np.sin(np.linspace(0, 20, 400)),
]
frequency_input = [0, 1, 2]

# Convert inputs to sequence of tensors
forecast_input_tensor = [
    torch.tensor(ts).to(model.device)
    for ts in forecast_input
]
frequency_input_tensor = torch.tensor(frequency_input, dtype=torch.long).to(model.device)

# Get predictions from the pre-trained model
with torch.no_grad():
    outputs = model(past_values=forecast_input_tensor, freq=frequency_input_tensor, return_dict=True)
    point_forecast_conv = outputs.mean_predictions.float().cpu().numpy()
    quantile_forecast_conv = outputs.full_predictions.float().cpu().numpy()
```

## TimesFmConfig[[transformers.TimesFmConfig]]

- **patch_length** (`int`, *optional*, defaults to 32) --
  The length of one patch in the input sequence.
- **context_length** (`int`, *optional*, defaults to 512) --
  The length of the input context.
- **horizon_length** (`int`, *optional*, defaults to 128) --
  The length of the prediction horizon.
- **freq_size** (`int`, *optional*, defaults to 3) --
  The number of frequency embeddings.
- **num_hidden_layers** (`int`, *optional*, defaults to `50`) --
  Number of hidden layers in the Transformer decoder.
- **hidden_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `1280`) --
  Dimension of the MLP representations.
- **head_dim** (`int`, *optional*, defaults to `80`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **tolerance** (`float`, *optional*, defaults to 1e-06) --
  The tolerance for the quantile loss.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **quantiles** (`list[float]`, *optional*, defaults to `[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]`) --
  The quantiles to predict.
- **pad_val** (`float`, *optional*, defaults to 1123581321.0) --
  The value used to pad the predictions.
- **attention_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout probability for the attention scores.
- **use_positional_embedding** (`bool`, *optional*, defaults to `False`) --
  Whether to add positional embeddings.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **min_timescale** (`int`, *optional*, defaults to 1) --
  The start of the geometric positional index. Determines the periodicity of
  the added signal.
- **max_timescale** (`int`, *optional*, defaults to 10000) --
  The end of the geometric positional index. Determines the frequency of the
  added signal.

This is the configuration class to store the configuration of a TimesFmModel. It is used to instantiate a Timesfm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/timesfm-2.0-500m-pytorch](https://huggingface.co/google/timesfm-2.0-500m-pytorch)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## TimesFmModel[[transformers.TimesFmModel]]

- **config** ([TimesFmConfig](/docs/transformers/v5.14.0/en/model_doc/timesfm#transformers.TimesFmConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Timesfm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "past_values_padding", "val": ": LongTensor"}, {"name": "freq", "val": ": )>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **past_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) --
  Past values of the time series that serves as input to the model.
- **past_values_padding** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  The padding indicator of the time series.
- **freq** (`torch.LongTensor` of shape `(batch_size,)`) --
  Frequency indices for the time series data.`TimesFmOutput` or `tuple(torch.FloatTensor)`A `TimesFmOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimesFmConfig](/docs/transformers/v5.14.0/en/model_doc/timesfm#transformers.TimesFmConfig)) and inputs.
The [TimesFmModel](/docs/transformers/v5.14.0/en/model_doc/timesfm#transformers.TimesFmModel) forward method, overrides the `__call__` special method.

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
- **loc** (`torch.Tensor` of shape `(batch_size, )`) -- The mean of the time series inputs.
- **scale** (`torch.Tensor` of shape `(batch_size,)`) -- The scale of the time series inputs.

## TimesFmModelForPrediction[[transformers.TimesFmModelForPrediction]]

TimesFM model for quantile and mean prediction.

- **past_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) --
  Past values of the time series that serves as input to the model.
- **freq** (`torch.LongTensor` of shape `(batch_size,)`) --
  Frequency indices for the time series data.
- **window_size** (`int`, *optional*) --
  Window size of trend + residual decomposition. If None then we do not do decomposition.
- **future_values** (`torch.Tensor`, *optional*) --
  Optional future time series values to be used for loss computation.
- **forecast_context_len** (`int`, *optional*) --
  Optional max context length.
- **return_forecast_on_context** (`bool`, *optional*) --
  True to return the forecast on the context when available, i.e. after the first input patch.
- **truncate_negative** (`bool`, *optional*) --
  Truncate to only non-negative values if any of the contexts have non-negative values,
  otherwise do nothing.`TimesFmOutputForPrediction` or `tuple(torch.FloatTensor)`A `TimesFmOutputForPrediction` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TimesFmConfig](/docs/transformers/v5.14.0/en/model_doc/timesfm#transformers.TimesFmConfig)) and inputs.
The [TimesFmModelForPrediction](/docs/transformers/v5.14.0/en/model_doc/timesfm#transformers.TimesFmModelForPrediction) forward method, overrides the `__call__` special method.

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
- **mean_predictions** (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The mean predictions of the time series.
- **full_predictions** (`torch.Tensor` of shape `(batch_size, sequence_length)`) -- The full predictions of the time series including the mean and the quantiles.
- **loss** (`torch.Tensor` of shape `(1,)`, *optional*, returned when `future_values` is provided) -- The loss of the TimesFM model.

Example:

```python
>>> from transformers import TimesFmModelForPrediction

>>> model = TimesFmModelForPrediction.from_pretrained("google/timesfm-2.0-500m-pytorch")

>>> forecast_input = [torch.linspace(0, 20, 100).sin(), torch.linspace(0, 20, 200).sin(), torch.linspace(0, 20, 400).sin()]
>>> frequency_input = torch.tensor([0, 1, 2], dtype=torch.long)

>>> # Generate
>>> with torch.no_grad():
>>>     outputs = model(past_values=forecast_input, freq=frequency_input, return_dict=True)
>>>     point_forecast_conv = outputs.mean_predictions
>>>     quantile_forecast_conv = outputs.full_predictions
```
