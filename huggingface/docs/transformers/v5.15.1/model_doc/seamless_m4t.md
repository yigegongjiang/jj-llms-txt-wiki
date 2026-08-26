# SeamlessM4T

## Overview

The SeamlessM4T model was proposed in [SeamlessM4T — Massively Multilingual & Multimodal Machine Translation](https://huggingface.co/papers/2308.11596) by the Seamless Communication team from Meta AI.

This is the **version 1** release of the model. For the updated **version 2** release, refer to the [Seamless M4T v2 docs](https://huggingface.co/docs/transformers/main/model_doc/seamless_m4t_v2).

SeamlessM4T is a collection of models designed to provide high quality translation, allowing people from different linguistic communities to communicate effortlessly through speech and text.

SeamlessM4T enables multiple tasks without relying on separate models:

- Speech-to-speech translation (S2ST)
- Speech-to-text translation (S2TT)
- Text-to-speech translation (T2ST)
- Text-to-text translation (T2TT)
- Automatic speech recognition (ASR)

[SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel) can perform all the above tasks, but each task also has its own dedicated sub-model.

The abstract from the paper is the following:

*What does it take to create the Babel Fish, a tool that can help individuals translate speech between any two languages? While recent breakthroughs in text-based models have pushed machine translation coverage beyond 200 languages, unified speech-to-speech translation models have yet to achieve similar strides. More specifically, conventional speech-to-speech translation systems rely on cascaded systems that perform translation progressively, putting high-performing unified systems out of reach. To address these gaps, we introduce SeamlessM4T, a single model that supports speech-to-speech translation, speech-to-text translation, text-to-speech translation, text-to-text translation, and automatic speech recognition for up to 100 languages. To build this, we used 1 million hours of open speech audio data to learn self-supervised speech representations with w2v-BERT 2.0. Subsequently, we created a multimodal corpus of automatically aligned speech translations. Filtered and combined with human-labeled and pseudo-labeled data, we developed the first multilingual system capable of translating from and into English for both speech and text. On FLEURS, SeamlessM4T sets a new standard for translations into multiple target languages, achieving an improvement of 20% BLEU over the previous SOTA in direct speech-to-text translation. Compared to strong cascaded models, SeamlessM4T improves the quality of into-English translation by 1.3 BLEU points in speech-to-text and by 2.6 ASR-BLEU points in speech-to-speech. Tested for robustness, our system performs better against background noises and speaker variations in speech-to-text tasks compared to the current SOTA model. Critically, we evaluated SeamlessM4T on gender bias and added toxicity to assess translation safety. Finally, all contributions in this work are open-sourced and accessible at https://github.com/facebookresearch/seamless_communication*

## Usage

First, load the processor and a checkpoint of the model:

```python
from transformers import AutoProcessor, SeamlessM4TModel

processor = AutoProcessor.from_pretrained("facebook/hf-seamless-m4t-medium")
model = SeamlessM4TModel.from_pretrained("facebook/hf-seamless-m4t-medium", device_map="auto")
```

You can seamlessly use this model on text or on audio, to generate either translated text or translated audio.

Here is how to use the processor to process text and audio:

```python
# let's load an audio sample from an Arabic speech corpus
from datasets import load_dataset

dataset = load_dataset("halabi2016/arabic_speech_corpus", split="test", streaming=True)
audio_sample = next(iter(dataset))["audio"]

# now, process it
audio_inputs = processor(audio=audio_sample["array"], return_tensors="pt").to(model.device)

# now, process some English test as well
text_inputs = processor(text = "Hello, my dog is cute", src_lang="eng", return_tensors="pt").to(model.device)
```

### Speech

[SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel) can *seamlessly* generate text or speech with few or no changes. Let's target Russian voice translation:

```python
audio_array_from_text = model.generate(**text_inputs, tgt_lang="rus")[0].cpu().numpy().squeeze()
audio_array_from_audio = model.generate(**audio_inputs, tgt_lang="rus")[0].cpu().numpy().squeeze()
```

With basically the same code, I've translated English text and Arabic speech to Russian speech samples.

### Text

Similarly, you can generate translated text from audio files or from text with the same model. You only have to pass `generate_speech=False` to [SeamlessM4TModel.generate()](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel.generate).
This time, let's translate to French.

```python
# from audio
output_tokens = model.generate(**audio_inputs, tgt_lang="fra", generate_speech=False)
translated_text_from_audio = processor.decode(output_tokens[0].tolist()[0], skip_special_tokens=True)

# from text
output_tokens = model.generate(**text_inputs, tgt_lang="fra", generate_speech=False)
translated_text_from_text = processor.decode(output_tokens[0].tolist()[0], skip_special_tokens=True)
```

### Tips

#### 1. Use dedicated models

[SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel) is transformers top level model to generate speech and text, but you can also use dedicated models that perform the task without additional components, thus reducing the memory footprint.
For example, you can replace the audio-to-audio generation snippet with the model dedicated to the S2ST task, the rest is exactly the same code:

```python
from transformers import SeamlessM4TForSpeechToSpeech

model = SeamlessM4TForSpeechToSpeech.from_pretrained("facebook/hf-seamless-m4t-medium", device_map="auto")
```

Or you can replace the text-to-text generation snippet with the model dedicated to the T2TT task, you only have to remove `generate_speech=False`.

```python
from transformers import SeamlessM4TForTextToText

model = SeamlessM4TForTextToText.from_pretrained("facebook/hf-seamless-m4t-medium", device_map="auto")
```

Feel free to try out [SeamlessM4TForSpeechToText](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForSpeechToText) and [SeamlessM4TForTextToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForTextToSpeech) as well.

#### 2. Change the speaker identity

You have the possibility to change the speaker used for speech synthesis with the `spkr_id` argument. Some `spkr_id` works better than other for some languages!

#### 3. Change the generation strategy

You can use different [generation strategies](../generation_strategies) for speech and text generation, e.g `.generate(input_ids=input_ids, text_num_beams=4, speech_do_sample=True)` which will successively perform beam-search decoding on the text model, and multinomial sampling on the speech model.

#### 4. Generate speech and text at the same time

Use `return_intermediate_token_ids=True` with [SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel) to return both speech and text !

## Model architecture

SeamlessM4T features a versatile architecture that smoothly handles the sequential generation of text and speech. This setup comprises two sequence-to-sequence (seq2seq) models. The first model translates the input modality into translated text, while the second model generates speech tokens, known as "unit tokens," from the translated text.

Each modality has its own dedicated encoder with a unique architecture. Additionally, for speech output, a vocoder inspired by the [HiFi-GAN](https://huggingface.co/papers/2010.05646) architecture is placed on top of the second seq2seq model.

Here's how the generation process works:

- Input text or speech is processed through its specific encoder.
- A decoder creates text tokens in the desired language.
- If speech generation is required, the second seq2seq model, following a standard encoder-decoder structure, generates unit tokens.
- These unit tokens are then passed through the final vocoder to produce the actual speech.

This model was contributed by [ylacombe](https://huggingface.co/ylacombe). The original code can be found [here](https://github.com/facebookresearch/seamless_communication).

## SeamlessM4TModel[[transformers.SeamlessM4TModel]]

#### transformers.SeamlessM4TModel[[transformers.SeamlessM4TModel]]

```python
transformers.SeamlessM4TModel(config, current_modality = 'text')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L3581)

**Parameters:**

config ([SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

current_modality (`str`, *optional*, defaults to `"text"`) : Default modality. Used to initialize the model.

The original SeamlessM4T Model transformer which can be used for every tasks available (S2ST, S2TT, T2TT, T2ST).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### generate[[transformers.SeamlessM4TModel.generate]]

```python
generate(input_ids: typing.Optional[torch.Tensor] = None, input_features: typing.Optional[torch.Tensor] = None, return_intermediate_token_ids: bool | None = None, tgt_lang: str | None = None, spkr_id: int | None = 0, generate_speech: bool | None = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L3787)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SeamlessM4TTokenizer](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTokenizer) or [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_banks)`, *optional*) : Input audio features. This should be returned by the [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) class or the [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor) class. See [SeamlessM4TFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor.__call__) for details.

return_intermediate_token_ids (`bool`, *optional*) : If `True`, also returns the intermediate generated text and unit tokens. Set to `True` if you also want to get translated text alongside the audio. Note that if `generate_speech=False`, this parameter will be ignored and the text tokens are returned.

tgt_lang (`str`, *optional*) : The language to use as target language for translation.

spkr_id (`int`, *optional*, defaults to 0) : The id of the speaker used for speech synthesis. Must be lower than `config.vocoder_num_spkrs`.

generate_speech (`bool`, *optional*, defaults to `True`) : If `False`, will only returns the text tokens and won't generate speech. 

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to [GenerationMixin.generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate). Keyword arguments are of two types:  - Without a prefix, they will be entered as `**kwargs` for the `generate` method of each sub-model, except for `decoder_input_ids` which will only be passed through the text components. - With a *text_* or *speech_* prefix, they will be input for the `generate` method of the text model and speech model respectively. It has the priority over the keywords without a prefix.  This means you can, for example, specify a generation strategy for one generation but not for the other.

**Returns:** `Union[SeamlessM4TGenerationOutput, tuple[Tensor], ModelOutput]`

- If `generate_speech` and `return_intermediate_token_ids`, returns `SeamlessM4TGenerationOutput`.
- If `generate_speech` and not `return_intermediate_token_ids`, returns a tuple composed of waveforms of
  shape `(batch_size, sequence_length)` and `waveform_lengths` which gives the length of each sample.
- If `generate_speech=False`, it will returns `ModelOutput`.

Generates translated token ids and/or translated audio waveforms.

This method successively calls the `.generate` function of two different sub-models. You can specify keyword
arguments at two different levels: general arguments that will be passed to both models, or prefixed arguments
that will be passed to one of them.

For example, calling `.generate(input_ids=input_ids, num_beams=4, speech_do_sample=True)` will successively
perform beam-search decoding on the text model, and multinomial beam-search sampling on the speech model.

For an overview of generation strategies and code examples, check out the [following
guide](./generation_strategies).

## SeamlessM4TForTextToSpeech[[transformers.SeamlessM4TForTextToSpeech]]

#### transformers.SeamlessM4TForTextToSpeech[[transformers.SeamlessM4TForTextToSpeech]]

```python
transformers.SeamlessM4TForTextToSpeech(config: SeamlessM4TConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2941)

**Parameters:**

config ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The text-to-speech SeamlessM4T Model transformer which can be used for T2ST.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### generate[[transformers.SeamlessM4TForTextToSpeech.generate]]

```python
generate(input_ids: typing.Optional[torch.Tensor] = None, return_intermediate_token_ids: bool | None = None, tgt_lang: str | None = None, spkr_id: int | None = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L3085)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SeamlessM4TTokenizer](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTokenizer) or [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

return_intermediate_token_ids (`bool`, *optional*) : If `True`, also returns the intermediate generated text and unit tokens. Set to `True` if you also want to get translated text alongside the audio.

tgt_lang (`str`, *optional*) : The language to use as target language for translation.

spkr_id (`int`, *optional*, defaults to 0) : The id of the speaker used for speech synthesis. Must be lower than `config.vocoder_num_spkrs`.

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to [GenerationMixin.generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate). Keyword arguments are of two types:  - Without a prefix, they will be entered as `**kwargs` for the `generate` method of each sub-model, except for `decoder_input_ids` which will only be passed through the text components. - With a *text_* or *speech_* prefix, they will be input for the `generate` method of the text model and speech model respectively. It has the priority over the keywords without a prefix.  This means you can, for example, specify a generation strategy for one generation but not for the other.

**Returns:** `Union[SeamlessM4TGenerationOutput, tuple[Tensor]]`

- If `return_intermediate_token_ids`, returns `SeamlessM4TGenerationOutput`.
- If not `return_intermediate_token_ids`, returns a tuple composed of waveforms of shape `(batch_size,
  sequence_length)` and `waveform_lengths` which gives the length of each sample.

Generates translated audio waveforms.

This method successively calls the `.generate` function of two different sub-models. You can specify keyword
arguments at two different levels: general arguments that will be passed to both models, or prefixed arguments
that will be passed to one of them.

For example, calling `.generate(input_ids, num_beams=4, speech_do_sample=True)` will successively perform
beam-search decoding on the text model, and multinomial beam-search sampling on the speech model.

For an overview of generation strategies and code examples, check out the [following
guide](./generation_strategies).

## SeamlessM4TForSpeechToSpeech[[transformers.SeamlessM4TForSpeechToSpeech]]

#### transformers.SeamlessM4TForSpeechToSpeech[[transformers.SeamlessM4TForSpeechToSpeech]]

```python
transformers.SeamlessM4TForSpeechToSpeech(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L3258)

**Parameters:**

config ([SeamlessM4TForSpeechToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForSpeechToSpeech)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The speech-to-speech SeamlessM4T Model transformer which can be used for S2ST.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### generate[[transformers.SeamlessM4TForSpeechToSpeech.generate]]

```python
generate(input_features: typing.Optional[torch.Tensor] = None, return_intermediate_token_ids: bool | None = None, tgt_lang: str | None = None, spkr_id: int | None = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L3401)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_banks)`) : Input audio features. This should be returned by the [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) class or the [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor) class. See [SeamlessM4TFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor.__call__) for details.

return_intermediate_token_ids (`bool`, *optional*) : If `True`, also returns the intermediate generated text and unit tokens. Set to `True` if you also want to get translated text alongside the audio.

tgt_lang (`str`, *optional*) : The language to use as target language for translation.

spkr_id (`int`, *optional*, defaults to 0) : The id of the speaker used for speech synthesis. Must be lower than `config.vocoder_num_spkrs`. 

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to [GenerationMixin.generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate). Keyword arguments are of two types:  - Without a prefix, they will be entered as `**kwargs` for the `generate` method of each sub-model, except for `decoder_input_ids` which will only be passed through the text components. - With a *text_* or *speech_* prefix, they will be input for the `generate` method of the text model and speech model respectively. It has the priority over the keywords without a prefix.  This means you can, for example, specify a generation strategy for one generation but not for the other.

**Returns:** `Union[SeamlessM4TGenerationOutput, tuple[Tensor]]`

- If `return_intermediate_token_ids`, returns `SeamlessM4TGenerationOutput`.
- If not `return_intermediate_token_ids`, returns a tuple composed of waveforms of shape `(batch_size,
  sequence_length)` and `waveform_lengths` which gives the length of each sample.

Generates translated audio waveforms.

This method successively calls the `.generate` function of two different sub-models. You can specify keyword
arguments at two different levels: general arguments that will be passed to both models, or prefixed arguments
that will be passed to one of them.

For example, calling `.generate(input_features, num_beams=4, speech_do_sample=True)` will successively perform
beam-search decoding on the text model, and multinomial beam-search sampling on the speech model.

For an overview of generation strategies and code examples, check out the [following
guide](./generation_strategies).

## SeamlessM4TForTextToText[[transformers.SeamlessM4TForTextToText]]

#### transformers.SeamlessM4TForTextToText[[transformers.SeamlessM4TForTextToText]]

```python
transformers.SeamlessM4TForTextToText(config: SeamlessM4TConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2433)

**Parameters:**

config ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The text-to-text SeamlessM4T Model transformer which can be used for T2TT.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SeamlessM4TForTextToText.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2469)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  Bart uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).  For translation and summarization training, `decoder_input_ids` should be provided. If no `decoder_input_ids` is provided, the model will create this tensor by shifting the `input_ids` to the right for denoising pre-training following the paper.

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `modeling_bart._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape`(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*): Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) and inputs.

The [SeamlessM4TForTextToText](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForTextToText) forward method, overrides the `__call__` special method.

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

#### generate[[transformers.SeamlessM4TForTextToText.generate]]

```python
generate(input_ids = None, tgt_lang = None, generation_config = None, logits_processor = None, stopping_criteria = None, prefix_allowed_tokens_fn = None, synced_gpus = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2567)

**Parameters:**

input_ids (`torch.Tensor` of varying shape depending on the modality, *optional*) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SeamlessM4TTokenizer](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTokenizer) or [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

tgt_lang (`str`, *optional*) : The language to use as target language for translation.

generation_config (`~generation.GenerationConfig`, *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which had the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

logits_processor (`LogitsProcessorList`, *optional*) : Custom logits processors that complement the default logits processors built from arguments and generation config. If a logit processor is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

stopping_criteria (`StoppingCriteriaList`, *optional*) : Custom stopping criteria that complement the default stopping criteria built from arguments and a generation config. If a stopping criteria is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

prefix_allowed_tokens_fn (`Callable[[int, torch.Tensor], list[int]]`, *optional*) : If provided, this function constraints the beam search to allowed tokens only at each step. If not provided no constraint is applied. This function takes 2 arguments: the batch ID `batch_id` and `input_ids`. It has to return a list with the allowed tokens for the next generation step conditioned on the batch ID `batch_id` and the previously generated tokens `inputs_ids`. This argument is useful for constrained generation conditioned on the prefix, as described in [Autoregressive Entity Retrieval](https://huggingface.co/papers/2010.00904).

synced_gpus (`bool`, *optional*, defaults to `False`) : Whether to continue running the while loop until max_length (needed to avoid deadlocking with `FullyShardedDataParallel` and DeepSpeed ZeRO Stage 3).

kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of `generate_config` and/or additional model-specific kwargs that will be forwarded to the `forward` function of the model.

**Returns:** [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`

A [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.FloatTensor`. The possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:
- [GenerateEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](./generation_strategies).

## SeamlessM4TForSpeechToText[[transformers.SeamlessM4TForSpeechToText]]

#### transformers.SeamlessM4TForSpeechToText[[transformers.SeamlessM4TForSpeechToText]]

```python
transformers.SeamlessM4TForSpeechToText(config: SeamlessM4TConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2684)

**Parameters:**

config ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The speech-to-text SeamlessM4T Model transformer which can be used for S2TT.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SeamlessM4TForSpeechToText.forward]]

```python
forward(input_features: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2717)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_banks)`) : Input audio features. This should be returned by the [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) class or the [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor) class. See [SeamlessM4TFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  Bart uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).  For translation and summarization training, `decoder_input_ids` should be provided. If no `decoder_input_ids` is provided, the model will create this tensor by shifting the `input_ids` to the right for denoising pre-training following the paper.

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `modeling_bart._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape`(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*): Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) and inputs.

The [SeamlessM4TForSpeechToText](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForSpeechToText) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> from transformers import AutoProcessor, SeamlessM4TForSpeechToText
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("facebook/hf-seamless-m4t-medium")
>>> model = SeamlessM4TForSpeechToText.from_pretrained("facebook/hf-seamless-m4t-medium")

>>> # audio file is decoded on the fly
>>> inputs = processor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")
>>> with torch.no_grad():
...     logits = model(**inputs).logits
>>> predicted_ids = torch.argmax(logits, dim=-1)

>>> # transcribe speech
>>> transcription = processor.batch_decode(predicted_ids)
>>> transcription[0]
...

>>> inputs["labels"] = processor(text=dataset[0]["text"], return_tensors="pt").input_ids

>>> # compute loss
>>> loss = model(**inputs).loss
>>> round(loss.item(), 2)
...
```

#### generate[[transformers.SeamlessM4TForSpeechToText.generate]]

```python
generate(input_features = None, tgt_lang = None, generation_config = None, logits_processor = None, stopping_criteria = None, prefix_allowed_tokens_fn = None, synced_gpus = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2822)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_banks)`) : Input audio features. This should be returned by the [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) class or the [SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor) class. See [SeamlessM4TFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor.__call__) for details. 

tgt_lang (`str`, *optional*) : The language to use as target language for translation.

generation_config (`~generation.GenerationConfig`, *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which had the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

logits_processor (`LogitsProcessorList`, *optional*) : Custom logits processors that complement the default logits processors built from arguments and generation config. If a logit processor is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

stopping_criteria (`StoppingCriteriaList`, *optional*) : Custom stopping criteria that complement the default stopping criteria built from arguments and a generation config. If a stopping criteria is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

prefix_allowed_tokens_fn (`Callable[[int, torch.Tensor], list[int]]`, *optional*) : If provided, this function constraints the beam search to allowed tokens only at each step. If not provided no constraint is applied. This function takes 2 arguments: the batch ID `batch_id` and `input_ids`. It has to return a list with the allowed tokens for the next generation step conditioned on the batch ID `batch_id` and the previously generated tokens `inputs_ids`. This argument is useful for constrained generation conditioned on the prefix, as described in [Autoregressive Entity Retrieval](https://huggingface.co/papers/2010.00904).

synced_gpus (`bool`, *optional*, defaults to `False`) : Whether to continue running the while loop until max_length (needed to avoid deadlocking with `FullyShardedDataParallel` and DeepSpeed ZeRO Stage 3).

kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of `generate_config` and/or additional model-specific kwargs that will be forwarded to the `forward` function of the model.

**Returns:** [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`

A [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.FloatTensor`. The possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:
- [GenerateEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](./generation_strategies).

## SeamlessM4TConfig[[transformers.SeamlessM4TConfig]]

#### transformers.SeamlessM4TConfig[[transformers.SeamlessM4TConfig]]

```python
transformers.SeamlessM4TConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 256102, t2u_vocab_size: int = 10082, hidden_size: int = 1024, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, use_cache: bool = True, max_position_embeddings: int = 1024, encoder_layerdrop: float | int = 0.05, decoder_layerdrop: float | int = 0.05, activation_function: str = 'relu', dropout: float | int = 0.1, attention_dropout: float | int = 0.1, activation_dropout: float | int = 0.0, scale_embedding: bool = True, encoder_layers: int = 24, encoder_ffn_dim: int = 8192, encoder_attention_heads: int = 16, decoder_layers: int = 24, decoder_ffn_dim: int = 8192, decoder_attention_heads: int = 16, decoder_start_token_id: int = 3, max_new_tokens: int | None = 256, pad_token_id: int | None = 0, bos_token_id: int | None = 2, eos_token_id: int | list[int] | None = 3, speech_encoder_layers: int = 24, speech_encoder_attention_heads: int = 16, speech_encoder_intermediate_size: int = 4096, speech_encoder_hidden_act: str = 'swish', speech_encoder_dropout: float | int = 0.0, add_adapter: bool = True, speech_encoder_layerdrop: float | int = 0.1, feature_projection_input_dim: int = 160, num_conv_pos_embeddings: int = 128, num_conv_pos_embedding_groups: int = 16, adaptor_kernel_size: int = 8, adaptor_stride: int = 8, adaptor_dropout: float | int = 0.1, num_adapter_layers: int = 1, position_embeddings_type: str = 'relative', rotary_embedding_base: int = 10000, max_source_positions: int = 4096, conv_depthwise_kernel_size: int = 31, t2u_bos_token_id: int | None = 0, t2u_pad_token_id: int | None = 1, t2u_eos_token_id: int | list[int] | None = 2, t2u_decoder_start_token_id: int = 2, t2u_max_new_tokens: int = 1024, t2u_encoder_layers: int = 6, t2u_encoder_ffn_dim: int = 8192, t2u_encoder_attention_heads: int = 16, t2u_decoder_layers: int = 6, t2u_decoder_ffn_dim: int = 8192, t2u_decoder_attention_heads: int = 16, t2u_max_position_embeddings: int = 2048, sampling_rate: int = 16000, upsample_initial_channel: int = 512, upsample_rates: list[int] | tuple[int, ...] = (5, 4, 4, 2, 2), upsample_kernel_sizes: list[int] | tuple[int, ...] = (11, 8, 8, 4, 4), resblock_kernel_sizes: list[int] | tuple[int, ...] = (3, 7, 11), resblock_dilation_sizes: list | tuple = ((1, 3, 5), (1, 3, 5), (1, 3, 5)), leaky_relu_slope: float = 0.1, unit_hifi_gan_vocab_size: int = 10000, unit_embed_dim: int = 1280, lang_embed_dim: int = 256, spkr_embed_dim: int = 256, vocoder_num_langs: int = 36, vocoder_num_spkrs: int = 200, variance_predictor_kernel_size: int = 3, var_pred_dropout: float | int = 0.5, vocoder_offset: int = 4, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/configuration_seamless_m4t.py#L24)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `256102`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

t2u_vocab_size (`int`, *optional*, defaults to 10082) : Unit vocabulary size of the SeamlessM4T model. Defines the number of different unit tokens that can be represented by the `inputs_ids` passed when calling the Text-To-Units sub-model of [~SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel), [~SeamlessM4TForSpeechToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForSpeechToSpeech) or [~SeamlessM4TForTextToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForTextToSpeech).

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

max_position_embeddings (`int`, *optional*, defaults to `1024`) : The maximum sequence length that this model might ever be used with.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.05`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.05`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

scale_embedding (`bool`, *optional*, defaults to `True`) : Whether to scale embeddings by dividing by sqrt(d_model).

encoder_layers (`int`, *optional*, defaults to `24`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `8192`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `24`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `8192`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_start_token_id (`int`, *optional*, defaults to `3`) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

max_new_tokens (`int`, *optional*, defaults to 256) : The maximum numbers of text tokens to generate, ignoring the number of tokens in the prompt.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `2`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `3`) : Token id used for end-of-stream in the vocabulary.

speech_encoder_layers (`int`, *optional*, defaults to 24) : Number of hidden layers in the Transformer speech encoder.

speech_encoder_attention_heads (`int`, *optional*, defaults to 16) : Number of attention heads for each attention layer in the Transformer speech encoder.

speech_encoder_intermediate_size (`int`, *optional*, defaults to 4096) : Dimension of the "intermediate" (i.e., feed-forward) layer in the Transformer speech encoder.

speech_encoder_hidden_act (`str` or `function`, *optional*, defaults to `"swish"`) : The non-linear activation function (function or string) in the speech encoder. If string, `"gelu"`, `"relu"`, `"selu"`, `"swish"` and `"gelu_new"` are supported.

speech_encoder_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for all layers in the speech encoder.

add_adapter (`bool`, *optional*, defaults to `True`) : Add an adapter layer on top of the speech encoder.

speech_encoder_layerdrop (`float`, *optional*, defaults to 0.1) : The LayerDrop probability for the speech encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

feature_projection_input_dim (`int`, *optional*, defaults to 160) : Input dimension of the input feature projection of the speech encoder, i.e the dimension after processing input audios with [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor).

num_conv_pos_embeddings (`int`, *optional*, defaults to 128) : Number of convolutional positional embeddings. Defines the kernel size of 1D convolutional positional embeddings layer of the speech encoder.

num_conv_pos_embedding_groups (`int`, *optional*, defaults to 16) : Number of groups of 1D convolutional positional embeddings layer of the speech encoder.

adaptor_kernel_size (`int`, *optional*, defaults to 8) : Kernel size of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

adaptor_stride (`int`, *optional*, defaults to 8) : Stride of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

adaptor_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for all layers in the speech adapter.

num_adapter_layers (`int`, *optional*, defaults to 1) : Number of convolutional layers that should be used in the adapter network. Only relevant if `add_adapter is True`.

position_embeddings_type (`str`, *optional*, defaults to `"relative"`) : Can be specified to `relative` or `rotary` for relative or rotary position embeddings respectively. If left `None` no relative position embedding is applied. Only applied to the speech encoder.

rotary_embedding_base (`int`, *optional*, defaults to 10000) : If `"rotary"` position embeddings are used, defines the size of the embedding base. Only applied to the speech encoder.

max_source_positions (`int`, *optional*, defaults to 4096) : if `"relative"` position embeddings are used, defines the maximum source input positions. Only applied to the speech encoder.

conv_depthwise_kernel_size (`int`, *optional*, defaults to 31) : Kernel size of convolutional depthwise 1D layer in Conformer blocks. Only applied to the speech encoder.

t2u_bos_token_id (`int`, *optional*, defaults to 0) : The id of the _beginning-of-stream_ unit token. Only applied to the text-to-unit seq2seq model.

t2u_pad_token_id (`int`, *optional*, defaults to 1) : The id of the _padding_ unit token. Only applied to the text-to-unit seq2seq model.

t2u_eos_token_id (`int`, *optional*, defaults to 2) : The id of the _end-of-stream_ unit token. Only applied to the text-to-unit seq2seq model.

t2u_decoder_start_token_id (`int`, *optional*, defaults to 2) : If an encoder-decoder model starts decoding with a different token than _bos_, the id of that token. Only applied to the text-to-unit seq2seq model.

t2u_max_new_tokens (`int`, *optional*, defaults to 1024) : The maximum numbers of unit tokens to generate, ignoring the number of tokens in the prompt. Only applied to the text-to-unit seq2seq model.

t2u_encoder_layers (`int`, *optional*, defaults to 6) : Number of hidden layers in the Transformer text-to-unit encoder.

t2u_encoder_ffn_dim (`int`, *optional*, defaults to 8192) : Dimension of the "intermediate" (i.e., feed-forward) layer in the Transformer text-to-unit encoder.

t2u_encoder_attention_heads (`int`, *optional*, defaults to 16) : Number of attention heads for each attention layer in the Transformer text-to-unit encoder.

t2u_decoder_layers (`int`, *optional*, defaults to 6) : Number of hidden layers in the Transformer text-to-unit decoder.

t2u_decoder_ffn_dim (`int`, *optional*, defaults to 8192) : Dimension of the "intermediate" (i.e., feed-forward) layer in the Transformer text-to-unit decoder.

t2u_decoder_attention_heads (`int`, *optional*, defaults to 16) : Number of attention heads for each attention layer in the Transformer text-to-unit decoder.

t2u_max_position_embeddings (`int`, *optional*, defaults to 2048) : The maximum sequence length that this model text-to-unit component might ever be used with. Typically set this to something large just in case (e.g., 512 or 1024 or 2048).

sampling_rate (`int`, *optional*, defaults to `16000`) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

upsample_initial_channel (`int`, *optional*, defaults to 512) : The number of input channels into the hifi-gan upsampling network. Applies to the vocoder only.

upsample_rates (`tuple[int]` or `list[int]`, *optional*, defaults to `[5, 4, 4, 2, 2]`) : A tuple of integers defining the stride of each 1D convolutional layer in the vocoder upsampling network. The length of *upsample_rates* defines the number of convolutional layers and has to match the length of *upsample_kernel_sizes*. Applies to the vocoder only.

upsample_kernel_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[11, 8, 8, 4, 4]`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the vocoder upsampling network. The length of *upsample_kernel_sizes* defines the number of convolutional layers and has to match the length of *upsample_rates*. Applies to the vocoder only.

resblock_kernel_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[3, 7, 11]`) : A tuple of integers defining the kernel sizes of the vocoder 1D convolutional layers in the multi-receptive field fusion (MRF) module. Applies to the vocoder only.

resblock_dilation_sizes (`tuple[tuple[int]]` or `list[list[int]]`, *optional*, defaults to `[[1, 3, 5], [1, 3, 5], [1, 3, 5]]`) : A nested tuple of integers defining the dilation rates of the vocoder dilated 1D convolutional layers in the multi-receptive field fusion (MRF) module. Applies to the vocoder only.

leaky_relu_slope (`float`, *optional*, defaults to 0.1) : The angle of the negative slope used by the leaky ReLU activation in the vocoder. Applies to the vocoder only.

unit_hifi_gan_vocab_size (`int`, *optional*, defaults to 10000) : Vocabulary size of the SeamlessM4T vocoder. Defines the number of different unit tokens that can be represented by the `inputs_ids` passed when calling the vocoder of [~SeamlessM4TModel](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TModel), [~SeamlessM4TForSpeechToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForSpeechToSpeech) or [~SeamlessM4TForTextToSpeech](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TForTextToSpeech).

unit_embed_dim (`int`, *optional*, defaults to 1280) : The projection dimension of the input ids given to the hifi-gan vocoder. Applies to the vocoder only.

lang_embed_dim (`int`, *optional*, defaults to 256) : The projection dimension of the target language given to the hifi-gan vocoder. Applies to the vocoder only.

spkr_embed_dim (`int`, *optional*, defaults to 256) : The projection dimension of the speaker id given to the hifi-gan vocoder. Applies to the vocoder only.

vocoder_num_langs (`int`, *optional*, defaults to 36) : Number of langs supported by the vocoder. Might be different from `t2u_num_langs`.

vocoder_num_spkrs (`int`, *optional*, defaults to 200) : Number of speakers supported by the vocoder.

variance_predictor_kernel_size (`int`, *optional*, defaults to 3) : Kernel size of the duration predictor. Applies to the vocoder only.

var_pred_dropout (`float`, *optional*, defaults to 0.5) : The dropout probability of the duration predictor. Applies to the vocoder only.

vocoder_offset (`int`, *optional*, defaults to 4) : Offset the unit token ids by this number to account for symbol tokens. Applies to the vocoder only.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a SeamlessM4TModel. It is used to instantiate a Seamless M4T
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/hf-seamless-m4t-medium](https://huggingface.co/facebook/hf-seamless-m4t-medium)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import SeamlessM4TModel, SeamlessM4TConfig

>>> # Initializing a SeamlessM4T "facebook/hf-seamless-m4t-medium" style configuration
>>> configuration = SeamlessM4TConfig()

>>> # Initializing a model from the "facebook/hf-seamless-m4t-medium" style configuration
>>> model = SeamlessM4TModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SeamlessM4TTokenizer[[transformers.SeamlessM4TTokenizer]]

#### transformers.SeamlessM4TTokenizer[[transformers.SeamlessM4TTokenizer]]

```python
transformers.SeamlessM4TTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', src_lang = 'eng', tgt_lang = 'fra', additional_special_tokens = None, keep_accents = None, vocab_file = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/tokenization_seamless_m4t.py#L33)

**Parameters:**

vocab (`list` or `dict`, *optional*) : List of (token, score) tuples or dict mapping tokens to indices. If not provided, uses default vocab.

merges (`str` or `list`, *optional*) : List of merge rules for BPE model. If not provided, uses empty list.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

src_lang (`str`, *optional*, defaults to `"eng"`) : The language to use as source language for translation.

tgt_lang (`str`, *optional*, defaults to `"fra"`) : The language to use as target language for translation.

additional_special_tokens (tuple or list of `str` or `tokenizers.AddedToken`, *optional*) : A tuple or a list of additional special tokens.

Construct a SeamlessM4T tokenizer (backed by HuggingFace's *tokenizers* library). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=bpe#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

The tokenization method is `<language code> <tokens> <eos>` for source language documents, and ` <language
code>  ` for target language documents.

Examples:

```python
>>> from transformers import SeamlessM4TTokenizer

>>> tokenizer = SeamlessM4TTokenizer.from_pretrained(
...     "facebook/hf-seamless-m4t-medium", src_lang="eng", tgt_lang="fra"
... )
>>> example_english_phrase = " UN Chief Says There Is No Military Solution in Syria"
>>> expected_translation_french = "Le chef de l'ONU affirme qu'il n'y a pas de solution militaire en Syrie."
>>> inputs = tokenizer(example_english_phrase, text_target=expected_translation_french, return_tensors="pt")
```

#### __call__[[transformers.SeamlessM4TTokenizer.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] | None = None, text_pair: str | list[str] | list[list[str]] | None = None, text_target: str | list[str] | list[list[str]] | None = None, text_pair_target: str | list[str] | list[list[str]] | None = None, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, pad_to_multiple_of: int | None = None, src_lang: str | None = None, tgt_lang: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/tokenization_seamless_m4t.py#L355)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

pad_to_multiple_of (`int`, *optional*, defaults to `None`) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

src_lang (`str`, *optional*) : A string representing the source language. If not specified, the last `src_lang` specified (either during initialization or when calling this tokenizer) will be used.

tgt_lang (`str`, *optional*) : A string representing the target language. If not specified, the last `tgt_lang` specified (either during initialization or when calling this tokenizer) will be used.

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to [TokenizersBackend.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__).

#### get_special_tokens_mask[[transformers.SeamlessM4TTokenizer.get_special_tokens_mask]]

```python
get_special_tokens_mask(token_ids_0: list[int], token_ids_1: list[int] | None = None, already_has_special_tokens: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L1311)

**Parameters:**

token_ids_0 : List of IDs for the (possibly already formatted) sequence.

token_ids_1 : Unused when `already_has_special_tokens=True`. Must be None in that case.

already_has_special_tokens : Whether the sequence is already formatted with special tokens.

**Returns:** A list of integers in the range [0, 1]

1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added.

For fast tokenizers, data collators call this with `already_has_special_tokens=True` to build a mask over an
already-formatted sequence. In that case, we compute the mask by checking membership in `all_special_ids`.

#### save_vocabulary[[transformers.SeamlessM4TTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_tokenizers.py#L509)

## SeamlessM4TTokenizerFast[[transformers.SeamlessM4TTokenizer]]

#### transformers.SeamlessM4TTokenizer[[transformers.SeamlessM4TTokenizer]]

```python
transformers.SeamlessM4TTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', src_lang = 'eng', tgt_lang = 'fra', additional_special_tokens = None, keep_accents = None, vocab_file = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/tokenization_seamless_m4t.py#L33)

**Parameters:**

vocab (`list` or `dict`, *optional*) : List of (token, score) tuples or dict mapping tokens to indices. If not provided, uses default vocab.

merges (`str` or `list`, *optional*) : List of merge rules for BPE model. If not provided, uses empty list.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

src_lang (`str`, *optional*, defaults to `"eng"`) : The language to use as source language for translation.

tgt_lang (`str`, *optional*, defaults to `"fra"`) : The language to use as target language for translation.

additional_special_tokens (tuple or list of `str` or `tokenizers.AddedToken`, *optional*) : A tuple or a list of additional special tokens.

Construct a SeamlessM4T tokenizer (backed by HuggingFace's *tokenizers* library). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=bpe#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

The tokenization method is `<language code> <tokens> <eos>` for source language documents, and ` <language
code>  ` for target language documents.

Examples:

```python
>>> from transformers import SeamlessM4TTokenizer

>>> tokenizer = SeamlessM4TTokenizer.from_pretrained(
...     "facebook/hf-seamless-m4t-medium", src_lang="eng", tgt_lang="fra"
... )
>>> example_english_phrase = " UN Chief Says There Is No Military Solution in Syria"
>>> expected_translation_french = "Le chef de l'ONU affirme qu'il n'y a pas de solution militaire en Syrie."
>>> inputs = tokenizer(example_english_phrase, text_target=expected_translation_french, return_tensors="pt")
```

#### __call__[[transformers.SeamlessM4TTokenizer.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] | None = None, text_pair: str | list[str] | list[list[str]] | None = None, text_target: str | list[str] | list[list[str]] | None = None, text_pair_target: str | list[str] | list[list[str]] | None = None, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, pad_to_multiple_of: int | None = None, src_lang: str | None = None, tgt_lang: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/tokenization_seamless_m4t.py#L355)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

pad_to_multiple_of (`int`, *optional*, defaults to `None`) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

src_lang (`str`, *optional*) : A string representing the source language. If not specified, the last `src_lang` specified (either during initialization or when calling this tokenizer) will be used.

tgt_lang (`str`, *optional*) : A string representing the target language. If not specified, the last `tgt_lang` specified (either during initialization or when calling this tokenizer) will be used.

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to [TokenizersBackend.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__).

## SeamlessM4TFeatureExtractor[[transformers.SeamlessM4TFeatureExtractor]]

#### transformers.SeamlessM4TFeatureExtractor[[transformers.SeamlessM4TFeatureExtractor]]

```python
transformers.SeamlessM4TFeatureExtractor(feature_size = 80, sampling_rate = 16000, num_mel_bins = 80, padding_value = 0.0, stride = 2, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/feature_extraction_seamless_m4t.py#L35)

**Parameters:**

feature_size (`int`, *optional*, defaults to 80) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

num_mel_bins (`int`, *optional*, defaults to 80) : Number of Mel-frequency bins.

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding vectors.

stride (`int`, *optional*, defaults to 2) : Stride used to reshape audios from shape (batch_size,num_frames,num_mel_bins) to (batch_size,num_frames//stride,num_mel_bins*stride).

Constructs a SeamlessM4T feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains most of the main methods. Users
should refer to this superclass for more information regarding those methods.

This class extracts mel-filter bank features from raw speech.

#### __call__[[transformers.SeamlessM4TFeatureExtractor.__call__]]

```python
__call__(raw_speech: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], padding: bool | str | transformers.utils.generic.PaddingStrategy = True, pad_to_multiple_of: int | None = 2, max_length: int | None = None, truncation: bool = False, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None, return_attention_mask: bool | None = None, do_normalize_per_mel_bins: bool | None = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/feature_extraction_seamless_m4t.py#L140)

**Parameters:**

raw_speech (`np.ndarray`, `torch.Tensor`, `list[float]`, `list[np.ndarray]`, `list[torch.Tensor]`, --

`list[list[float]]`, `list[list[list[float]]]`) : The sequence or batch of sequences to be padded. Each sequence can be a numpy array, a torch tensor, a list of float values, a list of numpy arrays, a list of torch tensors, a list of list of float values or a list of a list of list of float values. If `raw_speech` is a one-dimensional `np.ndarray`, `torch.Tensor` or a `list[float]`, `raw_speech` is considered a single-channel, single-sample sound. In all other cases, the first dimension of `raw_speech`, whether from an `np.ndarray`, a `torch.Tensor` or a `list[...]`, corresponds to the number of samples in the batch, and the number of channels (i.e. mono or stereo character) is derived from the other dimensions (1D -> single-channel waveform batches; 2D-> stereo-channel waveform batches).

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

pad_to_multiple_of (`int`, *optional*, defaults to 2) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

truncation (`bool`) : Activates truncation to cut input sequences longer than *max_length* to *max_length*.

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific feature_extractor's default.  [What are attention masks?](../glossary#attention-mask)    For SeamlessM4T models, `attention_mask` should always be passed for batched inference, to avoid subtle bugs.   

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sampling rate at which the `raw_speech` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors.

do_normalize_per_mel_bins (`bool`, *optional*, defaults to `True`) : Whether or not to zero-mean unit-variance normalize the input per mel-channel.

kwargs (*optional*) : Remaining dictionary of keyword arguments that will be passed to the tokenizer or the feature extractor.

Main method to featurize and prepare for the model one or several sequence(s).

## SeamlessM4TProcessor[[transformers.SeamlessM4TProcessor]]

#### transformers.SeamlessM4TProcessor[[transformers.SeamlessM4TProcessor]]

```python
transformers.SeamlessM4TProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/processing_seamless_m4t.py#L48)

**Parameters:**

feature_extractor (`SeamlessM4TFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`SeamlessM4TTokenizer`) : The tokenizer is a required input.

Constructs a SeamlessM4TProcessor which wraps a feature extractor and a tokenizer into a single processor.

[SeamlessM4TProcessor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TProcessor) offers all the functionalities of [SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) and [SeamlessM4TTokenizer](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTokenizer). See the
[~SeamlessM4TFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TFeatureExtractor) and [~SeamlessM4TTokenizer](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTokenizer) for more information.

#### __call__[[transformers.SeamlessM4TProcessor.__call__]]

```python
__call__(text: str | list[str] | list[list[str]] | None = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/processing_seamless_m4t.py#L54)

**Parameters:**

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **input_features** -- Audio input features to be fed to a model. Returned when `audios` is not `None`.

## SeamlessM4TCodeHifiGan[[transformers.SeamlessM4TCodeHifiGan]]

#### transformers.SeamlessM4TCodeHifiGan[[transformers.SeamlessM4TCodeHifiGan]]

```python
transformers.SeamlessM4TCodeHifiGan(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2278)

**Parameters:**

config ([SeamlessM4TCodeHifiGan](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TCodeHifiGan)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Code HiFi-GAN vocoder as described in this [repository](https://github.com/facebookresearch/speech-resynthesis).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SeamlessM4TCodeHifiGan.forward]]

```python
forward(input_ids: LongTensor, spkr_id: Tensor, lang_id: Tensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2355)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SeamlessM4TTextToUnitForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTextToUnitForConditionalGeneration). [What are input IDs?](../glossary#input-ids)

spkr_id (`int`, *optional*) : The id of the speaker used for speech synthesis. Must be lower than `config.vocoder_num_spkrs`.

tgt_lang (`str`, *optional*) : The language id to use as target language for translation.

## SeamlessM4THifiGan[[transformers.SeamlessM4THifiGan]]

#### transformers.SeamlessM4THifiGan[[transformers.SeamlessM4THifiGan]]

```python
transformers.SeamlessM4THifiGan(config: SeamlessM4TConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2201)

#### forward[[transformers.SeamlessM4THifiGan.forward]]

```python
forward(inputs_embeds: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2236)

**Parameters:**

spectrogram (`torch.FloatTensor`) : Tensor containing the log-mel spectrograms. Can be batched and of shape `(batch_size, sequence_length, model_in_dim)`, or un-batched and of shape `(sequence_length, model_in_dim)`. Note that `model_in_dim` is the sum of `config.unit_embed_dim`, `config.lang_embed_dim` and `config.spkr_embed_dim`.

**Returns:** `torch.FloatTensor`

Tensor containing the speech waveform. If the input spectrogram is batched, will be of
shape `(batch_size, num_frames,)`. If un-batched, will be of shape `(num_frames,)`.

Converts a log-mel spectrogram into a speech waveform. Passing a batch of log-mel spectrograms returns a batch
of speech waveforms. Passing a single, un-batched log-mel spectrogram returns a single, un-batched speech
waveform.

## SeamlessM4TTextToUnitModel[[transformers.SeamlessM4TTextToUnitModel]]

#### transformers.SeamlessM4TTextToUnitModel[[transformers.SeamlessM4TTextToUnitModel]]

```python
transformers.SeamlessM4TTextToUnitModel(config: SeamlessM4TConfig, embed_tokens_decoder: typing.Optional[torch.nn.Embedding] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L1885)

**Parameters:**

config ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

embed_tokens_decoder (`nn.Embedding`, *optional*) : input embedding of the decoder.

Transformer bare text-to-unit encoder-decoder. The encoder is a `SeamlessM4TEncoder` without embeddings and the decoder is a `SeamlessM4TDecoder`.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## SeamlessM4TTextToUnitForConditionalGeneration[[transformers.SeamlessM4TTextToUnitForConditionalGeneration]]

#### transformers.SeamlessM4TTextToUnitForConditionalGeneration[[transformers.SeamlessM4TTextToUnitForConditionalGeneration]]

```python
transformers.SeamlessM4TTextToUnitForConditionalGeneration(config: SeamlessM4TConfig, embed_tokens_decoder: typing.Optional[torch.nn.Embedding] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L1977)

**Parameters:**

config ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

embed_tokens_decoder (`nn.Embedding`, *optional*) : input embedding of the decoder.

Transformer text-to-unit encoder-decoder with a language model head. The base encoder-decoder model is a `SeamlessM4TTextToUnit`.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SeamlessM4TTextToUnitForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/seamless_m4t/modeling_seamless_m4t.py#L2021)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  Bart uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).  For translation and summarization training, `decoder_input_ids` should be provided. If no `decoder_input_ids` is provided, the model will create this tensor by shifting the `input_ids` to the right for denoising pre-training following the paper.

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `modeling_bart._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape`(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*): Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SeamlessM4TConfig](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TConfig)) and inputs.

The [SeamlessM4TTextToUnitForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/seamless_m4t#transformers.SeamlessM4TTextToUnitForConditionalGeneration) forward method, overrides the `__call__` special method.

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
