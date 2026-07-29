# Normalized Configurations

Model configuration classes in 🤗 Transformers are not standardized. Although Transformers implements an `attribute_map` attribute that mitigates the issue to some extent, it does not make it easy to reason on common configuration attributes in the code.
[NormalizedConfig](/docs/optimum/main/en/utils/normalized_config#optimum.utils.NormalizedConfig) classes try to fix that by allowing access to the configuration
attribute they wrap in a standardized way.

## Base class[[optimum.utils.NormalizedConfig]]

While it is possible to create `NormalizedConfig` subclasses for common use-cases, it is also possible to overwrite
the `original attribute name -> normalized attribute name` mapping directly using the
`with_args()` class method.

#### optimum.utils.NormalizedConfig[[optimum.utils.NormalizedConfig]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/normalized_config.py#L25)

Handles the normalization of `PretrainedConfig` attribute names, allowing to access attributes in a general way.

**Parameters:**

config (`PretrainedConfig`) : The config to normalize.

## Existing normalized configurations[[optimum.utils.NormalizedTextConfig]]

#### optimum.utils.NormalizedTextConfig[[optimum.utils.NormalizedTextConfig]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/normalized_config.py#L87)

#### optimum.utils.NormalizedSeq2SeqConfig[[optimum.utils.NormalizedSeq2SeqConfig]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/normalized_config.py#L99)

#### optimum.utils.NormalizedVisionConfig[[optimum.utils.NormalizedVisionConfig]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/normalized_config.py#L106)

#### optimum.utils.NormalizedTextAndVisionConfig[[optimum.utils.NormalizedTextAndVisionConfig]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/utils/normalized_config.py#L125)
