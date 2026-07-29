# Mistral 3

[Mistral 3](https://mistral.ai/news/mistral-small-3) is a latency optimized model with a lot fewer layers to reduce the time per forward pass. This model adds vision understanding and supports long context lengths of up to 128K tokens without compromising performance.

You can find the original Mistral 3 checkpoints under the [Mistral AI](https://huggingface.co/mistralai/models?search=mistral-small-3) organization.

> [!TIP]
> This model was contributed by [cyrilvallez](https://huggingface.co/cyrilvallez) and [yonigozlan](https://huggingface.co/yonigozlan).
> Click on the Mistral3 models in the right sidebar for more examples of how to apply Mistral3 to different tasks.

The example below demonstrates how to generate text for an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) and the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

messages = [
    {"role": "user",
        "content":[
            {"type": "image",
            "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg",},
            {"type": "text", "text": "Describe this image."}
        ,]
    ,}
,]

pipeline = pipeline(
    task="image-text-to-text",
    model="mistralai/Mistral-Small-3.1-24B-Instruct-2503",
    device=0
)
outputs = pipeline(text=messages, max_new_tokens=50, return_full_text=False)

outputs[0]["generated_text"]
'The image depicts a vibrant and lush garden scene featuring a variety of wildflowers and plants. The central focus is on a large, pinkish-purple flower, likely a Greater Celandine (Chelidonium majus), with a'
```

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model_checkpoint = "mistralai/Mistral-Small-3.1-24B-Instruct-2503"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(
    model_checkpoint,
    device_map="auto",
)

messages = [
    {"role": "user",
        "content":[
            {"type": "image",
            "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg",},
            {"type": "text", "text": "Describe this image."}
        ,]
    ,}
,]

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    tokenize=True, return_dict=True,
    return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=20)
decoded_output = processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True)

decoded_output
'The image depicts a vibrant and lush garden scene featuring a variety of wildflowers and plants. The central focus is on a large, pinkish-purple flower, likely a Greater Celandine (Chelidonium majus), with a'
```

## Notes

- Mistral 3 supports text-only generation.

```py
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText

model_checkpoint = ".mistralai/Mistral-Small-3.1-24B-Instruct-2503"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

SYSTEM_PROMPT = "You are a conversational agent that always answers straight to the point, always end your accurate response with an ASCII drawing of a cat."
user_prompt = "Give me 5 non-formal ways to say 'See you later' in French."

messages = [
    {"role": "system", "content": SYSTEM_PROMPT},
    {"role": "user", "content": user_prompt},
]

text = processor.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
inputs = processor(text=text, return_tensors="pt").to(0)
generate_ids = model.generate(**inputs, max_new_tokens=50, do_sample=False)
decoded_output = processor.batch_decode(generate_ids[:, inputs["input_ids"].shape[1] :], skip_special_tokens=True)[0]

print(decoded_output)
"1. À plus tard!
 2. Salut, à plus!
 3. À toute!
 4. À la prochaine!
 5. Je me casse, à plus!

```

 /\_/\
( o.o )
 > ^ <

```"
````

- Mistral 3 accepts batched image and text inputs.

```py
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText

model_checkpoint = "mistralai/Mistral-Small-3.1-24B-Instruct-2503"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
     [
         {
             "role": "user",
             "content": [
                 {"type": "image", "url": "https://llava-vl.github.io/static/images/view.jpg"},
                 {"type": "text", "text": "Write a haiku for this image"},
             ],
         },
     ],
     [
         {
             "role": "user",
             "content": [
                 {"type": "image", "url": "https://www.ilankelman.org/stopsigns/australia.jpg"},
                 {"type": "text", "text": "Describe this image"},
             ],
         },
     ],
 ]

 inputs = processor.apply_chat_template(messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

 output = model.generate(**inputs, max_new_tokens=25)

 decoded_outputs = processor.batch_decode(output, skip_special_tokens=True)
 decoded_outputs
["Write a haiku for this imageCalm waters reflect\nWhispers of the forest's breath\nPeace on wooden path"
, "Describe this imageThe image depicts a vibrant street scene in what appears to be a Chinatown district. The focal point is a traditional Chinese"]
```

- Mistral 3 also supported batched image and text inputs with a different number of images for each text. The example below quantizes the model with bitsandbytes.

```py
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText, BitsAndBytesConfig

model_checkpoint = "mistralai/Mistral-Small-3.1-24B-Instruct-2503"
processor = AutoProcessor.from_pretrained(model_checkpoint)
quantization_config = BitsAndBytesConfig(load_in_4bit=True)
model = AutoModelForImageTextToText.from_pretrained(
     model_checkpoint, quantization_config=quantization_config
 device_map="auto")

messages = [
     [
         {
             "role": "user",
             "content": [
                 {"type": "image", "url": "https://llava-vl.github.io/static/images/view.jpg"},
                 {"type": "text", "text": "Write a haiku for this image"},
             ],
         },
     ],
     [
         {
             "role": "user",
             "content": [
                 {"type": "image", "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"},
                 {"type": "image", "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg"},
                 {"type": "text", "text": "These images depict two different landmarks. Can you identify them?"},
             ],
         },
     ],
 ]

 inputs = processor.apply_chat_template(messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

 output = model.generate(**inputs, max_new_tokens=25)

 decoded_outputs = processor.batch_decode(output, skip_special_tokens=True)
 decoded_outputs
["Write a haiku for this imageSure, here is a haiku inspired by the image:\n\nCalm lake's wooden path\nSilent forest stands guard\n", "These images depict two different landmarks. Can you identify them? Certainly! The images depict two iconic landmarks:\n\n1. The first image shows the Statue of Liberty in New York City."]
```

## Mistral3Config[[transformers.Mistral3Config]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `10`) --
  The image token index used as a placeholder for input images.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **multimodal_projector_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use bias in the multimodal projector.
- **spatial_merge_size** (`int`, *optional*, defaults to `2`) --
  The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Mistral3Model. It is used to instantiate a Mistral3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [mistralai/Mistral-Small-3.1-24B-Instruct-2503](https://huggingface.co/mistralai/Mistral-Small-3.1-24B-Instruct-2503)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Mistral3ForConditionalGeneration, Mistral3Config, PixtralVisionConfig, MistralConfig

>>> # Initializing a Pixtral-vision config
>>> vision_config = PixtralVisionConfig()

>>> # Initializing a Mistral config
>>> text_config = MistralConfig()

>>> # Initializing a Mistral3 configuration
>>> configuration = Mistral3Config(vision_config, text_config)

>>> # Initializing a model from the mistral3.1 configuration
>>> model = Mistral3ForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MistralCommonBackend[[transformers.MistralCommonBackend]]

## Mistral3Model[[transformers.Mistral3Model]]

- **config** ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mistral3 model which consists of a vision backbone and a language model, without a language modeling head.

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
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor). See `PixtralImageProcessor.__call__()` for details ([PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) uses
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.`Mistral3ModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Mistral3ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) and inputs.
The [Mistral3Model](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

)>"}, {"name": "vision_feature_layer", "val": ": int | list[int] | None = None"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor). See `PixtralImageProcessor.__call__()` for details ([PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) uses
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) for processing images).
- **image_sizes** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) and inputs.
Obtains image last hidden states from the vision tower and apply multimodal projection.

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

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## Mistral3ForConditionalGeneration[[transformers.Mistral3ForConditionalGeneration]]

