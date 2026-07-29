# Dummy Input Generators

It is very common to have to generate dummy inputs to perform a task (tracing, exporting a model to some backend,
testing model outputs, etc). The goal of [DummyInputGenerator](/docs/optimum/main/en/utils/dummy_input_generators#optimum.utils.DummyInputGenerator) classes is to make this
generation easy and re-usable.

## Base class[[optimum.utils.DummyInputGenerator]]

#### optimum.utils.DummyInputGenerator[[optimum.utils.DummyInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L93)

Generates dummy inputs for the supported input names, in the requested framework.

concat_inputsoptimum.utils.DummyInputGenerator.concat_inputshttps://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L292[{"name": "inputs", "val": ""}, {"name": "dim", "val": ": int"}]- **inputs** --
  The list of tensors in a given framework to concatenate.
- **dim** (`int`) --
  The dimension along which to concatenate.0The tensor of the concatenation.

Concatenates inputs together.

**Parameters:**

inputs : The list of tensors in a given framework to concatenate.

dim (`int`) : The dimension along which to concatenate.

**Returns:**

The tensor of the concatenation.
#### constant_tensor[[optimum.utils.DummyInputGenerator.constant_tensor]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L245)

Generates a constant tensor.

**Parameters:**

shape (`List[int]`) : The shape of the constant tensor.

value (`Union[int, float]`, defaults to 1) : The value to fill the constant tensor with.

dtype (`Optional[Any]`, defaults to `None`) : The dtype of the constant tensor.

framework (`str`, defaults to `"pt"`) : The requested framework.

**Returns:**

A constant tensor in the requested framework.
#### generate[[optimum.utils.DummyInputGenerator.generate]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L114)

Generates the dummy input matching `input_name` for the requested framework.

**Parameters:**

input_name (`str`) : The name of the input to generate.

framework (`str`, defaults to `"pt"`) : The requested framework.

int_dtype (`str`, defaults to `"int64"`) : The dtypes of generated integer tensors.

float_dtype (`str`, defaults to `"fp32"`) : The dtypes of generated float tensors.

**Returns:**

A tensor in the requested framework of the input.
#### pad_input_on_dim[[optimum.utils.DummyInputGenerator.pad_input_on_dim]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L317)

Pads an input either to the desired length, or by a padding length.

**Parameters:**

input_ : The tensor to pad.

dim (`int`) : The dimension along which to pad.

desired_length (`Optional[int]`, defaults to `None`) : The desired length along the dimension after padding.

padding_length (`Optional[int]`, defaults to `None`) : The length to pad along the dimension.

value (`Union[int, float]`, defaults to 1) : The value to use for padding.

dtype (`Optional[Any]`, defaults to `None`) : The dtype of the padding.

**Returns:**

The padded tensor.
#### random_float_tensor[[optimum.utils.DummyInputGenerator.random_float_tensor]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L213)

Generates a tensor of random floats in the [min_value, max_value) range.

**Parameters:**

shape (`List[int]`) : The shape of the random tensor.

min_value (`float`, defaults to 0) : The minimum value allowed.

max_value (`float`, defaults to 1) : The maximum value allowed.

framework (`str`, defaults to `"pt"`) : The requested framework.

dtype (`str`, defaults to `"fp32"`) : The dtype of the generated float tensor. Could be "fp32", "fp16", "bf16".

**Returns:**

A random tensor in the requested framework.
#### random_int_tensor[[optimum.utils.DummyInputGenerator.random_int_tensor]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L134)

Generates a tensor of random integers in the [min_value, max_value) range.

**Parameters:**

shape (`List[int]`) : The shape of the random tensor.

max_value (`int`) : The maximum value allowed.

min_value (`int`, defaults to 0) : The minimum value allowed.

framework (`str`, defaults to `"pt"`) : The requested framework.

dtype (`str`, defaults to `"int64"`) : The dtype of the generated integer tensor. Could be "int64", "int32", "int8".

**Returns:**

A random tensor in the requested framework.
#### random_mask_tensor[[optimum.utils.DummyInputGenerator.random_mask_tensor]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L166)

Generates a mask tensor either right or left padded.

**Parameters:**

shape (`List[int]`) : The shape of the random tensor.

padding_side (`str`, defaults to "right") : The side on which the padding is applied.

framework (`str`, defaults to `"pt"`) : The requested framework.

dtype (`str`, defaults to `"int64"`) : The dtype of the generated integer tensor. Could be "int64", "int32", "int8".

**Returns:**

A random mask tensor either left padded or right padded in the requested framework.
#### supports_input[[optimum.utils.DummyInputGenerator.supports_input]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L100)

Checks whether the `DummyInputGenerator` supports the generation of the requested input.

**Parameters:**

input_name (`str`) : The name of the input to generate.

**Returns:**

``bool``

A boolean specifying whether the input is supported.

## Existing dummy input generators[[optimum.utils.DummyTextInputGenerator]]

#### optimum.utils.DummyTextInputGenerator[[optimum.utils.DummyTextInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L363)

Generates dummy encoder text inputs.

#### optimum.utils.DummyDecoderTextInputGenerator[[optimum.utils.DummyDecoderTextInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L519)

Generates dummy decoder text inputs.

#### optimum.utils.DummyPastKeyValuesGenerator[[optimum.utils.DummyPastKeyValuesGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L620)

Generates dummy past_key_values inputs.

#### optimum.utils.DummySeq2SeqPastKeyValuesGenerator[[optimum.utils.DummySeq2SeqPastKeyValuesGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L667)

Generates dummy past_key_values inputs for seq2seq architectures.

#### optimum.utils.DummyBboxInputGenerator[[optimum.utils.DummyBboxInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L755)

Generates dummy bbox inputs.

#### optimum.utils.DummyVisionInputGenerator[[optimum.utils.DummyVisionInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L795)

Generates dummy vision inputs.

#### optimum.utils.DummyAudioInputGenerator[[optimum.utils.DummyAudioInputGenerator]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/input_generators.py#L883)
