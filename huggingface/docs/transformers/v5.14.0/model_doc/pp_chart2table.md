# PP-Chart2Table

## Overview

**PP-Chart2Table** is a SOTA multimodal model developed by the PaddlePaddle team, specializing in chart parsing for both Chinese and English. Its high performance is driven by a novel "Shuffled Chart Data Retrieval" training task, which, combined with a refined token masking strategy, significantly improves its efficiency in converting charts to data tables. The model is further strengthened by an advanced data synthesis pipeline that uses high-quality seed data, RAG, and LLMs persona design to create a richer, more diverse training set. To address the challenge of large-scale unlabeled, out-of-distribution (OOD) data, the team implemented a two-stage distillation process, ensuring robust adaptability and generalization on real-world data.

## Model Architecture 
PP-Chart2Table adopts a multimodal fusion architecture that combines a vision tower for chart feature extraction and a language model for table structure generation, enabling end-to-end chart-to-table conversion.

## Usage

### Single input inference

The example below demonstrates how to classify image with PP-Chart2Table using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
from transformers import pipeline

pipe = pipeline("image-text-to-text", model="PaddlePaddle/PP-Chart2Table_safetensors")

# PPChart2TableProcessor uses hardcoded "Chart to table" instruction internally via chat template
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/chart_parsing_02.png",
            },
        ],
    },
]
result = pipe(text=conversation)
print(result[0]["generated_text"])
```

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model_path = "PaddlePaddle/PP-Chart2Table_safetensors"
model = AutoModelForImageTextToText.from_pretrained(
    model_path,
    device_map="auto",
)
processor = AutoProcessor.from_pretrained(model_path)

# PPChart2TableProcessor uses hardcoded "Chart to table" instruction internally via chat template
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/chart_parsing_02.png",
            },
        ],
    },
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    add_generation_prompt=True,
    truncation=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

generated_ids = model.generate(**inputs, do_sample=False, max_new_tokens=256)
generated_ids_trimmed = [out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)]
result = processor.batch_decode(generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False)
print(result)
```

### Batched inference

Here is how you can do it with PP-Chart2Table using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
from transformers import pipeline

pipe = pipeline("image-text-to-text", model="PaddlePaddle/PP-Chart2Table_safetensors")

# PPChart2TableProcessor uses hardcoded "Chart to table" instruction internally via chat template
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/chart_parsing_02.png",
            },
        ],
    },
]
result = pipe(text=[conversation, conversation])
print(result[0][0]["generated_text"])
```

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model_path = "PaddlePaddle/PP-Chart2Table_safetensors"
model = AutoModelForImageTextToText.from_pretrained(
    model_path,
    device_map="auto",
)
processor = AutoProcessor.from_pretrained(model_path)

# PPChart2TableProcessor uses hardcoded "Chart to table" instruction internally via chat template
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/chart_parsing_02.png",
            },
        ],
    },
]

batch_conversation = [conversation, conversation]
inputs = processor.apply_chat_template(
    batch_conversation,
    tokenize=True,
    add_generation_prompt=True,
    truncation=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

generated_ids = model.generate(**inputs, do_sample=False, max_new_tokens=256)
generated_ids_trimmed = [out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)]
result = processor.batch_decode(generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False)
print(result)
```

## PPChart2TableConfig[[transformers.PPChart2TableConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `151859`) --
  The image token index used as a placeholder for input images.
- **image_seq_length** (`int`, *optional*, defaults to `576`) --
  Sequence length of one image embedding.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Pp Chart2TableModel. It is used to instantiate a Pp Chart2Table
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-Chart2Table_safetensors](https://huggingface.co/PaddlePaddle/PP-Chart2Table_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GotOcr2ForConditionalGeneration, PPChart2TableConfig

>>> # Initializing a PPChart2Table style configuration
>>> configuration = PPChart2TableConfig()

>>> # Initializing a model from the PaddlePaddle/PP-Chart2Table_safetensors style configuration
>>> model = GotOcr2ForConditionalGeneration(configuration)  # underlying architecture is Got Ocr 2

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PPChart2TableImageProcessor[[transformers.PPChart2TableImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPChart2TableImageProcessor image processor.

## PPChart2TableImageProcessorPil[[transformers.PPChart2TableImageProcessorPil]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPChart2TableImageProcessor image processor.

## PPChart2TableProcessor[[transformers.PPChart2TableProcessor]]

- **image_processor** (`PPChart2TableImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a PPChart2TableProcessor which wraps a image processor and a tokenizer into a single processor.

[PPChart2TableProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_chart2table#transformers.PPChart2TableProcessor) offers all the functionalities of [PPChart2TableImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_chart2table#transformers.PPChart2TableImageProcessor) and `tokenizer_class`. See the
[~PPChart2TableImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_chart2table#transformers.PPChart2TableImageProcessor) and `~tokenizer_class` for more information.
