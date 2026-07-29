# Parakeet

## Overview

Parakeet models, [introduced by NVIDIA NeMo](https://developer.nvidia.com/blog/pushing-the-boundaries-of-speech-recognition-with-nemo-parakeet-asr-models/), are models that combine a [Fast Conformer](https://docs.nvidia.com/nemo-framework/user-guide/latest/nemotoolkit/asr/models.html#fast-conformer) encoder with connectionist temporal classification (CTC), recurrent neural network transducer (RNNT) or token and duration transducer (TDT) decoder for automatic speech recognition.

**Model Architecture**

- **Fast Conformer Encoder**: A linearly scalable Conformer architecture that processes mel-spectrogram features and reduces sequence length through subsampling. This is more efficient version of the Conformer Encoder found in [FastSpeech2Conformer](./fastspeech2_conformer) (see [ParakeetEncoder](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetEncoder) for the encoder implementation and details).
- [**ParakeetForCTC**](#parakeetforctc): a Fast Conformer Encoder + a CTC decoder
  - **CTC Decoder**: Simple but effective decoder consisting of:
    - 1D convolution projection from encoder hidden size to vocabulary size (for optimal NeMo compatibility).
    - CTC loss computation for training.
    - Greedy CTC decoding for inference.
- [**ParakeetForRNNT**](#parakeetforrnnt): a Fast Conformer Encoder + an RNN-T (RNN Transducer) decoder
  - **RNN-T Decoder**: Standard neural transducer:
    - LSTM prediction network maintains language context across token predictions.
    - Joint network combines encoder and decoder outputs.
    - Greedy transducer decoding for inference: a blank emission advances the encoder frame by one, a non-blank emission stays on the same frame.
- [**ParakeetForTDT**](#parakeetfortdt): a Fast Conformer Encoder + a TDT (Token Duration Transducer) decoder
  - **TDT Decoder**: Jointly predicts tokens and their durations, enabling efficient decoding:
    - LSTM prediction network maintains language context across token predictions.
    - Joint network combines encoder and decoder outputs.
    - Duration head predicts how many frames to skip, enabling fast inference.

The original implementation can be found in [NVIDIA NeMo](https://github.com/NVIDIA/NeMo).
Model checkpoints are to be found under [the NVIDIA organization](https://huggingface.co/nvidia/models?search=parakeet).

This model was contributed by [Nithin Rao Koluguri](https://huggingface.co/nithinraok), [Eustache Le Bihan](https://huggingface.co/eustlb), [Eric Bezzam](https://huggingface.co/bezzam), [Maksym Lypivskyi](https://huggingface.co/MaksL), and [Hainan Xu](https://huggingface.co/hainanx).

## Usage

### `ParakeetForCTC` usage

```python
from transformers import pipeline

pipe = pipeline("automatic-speech-recognition", model="nvidia/parakeet-ctc-1.1b")
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
# {'text': 'yesterday it was thirty five degrees in barcelona but today the temperature will go down to minus twenty degrees'}
```

```python
from datasets import Audio, load_dataset

from transformers import AutoModelForCTC, AutoProcessor

model_id = "nvidia/parakeet-ctc-1.1b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForCTC.from_pretrained(model_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
outputs = model.generate(**inputs)
print(processor.decode(outputs))
```

### `ParakeetForRNNT` usage

Parakeet RNN-T transcribes without casing or punctuation (like CTC), and the model can also perform token timestamping.

```py
from transformers import pipeline

pipe = pipeline("automatic-speech-recognition", model="nvidia/parakeet-rnnt-0.6b", revision="refs/pr/4")
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
# {'text': 'yesterday it was thirty five degrees in barcelona but today the temperature will go down to minus twenty degrees'}
```

```py
from transformers import AutoModelForRNNT, AutoProcessor
from datasets import load_dataset, Audio

model_id = "nvidia/parakeet-rnnt-0.6b"
revision = "refs/pr/4"
processor = AutoProcessor.from_pretrained(model_id, revision=revision)
model = AutoModelForRNNT.from_pretrained(model_id, revision=revision, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
print(processor.decode(output.sequences, skip_special_tokens=True))
```

Unlike TDT (which predicts a per-token duration), each RNN-T token is emitted at a single encoder frame, so its start and end span exactly one frame.

```py
from datasets import Audio, load_dataset
from transformers import AutoModelForRNNT, AutoProcessor

model_id = "nvidia/parakeet-rnnt-0.6b"
revision = "refs/pr/4"
processor = AutoProcessor.from_pretrained(model_id, revision=revision)
model = AutoModelForRNNT.from_pretrained(model_id, revision=revision, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:1]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
decoded_output, decoded_timestamps = processor.decode(
    output.sequences,
    durations=output.durations,
    skip_special_tokens=True,
)
print("Transcription:", decoded_output)
print("\nTimestamped tokens:", decoded_timestamps)

"""
Transcription: ['mister quilter is the apostle of the middle classes and we are glad to welcome his gospel']

Timestamped tokens: [[{'token': 'm', 'start': 0.4, 'end': 0.48}, {'token': 'is', 'start': 0.56, 'end': 0.64}, {'token': 'ter', 'start': 0.56, 'end': 0.64}, {'token': ' qu', 'start': 0.72, 'end': 0.8}, {'token': 'il', 'start': 0.96, 'end': 1.04}, {'token': 'ter', 'start': 1.12, 'end': 1.2}, {'token': ' is', 'start': 1.36, 'end': 1.44}, {'token': ' the', 'start': 1.52, 'end': 1.6}, {'token': ' ap', 'start': 1.68, 'end': 1.76}, {'token': 'o', 'start': 1.76, 'end': 1.84}, {'token': 'st', 'start': 1.84, 'end': 1.92}, {'token': 'le', 'start': 2.0, 'end': 2.08}, {'token': ' of', 'start': 2.16, 'end': 2.24}, {'token': ' the', 'start': 2.24, 'end': 2.32}, {'token': ' m', 'start': 2.4, 'end': 2.48}, {'token': 'id', 'start': 2.48, 'end': 2.56}, {'token': 'd', 'start': 2.56, 'end': 2.64}, {'token': 'le', 'start': 2.56, 'end': 2.64}, {'token': ' cl', 'start': 2.8, 'end': 2.88}, {'token': 'ass', 'start': 2.88, 'end': 2.96}, {'token': 'es', 'start': 3.12, 'end': 3.2}, {'token': ' and', 'start': 3.28, 'end': 3.36}, {'token': ' we', 'start': 3.44, 'end': 3.52}, {'token': ' are', 'start': 3.6, 'end': 3.68}, {'token': ' gl', 'start': 3.84, 'end': 3.92}, {'token': 'ad', 'start': 3.92, 'end': 4.0}, {'token': ' to', 'start': 4.08, 'end': 4.16}, {'token': ' we', 'start': 4.24, 'end': 4.32}, {'token': 'l', 'start': 4.32, 'end': 4.4}, {'token': 'c', 'start': 4.4, 'end': 4.48}, {'token': 'ome', 'start': 4.48, 'end': 4.56}, {'token': ' his', 'start': 4.72, 'end': 4.8}, {'token': ' go', 'start': 4.96, 'end': 5.04}, {'token': 's', 'start': 5.04, 'end': 5.12}, {'token': 'pe', 'start': 5.2, 'end': 5.28}, {'token': 'l', 'start': 5.36, 'end': 5.44}]]
"""
```

### `ParakeetForTDT` usage

Parakeet TDT transcripts include casing, and the model can also perform token timestamping.

```py
from transformers import pipeline

pipe = pipeline("automatic-speech-recognition", model="nvidia/parakeet-tdt-0.6b-v3")
out = pipe("https://huggingface.co/datasets/hf-internal-testing/dummy-audio-samples/resolve/main/bcn_weather.mp3")
print(out)
# {'text': 'Yesterday it was 35 degrees in Barcelona, but today the temperature will go down to minus 20 degrees.'}
```

```py
from transformers import AutoModelForTDT, AutoProcessor
from datasets import load_dataset, Audio

model_id = "nvidia/parakeet-tdt-0.6b-v3"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTDT.from_pretrained(model_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
print(processor.decode(output.sequences, skip_special_tokens=True))
```

```py
from datasets import Audio, load_dataset
from transformers import AutoModelForTDT, AutoProcessor

model_id = "nvidia/parakeet-tdt-0.6b-v3"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTDT.from_pretrained(model_id, device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:1]]

inputs = processor(speech_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)
output = model.generate(**inputs, return_dict_in_generate=True)
decoded_output, decoded_timestamps = processor.decode(
    output.sequences,
    durations=output.durations,
    skip_special_tokens=True,
)
print("Transcription:", decoded_output)
print("\nTimestamped tokens:", decoded_timestamps)

"""
Transcription: ['mister Quilter is the apostle of the middle classes, and we are glad to welcome his gospel.']

Timestamped tokens: [[{'token': 'm', 'start': 0.24, 'end': 0.48}, {'token': 'ister', 'start': 0.48, 'end': 0.64}, {'token': 'Qu', 'start': 0.64, 'end': 0.88}, {'token': 'il', 'start': 0.88, 'end': 1.12}, {'token': 'ter', 'start': 1.12, 'end': 1.36}, {'token': 'is', 'start': 1.36, 'end': 1.44}, {'token': 'the', 'start': 1.44, 'end': 1.6}, {'token': 'ap', 'start': 1.6, 'end': 1.76}, {'token': 'ost', 'start': 1.76, 'end': 1.92}, {'token': 'le', 'start': 2.0, 'end': 2.16}, {'token': 'of', 'start': 2.16, 'end': 2.24}, {'token': 'the', 'start': 2.24, 'end': 2.4}, {'token': 'mid', 'start': 2.4, 'end': 2.48}, {'token': 'd', 'start': 2.48, 'end': 2.56}, {'token': 'le', 'start': 2.56, 'end': 2.64}, {'token': 'clas', 'start': 2.72, 'end': 2.88}, {'token': 's', 'start': 2.88, 'end': 3.04}, {'token': 'es', 'start': 3.04, 'end': 3.12}, {'token': ',', 'start': 3.12, 'end': 3.12}, {'token': 'and', 'start': 3.2800000000000002, 'end': 3.44}, {'token': 'we', 'start': 3.44, 'end': 3.6}, {'token': 'are', 'start': 3.6, 'end': 3.7600000000000002}, {'token': 'gl', 'start': 3.7600000000000002, 'end': 3.92}, {'token': 'ad', 'start': 3.92, 'end': 4.08}, {'token': 'to', 'start': 4.08, 'end': 4.24}, {'token': 'wel', 'start': 4.24, 'end': 4.4}, {'token': 'c', 'start': 4.4, 'end': 4.48}, {'token': 'ome', 'start': 4.48, 'end': 4.72}, {'token': 'his', 'start': 4.72, 'end': 4.96}, {'token': 'gos', 'start': 4.96, 'end': 5.12}, {'token': 'pel', 'start': 5.36, 'end': 5.6000000000000005}, {'token': '.', 'start': 5.6000000000000005, 'end': 5.6000000000000005}]]
"""
```

### Making The Model Go Brrr

Parakeet supports full-graph compilation with CUDA graphs! This optimization is most effective when you know the maximum audio length you want to transcribe. The key idea is using static input shapes to avoid recompilation. For example, if you know your audio will be under 30 seconds, you can use the processor to pad all inputs to 30 seconds, preparing consistent input features and attention masks. See the example below!

```python
import torch
from datasets import Audio, load_dataset

from transformers import AutoModelForCTC, AutoProcessor

processor = AutoProcessor.from_pretrained("nvidia/parakeet-ctc-1.1b")
model = AutoModelForCTC.from_pretrained("nvidia/parakeet-ctc-1.1b", device_map="auto")

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:5]]

# Compile the generate method with fullgraph and CUDA graphs
model.generate = torch.compile(model.generate, fullgraph=True, mode="reduce-overhead")

# let's define processor kwargs to pad to 30 seconds
processor_kwargs = {
    "padding": "max_length",
    "max_length": 30 * processor.feature_extractor.sampling_rate,
}

# Define a timing context using CUDA events
class TimerContext:
    def __init__(self, name="Execution"):
        self.name = name
        self.start_event = None
        self.end_event = None

    def __enter__(self):
        # Use CUDA events for more accurate GPU timing
        self.start_event = torch.cuda.Event(enable_timing=True)
        self.end_event = torch.cuda.Event(enable_timing=True)
        self.start_event.record()
        return self

    def __exit__(self, *args):
        self.end_event.record()
        torch.cuda.synchronize()
        elapsed_time = self.start_event.elapsed_time(self.end_event) / 1000.0
        print(f"{self.name} time: {elapsed_time:.4f} seconds")

inputs = processor(speech_samples[0], **processor_kwargs)
inputs.to(model.device, dtype=model.dtype)
print("\n" + "="*50)
print("First generation - compiling...")
# Generate with the compiled model
with TimerContext("First generation"):
    outputs = model.generate(**inputs)
print(processor.decode(outputs))

inputs = processor(speech_samples[1], **processor_kwargs)
inputs.to(model.device, dtype=model.dtype)
print("\n" + "="*50)
print("Second generation - recording CUDA graphs...")
with TimerContext("Second generation"):
    outputs = model.generate(**inputs)
print(processor.decode(outputs))

inputs = processor(speech_samples[2], **processor_kwargs)
inputs.to(model.device, dtype=model.dtype)
print("\n" + "="*50)
print("Third generation - fast !!!")
with TimerContext("Third generation"):
    outputs = model.generate(**inputs)
print(processor.decode(outputs))

inputs = processor(speech_samples[3], **processor_kwargs)
inputs.to(model.device, dtype=model.dtype)
print("\n" + "="*50)
print("Fourth generation - still fast !!!")
with TimerContext("Fourth generation"):
    outputs = model.generate(**inputs)
print(processor.decode(outputs))
```

### CTC Training

```python
from datasets import Audio, load_dataset
from transformers import AutoModelForCTC, AutoProcessor

model_id = "nvidia/parakeet-ctc-1.1b"
NUM_SAMPLES = 5

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForCTC.from_pretrained(model_id, device_map="auto")
model.train()

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:NUM_SAMPLES]]
text_samples = ds["text"][:NUM_SAMPLES]

# passing `text` to the processor will prepare inputs' `labels` key
inputs = processor(audio=speech_samples, text=text_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)

outputs = model(**inputs)
print("Loss:", outputs.loss.item())
outputs.loss.backward()
```

### RNN-T Training

> [!NOTE]
> Computing the RNN-T loss requires [torchaudio](https://pytorch.org/audio) (`pip install torchaudio`).

```py
from datasets import Audio, load_dataset
import torch
from transformers import AutoModelForRNNT, AutoProcessor

model_id = "nvidia/parakeet-rnnt-0.6b"
revision = "refs/pr/4"
NUM_SAMPLES = 4

processor = AutoProcessor.from_pretrained(model_id, revision=revision)
model = AutoModelForRNNT.from_pretrained(model_id, revision=revision,  device_map="auto")
model.train()

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:NUM_SAMPLES]]
text_samples = ds["text"][:NUM_SAMPLES]

# passing `text` to the processor will prepare inputs' `labels` key
inputs = processor(audio=speech_samples, text=text_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)

outputs = model(**inputs)
print("Loss:", outputs.loss.item())
outputs.loss.backward()
```

### TDT Training

```py
from datasets import Audio, load_dataset
import torch
from transformers import AutoModelForTDT, AutoProcessor

model_id = "nvidia/parakeet-tdt-0.6b-v3"
NUM_SAMPLES = 4

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForTDT.from_pretrained(model_id, device_map="auto")
model.train()

ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))
speech_samples = [el['array'] for el in ds["audio"][:NUM_SAMPLES]]
text_samples = ds["text"][:NUM_SAMPLES]

# passing `text` to the processor will prepare inputs' `labels` key
inputs = processor(audio=speech_samples, text=text_samples, sampling_rate=processor.feature_extractor.sampling_rate)
inputs.to(model.device, dtype=model.dtype)

outputs = model(**inputs)
print("Loss:", outputs.loss.item())
outputs.loss.backward()
```

## ParakeetTokenizer[[transformers.ParakeetTokenizer]]

Inherits all methods from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). Users should refer to this superclass for more information regarding those methods,
except for `_decode` which is overridden to adapt it to CTC decoding:
1. Group consecutive tokens
2. Filter out the blank token

## ParakeetFeatureExtractor[[transformers.models.parakeet.feature_extraction_parakeet._LazyModule.__getattr__..Placeholder]]

.Placeholder"} anchor={"transformers.models.parakeet.feature_extraction_parakeet._LazyModule.__getattr__..Placeholder"} parameters={[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]}>

.Placeholder.__call__"} parameters={[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]}>

Call self as a function.

## ParakeetProcessor[[transformers.ParakeetProcessor]]

'"}, {"name": "decoder_type", "val": " = None"}]}>
- **feature_extractor** (`feature_extractor_class`) --
  The feature extractor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **blank_token** (`str`, *optional*, defaults to `"<blank>"`) --
  Blank token for transducer decoding.
- **decoder_type** (`str`, *optional*) --
  Decoding/timestamp emission mode. Possible values:

  - `"ctc"`: Consecutive identical tokens are merged into one emission.
  - `"rnnt"`: Repeated tokens are kept; each token gets a 1-frame timestamp span.
  - `"tdt"`: Repeated tokens are kept; each token span is based on its predicted duration. Punctuation is attached to the preceding token.

  If `None` (older checkpoints) the decoder type is inferred automatically for backward compatibility.
Constructs a ParakeetProcessor which wraps a feature extractor and a tokenizer into a single processor.

[ParakeetProcessor](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetProcessor) offers all the functionalities of `feature_extractor_class` and `tokenizer_class`. See the
`~feature_extractor_class` and `~tokenizer_class` for more information.

- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **sampling_rate** (`int`, *optional*) --
  The sampling rate of the input audio in Hz. This should match the sampling rate expected by the feature
  extractor (defaults to 16000 Hz). If provided, it will be validated against the processor's expected
  sampling rate, and an error will be raised if they don't match. If not provided, a warning will be
  issued and the default sampling rate will be assumed.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

Forward arguments to [decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode) and post-process the timestamps (if provided for TDT) as
in the NeMo library.

## ParakeetEncoderConfig[[transformers.ParakeetEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **convolution_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in convolutions of the conformer's convolution module.
- **conv_kernel_size** (`int`, *optional*, defaults to 9) --
  The kernel size of the convolution layers in the Conformer block.
- **subsampling_factor** (`int`, *optional*, defaults to 8) --
  The factor by which the input sequence is subsampled.
- **subsampling_conv_channels** (`int`, *optional*, defaults to 256) --
  The number of channels in the subsampling convolution layers.
- **num_mel_bins** (`int`, *optional*, defaults to 80) --
  Number of mel features.
- **subsampling_conv_kernel_size** (`int`, *optional*, defaults to 3) --
  The kernel size of the subsampling convolution layers.
- **subsampling_conv_stride** (`int`, *optional*, defaults to 2) --
  The stride of the subsampling convolution layers.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **dropout_positions** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the positions in the input sequence.
- **layerdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for
  more details.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for activations inside the fully connected layer.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `5000`) --
  The maximum sequence length that this model might ever be used with.
- **scale_input** (`bool`, *optional*, defaults to `True`) --
  Whether to scale the input embeddings.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a ParakeetModel. It is used to instantiate a Parakeet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/parakeet-ctc-1.1b](https://huggingface.co/nvidia/parakeet-ctc-1.1b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ParakeetEncoderModel, ParakeetEncoderConfig

>>> # Initializing a `ParakeetEncoder` configuration
>>> configuration = ParakeetEncoderConfig()

>>> # Initializing a model from the configuration
>>> model = ParakeetEncoderModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ParakeetCTCConfig[[transformers.ParakeetCTCConfig]]

- **vocab_size** (`int`, *optional*, defaults to `1025`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **ctc_loss_reduction** (`str`, *optional*, defaults to `"mean"`) --
  Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training an
  instance of [ParakeetForCTC](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetForCTC).
- **ctc_zero_infinity** (`bool`, *optional*, defaults to `True`) --
  Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly
  occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance
  of [ParakeetForCTC](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetForCTC).
- **encoder_config** (`Union[dict, ParakeetEncoderConfig]`, *optional*) --
  The config object or dictionary of the encoder.
- **pad_token_id** (`int`, *optional*, defaults to `1024`) --
  Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a ParakeetModel. It is used to instantiate a Parakeet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/parakeet-ctc-1.1b](https://huggingface.co/nvidia/parakeet-ctc-1.1b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ParakeetForCTC, ParakeetCTCConfig
>>> # Initializing a Parakeet configuration
>>> configuration = ParakeetCTCConfig()
>>> # Initializing a model from the configuration
>>> model = ParakeetForCTC(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## ParakeetRNNTConfig[[transformers.ParakeetRNNTConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **vocab_size** (`int`, *optional*, defaults to `8193`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **decoder_hidden_size** (`int`, *optional*, defaults to 640) --
  Hidden size of the LSTM prediction network and joint network.
- **num_decoder_layers** (`int`, *optional*, defaults to 2) --
  Number of LSTM layers in the prediction network.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_symbols_per_step** (`int`, *optional*, defaults to 10) --
  Maximum number of symbols to emit per encoder time step during greedy decoding.
- **encoder_config** (`Union[dict, ParakeetEncoderConfig]`, *optional*) --
  The config object or dictionary of the encoder.
- **pad_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for padding in the vocabulary.
- **blank_token_id** (`int`, *optional*, defaults to 8192) --
  Blank token id. Different from `pad_token_id` for RNN-T.

This is the configuration class to store the configuration of a ParakeetModel. It is used to instantiate a Parakeet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/parakeet-rnnt-0.6b](https://huggingface.co/nvidia/parakeet-rnnt-0.6b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ParakeetForRNNT, ParakeetRNNTConfig

>>> # Initializing a Parakeet RNN-T configuration
>>> configuration = ParakeetRNNTConfig()

>>> # Initializing a model from the configuration
>>> model = ParakeetForRNNT(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ParakeetTDTConfig[[transformers.ParakeetTDTConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **vocab_size** (`int`, *optional*, defaults to `8193`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **decoder_hidden_size** (`int`, *optional*, defaults to 640) --
  Hidden size of the LSTM prediction network and joint network.
- **num_decoder_layers** (`int`, *optional*, defaults to 2) --
  Number of LSTM layers in the prediction network.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_symbols_per_step** (`int`, *optional*, defaults to 10) --
  Maximum number of symbols to emit per encoder time step during greedy decoding.
- **encoder_config** (`Union[dict, ParakeetEncoderConfig]`, *optional*) --
  The config object or dictionary of the encoder.
- **pad_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for padding in the vocabulary.
- **blank_token_id** (`int`, *optional*, defaults to 8192) --
  Blank token id. Different from `pad_token_id` for TDT.
- **durations** (`list[int]`, *optional*, defaults to `[0, 1, 2, 3, 4]`) --
  Token duration values that can be predicted. Each value represents how many frames a token or blank
  emission spans.

This is the configuration class to store the configuration of a ParakeetModel. It is used to instantiate a Parakeet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/parakeet-tdt-0.6b-v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ParakeetForTDT, ParakeetTDTConfig

>>> # Initializing a Parakeet TDT configuration
>>> configuration = ParakeetTDTConfig()

>>> # Initializing a model from the configuration
>>> model = ParakeetForTDT(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ParakeetEncoder[[transformers.ParakeetEncoder]]

- **config** ([ParakeetEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetEncoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Parakeet Encoder model, based on the [Fast Conformer architecture](https://huggingface.co/papers/2305.05084).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_attention_mask", "val": ": bool = True"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_features** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **output_attention_mask** (`bool`, *optional*, defaults to `True`) --
  Whether to return the output attention mask. Only effective when `attention_mask` is provided.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ParakeetEncoder](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetEncoder) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import AutoProcessor, ParakeetEncoder
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/parakeet-ctc-1.1b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> encoder = ParakeetEncoder.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> encoder_outputs = encoder(**inputs)

>>> print(encoder_outputs.last_hidden_state.shape)
```

## ParakeetForCTC[[transformers.ParakeetForCTC]]

- **config** ([ParakeetCTCConfig](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetCTCConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Parakeet Encoder with a Connectionist Temporal Classification (CTC) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "labels", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_features** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.[CausalLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or `tuple(torch.FloatTensor)`A [CausalLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ParakeetForCTC](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetForCTC) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoProcessor, ParakeetForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/parakeet-ctc-1.1b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = ParakeetForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], text=ds[0]["text"])
>>> outputs = model(**inputs)

>>> print(outputs.loss)
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "return_dict_in_generate", "val": ": bool = False"}, {"name": "compile_config", "val": ": transformers.generation.configuration_utils.CompileConfig | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>

compile_config ([CompileConfig](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.CompileConfig), *optional*):
If provided, `torch.compile` will be applied to the forward calls in the decoding loop.

Example:

```python
>>> from transformers import AutoProcessor, ParakeetForCTC
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/parakeet-ctc-1.1b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = ParakeetForCTC.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"], text=ds[0]["text"])
>>> predicted_ids = model.generate(**inputs)
>>> transcription = processor.batch_decode(predicted_ids, skip_special_tokens=True)

>>> print(transcription)
```

## ParakeetForRNNT[[transformers.ParakeetForRNNT]]

- **config** ([ParakeetRNNTConfig](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetRNNTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Parakeet Encoder with an RNN-T (RNN Transducer) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_features** (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, 1)`, *optional*) --
  Decoder input token ids for single-step inference.
- **decoder_cache** (`ParakeetRNNTDecoderCache`, *optional*) --
  Decoder LSTM cache. When provided and initialized, the cached `decoder_output` is reused
  (e.g. during blank-skipping) instead of running the decoder. When `input_ids` is provided,
  the decoder runs and the cache is updated in-place.
- **use_decoder_cache** (`bool`, *optional*) --
  Whether to use a decoder cache. When `True` and `decoder_cache` is `None`, a new cache
  is created automatically during the forward pass.
- **encoder_outputs** (`tuple(torch.FloatTensor)`, *optional*) --
  Pre-computed encoder outputs (last_hidden_state, pooler_output, hidden_states, attentions, attention_mask).
  Can be a tuple or `ParakeetEncoderModelOutput`.
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.`ParakeetRNNTOutput` or `tuple(torch.FloatTensor)`A `ParakeetRNNTOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ParakeetForRNNT](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetForRNNT) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*) -- RNN-T loss, returned when `labels` are provided.
- **logits** (`torch.FloatTensor`) -- Joint token logits. Shape is `(batch, T, U+1, vocab)` for training
  or `(batch, 1, 1, vocab)` for single-step inference.
- **decoder_cache** (`ParakeetRNNTDecoderCache`, *optional*) -- Decoder LSTM cache containing hidden state, cell state, and last output.

Example:

```python
>>> from transformers import AutoProcessor, ParakeetForRNNT
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/parakeet-rnnt-0.6b"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = ParakeetForRNNT.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> outputs = model(**inputs)
```

## ParakeetForTDT[[transformers.ParakeetForTDT]]

- **config** ([ParakeetTDTConfig](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetTDTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Parakeet Encoder with a TDT (Token Duration Transducer) head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_features** (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details (`processor_class` uses
  `feature_extractor_class` for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, 1)`, *optional*) --
  Decoder input token ids for single-step inference.
- **decoder_cache** (`ParakeetRNNTDecoderCache`, *optional*) --
  Decoder LSTM cache. When provided and initialized, the cached `decoder_output` is reused
  (e.g. during blank-skipping) instead of running the decoder. When `input_ids` is provided,
  the decoder runs and the cache is updated in-place.
- **use_decoder_cache** (`bool`, *optional*) --
  Whether to use a decoder cache. When `True` and `decoder_cache` is `None`, a new cache
  is created automatically during the forward pass.
- **encoder_outputs** (`tuple(torch.FloatTensor)`, *optional*) --
  Pre-computed encoder outputs (last_hidden_state, pooler_output, hidden_states, attentions, attention_mask).
  Can be a tuple or `ParakeetEncoderModelOutput`.
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.`ParakeetRNNTOutput` or `tuple(torch.FloatTensor)`A `ParakeetRNNTOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [ParakeetForTDT](/docs/transformers/v5.14.0/en/model_doc/parakeet#transformers.ParakeetForTDT) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*) -- RNN-T loss, returned when `labels` are provided.
- **logits** (`torch.FloatTensor`) -- Joint token logits. Shape is `(batch, T, U+1, vocab)` for training
  or `(batch, 1, 1, vocab)` for single-step inference.
- **decoder_cache** (`ParakeetRNNTDecoderCache`, *optional*) -- Decoder LSTM cache containing hidden state, cell state, and last output.

Example:

```python
>>> from transformers import AutoProcessor, ParakeetForTDT
>>> from datasets import load_dataset, Audio

>>> model_id = "nvidia/parakeet-tdt-0.6b-v3"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = ParakeetForTDT.from_pretrained(model_id)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=processor.feature_extractor.sampling_rate))

>>> inputs = processor(ds[0]["audio"]["array"])
>>> outputs = model(**inputs)
```
