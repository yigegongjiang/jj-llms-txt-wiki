# ShieldGemma 2

## Overview

The ShieldGemma 2 model was proposed in a [technical report](https://huggingface.co/papers/2504.01081) by Google. ShieldGemma 2, built on [Gemma 3](https://ai.google.dev/gemma/docs/core/model_card_3), is a 4 billion (4B) parameter model that checks the safety of both synthetic and natural images against key categories to help you build robust datasets and models. With this addition to the Gemma family of models, researchers and developers can now easily minimize the risk of harmful content in their models across key areas of harm as defined below:

- No Sexually Explicit content: The image shall not contain content that depicts explicit or graphic sexual acts (e.g., pornography, erotic nudity, depictions of rape or sexual assault).
- No Dangerous Content: The image shall not contain content that facilitates or encourages activities that could cause real-world harm (e.g., building firearms and explosive devices, promotion of terrorism, instructions for suicide).
- No Violence/Gore content: The image shall not contain content that depicts shocking, sensational, or gratuitous violence (e.g., excessive blood and gore, gratuitous violence against animals, extreme injury or moment of death).

We recommend using ShieldGemma 2 as an input filter to vision language models, or as an output filter of image generation systems. To train a robust image safety model, we curated training datasets of natural and synthetic images and instruction-tuned Gemma 3 to demonstrate strong performance.

This model was contributed by [Ryan Mullins](https://huggingface.co/RyanMullins).

## Usage Example

- ShieldGemma 2 provides a Processor that accepts a list of `images` and an optional list of `policies` as input, and constructs a batch of prompts as the product of these two lists using the provided chat template.
- You can extend ShieldGemma's built-in policies with the `custom_policies` argument to the Processor. Using the same key as one of the built-in policies will overwrite that policy with your custom definition.
- ShieldGemma 2 does not support the image cropping capabilities used by Gemma 3.

### Classification against Built-in Policies

```python
import requests
from PIL import Image

from transformers import AutoProcessor, ShieldGemma2ForImageClassification

model_id = "google/shieldgemma-2-4b-it"
model = ShieldGemma2ForImageClassification.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg"
image = Image.open(requests.get(url, stream=True).raw)

inputs = processor(images=[image], return_tensors="pt").to(model.device)

output = model(**inputs)
print(output.probabilities)
```

### Classification against Custom Policies

```python
import requests
from PIL import Image

from transformers import AutoProcessor, ShieldGemma2ForImageClassification

model_id = "google/shieldgemma-2-4b-it"
model = ShieldGemma2ForImageClassification.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg"
image = Image.open(requests.get(url, stream=True).raw)

custom_policies = {
    "key_a": "description_a",
    "key_b": "description_b",
}

inputs = processor(
    images=[image],
    custom_policies=custom_policies,
    policies=["dangerous", "key_a", "key_b"],
    return_tensors="pt",
).to(model.device)

output = model(**inputs)
print(output.probabilities)
```

## ShieldGemma2Processor[[transformers.ShieldGemma2Processor]]

- **images** -- A single image or a list of images to include in the batch.
- **text** -- Not supported.
- **videos** -- Not supported.
- **audio** -- Not supported.
- **kwargs** -- An optional dictionary of keyword arguments to configure the
  processor. Possible values include:

  *   `custom_policies`: Additional policy definitions that augment the `self.policy_definitions` passed
    into the constructor. Note that `custom_policies` that share a key with `self.policy_definitions`
    will override the policy description
  *   `policies`: (Optional) a list of keys in the joint `self.policy_definitions | custom_policies`
    dictionary of specific interest for the provided images. If empty or None, prompts will be
    generated for every key in the joint dictionary.A `BatchFeature` containing `input_ids`, `pixel_values`, etc. where each Tensor is of shape
`(len(images) * len(policies), )`, and the order within the batch will be
img1_policy1, ... img1_policyN, ... imgM_policyN.
Generates a batch of inputs from the provided images.

ShieldGemma was trained to classify image content for policy compliance using a specific prompt construction.
This processor generates a batch of such prompts from the provided images by:

1.  Creating a list of conversations, one for each `<image, policy>` pair;
2.  Converting these conversations to text using `self.apply_chat_template()`; and
3.  Encoding the conversations and images using the same techniques as `Gemma3Processor`.

## ShieldGemma2Config[[transformers.ShieldGemma2Config]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **mm_tokens_per_image** (`int`, *optional*, defaults to 256) --
  The number of tokens per image embedding.
- **boi_token_index** (`int`, *optional*, defaults to 255999) --
  The begin-of-image token index to wrap the image prompt.
- **eoi_token_index** (`int`, *optional*, defaults to 256000) --
  The end-of-image token index to wrap the image prompt.
- **image_token_index** (`int`, *optional*, defaults to `262144`) --
  The image token index used as a placeholder for input images.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Shieldgemma2Model. It is used to instantiate a Shieldgemma2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/shieldgemma-2-4b-it](https://huggingface.co/google/shieldgemma-2-4b-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ShieldGemma2ForConditionalGeneration, ShieldGemma2Config, SiglipVisionConfig, ShieldGemma2TextConfig

>>> # Initializing a Siglip-like vision config
>>> vision_config = SiglipVisionConfig()

>>> # Initializing a ShieldGemma2 Text config
>>> text_config = ShieldGemma2TextConfig()

>>> # Initializing a ShieldGemma2 gemma-3-4b style configuration
>>> configuration = ShieldGemma2Config(vision_config, text_config)

>>> # Initializing a model from the gemma-3-4b style configuration
>>> model = ShieldGemma2TextConfig(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ShieldGemma2ForImageClassification[[transformers.ShieldGemma2ForImageClassification]]

- **config** ([ShieldGemma2Config](/docs/transformers/v5.14.0/en/model_doc/shieldgemma2#transformers.ShieldGemma2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Shieldgemma2 Model with an image classification head on top e.g. for ImageNet.

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
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
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
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
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
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).A `ShieldGemma2ImageClassifierOutputWithNoAttention` instance containing the logits and probabilities
associated with the model predicting the `Yes` or `No` token as the response to that prompt, captured in the
following properties.

*   `logits` (`torch.Tensor` of shape `(batch_size, 2)`):
  The first position along dim=1 is the logits for the `Yes` token and the second position along dim=1 is
  the logits for the `No` token.
*   `probabilities` (`torch.Tensor` of shape `(batch_size, 2)`):
  The first position along dim=1 is the probability of predicting the `Yes` token and the second position
  along dim=1 is the probability of predicting the `No` token.

ShieldGemma prompts are constructed such that predicting the `Yes` token means the content *does violate* the
policy as described. If you are only interested in the violative condition, use
`violated = outputs.probabilities[:, 1]` to extract that slice from the output tensors.

When used with the `ShieldGemma2Processor`, the `batch_size` will be equal to `len(images) * len(policies)`,
and the order within the batch will be img1_policy1, ... img1_policyN, ... imgM_policyN.
The [ShieldGemma2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/shieldgemma2#transformers.ShieldGemma2ForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.
