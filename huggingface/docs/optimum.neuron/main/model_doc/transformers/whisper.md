# Whisper

## Overview

[Whisper](https://hf.co/papers/2212.04356) is a encoder-decoder (sequence-to-sequence) transformer pretrained on 680,000 hours of labeled audio data. This amount of pretraining data enables zero-shot performance on audio tasks in English and many other languages. The decoder allows Whisper to map the encoders learned speech representations to useful outputs, such as text, without additional fine-tuning. Whisper just works out of the box.

You can find all the original Whisper checkpoints under the [Whisper](https://huggingface.co/collections/openai/whisper-release-6501bba2cf999715fd953013) collection.

## Export to Neuron

To deploy 🤗 [Transformers](https://huggingface.co/docs/transformers/index) models on Neuron devices, you first need to compile the models and export them to a serialized format for inference. Below are two approaches to compile the model, you can choose the one that best suits your needs:

### Option 1: CLI

You can export the model using the Optimum command-line interface as follows:

```bash
optimum-cli export neuron --model openai/whisper-tiny --task automatic-speech-recognition --batch_size 1 --sequence_length 128 --auto_cast all --auto_cast_type bf16 whisper_tiny_neuronx/
```

> [!TIP]
> Execute `optimum-cli export neuron --help` to display all command line options and their description.

### Option 2: Python API

```python
from optimum.neuron import NeuronWhisperForConditionalGeneration

compiler_args = {"auto_cast": "all", "auto_cast_type": "bf16"}
input_shapes = {"batch_size": 1, "sequence_length": 128}
neuron_model = NeuronWhisperForConditionalGeneration.from_pretrained(
    "openai/whisper-tiny",
    export=True,
    inline_weights_to_neff=False,
    **compiler_args,
    **input_shapes,
)
# Save locally
neuron_model.save_pretrained("whisper_tiny_neuronx")

# Upload to the HuggingFace Hub
neuron_model.push_to_hub(
    "whisper_tiny_neuronx", repository_id="my-neuron-repo"  # Replace with your repo id, eg. "Jingya/whisper_tiny_neuronx"
)
```

## Usage Example

To use the model that we just exported, there are two options. We can eithe use the [NeuronWhisperForConditionalGeneration](/docs/optimum.neuron/main/en/model_doc/transformers/whisper#optimum.neuron.NeuronWhisperForConditionalGeneration) class or use the `Pipeline`. The example below demonstrates how to automatically transcribe speech into text these two approaches.

### With `NeuronWhisperForConditionalGeneration`

```python
from datasets import load_dataset
from transformers import AutoProcessor
from optimum.neuron import NeuronWhisperForConditionalGeneration

# Select an audio file and read it:
ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
audio_sample = ds[0]["audio"]

# Use the model and processor to transcribe the audio:
processor = AutoProcessor.from_pretrained("Jingya/whisper_tiny_neuronx")
input_features = processor(
    audio_sample["array"], sampling_rate=audio_sample["sampling_rate"], return_tensors="pt"
).input_features

# Inference
neuron_model = NeuronWhisperForConditionalGeneration.from_pretrained("Jingya/whisper_tiny_neuronx")
predicted_ids = neuron_model.generate(input_features)
transcription = processor.batch_decode(predicted_ids, skip_special_tokens=True)
#  Mr. Quilter is the apostle of the middle classes and we are glad to welcome his gospel.
```

### With `pipeline`

```python
from transformers import AutoProcessor
from optimum.neuron import NeuronWhisperForConditionalGeneration, pipeline

processor = AutoProcessor.from_pretrained("Jingya/whisper_tiny_neuronx")
neuron_model = NeuronWhisperForConditionalGeneration.from_pretrained("Jingya/whisper_tiny_neuronx")

pipeline = pipeline(
    task="automatic-speech-recognition",
    model=neuron_model,
    tokenizer=processor.tokenizer,
    feature_extractor=processor.feature_extractor,
)
pipeline("https://huggingface.co/datasets/Narsil/asr_dummy/resolve/main/mlk.flac")
#  I have a dream. Good one day. This nation will rise up. Live out the true meaning of its dream.
```

## NeuronWhisperForConditionalGeneration[[optimum.neuron.NeuronWhisperForConditionalGeneration]]

#### optimum.neuron.NeuronWhisperForConditionalGeneration[[optimum.neuron.NeuronWhisperForConditionalGeneration]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/whisper/modeling_whisper.py#L132)

Whisper Neuron model with a language modeling head that can be used for automatic speech recognition.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronWhisperForConditionalGeneration.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/whisper/modeling_whisper.py#L190[{"name": "input_features", "val": ": torch.FloatTensor | None = None"}, {"name": "decoder_input_ids", "val": ": torch.LongTensor | None = None"}, {"name": "encoder_outputs", "val": ": tuple[torch.FloatTensor] | None = None"}, {"name": "**kwargs", "val": ""}]- **input_features** (`torch.FloatTensor | None` of shape `(batch_size, feature_size, sequence_length)`) --
  Float values mel features extracted from the raw speech waveform. Raw speech waveform can be obtained by
  loading a `.flac` or `.wav` audio file into an array of type `list[float]` or a `numpy.ndarray`, *e.g.* via
  the soundfile library (`pip install soundfile`). To prepare the array into `input_features`, the
  `AutoFeatureExtractor` should be used for extracting the mel features, padding and conversion into a
  tensor of type `torch.FloatTensor`. See `~WhisperFeatureExtractor.__call__`
- **decoder_input_ids** (`torch.LongTensor | None` of shape `(batch_size, max_sequence_length)`) --
  Indices of decoder input sequence tokens in the vocabulary. Indices can be obtained using `WhisperTokenizer`.
  See `PreTrainedTokenizer.encode` and `PreTrainedTokenizer.__call__` for details. Since the cache is not yet
  supported for Whisper, it needs to be padded to the `sequence_length` used for the compilation.
- **encoder_outputs** (`tuple[torch.FloatTensor | None]`) --
  Tuple consists of `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.0
The `NeuronWhisperForConditionalGeneration` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

**Parameters:**

encoder (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module of the encoder with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

decoder (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module of the decoder with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.
