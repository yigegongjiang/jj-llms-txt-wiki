# pipelines

Pipelines provide a high-level, easy to use, API for running machine learning models.

**Example:** Instantiate pipeline using the `pipeline` function.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('sentiment-analysis');
const output = await classifier('I love transformers!');
// [{'label': 'POSITIVE', 'score': 0.999817686}]
```

* [pipelines](#module_pipelines)
    * _static_
        * [`.pipeline(task, [model], [options])`](#module_pipelines.pipeline) ⇒ Promise.&lt;AllTasks&gt;
            * [`~files_loading`](#module_pipelines.pipeline..files_loading) : FilesLoadingMap
            * [`~metadata`](#module_pipelines.pipeline..metadata) : Array.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;
    * _inner_
        * [`~AllTasks`](#module_pipelines..AllTasks) : string

* * *

## `pipelines.pipeline(task, [model], [options])` ⇒ Promise.&lt;AllTasks&gt;

Utility factory method to build a `Pipeline` object.

**Kind**: static method of [pipelines](#module_pipelines)  
**Returns**: Promise.&lt;AllTasks&gt; - A Pipeline object for the specified task.  
**Throws**:

- Error If an unsupported pipeline is requested.

  
    
      ParamTypeDefaultDescription
    
  
  

    taskTThe task defining which pipeline will be returned. Currently accepted tasks are:

&quot;audio-classification&quot;: will return a AudioClassificationPipeline.
&quot;automatic-speech-recognition&quot;: will return a AutomaticSpeechRecognitionPipeline.
&quot;background-removal&quot;: will return a BackgroundRemovalPipeline.
&quot;depth-estimation&quot;: will return a DepthEstimationPipeline.
&quot;document-question-answering&quot;: will return a DocumentQuestionAnsweringPipeline.
&quot;feature-extraction&quot;: will return a FeatureExtractionPipeline.
&quot;fill-mask&quot;: will return a FillMaskPipeline.
&quot;image-classification&quot;: will return a ImageClassificationPipeline.
&quot;image-segmentation&quot;: will return a ImageSegmentationPipeline.
&quot;image-to-text&quot;: will return a ImageToTextPipeline.
&quot;object-detection&quot;: will return a ObjectDetectionPipeline.
&quot;question-answering&quot;: will return a QuestionAnsweringPipeline.
&quot;summarization&quot;: will return a SummarizationPipeline.
&quot;text2text-generation&quot;: will return a Text2TextGenerationPipeline.
&quot;text-classification&quot; (alias &quot;sentiment-analysis&quot; available): will return a TextClassificationPipeline.
&quot;text-generation&quot;: will return a TextGenerationPipeline.
&quot;token-classification&quot; (alias &quot;ner&quot; available): will return a TokenClassificationPipeline.
&quot;translation&quot;: will return a TranslationPipeline.
&quot;translation_xx_to_yy&quot;: will return a TranslationPipeline.
&quot;zero-shot-classification&quot;: will return a ZeroShotClassificationPipeline.
&quot;zero-shot-audio-classification&quot;: will return a ZeroShotAudioClassificationPipeline.
&quot;zero-shot-image-classification&quot;: will return a ZeroShotImageClassificationPipeline.
&quot;zero-shot-object-detection&quot;: will return a ZeroShotObjectDetectionPipeline.

    
    [model]stringnullThe name of the pre-trained model to use. If not specified, the default model for the task will be used.

    
    [options]PretrainedModelOptionsOptional parameters for the pipeline.

      

* [`.pipeline(task, [model], [options])`](#module_pipelines.pipeline) ⇒ Promise.&lt;AllTasks&gt;
    * [`~files_loading`](#module_pipelines.pipeline..files_loading) : FilesLoadingMap
    * [`~metadata`](#module_pipelines.pipeline..metadata) : Array.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;

* * *

### `pipeline~files_loading` : FilesLoadingMap

**Kind**: inner property of [pipeline](#module_pipelines.pipeline)  

* * *

### `pipeline~metadata` : Array.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;

**Kind**: inner constant of [pipeline](#module_pipelines.pipeline)  

* * *

## `pipelines~AllTasks` : string

All possible pipeline types.

**Kind**: inner typedef of [pipelines](#module_pipelines)  

* * *
