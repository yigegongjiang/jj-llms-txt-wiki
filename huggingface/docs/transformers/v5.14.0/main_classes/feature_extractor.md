# Feature Extractor

A feature extractor is in charge of preparing input features for audio models. This includes feature extraction from sequences, e.g., pre-processing audio files to generate Log-Mel Spectrogram features, and conversion to NumPy and PyTorch tensors.

## FeatureExtractionMixin[[transformers.FeatureExtractionMixin]]

This is a feature extraction mixin used to provide saving/loading functionality for sequential and audio feature
extractors.

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  This can be either:

  - a string, the *model id* of a pretrained feature_extractor hosted inside a model repo on
    huggingface.co.
  - a path to a *directory* containing a feature extractor file saved using the
    [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) method, e.g.,
    `./my_model_directory/`.
  - a path to a saved feature extractor JSON *file*, e.g.,
    `./my_model_directory/preprocessor_config.json`.
- **cache_dir** (`str` or `os.PathLike`, *optional*) --
  Path to a directory in which a downloaded pretrained model feature extractor should be cached if the
  standard cache should not be used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force to (re-)download the feature extractor files and override the cached versions
  if they exist.
- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.
- **token** (`str` or `bool`, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use
  the token generated when running `hf auth login` (stored in `~/.huggingface`).
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a
  git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any
  identifier allowed by git.

  

  To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.

  

- **return_unused_kwargs** (`bool`, *optional*, defaults to `False`) --
  If `False`, then this function returns just the final feature extractor object. If `True`, then this
  functions returns a `Tuple(feature_extractor, unused_kwargs)` where *unused_kwargs* is a dictionary
  consisting of the key/value pairs whose keys are not feature extractor attributes: i.e., the part of
  `kwargs` which has not been used to update `feature_extractor` and is otherwise ignored.
- **kwargs** (`dict[str, Any]`, *optional*) --
  The values in kwargs of any keys which are feature extractor attributes will be used to override the
  loaded values. Behavior concerning key/value pairs whose keys are *not* feature extractor attributes is
  controlled by the `return_unused_kwargs` keyword parameter.A feature extractor of type [FeatureExtractionMixin](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin).

Instantiate a type of [FeatureExtractionMixin](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) from a feature extractor, *e.g.* a
derived class of [SequenceFeatureExtractor](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor).

Examples:

```python
# We can't instantiate directly the base class *FeatureExtractionMixin* nor *SequenceFeatureExtractor* so let's show the examples on a
# derived class: *Wav2Vec2FeatureExtractor*
feature_extractor = Wav2Vec2FeatureExtractor.from_pretrained(
    "facebook/wav2vec2-base-960h"
)  # Download feature_extraction_config from huggingface.co and cache.
feature_extractor = Wav2Vec2FeatureExtractor.from_pretrained(
    "./test/saved_model/"
)  # E.g. feature_extractor (or model) was saved using *save_pretrained('./test/saved_model/')*
feature_extractor = Wav2Vec2FeatureExtractor.from_pretrained("./test/saved_model/preprocessor_config.json")
feature_extractor = Wav2Vec2FeatureExtractor.from_pretrained(
    "facebook/wav2vec2-base-960h", return_attention_mask=False, foo=False
)
assert feature_extractor.return_attention_mask is False
feature_extractor, unused_kwargs = Wav2Vec2FeatureExtractor.from_pretrained(
    "facebook/wav2vec2-base-960h", return_attention_mask=False, foo=False, return_unused_kwargs=True
)
assert feature_extractor.return_attention_mask is False
assert unused_kwargs == {"foo": False}
```

- **save_directory** (`str` or `os.PathLike`) --
  Directory where the feature extractor JSON file will be saved (will be created if it does not exist).
- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the
  repository you want to push to with `repo_id` (will default to the name of `save_directory` in your
  namespace).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save a feature_extractor object to the directory `save_directory`, so that it can be re-loaded using the
[from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained) class method.

## SequenceFeatureExtractor[[transformers.SequenceFeatureExtractor]]

- **feature_size** (`int`) --
  The feature dimension of the extracted features.
- **sampling_rate** (`int`) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **padding_value** (`float`) --
  The value that is used to fill the padding values / vectors.

This is a general feature extraction class for speech recognition.

- **processed_features** ([BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature), list of [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature), `dict[str, list[float]]`, `dict[str, list[list[float]]` or `list[dict[str, list[float]]]`) --
  Processed inputs. Can represent one input ([BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) or `dict[str, list[float]]`) or a batch of
  input values / vectors (list of [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature), *dict[str, list[list[float]]]* or *list[dict[str,
  list[float]]]*) so you can use this method during preprocessing as well as in a PyTorch Dataloader
  collate function.

  Instead of `list[float]` you can have tensors (numpy arrays or PyTorch tensors),
  see the note above for the return type.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) --
  Select a strategy to pad the returned sequences (according to the model's padding side and padding
  index) among:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **max_length** (`int`, *optional*) --
  Maximum length of the returned list and optionally padding length (see above).
