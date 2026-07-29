# Data[[timm.data.create_dataset]]

- **name** -- Dataset name, empty is okay for folder based datasets
- **root** -- Root folder of dataset (All)
- **split** -- Dataset split (All)
- **search_split** -- Search for split specific child fold from root so one can specify
  `imagenet/` instead of `/imagenet/val`, etc on cmd line / config. (Folder, Torch)
- **class_map** -- Specify class -> index mapping via text file or dict (Folder)
- **load_bytes** -- Load data, return images as undecoded bytes (Folder)
- **download** -- Download dataset if not present and supported (HFIDS, TFDS, Torch)
- **is_training** -- Create dataset in train mode, this is different from the split.
  For Iterable / TDFS it enables shuffle, ignored for other datasets. (TFDS, WDS, HFIDS)
- **batch_size** -- Batch size hint for iterable datasets (TFDS, WDS, HFIDS)
- **seed** -- Seed for iterable datasets (TFDS, WDS, HFIDS)
- **repeats** -- Dataset repeats per iteration i.e. epoch (TFDS, WDS, HFIDS)
- **input_img_mode** -- Input image color conversion mode e.g. 'RGB', 'L' (folder, TFDS, WDS, HFDS, HFIDS)
- **trust_remote_code** -- Trust remote code in Hugging Face Datasets if True (HFDS, HFIDS)
- ****kwargs** -- Other args to pass through to underlying Dataset and/or Reader classesDataset object
Dataset factory method

In parentheses after each arg are the type of dataset supported for each arg, one of:
* Folder - default, timm folder (or tar) based ImageDataset
* Torch - torchvision based datasets
* HFDS - Hugging Face Datasets
* HFIDS - Hugging Face Datasets Iterable (streaming mode, with IterableDataset)
* TFDS - Tensorflow-datasets wrapper in IterabeDataset interface via IterableImageDataset
* WDS - Webdataset
* All - any of the above

- **dataset** -- The image dataset to load.
- **input_size** -- Target input size (channels, height, width) tuple or size scalar.
- **batch_size** -- Number of samples in a batch.
- **is_training** -- Return training (random) transforms.
- **no_aug** -- Disable augmentation for training (useful for debug).
- **re_prob** -- Random erasing probability.
- **re_mode** -- Random erasing fill mode.
- **re_count** -- Number of random erasing regions.
- **re_split** -- Control split of random erasing across batch size.
- **scale** -- Random resize scale range (crop area, < 1.0 => zoom in).
- **ratio** -- Random aspect ratio range (crop ratio for RRC, ratio adjustment factor for RKR).
- **hflip** -- Horizontal flip probability.
- **vflip** -- Vertical flip probability.
- **color_jitter** -- Random color jitter component factors (brightness, contrast, saturation, hue).
  Scalar is applied as (scalar,) * 3 (no hue).
- **color_jitter_prob** -- Apply color jitter with this probability if not None (for SimlCLR-like aug
- **grayscale_prob** -- Probability of converting image to grayscale (for SimCLR-like aug).
- **gaussian_blur_prob** -- Probability of applying gaussian blur (for SimCLR-like aug).
- **auto_augment** -- Auto augment configuration string (see auto_augment.py).
- **num_aug_repeats** -- Enable special sampler to repeat same augmentation across distributed GPUs.
- **num_aug_splits** -- Enable mode where augmentations can be split across the batch.
- **interpolation** -- Image interpolation mode.
- **mean** -- Image normalization mean.
- **std** -- Image normalization standard deviation.
- **num_workers** -- Num worker processes per DataLoader.
- **distributed** -- Enable dataloading for distributed training.
- **crop_pct** -- Inference crop percentage (output size / resize size).
- **crop_mode** -- Inference crop mode. One of ['squash', 'border', 'center']. Defaults to 'center' when None.
- **crop_border_pixels** -- Inference crop border of specified # pixels around edge of original image.
- **collate_fn** -- Override default collate_fn.
- **pin_memory** -- Pin memory for device transfer.
- **fp16** -- Deprecated argument for half-precision input dtype. Use img_dtype.
- **img_dtype** -- Data type for input image.
- **device** -- Device to transfer inputs and targets to.
- **use_prefetcher** -- Use efficient pre-fetcher to load samples onto device.
- **use_multi_epochs_loader** --
- **persistent_workers** -- Enable persistent worker processes.
- **worker_seeding** -- Control worker random seeding at init.
- **tf_preprocessing** -- Use TF 1.0 inference preprocessing for testing model ports.DataLoader

- **input_size** -- Target input size (channels, height, width) tuple or size scalar.
- **is_training** -- Return training (random) transforms.
- **no_aug** -- Disable augmentation for training (useful for debug).
- **train_crop_mode** -- Training random crop mode ('rrc', 'rkrc', 'rkrr').
- **scale** -- Random resize scale range (crop area, < 1.0 => zoom in).
- **ratio** -- Random aspect ratio range (crop ratio for RRC, ratio adjustment factor for RKR).
- **hflip** -- Horizontal flip probability.
- **vflip** -- Vertical flip probability.
- **color_jitter** -- Random color jitter component factors (brightness, contrast, saturation, hue).
  Scalar is applied as (scalar,) * 3 (no hue).
- **color_jitter_prob** -- Apply color jitter with this probability if not None (for SimlCLR-like aug).
- **grayscale_prob** -- Probability of converting image to grayscale (for SimCLR-like aug).
- **gaussian_blur_prob** -- Probability of applying gaussian blur (for SimCLR-like aug).
- **auto_augment** -- Auto augment configuration string (see auto_augment.py).
- **interpolation** -- Image interpolation mode.
- **mean** -- Image normalization mean.
- **std** -- Image normalization standard deviation.
- **re_prob** -- Random erasing probability.
- **re_mode** -- Random erasing fill mode.
- **re_count** -- Number of random erasing regions.
- **re_num_splits** -- Control split of random erasing across batch size.
- **crop_pct** -- Inference crop percentage (output size / resize size).
- **crop_mode** -- Inference crop mode. One of ['squash', 'border', 'center']. Defaults to 'center' when None.
- **crop_border_pixels** -- Inference crop border of specified # pixels around edge of original image.
- **tf_preprocessing** -- Use TF 1.0 inference preprocessing for testing model ports
- **use_prefetcher** -- Pre-fetcher enabled. Do not convert image to tensor or normalize.
- **normalize** -- Normalization tensor output w/ provided mean/std (if prefetcher not used).
- **separate** -- Output transforms in 3-stage tuple.Composed transforms or tuple thereof