- **config** ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MISTRAL3 model which consists of a vision backbone and a language model.

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
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor). See `PixtralImageProcessor.__call__()` for details ([PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) uses
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.`Mistral3CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Mistral3CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) and inputs.
The [Mistral3ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Mistral3ForConditionalGeneration

>>> model = Mistral3ForConditionalGeneration.from_pretrained("mistralai/Mistral-Small-3.1-24B-Instruct-2503")
>>> processor = AutoProcessor.from_pretrained("mistralai/Mistral-Small-3.1-24B-Instruct-2503")

>>> prompt = "<s>[INST][IMG]What is the image?[/INST]"
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_new_tokens=15)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"What is the image?The image depicts two cats lying on a pink blanket."
```

)>"}, {"name": "vision_feature_layer", "val": ": int | list[int] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor). See `PixtralImageProcessor.__call__()` for details ([PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) uses
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) for processing images).
- **image_sizes** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Mistral3Config](/docs/transformers/v5.14.0/en/model_doc/mistral3#transformers.Mistral3Config)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Mistral3ForConditionalGeneration

>>> model = Mistral3ForConditionalGeneration.from_pretrained("mistralai/Mistral-Small-3.1-24B-Instruct-2503")
>>> processor = AutoProcessor.from_pretrained("mistralai/Mistral-Small-3.1-24B-Instruct-2503")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```