- **truncation** (`bool`) --
  Activates truncation to cut input sequences longer than `max_length` to `max_length`.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value.

  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability
  `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific feature_extractor's default.

  [What are attention masks?](../glossary#attention-mask)
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors instead of list of python integers. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return Numpy `np.ndarray` objects.

Pad input values / input vectors or a batch of input values / input vectors up to predefined length or to the
max sequence length in the batch.

Padding side (left/right) padding values are defined at the feature extractor level (with `self.padding_side`,
`self.padding_value`)

If the `processed_features` passed are dictionary of numpy arrays or PyTorch tensors  the
result will use the same type unless you provide a different tensor type with `return_tensors`. In the case of
PyTorch tensors, you will lose the specific device of your tensors however.

## BatchFeature[[transformers.BatchFeature]]

- **data** (`dict`, *optional*) --
  Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) --
  You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) --
  List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

Holds the output of the [pad()](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor.pad) and feature extractor specific `__call__` methods.

This class is derived from a python dictionary and can be used as a dictionary.

- **tensor_type** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  The type of tensors to use. If `str`, should be one of the values of the enum [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType). If
  `None`, no modification is done.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) --
  List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

Convert the inner content to tensors.

Note:
Values that don't have an array-like structure (e.g., strings, dicts, lists of strings) are
automatically skipped and won't be converted to tensors. Ragged arrays (lists of arrays with
different lengths) are still attempted, though they may raise errors during conversion.

- **args** (`Tuple`) --
  Will be passed to the `to(...)` function of the tensors.
- **kwargs** (`Dict`, *optional*) --
  Will be passed to the `to(...)` function of the tensors.
  To enable asynchronous data transfer, set the `non_blocking` flag in `kwargs` (defaults to `False`).[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)The same instance after modification.

Send all values to device by calling `v.to(*args, **kwargs)` (PyTorch only). This should support casting in
different `dtypes` and sending the `BatchFeature` to a different `device`.

## ImageFeatureExtractionMixin[[transformers.ImageFeatureExtractionMixin]]

Mixin that contain utilities for preparing image features.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor` of shape (n_channels, height, width) or (height, width, n_channels)) --
  The image to resize.
- **size** (`int` or `tuple[int, int]`) --
  The size to which crop the image.new_imageA center cropped `PIL.Image.Image` or `np.ndarray` or `torch.Tensor` of shape: (n_channels,
height, width).

Crops `image` to the given size using a center crop. Note that if the image is too small to be cropped to the
size given, it will be padded (so the returned result has the size asked).

- **image** (`PIL.Image.Image`) --
  The image to convert.

Converts `PIL.Image.Image` to RGB format.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to expand.

Expands 2-dimensional `image` to 3 dimensions.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image whose color channels to flip. If `np.ndarray` or `torch.Tensor`, the channel dimension should
  be first.

Flips the channel order of `image` from RGB to BGR, or vice versa. Note that this will trigger a conversion of
`image` to a NumPy array if it's a PIL Image.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to normalize.
- **mean** (`list[float]` or `np.ndarray` or `torch.Tensor`) --
  The mean (per channel) to use for normalization.
- **std** (`list[float]` or `np.ndarray` or `torch.Tensor`) --
  The standard deviation (per channel) to use for normalization.
- **rescale** (`bool`, *optional*, defaults to `False`) --
  Whether or not to rescale the image to be between 0 and 1. If a PIL image is provided, scaling will
  happen automatically.

Normalizes `image` with `mean` and `std`. Note that this will trigger a conversion of `image` to a NumPy array
if it's a PIL Image.

Rescale a numpy image by scale amount

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to resize.
- **size** (`int` or `tuple[int, int]`) --
  The size to use for resizing the image. If `size` is a sequence like (h, w), output size will be
  matched to this.

  If `size` is an int and `default_to_square` is `True`, then image will be resized to (size, size). If
  `size` is an int and `default_to_square` is `False`, then smaller edge of the image will be matched to
  this number. i.e, if height > width, then image will be rescaled to (size * height / width, size).
- **resample** (`int`, *optional*, defaults to `PILImageResampling.BILINEAR`) --
  The filter to user for resampling.
- **default_to_square** (`bool`, *optional*, defaults to `True`) --
  How to convert `size` when it is a single int. If set to `True`, the `size` will be converted to a
  square (`size`,`size`). If set to `False`, will replicate
  [`torchvision.transforms.Resize`](https://pytorch.org/vision/stable/transforms.html#torchvision.transforms.Resize)
  with support for resizing only the smallest edge and providing an optional `max_size`.
- **max_size** (`int`, *optional*, defaults to `None`) --
  The maximum allowed for the longer edge of the resized image: if the longer edge of the image is
  greater than `max_size` after being resized according to `size`, then the image is resized again so
  that the longer edge is equal to `max_size`. As a result, `size` might be overruled, i.e the smaller
  edge may be shorter than `size`. Only used if `default_to_square` is `False`.imageA resized `PIL.Image.Image`.

Resizes `image`. Enforces conversion of input to PIL.Image.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to rotate. If `np.ndarray` or `torch.Tensor`, will be converted to `PIL.Image.Image` before
  rotating.imageA rotated `PIL.Image.Image`.

Returns a rotated copy of `image`. This method returns a copy of `image`, rotated the given number of degrees
counter clockwise around its centre.

- **image** (`PIL.Image.Image` or `np.ndarray` or `torch.Tensor`) --
  The image to convert to a NumPy array.
- **rescale** (`bool`, *optional*) --
  Whether or not to apply the scaling factor (to make pixel values floats between 0. and 1.). Will
  default to `True` if the image is a PIL Image or an array/tensor of integers, `False` otherwise.
- **channel_first** (`bool`, *optional*, defaults to `True`) --
  Whether or not to permute the dimensions of the image to put the channel dimension first.

Converts `image` to a numpy array. Optionally rescales it and puts the channel dimension as the first
dimension.

- **image** (`PIL.Image.Image` or `numpy.ndarray` or `torch.Tensor`) --
  The image to convert to the PIL Image format.
- **rescale** (`bool`, *optional*) --
  Whether or not to apply the scaling factor (to make pixel values integers between 0 and 255). Will
  default to `True` if the image type is a floating type, `False` otherwise.

Converts `image` to a PIL Image. Optionally rescales it and puts the channel dimension back as the last axis if
needed.
