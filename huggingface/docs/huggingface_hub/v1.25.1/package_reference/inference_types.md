# Inference types

This page lists the types (e.g. dataclasses) available for each task supported on the Hugging Face Hub.
Each task is specified using a JSON schema, and the types are generated from these schemas - with some customization
due to Python requirements.
Visit [@huggingface.js/tasks](https://github.com/huggingface/huggingface.js/tree/main/packages/tasks/src/tasks)
to find the JSON schemas for each task.

This part of the lib is still under development and will be improved in future releases.

## audio_classification[[huggingface_hub.AudioClassificationInput]]

Inputs for Audio Classification inference

Outputs for Audio Classification inference

Additional inference parameters for Audio Classification

## audio_to_audio[[huggingface_hub.AudioToAudioInput]]

Inputs for Audio to Audio inference

Outputs of inference for the Audio To Audio task
A generated audio file with its label.

## automatic_speech_recognition[[huggingface_hub.AutomaticSpeechRecognitionGenerationParameters]]

Parametrization of the text generation process

Inputs for Automatic Speech Recognition inference

Outputs of inference for the Automatic Speech Recognition task

Additional inference parameters for Automatic Speech Recognition

## chat_completion[[huggingface_hub.ChatCompletionInput]]

Chat Completion Input.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

Chat Completion Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

Chat Completion Stream Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

## depth_estimation[[huggingface_hub.DepthEstimationInput]]

Inputs for Depth Estimation inference

Outputs of inference for the Depth Estimation task

## document_question_answering[[huggingface_hub.DocumentQuestionAnsweringInput]]

Inputs for Document Question Answering inference

One (document, question) pair to answer

Outputs of inference for the Document Question Answering task

Additional inference parameters for Document Question Answering

## feature_extraction[[huggingface_hub.FeatureExtractionInput]]

Feature Extraction Input.
Auto-generated from TEI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tei-import.ts.

## fill_mask[[huggingface_hub.FillMaskInput]]

Inputs for Fill Mask inference

Outputs of inference for the Fill Mask task

Additional inference parameters for Fill Mask

## image_classification[[huggingface_hub.ImageClassificationInput]]

Inputs for Image Classification inference

Outputs of inference for the Image Classification task

Additional inference parameters for Image Classification

## image_segmentation[[huggingface_hub.ImageSegmentationInput]]

Inputs for Image Segmentation inference

Outputs of inference for the Image Segmentation task
A predicted mask / segment

Additional inference parameters for Image Segmentation

## image_text_to_image[[huggingface_hub.ImageTextToImageInput]]

Inputs for Image Text To Image inference. Either inputs (image) or prompt (in parameters)
must be provided, or both.

Outputs of inference for the Image Text To Image task

Additional inference parameters for Image Text To Image

The size in pixels of the output image. This parameter is only supported by some
providers and for specific models. It will be ignored when unsupported.

## image_text_to_video[[huggingface_hub.ImageTextToVideoInput]]

Inputs for Image Text To Video inference. Either inputs (image) or prompt (in parameters)
must be provided, or both.

Outputs of inference for the Image Text To Video task

Additional inference parameters for Image Text To Video

The size in pixel of the output video frames.

## image_to_image[[huggingface_hub.ImageToImageInput]]

Inputs for Image To Image inference

Outputs of inference for the Image To Image task

Additional inference parameters for Image To Image

The size in pixels of the output image. This parameter is only supported by some
providers and for specific models. It will be ignored when unsupported.

## image_to_text[[huggingface_hub.ImageToTextGenerationParameters]]

Parametrization of the text generation process

Inputs for Image To Text inference

Outputs of inference for the Image To Text task

Additional inference parameters for Image To Text

## image_to_video[[huggingface_hub.ImageToVideoInput]]

Inputs for Image To Video inference

Outputs of inference for the Image To Video task

Additional inference parameters for Image To Video

The size in pixel of the output video frames.

## object_detection[[huggingface_hub.ObjectDetectionBoundingBox]]

The predicted bounding box. Coordinates are relative to the top left corner of the input
image.

Inputs for Object Detection inference

Outputs of inference for the Object Detection task

Additional inference parameters for Object Detection

## question_answering[[huggingface_hub.QuestionAnsweringInput]]

Inputs for Question Answering inference

One (context, question) pair to answer

Outputs of inference for the Question Answering task

Additional inference parameters for Question Answering

## sentence_similarity[[huggingface_hub.SentenceSimilarityInput]]

Inputs for Sentence similarity inference

## summarization[[huggingface_hub.SummarizationInput]]

Inputs for Summarization inference

Outputs of inference for the Summarization task

Additional inference parameters for summarization.

## table_question_answering[[huggingface_hub.TableQuestionAnsweringInput]]

Inputs for Table Question Answering inference

One (table, question) pair to answer

Outputs of inference for the Table Question Answering task

Additional inference parameters for Table Question Answering

## text2text_generation[[huggingface_hub.Text2TextGenerationInput]]

Inputs for Text2text Generation inference

Outputs of inference for the Text2text Generation task

Additional inference parameters for Text2text Generation

## text_classification[[huggingface_hub.TextClassificationInput]]

Inputs for Text Classification inference

Outputs of inference for the Text Classification task

Additional inference parameters for Text Classification

## text_generation[[huggingface_hub.TextGenerationInput]]

Text Generation Input.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

Text Generation Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

Text Generation Stream Output.
Auto-generated from TGI specs.
For more details, check out
https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/scripts/inference-tgi-import.ts.

## text_to_audio[[huggingface_hub.TextToAudioGenerationParameters]]

Parametrization of the text generation process

Inputs for Text To Audio inference

Outputs of inference for the Text To Audio task

Additional inference parameters for Text To Audio

## text_to_image[[huggingface_hub.TextToImageInput]]

Inputs for Text To Image inference

Outputs of inference for the Text To Image task

Additional inference parameters for Text To Image

## text_to_speech[[huggingface_hub.TextToSpeechGenerationParameters]]

Parametrization of the text generation process

Inputs for Text To Speech inference

Outputs of inference for the Text To Speech task

Additional inference parameters for Text To Speech

## text_to_video[[huggingface_hub.TextToVideoInput]]

Inputs for Text To Video inference

Outputs of inference for the Text To Video task

Additional inference parameters for Text To Video

## token_classification[[huggingface_hub.TokenClassificationInput]]

Inputs for Token Classification inference

Outputs of inference for the Token Classification task

Additional inference parameters for Token Classification

## translation[[huggingface_hub.TranslationInput]]

Inputs for Translation inference

Outputs of inference for the Translation task

Additional inference parameters for Translation

## video_classification[[huggingface_hub.VideoClassificationInput]]

Inputs for Video Classification inference

Outputs of inference for the Video Classification task

Additional inference parameters for Video Classification

## visual_question_answering[[huggingface_hub.VisualQuestionAnsweringInput]]

Inputs for Visual Question Answering inference

One (image, question) pair to answer

Outputs of inference for the Visual Question Answering task

Additional inference parameters for Visual Question Answering

## zero_shot_classification[[huggingface_hub.ZeroShotClassificationInput]]

Inputs for Zero Shot Classification inference

Outputs of inference for the Zero Shot Classification task

Additional inference parameters for Zero Shot Classification

## zero_shot_image_classification[[huggingface_hub.ZeroShotImageClassificationInput]]

Inputs for Zero Shot Image Classification inference

Outputs of inference for the Zero Shot Image Classification task

Additional inference parameters for Zero Shot Image Classification

## zero_shot_object_detection[[huggingface_hub.ZeroShotObjectDetectionBoundingBox]]

The predicted bounding box. Coordinates are relative to the top left corner of the input
image.

Inputs for Zero Shot Object Detection inference

Outputs of inference for the Zero Shot Object Detection task

Additional inference parameters for Zero Shot Object Detection
