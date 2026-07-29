# TVP

## Overview

The text-visual prompting (TVP) framework was proposed in the paper [Text-Visual Prompting for Efficient 2D Temporal Video Grounding](https://huggingface.co/papers/2303.04995) by Yimeng Zhang, Xin Chen, Jinghan Jia, Sijia Liu, Ke Ding.

The abstract from the paper is the following:

*In this paper, we study the problem of temporal video grounding (TVG), which aims to predict the starting/ending time points of moments described by a text sentence within a long untrimmed video. Benefiting from fine-grained 3D visual features, the TVG techniques have achieved remarkable progress in recent years. However, the high complexity of 3D convolutional neural networks (CNNs) makes extracting dense 3D visual features time-consuming, which calls for intensive memory and computing resources. Towards efficient TVG, we propose a novel text-visual prompting (TVP) framework, which incorporates optimized perturbation patterns (that we call ‘prompts’) into both visual inputs and textual features of a TVG model. In sharp contrast to 3D CNNs, we show that TVP allows us to effectively co-train vision encoder and language encoder in a 2D TVG model and improves the performance of cross-modal feature fusion using only low-complexity sparse 2D visual features. Further, we propose a Temporal-Distance IoU (TDIoU) loss for efficient learning of TVG. Experiments on two benchmark datasets, Charades-STA and ActivityNet Captions datasets, empirically show that the proposed TVP significantly boosts the performance of 2D TVG (e.g., 9.79% improvement on Charades-STA and 30.77% improvement on ActivityNet Captions) and achieves 5× inference acceleration over TVG using 3D visual features.*

This research addresses temporal video grounding (TVG), which is the process of pinpointing the start and end times of specific events in a long video, as described by a text sentence. Text-visual prompting (TVP), is proposed to enhance TVG. TVP involves integrating specially designed patterns, known as 'prompts', into both the visual (image-based) and textual (word-based) input components of a TVG model. These prompts provide additional spatial-temporal context, improving the model's ability to accurately determine event timings in the video. The approach employs 2D visual inputs in place of 3D ones. Although 3D inputs offer more spatial-temporal detail, they are also more time-consuming to process. The use of 2D inputs with the prompting method aims to provide similar levels of context and accuracy more efficiently.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tvp_architecture.png"
alt="drawing" width="600"/>

 TVP architecture. Taken from the original paper. 

This model was contributed by [Jiqing Feng](https://huggingface.co/Jiqing). The original code can be found [here](https://github.com/intel/TVP).

## Usage tips and examples

Prompts are optimized perturbation patterns, which would be added to input video frames or text features. Universal set refers to using the same exact set of prompts for any input, this means that these prompts are added consistently to all video frames and text features, regardless of the input's content.

TVP consists of a visual encoder and cross-modal encoder. A universal set of visual prompts and text prompts to be integrated into sampled video frames and textual features, respectively. Specially, a set of different visual prompts are applied to uniformly-sampled frames of one untrimmed video in order.

The goal of this model is to incorporate trainable prompts into both visual inputs and textual features to temporal video grounding(TVG) problems.
In principle, one can apply any visual, cross-modal encoder in the proposed architecture.

The [TvpProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpProcessor) wraps [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor) into a single instance to both
encode the text and prepare the images respectively.

The following example shows how to run temporal video grounding using [TvpProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpProcessor) and [TvpForVideoGrounding](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpForVideoGrounding).

```python
import av
import cv2
import numpy as np
from huggingface_hub import hf_hub_download

from transformers import AutoProcessor, TvpForVideoGrounding

def pyav_decode(container, sampling_rate, num_frames, clip_idx, num_clips, target_fps):
    '''
    Convert the video from its original fps to the target_fps and decode the video with PyAV decoder.
    Args:
        container (container): pyav container.
        sampling_rate (int): frame sampling rate (interval between two sampled frames).
        num_frames (int): number of frames to sample.
        clip_idx (int): if clip_idx is -1, perform random temporal sampling.
            If clip_idx is larger than -1, uniformly split the video to num_clips
            clips, and select the clip_idx-th video clip.
        num_clips (int): overall number of clips to uniformly sample from the given video.
        target_fps (int): the input video may have different fps, convert it to
            the target video fps before frame sampling.
    Returns:
        frames (tensor): decoded frames from the video. Return None if the no
            video stream was found.
        fps (float): the number of frames per second of the video.
    '''
    video = container.streams.video[0]
    fps = float(video.average_rate)
    clip_size = sampling_rate * num_frames / target_fps * fps
    delta = max(num_frames - clip_size, 0)
    start_idx = delta * clip_idx / num_clips
    end_idx = start_idx + clip_size - 1
    timebase = video.duration / num_frames
    video_start_pts = int(start_idx * timebase)
    video_end_pts = int(end_idx * timebase)
    seek_offset = max(video_start_pts - 1024, 0)
    container.seek(seek_offset, any_frame=False, backward=True, stream=video)
    frames = {}
    for frame in container.decode(video=0):
        if frame.pts < video_start_pts:
            continue
        frames[frame.pts] = frame
        if frame.pts > video_end_pts:
            break
    frames = [frames[pts] for pts in sorted(frames)]
    return frames, fps

def decode(container, sampling_rate, num_frames, clip_idx, num_clips, target_fps):
    '''
    Decode the video and perform temporal sampling.
    Args:
        container (container): pyav container.
        sampling_rate (int): frame sampling rate (interval between two sampled frames).
        num_frames (int): number of frames to sample.
        clip_idx (int): if clip_idx is -1, perform random temporal sampling.
            If clip_idx is larger than -1, uniformly split the video to num_clips
            clips, and select the clip_idx-th video clip.
        num_clips (int): overall number of clips to uniformly sample from the given video.
        target_fps (int): the input video may have different fps, convert it to
            the target video fps before frame sampling.
    Returns:
        frames (tensor): decoded frames from the video.
    '''
    assert clip_idx >= -2, f"Not a valid clip_idx {clip_idx}"
    frames, fps = pyav_decode(container, sampling_rate, num_frames, clip_idx, num_clips, target_fps)
    clip_size = sampling_rate * num_frames / target_fps * fps
    index = np.linspace(0, clip_size - 1, num_frames)
    index = np.clip(index, 0, len(frames) - 1).astype(np.int64)
    frames = np.array([frames[idx].to_rgb().to_ndarray() for idx in index])
    frames = frames.transpose(0, 3, 1, 2)
    return frames

file = hf_hub_download(repo_id="Intel/tvp_demo", filename="AK2KG.mp4", repo_type="dataset")
model = TvpForVideoGrounding.from_pretrained("Intel/tvp-base", device_map="auto")

decoder_kwargs = dict(
    container=av.open(file, metadata_errors="ignore"),
    sampling_rate=1,
    num_frames=model.config.num_frames,
    clip_idx=0,
    num_clips=1,
    target_fps=3,
)
raw_sampled_frms = decode(**decoder_kwargs)

text = "a person is sitting on a bed."
processor = AutoProcessor.from_pretrained("Intel/tvp-base")
model_inputs = processor(
    text=[text], videos=list(raw_sampled_frms), return_tensors="pt", max_text_length=100#, size=size
)

model_inputs["pixel_values"] = model_inputs["pixel_values"].to(model.dtype)
output = model(**model_inputs)

def get_video_duration(filename):
    cap = cv2.VideoCapture(filename)
    if cap.isOpened():
        rate = cap.get(5)
        frame_num = cap.get(7)
        duration = frame_num/rate
        return duration
    return -1

duration = get_video_duration(file)
start, end = processor.post_process_video_grounding(output.logits, duration)

print(f"The time slot of the video corresponding to the text \"{text}\" is from {start}s to {end}s")
```

Tips:

- This implementation of TVP uses [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) to generate text embeddings and Resnet-50 model to compute visual embeddings.
- Checkpoints for pre-trained [tvp-base](https://huggingface.co/Intel/tvp-base) is released.
- Please refer to [Table 2](https://huggingface.co/papers/2303.04995) for TVP's performance on Temporal Video Grounding task.

## TvpConfig[[transformers.TvpConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **distance_loss_weight** (`float`, *optional*, defaults to 1.0) --
  The weight of distance loss.
- **duration_loss_weight** (`float`, *optional*, defaults to 0.1) --
  The weight of duration loss.
- **visual_prompter_type** (`str`, *optional*, defaults to `"framepad"`) --
  Visual prompt type. The type of padding. Framepad means padding on each frame. Should be one of "framepad"
  or "framedownpad"
- **visual_prompter_apply** (`str`, *optional*, defaults to `"replace"`) --
  The way of applying visual prompt. Replace means use the value of prompt to change the original value in
  visual inputs. Should be one of "replace", or "add", or "remove".
- **visual_prompt_size** (`int`, *optional*, defaults to 96) --
  The size of visual prompt.
- **max_img_size** (`int`, *optional*, defaults to 448) --
  The maximum size of frame.
- **num_frames** (`int`, *optional*, defaults to 48) --
  The number of frames extracted from a video.
- **vocab_size** (`int`, *optional*, defaults to `30522`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **type_vocab_size** (`int`, *optional*, defaults to `2`) --
  The vocabulary size of the `token_type_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to 512) --
  The maximum sequence length that this model might ever be used with. Typically set this to something large
  just in case (e.g., 512 or 1024 or 2048).
- **max_grid_col_position_embeddings** (`int`, *optional*, defaults to 100) --
  The largest number of horizontal patches from a video frame.
- **max_grid_row_position_embeddings** (`int`, *optional*, defaults to 100) --
  The largest number of vertical patches from a video frame.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a TvpModel. It is used to instantiate a Tvp
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Intel/tvp-base](https://huggingface.co/Intel/tvp-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## TvpImageProcessor[[transformers.TvpImageProcessor]]

- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the channel order of the image from RGB to BGR.
- **constant_values** (`float`, *kwargs* or `List[float]`, *optional*, defaults to `self.constant_values`) --
  Value used to fill the padding area when `pad_mode` is `'constant'`.
- **pad_mode** (`str`, *kwargs*, *optional*, defaults to `self.pad_mode`) --
  Padding mode to use — `'constant'`, `'edge'`, `'reflect'`, or `'symmetric'`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a TvpImageProcessor image processor.

- **videos** (`ImageInput` or `list[ImageInput]` or `list[list[ImageInput]]`) --
  Frames to preprocess.
- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the channel order of the image from RGB to BGR.
- **constant_values** (`float`, *kwargs* or `List[float]`, *optional*, defaults to `self.constant_values`) --
  Value used to fill the padding area when `pad_mode` is `'constant'`.
- **pad_mode** (`str`, *kwargs*, *optional*, defaults to `self.pad_mode`) --
  Padding mode to use — `'constant'`, `'edge'`, `'reflect'`, or `'symmetric'`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## TvpImageProcessorPil[[transformers.TvpImageProcessorPil]]

- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the channel order of the image from RGB to BGR.
- **constant_values** (`float`, *kwargs* or `List[float]`, *optional*, defaults to `self.constant_values`) --
  Value used to fill the padding area when `pad_mode` is `'constant'`.
- **pad_mode** (`str`, *kwargs*, *optional*, defaults to `self.pad_mode`) --
  Padding mode to use — `'constant'`, `'edge'`, `'reflect'`, or `'symmetric'`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a TvpImageProcessor image processor.

- **videos** (`ImageInput` or `list[ImageInput]` or `list[list[ImageInput]]`) --
  Frames to preprocess.
- **do_flip_channel_order** (`bool`, *kwargs*, *optional*, defaults to `self.do_flip_channel_order`) --
  Whether to flip the channel order of the image from RGB to BGR.
- **constant_values** (`float`, *kwargs* or `List[float]`, *optional*, defaults to `self.constant_values`) --
  Value used to fill the padding area when `pad_mode` is `'constant'`.
- **pad_mode** (`str`, *kwargs*, *optional*, defaults to `self.pad_mode`) --
  Padding mode to use — `'constant'`, `'edge'`, `'reflect'`, or `'symmetric'`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## TvpProcessor[[transformers.TvpProcessor]]

- **image_processor** (`TvpImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`BertTokenizer`) --
  The tokenizer is a required input.
Constructs a TvpProcessor which wraps a image processor and a tokenizer into a single processor.

[TvpProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpProcessor) offers all the functionalities of [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor) and [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer). See the
[~TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor) and [~BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

## TvpModel[[transformers.TvpModel]]

- **config** ([TvpModel](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Tvp Model transformer outputting BaseModelOutputWithPooling object without any specific head on top.

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
  [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor). See `TvpImageProcessor.__call__()` for details ([TvpProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpProcessor) uses
  [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor) for processing images).
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TvpConfig](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpConfig)) and inputs.
The [TvpModel](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpModel) forward method, overrides the `__call__` special method.

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

Examples:
```python
>>> import torch
>>> from transformers import AutoConfig, AutoTokenizer, TvpModel

>>> model = TvpModel.from_pretrained("Jiqing/tiny-random-tvp")

>>> tokenizer = AutoTokenizer.from_pretrained("Jiqing/tiny-random-tvp")

>>> pixel_values = torch.rand(1, 1, 3, 448, 448)
>>> text_inputs = tokenizer("This is an example input", return_tensors="pt")
>>> output = model(text_inputs.input_ids, pixel_values, text_inputs.attention_mask)
```

## TvpForVideoGrounding[[transformers.TvpForVideoGrounding]]

- **config** ([TvpForVideoGrounding](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpForVideoGrounding)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Tvp Model with a video grounding head on top computing IoU, distance, and duration loss.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>] | None = None"}, {"name": "output_attentions", "val": ": bool | None = None"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "return_dict", "val": ": bool | None = None"}, {"name": "interpolate_pos_encoding", "val": ": bool = False"}, {"name": "**kwargs", "val": ""}]}>
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor). See `TvpImageProcessor.__call__()` for details ([TvpProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpProcessor) uses
  [TvpImageProcessor](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpImageProcessor) for processing images).
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **labels** (`torch.FloatTensor` of shape `(batch_size, 3)`, *optional*) --
  The labels contains duration, start time, and end time of the video corresponding to the text.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`TvpVideoGroundingOutput` or `tuple(torch.FloatTensor)`A `TvpVideoGroundingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([TvpConfig](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpConfig)) and inputs.
The [TvpForVideoGrounding](/docs/transformers/v5.14.0/en/model_doc/tvp#transformers.TvpForVideoGrounding) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Temporal-Distance IoU loss for video grounding.
- **logits** (`torch.FloatTensor` of shape `(batch_size, 2)`) -- Contains start_time/duration and end_time/duration. It is the time slot of the videos corresponding to the
  input texts.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

Examples:
```python
>>> import torch
>>> from transformers import AutoConfig, AutoTokenizer, TvpForVideoGrounding

>>> model = TvpForVideoGrounding.from_pretrained("Jiqing/tiny-random-tvp")

>>> tokenizer = AutoTokenizer.from_pretrained("Jiqing/tiny-random-tvp")

>>> pixel_values = torch.rand(1, 1, 3, 448, 448)
>>> text_inputs = tokenizer("This is an example input", return_tensors="pt")
>>> output = model(text_inputs.input_ids, pixel_values, text_inputs.attention_mask)
```
