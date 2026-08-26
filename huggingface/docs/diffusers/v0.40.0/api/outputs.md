# Outputs

All model outputs are subclasses of [BaseOutput](/docs/diffusers/v0.40.0/en/api/outputs#diffusers.utils.BaseOutput), data structures containing all the information returned by the model. The outputs can also be used as tuples or dictionaries.

For example:

```python
from diffusers import DDIMPipeline

pipeline = DDIMPipeline.from_pretrained("google/ddpm-cifar10-32")
outputs = pipeline()
```

The `outputs` object is a [ImagePipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/ddim#diffusers.ImagePipelineOutput) which means it has an image attribute.

You can access each attribute as you normally would or with a keyword lookup, and if that attribute is not returned by the model, you will get `None`:

```python
outputs.images
outputs["images"]
```

When considering the `outputs` object as a tuple, it only considers the attributes that don't have `None` values.
For instance, retrieving an image by indexing into it returns the tuple `(outputs.images)`:

```python
outputs[:1]
```

> [!TIP]
> To check a specific pipeline or model output, refer to its corresponding API documentation.

## BaseOutput[[diffusers.utils.BaseOutput]]

#### diffusers.utils.BaseOutput[[diffusers.utils.BaseOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/outputs.py#L40)

Base class for all model outputs as dataclass. Has a `__getitem__` that allows indexing by integer or slice (like a
tuple) or strings (like a dictionary) that will ignore the `None` attributes. Otherwise behaves like a regular
Python dictionary.

> [!WARNING] > You can't unpack a `BaseOutput` directly. Use the [to_tuple()](/docs/diffusers/v0.40.0/en/api/outputs#diffusers.utils.BaseOutput.to_tuple) method to convert
it to a tuple > first.

#### to_tuple[[diffusers.utils.BaseOutput.to_tuple]]

```python
to_tuple()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/outputs.py#L130)

Convert self to a tuple containing all the attributes/keys that are not `None`.

## ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

#### diffusers.ImagePipelineOutput[[diffusers.ImagePipelineOutput]]

```python
diffusers.ImagePipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L135)

**Parameters:**

images (`List[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

Output class for image pipelines.

## AudioPipelineOutput[[diffusers.AudioPipelineOutput]]

#### diffusers.AudioPipelineOutput[[diffusers.AudioPipelineOutput]]

```python
diffusers.AudioPipelineOutput(audios: ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/pipeline_utils.py#L149)

**Parameters:**

audios (`np.ndarray`) : List of denoised audio samples of a NumPy array of shape `(batch_size, num_channels, sample_rate)`.

Output class for audio pipelines.

## ImageTextPipelineOutput[[diffusers.ImageTextPipelineOutput]]

#### diffusers.ImageTextPipelineOutput[[diffusers.ImageTextPipelineOutput]]

```python
diffusers.ImageTextPipelineOutput(images: list[PIL.Image.Image] | numpy.ndarray | None, text: list[str] | list[list[str]] | None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/deprecated/unidiffuser/pipeline_unidiffuser.py#L48)

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : list of denoised PIL images of length `batch_size` or NumPy array of shape `(batch_size, height, width, num_channels)`.

text (`list[str]` or `list[list[str]]`) : list of generated text strings of length `batch_size` or a list of list of strings whose outer list has length `batch_size`.

Output class for joint image-text pipelines.
