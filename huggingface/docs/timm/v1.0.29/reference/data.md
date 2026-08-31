# Data[[timm.data.create_dataset]]

#### timm.data.create_dataset[[timm.data.create_dataset]]

```python
timm.data.create_dataset(name: str, root: typing.Optional[str] = None, split: str = 'validation', search_split: bool = True, class_map: dict = None, load_bytes: bool = False, is_training: bool = False, download: bool = False, batch_size: int = 1, num_samples: typing.Optional[int] = None, seed: int = 42, repeats: int = 0, input_img_mode: str = 'RGB', trust_remote_code: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/data/dataset_factory.py#L63)

**Parameters:**

name : Dataset name, empty is okay for folder based datasets

root : Root folder of dataset (All)

split : Dataset split (All)

search_split : Search for split specific child fold from root so one can specify `imagenet/` instead of `/imagenet/val`, etc on cmd line / config. (Folder, Torch)

class_map : Specify class -> index mapping via text file or dict (Folder)

load_bytes : Load data, return images as undecoded bytes (Folder)

download : Download dataset if not present and supported (HFIDS, TFDS, Torch)

is_training : Create dataset in train mode, this is different from the split. For Iterable / TDFS it enables shuffle, ignored for other datasets. (TFDS, WDS, HFIDS)

batch_size : Batch size hint for iterable datasets (TFDS, WDS, HFIDS)

seed : Seed for iterable datasets (TFDS, WDS, HFIDS)

repeats : Dataset repeats per iteration i.e. epoch (TFDS, WDS, HFIDS)

input_img_mode : Input image color conversion mode e.g. 'RGB', 'L' (folder, TFDS, WDS, HFDS, HFIDS)

trust_remote_code : Trust remote code in Hugging Face Datasets if True (HFDS, HFIDS)

- ****kwargs** : Other args to pass through to underlying Dataset and/or Reader classes

**Returns:**

Dataset object

Dataset factory method

In parentheses after each arg are the type of dataset supported for each arg, one of:
* Folder - default, timm folder (or tar) based ImageDataset
* Torch - torchvision based datasets
* HFDS - Hugging Face Datasets
* HFIDS - Hugging Face Datasets Iterable (streaming mode, with IterableDataset)
* TFDS - Tensorflow-datasets wrapper in IterabeDataset interface via IterableImageDataset
* WDS - Webdataset
* All - any of the above

#### timm.data.create_loader[[timm.data.create_loader]]

```python
timm.data.create_loader(dataset: typing.Union[timm.data.dataset.ImageDataset, timm.data.dataset.IterableImageDataset], input_size: typing.Union[int, typing.Tuple[int, int], typing.Tuple[int, int, int]], batch_size: int, is_training: bool = False, no_aug: bool = False, re_prob: float = 0.0, re_mode: str = 'const', re_count: int = 1, re_split: bool = False, train_crop_mode: typing.Optional[str] = None, scale: typing.Optional[typing.Tuple[float, float]] = None, ratio: typing.Optional[typing.Tuple[float, float]] = None, hflip: float = 0.5, vflip: float = 0.0, color_jitter: float = 0.4, color_jitter_prob: typing.Optional[float] = None, grayscale_prob: float = 0.0, gaussian_blur_prob: float = 0.0, auto_augment: typing.Optional[str] = None, num_aug_repeats: int = 0, num_aug_splits: int = 0, interpolation: str = 'bilinear', mean: typing.Tuple[float, ...] = (0.485, 0.456, 0.406), std: typing.Tuple[float, ...] = (0.229, 0.224, 0.225), num_workers: int = 1, distributed: bool = False, crop_pct: typing.Optional[float] = None, crop_mode: typing.Optional[str] = None, crop_border_pixels: typing.Optional[int] = None, collate_fn: typing.Optional[typing.Callable] = None, pin_memory: bool = False, fp16: bool = False, img_dtype: dtype = torch.float32, device: device = torch.device(), use_prefetcher: bool = True, use_multi_epochs_loader: bool = False, persistent_workers: bool = True, worker_seeding: str = 'all', tf_preprocessing: bool = False, input_size_choices: typing.Optional[typing.Sequence[typing.Union[int, typing.Tuple[int, int], typing.Tuple[int, int, int]]]] = None, batch_size_choices: typing.Optional[typing.Sequence[int]] = None, batch_choice_weights: typing.Optional[typing.Sequence[float]] = None, batch_choice_seed: int = 0, batch_choice_schedule: str = 'constant', batch_schedule_epochs: typing.Optional[int] = None, batch_schedule_spread: float = 0.65, batch_schedule_random_mix: float = 0.1, num_batches: typing.Optional[int] = None)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/data/loader.py#L205)

