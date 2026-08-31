# Inference types

This page lists the types (e.g. dataclasses) available for each task supported on the Hugging Face Hub.
Each task is specified using a JSON schema, and the types are generated from these schemas - with some customization
due to Python requirements.
Visit [@huggingface.js/tasks](https://github.com/huggingface/huggingface.js/tree/main/packages/tasks/src/tasks)
to find the JSON schemas for each task.

This part of the lib is still under development and will be improved in future releases.

## audio_classification[[huggingface_hub.AudioClassificationInput]]

#### huggingface_hub.AudioClassificationInput[[huggingface_hub.AudioClassificationInput]]

```python
huggingface_hub.AudioClassificationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.audio_classification.AudioClassificationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/audio_classification.py#L25)

Inputs for Audio Classification inference

#### huggingface_hub.AudioClassificationOutputElement[[huggingface_hub.AudioClassificationOutputElement]]

```python
huggingface_hub.AudioClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/audio_classification.py#L37)

Outputs for Audio Classification inference

#### huggingface_hub.AudioClassificationParameters[[huggingface_hub.AudioClassificationParameters]]

```python
huggingface_hub.AudioClassificationParameters(function_to_apply: typing.Optional[ForwardRef('AudioClassificationOutputTransform')] = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/audio_classification.py#L15)

Additional inference parameters for Audio Classification

## audio_to_audio[[huggingface_hub.AudioToAudioInput]]

#### huggingface_hub.AudioToAudioInput[[huggingface_hub.AudioToAudioInput]]

```python
huggingface_hub.AudioToAudioInput(inputs: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/audio_to_audio.py#L12)

Inputs for Audio to Audio inference

#### huggingface_hub.AudioToAudioOutputElement[[huggingface_hub.AudioToAudioOutputElement]]

```python
huggingface_hub.AudioToAudioOutputElement(blob: typing.Any, content_type: str, label: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/audio_to_audio.py#L20)

Outputs of inference for the Audio To Audio task
A generated audio file with its label.

## automatic_speech_recognition[[huggingface_hub.AutomaticSpeechRecognitionGenerationParameters]]

#### huggingface_hub.AutomaticSpeechRecognitionGenerationParameters[[huggingface_hub.AutomaticSpeechRecognitionGenerationParameters]]

```python
huggingface_hub.AutomaticSpeechRecognitionGenerationParameters(do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('AutomaticSpeechRecognitionEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/automatic_speech_recognition.py#L15)

Parametrization of the text generation process

#### huggingface_hub.AutomaticSpeechRecognitionInput[[huggingface_hub.AutomaticSpeechRecognitionInput]]

```python
huggingface_hub.AutomaticSpeechRecognitionInput(inputs: str, parameters: huggingface_hub.inference._generated.types.automatic_speech_recognition.AutomaticSpeechRecognitionParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/automatic_speech_recognition.py#L85)

Inputs for Automatic Speech Recognition inference

#### huggingface_hub.AutomaticSpeechRecognitionOutput[[huggingface_hub.AutomaticSpeechRecognitionOutput]]

```python
huggingface_hub.AutomaticSpeechRecognitionOutput(text: str, chunks: list[huggingface_hub.inference._generated.types.automatic_speech_recognition.AutomaticSpeechRecognitionOutputChunk] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/automatic_speech_recognition.py#L105)

Outputs of inference for the Automatic Speech Recognition task

#### huggingface_hub.AutomaticSpeechRecognitionOutputChunk[[huggingface_hub.AutomaticSpeechRecognitionOutputChunk]]

```python
huggingface_hub.AutomaticSpeechRecognitionOutputChunk(text: str, timestamp: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/automatic_speech_recognition.py#L97)

#### huggingface_hub.AutomaticSpeechRecognitionParameters[[huggingface_hub.AutomaticSpeechRecognitionParameters]]

```python
huggingface_hub.AutomaticSpeechRecognitionParameters(generation_parameters: huggingface_hub.inference._generated.types.automatic_speech_recognition.AutomaticSpeechRecognitionGenerationParameters | None = None, return_timestamps: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/automatic_speech_recognition.py#L75)

Additional inference parameters for Automatic Speech Recognition

## chat_completion[[huggingface_hub.ChatCompletionInput]]

#### huggingface_hub.ChatCompletionInput[[huggingface_hub.ChatCompletionInput]]

```python
huggingface_hub.ChatCompletionInput(messages: list, frequency_penalty: float | None = None, logit_bias: list[float] | None = None, logprobs: bool | None = None, max_tokens: int | None = None, model: str | None = None, n: int | None = None, presence_penalty: float | None = None, response_format: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatText, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONSchema, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONObject, NoneType] = None, seed: int | None = None, stop: list[str] | None = None, stream: bool | None = None, stream_options: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputStreamOptions | None = None, temperature: float | None = None, tool_choice: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputToolChoiceClass, ForwardRef('ChatCompletionInputToolChoiceEnum'), NoneType] = None, tool_prompt: str | None = None, tools: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputTool] | None = None, top_logprobs: int | None = None, top_p: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L125)

Chat Completion Input.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.ChatCompletionInputFunctionDefinition[[huggingface_hub.ChatCompletionInputFunctionDefinition]]

```python
huggingface_hub.ChatCompletionInputFunctionDefinition(name: str, parameters: typing.Any, description: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L27)

#### huggingface_hub.ChatCompletionInputFunctionName[[huggingface_hub.ChatCompletionInputFunctionName]]

```python
huggingface_hub.ChatCompletionInputFunctionName(name: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L106)

#### huggingface_hub.ChatCompletionInputJSONSchema[[huggingface_hub.ChatCompletionInputJSONSchema]]

```python
huggingface_hub.ChatCompletionInputJSONSchema(name: str, description: str | None = None, schema: dict[str, object] | None = None, strict: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L49)

#### huggingface_hub.ChatCompletionInputMessage[[huggingface_hub.ChatCompletionInputMessage]]

```python
huggingface_hub.ChatCompletionInputMessage(role: str, content: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputMessageChunk] | str | None = None, name: str | None = None, tool_calls: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputToolCall] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L41)

#### huggingface_hub.ChatCompletionInputMessageChunk[[huggingface_hub.ChatCompletionInputMessageChunk]]

```python
huggingface_hub.ChatCompletionInputMessageChunk(type: ChatCompletionInputMessageChunkType, image_url: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputURL | None = None, text: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L20)

#### huggingface_hub.ChatCompletionInputResponseFormatJSONObject[[huggingface_hub.ChatCompletionInputResponseFormatJSONObject]]

```python
huggingface_hub.ChatCompletionInputResponseFormatJSONObject(type: typing.Literal['json_object'])
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L84)

#### huggingface_hub.ChatCompletionInputResponseFormatJSONSchema[[huggingface_hub.ChatCompletionInputResponseFormatJSONSchema]]

```python
huggingface_hub.ChatCompletionInputResponseFormatJSONSchema(type: typing.Literal['json_schema'], json_schema: ChatCompletionInputJSONSchema)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L78)

#### huggingface_hub.ChatCompletionInputResponseFormatText[[huggingface_hub.ChatCompletionInputResponseFormatText]]

```python
huggingface_hub.ChatCompletionInputResponseFormatText(type: typing.Literal['text'])
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L73)

#### huggingface_hub.ChatCompletionInputStreamOptions[[huggingface_hub.ChatCompletionInputStreamOptions]]

```python
huggingface_hub.ChatCompletionInputStreamOptions(include_usage: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L96)

#### huggingface_hub.ChatCompletionInputTool[[huggingface_hub.ChatCompletionInputTool]]

```python
huggingface_hub.ChatCompletionInputTool(function: ChatCompletionInputFunctionDefinition, type: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L119)

#### huggingface_hub.ChatCompletionInputToolCall[[huggingface_hub.ChatCompletionInputToolCall]]

```python
huggingface_hub.ChatCompletionInputToolCall(function: ChatCompletionInputFunctionDefinition, id: str, type: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L34)

#### huggingface_hub.ChatCompletionInputToolChoiceClass[[huggingface_hub.ChatCompletionInputToolChoiceClass]]

```python
huggingface_hub.ChatCompletionInputToolChoiceClass(function: ChatCompletionInputFunctionName)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L111)

#### huggingface_hub.ChatCompletionInputURL[[huggingface_hub.ChatCompletionInputURL]]

```python
huggingface_hub.ChatCompletionInputURL(url: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L12)

#### huggingface_hub.ChatCompletionOutput[[huggingface_hub.ChatCompletionOutput]]

```python
huggingface_hub.ChatCompletionOutput(choices: list, created: int, id: str, model: str, system_fingerprint: str, usage: ChatCompletionOutputUsage)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L263)

Chat Completion Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.ChatCompletionOutputComplete[[huggingface_hub.ChatCompletionOutputComplete]]

```python
huggingface_hub.ChatCompletionOutputComplete(finish_reason: str, index: int, message: ChatCompletionOutputMessage, logprobs: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionOutputLogprobs | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L248)

#### huggingface_hub.ChatCompletionOutputFunctionDefinition[[huggingface_hub.ChatCompletionOutputFunctionDefinition]]

```python
huggingface_hub.ChatCompletionOutputFunctionDefinition(arguments: str, name: str, description: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L225)

#### huggingface_hub.ChatCompletionOutputLogprob[[huggingface_hub.ChatCompletionOutputLogprob]]

```python
huggingface_hub.ChatCompletionOutputLogprob(logprob: float, token: str, top_logprobs: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L213)

#### huggingface_hub.ChatCompletionOutputLogprobs[[huggingface_hub.ChatCompletionOutputLogprobs]]

```python
huggingface_hub.ChatCompletionOutputLogprobs(content: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L220)

#### huggingface_hub.ChatCompletionOutputMessage[[huggingface_hub.ChatCompletionOutputMessage]]

```python
huggingface_hub.ChatCompletionOutputMessage(role: str, content: str | None = None, reasoning: str | None = None, tool_call_id: str | None = None, tool_calls: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionOutputToolCall] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L239)

#### huggingface_hub.ChatCompletionOutputToolCall[[huggingface_hub.ChatCompletionOutputToolCall]]

```python
huggingface_hub.ChatCompletionOutputToolCall(function: ChatCompletionOutputFunctionDefinition, id: str, type: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L232)

#### huggingface_hub.ChatCompletionOutputTopLogprob[[huggingface_hub.ChatCompletionOutputTopLogprob]]

```python
huggingface_hub.ChatCompletionOutputTopLogprob(logprob: float, token: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L207)

#### huggingface_hub.ChatCompletionOutputUsage[[huggingface_hub.ChatCompletionOutputUsage]]

```python
huggingface_hub.ChatCompletionOutputUsage(completion_tokens: int, prompt_tokens: int, total_tokens: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L256)

#### huggingface_hub.ChatCompletionStreamOutput[[huggingface_hub.ChatCompletionStreamOutput]]

```python
huggingface_hub.ChatCompletionStreamOutput(choices: list, created: int, id: str, model: str, system_fingerprint: str, usage: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionStreamOutputUsage | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L335)

Chat Completion Stream Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.ChatCompletionStreamOutputChoice[[huggingface_hub.ChatCompletionStreamOutputChoice]]

```python
huggingface_hub.ChatCompletionStreamOutputChoice(delta: ChatCompletionStreamOutputDelta, index: int, finish_reason: str | None = None, logprobs: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionStreamOutputLogprobs | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L320)

#### huggingface_hub.ChatCompletionStreamOutputDelta[[huggingface_hub.ChatCompletionStreamOutputDelta]]

```python
huggingface_hub.ChatCompletionStreamOutputDelta(role: str, content: str | None = None, reasoning: str | None = None, tool_call_id: str | None = None, tool_calls: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionStreamOutputDeltaToolCall] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L293)

#### huggingface_hub.ChatCompletionStreamOutputDeltaToolCall[[huggingface_hub.ChatCompletionStreamOutputDeltaToolCall]]

```python
huggingface_hub.ChatCompletionStreamOutputDeltaToolCall(function: ChatCompletionStreamOutputFunction, id: str, index: int, type: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L285)

#### huggingface_hub.ChatCompletionStreamOutputFunction[[huggingface_hub.ChatCompletionStreamOutputFunction]]

```python
huggingface_hub.ChatCompletionStreamOutputFunction(arguments: str, name: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L279)

#### huggingface_hub.ChatCompletionStreamOutputLogprob[[huggingface_hub.ChatCompletionStreamOutputLogprob]]

```python
huggingface_hub.ChatCompletionStreamOutputLogprob(logprob: float, token: str, top_logprobs: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L308)

#### huggingface_hub.ChatCompletionStreamOutputLogprobs[[huggingface_hub.ChatCompletionStreamOutputLogprobs]]

```python
huggingface_hub.ChatCompletionStreamOutputLogprobs(content: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L315)

#### huggingface_hub.ChatCompletionStreamOutputTopLogprob[[huggingface_hub.ChatCompletionStreamOutputTopLogprob]]

```python
huggingface_hub.ChatCompletionStreamOutputTopLogprob(logprob: float, token: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L302)

#### huggingface_hub.ChatCompletionStreamOutputUsage[[huggingface_hub.ChatCompletionStreamOutputUsage]]

```python
huggingface_hub.ChatCompletionStreamOutputUsage(completion_tokens: int, prompt_tokens: int, total_tokens: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/chat_completion.py#L328)

## depth_estimation[[huggingface_hub.DepthEstimationInput]]

#### huggingface_hub.DepthEstimationInput[[huggingface_hub.DepthEstimationInput]]

```python
huggingface_hub.DepthEstimationInput(inputs: typing.Any, parameters: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/depth_estimation.py#L12)

Inputs for Depth Estimation inference

#### huggingface_hub.DepthEstimationOutput[[huggingface_hub.DepthEstimationOutput]]

```python
huggingface_hub.DepthEstimationOutput(depth: typing.Any, predicted_depth: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/depth_estimation.py#L22)

Outputs of inference for the Depth Estimation task

## document_question_answering[[huggingface_hub.DocumentQuestionAnsweringInput]]

#### huggingface_hub.DocumentQuestionAnsweringInput[[huggingface_hub.DocumentQuestionAnsweringInput]]

```python
huggingface_hub.DocumentQuestionAnsweringInput(inputs: DocumentQuestionAnsweringInputData, parameters: huggingface_hub.inference._generated.types.document_question_answering.DocumentQuestionAnsweringParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/document_question_answering.py#L56)

Inputs for Document Question Answering inference

#### huggingface_hub.DocumentQuestionAnsweringInputData[[huggingface_hub.DocumentQuestionAnsweringInputData]]

```python
huggingface_hub.DocumentQuestionAnsweringInputData(image: typing.Any, question: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/document_question_answering.py#L12)

One (document, question) pair to answer

#### huggingface_hub.DocumentQuestionAnsweringOutputElement[[huggingface_hub.DocumentQuestionAnsweringOutputElement]]

```python
huggingface_hub.DocumentQuestionAnsweringOutputElement(answer: str, end: int, score: float, start: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/document_question_answering.py#L66)

Outputs of inference for the Document Question Answering task

#### huggingface_hub.DocumentQuestionAnsweringParameters[[huggingface_hub.DocumentQuestionAnsweringParameters]]

```python
huggingface_hub.DocumentQuestionAnsweringParameters(doc_stride: int | None = None, handle_impossible_answer: bool | None = None, lang: str | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None, word_boxes: list[list[float] | str] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/document_question_answering.py#L22)

Additional inference parameters for Document Question Answering

## feature_extraction[[huggingface_hub.FeatureExtractionInput]]

#### huggingface_hub.FeatureExtractionInput[[huggingface_hub.FeatureExtractionInput]]

```python
huggingface_hub.FeatureExtractionInput(inputs: list[str] | str, normalize: bool | None = None, prompt_name: str | None = None, truncate: bool | None = None, truncation_direction: typing.Optional[ForwardRef('FeatureExtractionInputTruncationDirection')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/feature_extraction.py#L15)

Feature Extraction Input.
Auto-generated from TEI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tei-import.ts.

## fill_mask[[huggingface_hub.FillMaskInput]]

#### huggingface_hub.FillMaskInput[[huggingface_hub.FillMaskInput]]

```python
huggingface_hub.FillMaskInput(inputs: str, parameters: huggingface_hub.inference._generated.types.fill_mask.FillMaskParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/fill_mask.py#L26)

Inputs for Fill Mask inference

#### huggingface_hub.FillMaskOutputElement[[huggingface_hub.FillMaskOutputElement]]

```python
huggingface_hub.FillMaskOutputElement(score: float, sequence: str, token: int, token_str: typing.Any, fill_mask_output_token_str: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/fill_mask.py#L36)

Outputs of inference for the Fill Mask task

#### huggingface_hub.FillMaskParameters[[huggingface_hub.FillMaskParameters]]

```python
huggingface_hub.FillMaskParameters(targets: list[str] | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/fill_mask.py#L12)

Additional inference parameters for Fill Mask

## image_classification[[huggingface_hub.ImageClassificationInput]]

#### huggingface_hub.ImageClassificationInput[[huggingface_hub.ImageClassificationInput]]

```python
huggingface_hub.ImageClassificationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.image_classification.ImageClassificationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_classification.py#L25)

Inputs for Image Classification inference

#### huggingface_hub.ImageClassificationOutputElement[[huggingface_hub.ImageClassificationOutputElement]]

```python
huggingface_hub.ImageClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_classification.py#L37)

Outputs of inference for the Image Classification task

#### huggingface_hub.ImageClassificationParameters[[huggingface_hub.ImageClassificationParameters]]

```python
huggingface_hub.ImageClassificationParameters(function_to_apply: typing.Optional[ForwardRef('ImageClassificationOutputTransform')] = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_classification.py#L15)

Additional inference parameters for Image Classification

## image_segmentation[[huggingface_hub.ImageSegmentationInput]]

#### huggingface_hub.ImageSegmentationInput[[huggingface_hub.ImageSegmentationInput]]

```python
huggingface_hub.ImageSegmentationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.image_segmentation.ImageSegmentationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_segmentation.py#L29)

Inputs for Image Segmentation inference

#### huggingface_hub.ImageSegmentationOutputElement[[huggingface_hub.ImageSegmentationOutputElement]]

```python
huggingface_hub.ImageSegmentationOutputElement(label: str, mask: str, score: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_segmentation.py#L41)

Outputs of inference for the Image Segmentation task
A predicted mask / segment

#### huggingface_hub.ImageSegmentationParameters[[huggingface_hub.ImageSegmentationParameters]]

```python
huggingface_hub.ImageSegmentationParameters(mask_threshold: float | None = None, overlap_mask_area_threshold: float | None = None, subtask: typing.Optional[ForwardRef('ImageSegmentationSubtask')] = None, threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_segmentation.py#L15)

Additional inference parameters for Image Segmentation

## image_text_to_image[[huggingface_hub.ImageTextToImageInput]]

#### huggingface_hub.ImageTextToImageInput[[huggingface_hub.ImageTextToImageInput]]

```python
huggingface_hub.ImageTextToImageInput(inputs: str | None = None, parameters: huggingface_hub.inference._generated.types.image_text_to_image.ImageTextToImageParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_image.py#L48)

Inputs for Image Text To Image inference. Either inputs (image) or prompt (in parameters)
must be provided, or both.

#### huggingface_hub.ImageTextToImageOutput[[huggingface_hub.ImageTextToImageOutput]]

```python
huggingface_hub.ImageTextToImageOutput(image: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_image.py#L63)

Outputs of inference for the Image Text To Image task

#### huggingface_hub.ImageTextToImageParameters[[huggingface_hub.ImageTextToImageParameters]]

```python
huggingface_hub.ImageTextToImageParameters(guidance_scale: float | None = None, negative_prompt: str | None = None, num_inference_steps: int | None = None, prompt: str | None = None, seed: int | None = None, target_size: huggingface_hub.inference._generated.types.image_text_to_image.ImageTextToImageTargetSize | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_image.py#L22)

Additional inference parameters for Image Text To Image

#### huggingface_hub.ImageTextToImageTargetSize[[huggingface_hub.ImageTextToImageTargetSize]]

```python
huggingface_hub.ImageTextToImageTargetSize(height: int, width: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_image.py#L12)

The size in pixels of the output image. This parameter is only supported by some
providers and for specific models. It will be ignored when unsupported.

## image_text_to_video[[huggingface_hub.ImageTextToVideoInput]]

#### huggingface_hub.ImageTextToVideoInput[[huggingface_hub.ImageTextToVideoInput]]

```python
huggingface_hub.ImageTextToVideoInput(inputs: str | None = None, parameters: huggingface_hub.inference._generated.types.image_text_to_video.ImageTextToVideoParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_video.py#L46)

Inputs for Image Text To Video inference. Either inputs (image) or prompt (in parameters)
must be provided, or both.

#### huggingface_hub.ImageTextToVideoOutput[[huggingface_hub.ImageTextToVideoOutput]]

```python
huggingface_hub.ImageTextToVideoOutput(video: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_video.py#L61)

Outputs of inference for the Image Text To Video task

#### huggingface_hub.ImageTextToVideoParameters[[huggingface_hub.ImageTextToVideoParameters]]

```python
huggingface_hub.ImageTextToVideoParameters(guidance_scale: float | None = None, negative_prompt: str | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, prompt: str | None = None, seed: int | None = None, target_size: huggingface_hub.inference._generated.types.image_text_to_video.ImageTextToVideoTargetSize | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_video.py#L20)

Additional inference parameters for Image Text To Video

#### huggingface_hub.ImageTextToVideoTargetSize[[huggingface_hub.ImageTextToVideoTargetSize]]

```python
huggingface_hub.ImageTextToVideoTargetSize(height: int, width: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_text_to_video.py#L12)

The size in pixel of the output video frames.

## image_to_image[[huggingface_hub.ImageToImageInput]]

#### huggingface_hub.ImageToImageInput[[huggingface_hub.ImageToImageInput]]

```python
huggingface_hub.ImageToImageInput(inputs: str, parameters: huggingface_hub.inference._generated.types.image_to_image.ImageToImageParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_image.py#L44)

Inputs for Image To Image inference

#### huggingface_hub.ImageToImageOutput[[huggingface_hub.ImageToImageOutput]]

```python
huggingface_hub.ImageToImageOutput(image: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_image.py#L56)

Outputs of inference for the Image To Image task

#### huggingface_hub.ImageToImageParameters[[huggingface_hub.ImageToImageParameters]]

```python
huggingface_hub.ImageToImageParameters(guidance_scale: float | None = None, negative_prompt: str | None = None, num_inference_steps: int | None = None, prompt: str | None = None, target_size: huggingface_hub.inference._generated.types.image_to_image.ImageToImageTargetSize | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_image.py#L22)

Additional inference parameters for Image To Image

#### huggingface_hub.ImageToImageTargetSize[[huggingface_hub.ImageToImageTargetSize]]

```python
huggingface_hub.ImageToImageTargetSize(height: int, width: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_image.py#L12)

The size in pixels of the output image. This parameter is only supported by some
providers and for specific models. It will be ignored when unsupported.

## image_to_text[[huggingface_hub.ImageToTextGenerationParameters]]

#### huggingface_hub.ImageToTextGenerationParameters[[huggingface_hub.ImageToTextGenerationParameters]]

```python
huggingface_hub.ImageToTextGenerationParameters(do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('ImageToTextEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_text.py#L15)

Parametrization of the text generation process

#### huggingface_hub.ImageToTextInput[[huggingface_hub.ImageToTextInput]]

```python
huggingface_hub.ImageToTextInput(inputs: typing.Any, parameters: huggingface_hub.inference._generated.types.image_to_text.ImageToTextParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_text.py#L85)

Inputs for Image To Text inference

#### huggingface_hub.ImageToTextOutput[[huggingface_hub.ImageToTextOutput]]

```python
huggingface_hub.ImageToTextOutput(generated_text: typing.Any, image_to_text_output_generated_text: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_text.py#L95)

Outputs of inference for the Image To Text task

#### huggingface_hub.ImageToTextParameters[[huggingface_hub.ImageToTextParameters]]

```python
huggingface_hub.ImageToTextParameters(generation_parameters: huggingface_hub.inference._generated.types.image_to_text.ImageToTextGenerationParameters | None = None, max_new_tokens: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_text.py#L75)

Additional inference parameters for Image To Text

## image_to_video[[huggingface_hub.ImageToVideoInput]]

#### huggingface_hub.ImageToVideoInput[[huggingface_hub.ImageToVideoInput]]

```python
huggingface_hub.ImageToVideoInput(inputs: str, parameters: huggingface_hub.inference._generated.types.image_to_video.ImageToVideoParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_video.py#L44)

Inputs for Image To Video inference

#### huggingface_hub.ImageToVideoOutput[[huggingface_hub.ImageToVideoOutput]]

```python
huggingface_hub.ImageToVideoOutput(video: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_video.py#L56)

Outputs of inference for the Image To Video task

#### huggingface_hub.ImageToVideoParameters[[huggingface_hub.ImageToVideoParameters]]

```python
huggingface_hub.ImageToVideoParameters(guidance_scale: float | None = None, negative_prompt: str | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, prompt: str | None = None, seed: int | None = None, target_size: huggingface_hub.inference._generated.types.image_to_video.ImageToVideoTargetSize | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_video.py#L20)

Additional inference parameters for Image To Video

#### huggingface_hub.ImageToVideoTargetSize[[huggingface_hub.ImageToVideoTargetSize]]

```python
huggingface_hub.ImageToVideoTargetSize(height: int, width: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/image_to_video.py#L12)

The size in pixel of the output video frames.

## object_detection[[huggingface_hub.ObjectDetectionBoundingBox]]

#### huggingface_hub.ObjectDetectionBoundingBox[[huggingface_hub.ObjectDetectionBoundingBox]]

```python
huggingface_hub.ObjectDetectionBoundingBox(xmax: int, xmin: int, ymax: int, ymin: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/object_detection.py#L30)

The predicted bounding box. Coordinates are relative to the top left corner of the input
image.

#### huggingface_hub.ObjectDetectionInput[[huggingface_hub.ObjectDetectionInput]]

```python
huggingface_hub.ObjectDetectionInput(inputs: str, parameters: huggingface_hub.inference._generated.types.object_detection.ObjectDetectionParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/object_detection.py#L18)

Inputs for Object Detection inference

#### huggingface_hub.ObjectDetectionOutputElement[[huggingface_hub.ObjectDetectionOutputElement]]

```python
huggingface_hub.ObjectDetectionOutputElement(box: ObjectDetectionBoundingBox, label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/object_detection.py#L46)

Outputs of inference for the Object Detection task

#### huggingface_hub.ObjectDetectionParameters[[huggingface_hub.ObjectDetectionParameters]]

```python
huggingface_hub.ObjectDetectionParameters(threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/object_detection.py#L10)

Additional inference parameters for Object Detection

## question_answering[[huggingface_hub.QuestionAnsweringInput]]

#### huggingface_hub.QuestionAnsweringInput[[huggingface_hub.QuestionAnsweringInput]]

```python
huggingface_hub.QuestionAnsweringInput(inputs: QuestionAnsweringInputData, parameters: huggingface_hub.inference._generated.types.question_answering.QuestionAnsweringParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/question_answering.py#L52)

Inputs for Question Answering inference

#### huggingface_hub.QuestionAnsweringInputData[[huggingface_hub.QuestionAnsweringInputData]]

```python
huggingface_hub.QuestionAnsweringInputData(context: str, question: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/question_answering.py#L10)

One (context, question) pair to answer

#### huggingface_hub.QuestionAnsweringOutputElement[[huggingface_hub.QuestionAnsweringOutputElement]]

```python
huggingface_hub.QuestionAnsweringOutputElement(answer: str, end: int, score: float, start: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/question_answering.py#L62)

Outputs of inference for the Question Answering task

#### huggingface_hub.QuestionAnsweringParameters[[huggingface_hub.QuestionAnsweringParameters]]

```python
huggingface_hub.QuestionAnsweringParameters(align_to_words: bool | None = None, doc_stride: int | None = None, handle_impossible_answer: bool | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/question_answering.py#L20)

Additional inference parameters for Question Answering

## sentence_similarity[[huggingface_hub.SentenceSimilarityInput]]

#### huggingface_hub.SentenceSimilarityInput[[huggingface_hub.SentenceSimilarityInput]]

```python
huggingface_hub.SentenceSimilarityInput(inputs: SentenceSimilarityInputData, parameters: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/sentence_similarity.py#L22)

Inputs for Sentence similarity inference

#### huggingface_hub.SentenceSimilarityInputData[[huggingface_hub.SentenceSimilarityInputData]]

```python
huggingface_hub.SentenceSimilarityInputData(sentences: list, source_sentence: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/sentence_similarity.py#L12)

## summarization[[huggingface_hub.SummarizationInput]]

#### huggingface_hub.SummarizationInput[[huggingface_hub.SummarizationInput]]

```python
huggingface_hub.SummarizationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.summarization.SummarizationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/summarization.py#L27)

Inputs for Summarization inference

#### huggingface_hub.SummarizationOutput[[huggingface_hub.SummarizationOutput]]

```python
huggingface_hub.SummarizationOutput(summary_text: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/summarization.py#L37)

Outputs of inference for the Summarization task

#### huggingface_hub.SummarizationParameters[[huggingface_hub.SummarizationParameters]]

```python
huggingface_hub.SummarizationParameters(clean_up_tokenization_spaces: bool | None = None, generate_parameters: dict[str, typing.Any] | None = None, truncation: typing.Optional[ForwardRef('SummarizationTruncationStrategy')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/summarization.py#L15)

Additional inference parameters for summarization.

## table_question_answering[[huggingface_hub.TableQuestionAnsweringInput]]

#### huggingface_hub.TableQuestionAnsweringInput[[huggingface_hub.TableQuestionAnsweringInput]]

```python
huggingface_hub.TableQuestionAnsweringInput(inputs: TableQuestionAnsweringInputData, parameters: huggingface_hub.inference._generated.types.table_question_answering.TableQuestionAnsweringParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/table_question_answering.py#L40)

Inputs for Table Question Answering inference

#### huggingface_hub.TableQuestionAnsweringInputData[[huggingface_hub.TableQuestionAnsweringInputData]]

```python
huggingface_hub.TableQuestionAnsweringInputData(question: str, table: dict)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/table_question_answering.py#L12)

One (table, question) pair to answer

#### huggingface_hub.TableQuestionAnsweringOutputElement[[huggingface_hub.TableQuestionAnsweringOutputElement]]

```python
huggingface_hub.TableQuestionAnsweringOutputElement(answer: str, cells: list, coordinates: list, aggregator: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/table_question_answering.py#L50)

Outputs of inference for the Table Question Answering task

#### huggingface_hub.TableQuestionAnsweringParameters[[huggingface_hub.TableQuestionAnsweringParameters]]

```python
huggingface_hub.TableQuestionAnsweringParameters(padding: typing.Optional[ForwardRef('Padding')] = None, sequential: bool | None = None, truncation: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/table_question_answering.py#L25)

Additional inference parameters for Table Question Answering

## text2text_generation[[huggingface_hub.Text2TextGenerationInput]]

#### huggingface_hub.Text2TextGenerationInput[[huggingface_hub.Text2TextGenerationInput]]

```python
huggingface_hub.Text2TextGenerationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text2text_generation.Text2TextGenerationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text2text_generation.py#L27)

Inputs for Text2text Generation inference

#### huggingface_hub.Text2TextGenerationOutput[[huggingface_hub.Text2TextGenerationOutput]]

```python
huggingface_hub.Text2TextGenerationOutput(generated_text: typing.Any, text2_text_generation_output_generated_text: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text2text_generation.py#L37)

Outputs of inference for the Text2text Generation task

#### huggingface_hub.Text2TextGenerationParameters[[huggingface_hub.Text2TextGenerationParameters]]

```python
huggingface_hub.Text2TextGenerationParameters(clean_up_tokenization_spaces: bool | None = None, generate_parameters: dict[str, typing.Any] | None = None, truncation: typing.Optional[ForwardRef('Text2TextGenerationTruncationStrategy')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text2text_generation.py#L15)

Additional inference parameters for Text2text Generation

## text_classification[[huggingface_hub.TextClassificationInput]]

#### huggingface_hub.TextClassificationInput[[huggingface_hub.TextClassificationInput]]

```python
huggingface_hub.TextClassificationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_classification.TextClassificationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_classification.py#L25)

Inputs for Text Classification inference

#### huggingface_hub.TextClassificationOutputElement[[huggingface_hub.TextClassificationOutputElement]]

```python
huggingface_hub.TextClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_classification.py#L35)

Outputs of inference for the Text Classification task

#### huggingface_hub.TextClassificationParameters[[huggingface_hub.TextClassificationParameters]]

```python
huggingface_hub.TextClassificationParameters(function_to_apply: typing.Optional[ForwardRef('TextClassificationOutputTransform')] = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_classification.py#L15)

Additional inference parameters for Text Classification

## text_generation[[huggingface_hub.TextGenerationInput]]

#### huggingface_hub.TextGenerationInput[[huggingface_hub.TextGenerationInput]]

```python
huggingface_hub.TextGenerationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_generation.TextGenerationInputGenerateParameters | None = None, stream: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L76)

Text Generation Input.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.TextGenerationInputGenerateParameters[[huggingface_hub.TextGenerationInputGenerateParameters]]

```python
huggingface_hub.TextGenerationInputGenerateParameters(adapter_id: str | None = None, best_of: int | None = None, decoder_input_details: bool | None = None, details: bool | None = None, do_sample: bool | None = None, frequency_penalty: float | None = None, grammar: huggingface_hub.inference._generated.types.text_generation.TextGenerationInputGrammarType | None = None, max_new_tokens: int | None = None, repetition_penalty: float | None = None, return_full_text: bool | None = None, seed: int | None = None, stop: list[str] | None = None, temperature: float | None = None, top_k: int | None = None, top_n_tokens: int | None = None, top_p: float | None = None, truncate: int | None = None, typical_p: float | None = None, watermark: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L25)

#### huggingface_hub.TextGenerationInputGrammarType[[huggingface_hub.TextGenerationInputGrammarType]]

```python
huggingface_hub.TextGenerationInputGrammarType(type: TypeEnum, value: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L15)

#### huggingface_hub.TextGenerationOutput[[huggingface_hub.TextGenerationOutput]]

```python
huggingface_hub.TextGenerationOutput(generated_text: str, details: huggingface_hub.inference._generated.types.text_generation.TextGenerationOutputDetails | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L129)

Text Generation Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.TextGenerationOutputBestOfSequence[[huggingface_hub.TextGenerationOutputBestOfSequence]]

```python
huggingface_hub.TextGenerationOutputBestOfSequence(finish_reason: TextGenerationOutputFinishReason, generated_text: str, generated_tokens: int, prefill: list, tokens: list, seed: int | None = None, top_tokens: list[list[huggingface_hub.inference._generated.types.text_generation.TextGenerationOutputToken]] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L107)

#### huggingface_hub.TextGenerationOutputDetails[[huggingface_hub.TextGenerationOutputDetails]]

```python
huggingface_hub.TextGenerationOutputDetails(finish_reason: TextGenerationOutputFinishReason, generated_tokens: int, prefill: list, tokens: list, best_of_sequences: list[huggingface_hub.inference._generated.types.text_generation.TextGenerationOutputBestOfSequence] | None = None, seed: int | None = None, top_tokens: list[list[huggingface_hub.inference._generated.types.text_generation.TextGenerationOutputToken]] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L118)

#### huggingface_hub.TextGenerationOutputPrefillToken[[huggingface_hub.TextGenerationOutputPrefillToken]]

```python
huggingface_hub.TextGenerationOutputPrefillToken(id: int, logprob: float, text: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L92)

#### huggingface_hub.TextGenerationOutputToken[[huggingface_hub.TextGenerationOutputToken]]

```python
huggingface_hub.TextGenerationOutputToken(id: int, logprob: float, special: bool, text: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L99)

#### huggingface_hub.TextGenerationStreamOutput[[huggingface_hub.TextGenerationStreamOutput]]

```python
huggingface_hub.TextGenerationStreamOutput(index: int, token: TextGenerationStreamOutputToken, details: huggingface_hub.inference._generated.types.text_generation.TextGenerationStreamOutputStreamDetails | None = None, generated_text: str | None = None, top_tokens: list[huggingface_hub.inference._generated.types.text_generation.TextGenerationStreamOutputToken] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L157)

Text Generation Stream Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

#### huggingface_hub.TextGenerationStreamOutputStreamDetails[[huggingface_hub.TextGenerationStreamOutputStreamDetails]]

```python
huggingface_hub.TextGenerationStreamOutputStreamDetails(finish_reason: TextGenerationOutputFinishReason, generated_tokens: int, input_length: int, seed: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L141)

#### huggingface_hub.TextGenerationStreamOutputToken[[huggingface_hub.TextGenerationStreamOutputToken]]

```python
huggingface_hub.TextGenerationStreamOutputToken(id: int, logprob: float, special: bool, text: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_generation.py#L149)

## text_to_audio[[huggingface_hub.TextToAudioGenerationParameters]]

#### huggingface_hub.TextToAudioGenerationParameters[[huggingface_hub.TextToAudioGenerationParameters]]

```python
huggingface_hub.TextToAudioGenerationParameters(do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('TextToAudioEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_audio.py#L15)

Parametrization of the text generation process

#### huggingface_hub.TextToAudioInput[[huggingface_hub.TextToAudioInput]]

```python
huggingface_hub.TextToAudioInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_to_audio.TextToAudioParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_audio.py#L83)

Inputs for Text To Audio inference

#### huggingface_hub.TextToAudioOutput[[huggingface_hub.TextToAudioOutput]]

```python
huggingface_hub.TextToAudioOutput(audio: typing.Any, sampling_rate: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_audio.py#L93)

Outputs of inference for the Text To Audio task

#### huggingface_hub.TextToAudioParameters[[huggingface_hub.TextToAudioParameters]]

```python
huggingface_hub.TextToAudioParameters(generation_parameters: huggingface_hub.inference._generated.types.text_to_audio.TextToAudioGenerationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_audio.py#L75)

Additional inference parameters for Text To Audio

## text_to_image[[huggingface_hub.TextToImageInput]]

#### huggingface_hub.TextToImageInput[[huggingface_hub.TextToImageInput]]

```python
huggingface_hub.TextToImageInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_to_image.TextToImageParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_image.py#L36)

Inputs for Text To Image inference

#### huggingface_hub.TextToImageOutput[[huggingface_hub.TextToImageOutput]]

```python
huggingface_hub.TextToImageOutput(image: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_image.py#L46)

Outputs of inference for the Text To Image task

#### huggingface_hub.TextToImageParameters[[huggingface_hub.TextToImageParameters]]

```python
huggingface_hub.TextToImageParameters(guidance_scale: float | None = None, height: int | None = None, negative_prompt: str | None = None, num_inference_steps: int | None = None, scheduler: str | None = None, seed: int | None = None, width: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_image.py#L12)

Additional inference parameters for Text To Image

## text_to_speech[[huggingface_hub.TextToSpeechGenerationParameters]]

#### huggingface_hub.TextToSpeechGenerationParameters[[huggingface_hub.TextToSpeechGenerationParameters]]

```python
huggingface_hub.TextToSpeechGenerationParameters(do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('TextToSpeechEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_speech.py#L15)

Parametrization of the text generation process

#### huggingface_hub.TextToSpeechInput[[huggingface_hub.TextToSpeechInput]]

```python
huggingface_hub.TextToSpeechInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_to_speech.TextToSpeechParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_speech.py#L83)

Inputs for Text To Speech inference

#### huggingface_hub.TextToSpeechOutput[[huggingface_hub.TextToSpeechOutput]]

```python
huggingface_hub.TextToSpeechOutput(audio: typing.Any, sampling_rate: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_speech.py#L93)

Outputs of inference for the Text To Speech task

#### huggingface_hub.TextToSpeechParameters[[huggingface_hub.TextToSpeechParameters]]

```python
huggingface_hub.TextToSpeechParameters(generation_parameters: huggingface_hub.inference._generated.types.text_to_speech.TextToSpeechGenerationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_speech.py#L75)

Additional inference parameters for Text To Speech

## text_to_video[[huggingface_hub.TextToVideoInput]]

#### huggingface_hub.TextToVideoInput[[huggingface_hub.TextToVideoInput]]

```python
huggingface_hub.TextToVideoInput(inputs: str, parameters: huggingface_hub.inference._generated.types.text_to_video.TextToVideoParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_video.py#L32)

Inputs for Text To Video inference

#### huggingface_hub.TextToVideoOutput[[huggingface_hub.TextToVideoOutput]]

```python
huggingface_hub.TextToVideoOutput(video: typing.Any)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_video.py#L42)

Outputs of inference for the Text To Video task

#### huggingface_hub.TextToVideoParameters[[huggingface_hub.TextToVideoParameters]]

```python
huggingface_hub.TextToVideoParameters(guidance_scale: float | None = None, negative_prompt: list[str] | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, seed: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/text_to_video.py#L12)

Additional inference parameters for Text To Video

## token_classification[[huggingface_hub.TokenClassificationInput]]

#### huggingface_hub.TokenClassificationInput[[huggingface_hub.TokenClassificationInput]]

```python
huggingface_hub.TokenClassificationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.token_classification.TokenClassificationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/token_classification.py#L27)

Inputs for Token Classification inference

#### huggingface_hub.TokenClassificationOutputElement[[huggingface_hub.TokenClassificationOutputElement]]

```python
huggingface_hub.TokenClassificationOutputElement(end: int, score: float, start: int, word: str, entity: str | None = None, entity_group: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/token_classification.py#L37)

Outputs of inference for the Token Classification task

#### huggingface_hub.TokenClassificationParameters[[huggingface_hub.TokenClassificationParameters]]

```python
huggingface_hub.TokenClassificationParameters(aggregation_strategy: typing.Optional[ForwardRef('TokenClassificationAggregationStrategy')] = None, ignore_labels: list[str] | None = None, stride: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/token_classification.py#L15)

Additional inference parameters for Token Classification

## translation[[huggingface_hub.TranslationInput]]

#### huggingface_hub.TranslationInput[[huggingface_hub.TranslationInput]]

```python
huggingface_hub.TranslationInput(inputs: str, parameters: huggingface_hub.inference._generated.types.translation.TranslationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/translation.py#L35)

Inputs for Translation inference

#### huggingface_hub.TranslationOutput[[huggingface_hub.TranslationOutput]]

```python
huggingface_hub.TranslationOutput(translation_text: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/translation.py#L45)

Outputs of inference for the Translation task

#### huggingface_hub.TranslationParameters[[huggingface_hub.TranslationParameters]]

```python
huggingface_hub.TranslationParameters(clean_up_tokenization_spaces: bool | None = None, generate_parameters: dict[str, typing.Any] | None = None, src_lang: str | None = None, tgt_lang: str | None = None, truncation: typing.Optional[ForwardRef('TranslationTruncationStrategy')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/translation.py#L15)

Additional inference parameters for Translation

## video_classification[[huggingface_hub.VideoClassificationInput]]

#### huggingface_hub.VideoClassificationInput[[huggingface_hub.VideoClassificationInput]]

```python
huggingface_hub.VideoClassificationInput(inputs: typing.Any, parameters: huggingface_hub.inference._generated.types.video_classification.VideoClassificationParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/video_classification.py#L29)

Inputs for Video Classification inference

#### huggingface_hub.VideoClassificationOutputElement[[huggingface_hub.VideoClassificationOutputElement]]

```python
huggingface_hub.VideoClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/video_classification.py#L39)

Outputs of inference for the Video Classification task

#### huggingface_hub.VideoClassificationParameters[[huggingface_hub.VideoClassificationParameters]]

```python
huggingface_hub.VideoClassificationParameters(frame_sampling_rate: int | None = None, function_to_apply: typing.Optional[ForwardRef('VideoClassificationOutputTransform')] = None, num_frames: int | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/video_classification.py#L15)

Additional inference parameters for Video Classification

## visual_question_answering[[huggingface_hub.VisualQuestionAnsweringInput]]

#### huggingface_hub.VisualQuestionAnsweringInput[[huggingface_hub.VisualQuestionAnsweringInput]]

```python
huggingface_hub.VisualQuestionAnsweringInput(inputs: VisualQuestionAnsweringInputData, parameters: huggingface_hub.inference._generated.types.visual_question_answering.VisualQuestionAnsweringParameters | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/visual_question_answering.py#L33)

Inputs for Visual Question Answering inference

#### huggingface_hub.VisualQuestionAnsweringInputData[[huggingface_hub.VisualQuestionAnsweringInputData]]

```python
huggingface_hub.VisualQuestionAnsweringInputData(image: typing.Any, question: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/visual_question_answering.py#L12)

One (image, question) pair to answer

#### huggingface_hub.VisualQuestionAnsweringOutputElement[[huggingface_hub.VisualQuestionAnsweringOutputElement]]

```python
huggingface_hub.VisualQuestionAnsweringOutputElement(score: float, answer: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/visual_question_answering.py#L43)

Outputs of inference for the Visual Question Answering task

#### huggingface_hub.VisualQuestionAnsweringParameters[[huggingface_hub.VisualQuestionAnsweringParameters]]

```python
huggingface_hub.VisualQuestionAnsweringParameters(top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/visual_question_answering.py#L22)

Additional inference parameters for Visual Question Answering

## zero_shot_classification[[huggingface_hub.ZeroShotClassificationInput]]

#### huggingface_hub.ZeroShotClassificationInput[[huggingface_hub.ZeroShotClassificationInput]]

```python
huggingface_hub.ZeroShotClassificationInput(inputs: str, parameters: ZeroShotClassificationParameters)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_classification.py#L27)

Inputs for Zero Shot Classification inference

#### huggingface_hub.ZeroShotClassificationOutputElement[[huggingface_hub.ZeroShotClassificationOutputElement]]

```python
huggingface_hub.ZeroShotClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_classification.py#L37)

Outputs of inference for the Zero Shot Classification task

#### huggingface_hub.ZeroShotClassificationParameters[[huggingface_hub.ZeroShotClassificationParameters]]

```python
huggingface_hub.ZeroShotClassificationParameters(candidate_labels: list, hypothesis_template: str | None = None, multi_label: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_classification.py#L10)

Additional inference parameters for Zero Shot Classification

## zero_shot_image_classification[[huggingface_hub.ZeroShotImageClassificationInput]]

#### huggingface_hub.ZeroShotImageClassificationInput[[huggingface_hub.ZeroShotImageClassificationInput]]

```python
huggingface_hub.ZeroShotImageClassificationInput(inputs: str, parameters: ZeroShotImageClassificationParameters)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_image_classification.py#L22)

Inputs for Zero Shot Image Classification inference

#### huggingface_hub.ZeroShotImageClassificationOutputElement[[huggingface_hub.ZeroShotImageClassificationOutputElement]]

```python
huggingface_hub.ZeroShotImageClassificationOutputElement(label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_image_classification.py#L32)

Outputs of inference for the Zero Shot Image Classification task

#### huggingface_hub.ZeroShotImageClassificationParameters[[huggingface_hub.ZeroShotImageClassificationParameters]]

```python
huggingface_hub.ZeroShotImageClassificationParameters(candidate_labels: list, hypothesis_template: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_image_classification.py#L10)

Additional inference parameters for Zero Shot Image Classification

## zero_shot_object_detection[[huggingface_hub.ZeroShotObjectDetectionBoundingBox]]

#### huggingface_hub.ZeroShotObjectDetectionBoundingBox[[huggingface_hub.ZeroShotObjectDetectionBoundingBox]]

```python
huggingface_hub.ZeroShotObjectDetectionBoundingBox(xmax: int, xmin: int, ymax: int, ymin: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_object_detection.py#L28)

The predicted bounding box. Coordinates are relative to the top left corner of the input
image.

#### huggingface_hub.ZeroShotObjectDetectionInput[[huggingface_hub.ZeroShotObjectDetectionInput]]

```python
huggingface_hub.ZeroShotObjectDetectionInput(inputs: str, parameters: ZeroShotObjectDetectionParameters)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_object_detection.py#L18)

Inputs for Zero Shot Object Detection inference

#### huggingface_hub.ZeroShotObjectDetectionOutputElement[[huggingface_hub.ZeroShotObjectDetectionOutputElement]]

```python
huggingface_hub.ZeroShotObjectDetectionOutputElement(box: ZeroShotObjectDetectionBoundingBox, label: str, score: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_object_detection.py#L40)

Outputs of inference for the Zero Shot Object Detection task

#### huggingface_hub.ZeroShotObjectDetectionParameters[[huggingface_hub.ZeroShotObjectDetectionParameters]]

```python
huggingface_hub.ZeroShotObjectDetectionParameters(candidate_labels: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/types/zero_shot_object_detection.py#L10)

Additional inference parameters for Zero Shot Object Detection