**Parameters:**

dataset : The image dataset to load.

input_size : Target input size (channels, height, width) tuple or size scalar.

batch_size : Number of samples in a batch.

is_training : Return training (random) transforms.

no_aug : Disable augmentation for training (useful for debug).

re_prob : Random erasing probability.

re_mode : Random erasing fill mode.

re_count : Number of random erasing regions.

re_split : Control split of random erasing across batch size.

scale : Random resize scale range (crop area, &amp;lt; 1.0 => zoom in).

ratio : Random aspect ratio range (crop ratio for RRC, ratio adjustment factor for RKR).

hflip : Horizontal flip probability.

vflip : Vertical flip probability.

color_jitter : Random color jitter component factors (brightness, contrast, saturation, hue). Scalar is applied as (scalar,) * 3 (no hue).

color_jitter_prob : Apply color jitter with this probability if not None (for SimlCLR-like aug

grayscale_prob : Probability of converting image to grayscale (for SimCLR-like aug).

gaussian_blur_prob : Probability of applying gaussian blur (for SimCLR-like aug).

auto_augment : Auto augment configuration string (see auto_augment.py).

num_aug_repeats : Enable special sampler to repeat same augmentation across distributed GPUs.

num_aug_splits : Enable mode where augmentations can be split across the batch.

interpolation : Image interpolation mode.

mean : Image normalization mean.

std : Image normalization standard deviation.

num_workers : Num worker processes per DataLoader.

distributed : Enable dataloading for distributed training.

crop_pct : Inference crop percentage (output size / resize size).

crop_mode : Inference crop mode. One of ['squash', 'border', 'center']. Defaults to 'center' when None.

crop_border_pixels : Inference crop border of specified # pixels around edge of original image.

collate_fn : Override default collate_fn.

pin_memory : Pin memory for device transfer.

fp16 : Deprecated argument for half-precision input dtype. Use img_dtype.

img_dtype : Data type for input image.

device : Device to transfer inputs and targets to.

use_prefetcher : Use efficient pre-fetcher to load samples onto device.

use_multi_epochs_loader --

persistent_workers : Enable persistent worker processes when `num_workers` is greater than zero.

worker_seeding : Control worker random seeding at init.

tf_preprocessing : Use TF 1.0 inference preprocessing for testing model ports.

input_size_choices : Per-batch input size choices for scheduled resolution training.

batch_size_choices : Batch size corresponding to each input size choice. Uses `batch_size` for all choices when not specified.

batch_choice_weights : Sampling weights for input size choices. Uniform when not specified.

batch_choice_seed : Random seed used to create and shuffle the batch schedule.

batch_choice_schedule : Choice schedule mode, either `constant` or `progressive`.

batch_schedule_epochs : Number of epochs over which a progressive schedule moves through its choices.

batch_schedule_spread : Standard deviation of the progressive choice window, in choice-index units.

batch_schedule_random_mix : Fraction of uniform random exploration over nonzero-weight choices in a progressive schedule.

num_batches : Fixed number of loader batches per epoch. Inferred from the schedule-average batch size for progressive schedules when not specified.

**Returns:**

DataLoader

#### timm.data.create_transform[[timm.data.create_transform]]

```python
timm.data.create_transform(input_size: typing.Union[int, typing.Tuple[int, int], typing.Tuple[int, int, int]] = 224, is_training: bool = False, no_aug: bool = False, train_crop_mode: typing.Optional[str] = None, scale: typing.Optional[typing.Tuple[float, float]] = None, ratio: typing.Optional[typing.Tuple[float, float]] = None, hflip: float = 0.5, vflip: float = 0.0, color_jitter: typing.Union[float, typing.Tuple[float, ...]] = 0.4, color_jitter_prob: typing.Optional[float] = None, grayscale_prob: float = 0.0, gaussian_blur_prob: float = 0.0, auto_augment: typing.Optional[str] = None, interpolation: str = 'bilinear', mean: typing.Tuple[float, ...] = (0.485, 0.456, 0.406), std: typing.Tuple[float, ...] = (0.229, 0.224, 0.225), re_prob: float = 0.0, re_mode: str = 'const', re_count: int = 1, re_num_splits: int = 0, crop_pct: typing.Optional[float] = None, crop_mode: typing.Optional[str] = None, crop_border_pixels: typing.Optional[int] = None, tf_preprocessing: bool = False, use_prefetcher: bool = False, normalize: bool = True, separate: bool = False, naflex: bool = False, patch_size: typing.Union[int, typing.Tuple[int, int]] = 16, max_seq_len: int = 576, patchify: bool = False, patchify_channels_last: bool = True, patchify_flatten: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/data/transforms_factory.py#L387)

**Parameters:**

input_size : Target input size (channels, height, width) tuple or size scalar.

is_training : Return training (random) transforms.

no_aug : Disable augmentation for training (useful for debug).

train_crop_mode : Training random crop mode ('rrc', 'rkrc', 'rkrr').

scale : Random resize scale range (crop area, < 1.0 => zoom in).

ratio : Random aspect ratio range (crop ratio for RRC, ratio adjustment factor for RKR).

hflip : Horizontal flip probability.

vflip : Vertical flip probability.

color_jitter : Random color jitter component factors (brightness, contrast, saturation, hue). Scalar is applied as (scalar,) * 3 (no hue).

color_jitter_prob : Apply color jitter with this probability if not None (for SimlCLR-like aug).

grayscale_prob : Probability of converting image to grayscale (for SimCLR-like aug).

gaussian_blur_prob : Probability of applying gaussian blur (for SimCLR-like aug).

auto_augment : Auto augment configuration string (see auto_augment.py).

interpolation : Image interpolation mode.

mean : Image normalization mean.

std : Image normalization standard deviation.

re_prob : Random erasing probability.

re_mode : Random erasing fill mode.

re_count : Number of random erasing regions.

re_num_splits : Control split of random erasing across batch size.

crop_pct : Inference crop percentage (output size / resize size).

crop_mode : Inference crop mode. One of ['squash', 'border', 'center']. Defaults to 'center' when None.

crop_border_pixels : Inference crop border of specified # pixels around edge of original image.

tf_preprocessing : Use TF 1.0 inference preprocessing for testing model ports

use_prefetcher : Pre-fetcher enabled. Do not convert image to tensor or normalize.

normalize : Normalization tensor output w/ provided mean/std (if prefetcher not used).

separate : Output transforms in 3-stage tuple.

patchify : Patchify the output instead of relying on prefetcher.

patchify_channels_last : Use channels-last layout within each patch.

patchify_flatten : Flatten eval patches into vectors. Disable when the model needs their spatial dimensions for patch-size interpolation.

**Returns:**

Composed transforms or tuple thereof

#### timm.data.resolve_data_config[[timm.data.resolve_data_config]]

```python
timm.data.resolve_data_config(args = None, pretrained_cfg = None, model = None, use_test_size = False, verbose = False)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/data/config.py#L8)
