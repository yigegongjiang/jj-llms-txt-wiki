# processors

Processors turn raw inputs (images, audio, text) into the tensor
shapes a model expects. Pipelines pick the right processor automatically;
call one directly only when you need to preprocess without running
inference.

Three `Auto*` entry points cover the common cases:
- `AutoProcessor` — multi-modal (tokenizer + image/audio), e.g. Whisper, CLIP.
- `AutoImageProcessor` — vision-only models.
- `AutoFeatureExtractor` — audio-only models.

**Example:** Prepare audio for Whisper.
```javascript
import { AutoProcessor, load_audio } from '@huggingface/transformers';

const processor = await AutoProcessor.from_pretrained('onnx-community/whisper-tiny.en');
const audio = await load_audio('https://huggingface.co/datasets/Narsil/asr_dummy/resolve/main/mlk.flac', 16000);
const { input_features } = await processor(audio);
// Tensor {
//   data: Float32Array(240000) [0.4752984642982483, 0.5597258806228638, 0.56434166431427, ...],
//   dims: [1, 80, 3000],
//   type: 'float32',
//   size: 240000,
// }
```

## On this page

**Classes** — [`FeatureExtractor`](#module_processors.FeatureExtractor) · [`ImageProcessor`](#module_processors.ImageProcessor) · [`AutoFeatureExtractor`](#module_processors.AutoFeatureExtractor) · [`AutoImageProcessor`](#module_processors.AutoImageProcessor) · [`AutoProcessor`](#module_processors.AutoProcessor) · [`Processor`](#module_processors.Processor)

## Classes

### FeatureExtractor

Base class for audio feature extractors.

#### `FeatureExtractor.constructor(config)`

Create a feature extractor from a parsed `preprocessor_config.json`.

**Parameters**

- `config` (`Object`) — The configuration for the feature extractor.

#### `FeatureExtractor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the feature extractor classes of the library from a pretrained model.

The feature extractor class to instantiate is selected based on the `feature_extractor_type` property of
the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model ID* of a pretrained feature extractor hosted inside a model repo on huggingface.co.
  Valid model IDs can be located at the root level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing feature_extractor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the feature_extractor.

**Returns:** `Promise`<[`FeatureExtractor`](./processors#module_processors.FeatureExtractor)> — A new feature extractor instance.

### ImageProcessor

Base class for image processors.

#### `ImageProcessor(images, args)`

Preprocess one or more images and batch the result into `pixel_values`.

**Parameters**

- `images` ([`RawImage`](./utils/image#module_utils/image.RawImage) | [`RawImage`](./utils/image#module_utils/image.RawImage)[]) — The image or images to preprocess.
- `args` (`...any`) — Additional arguments.

**Returns:** `Promise`<[`ImageProcessorResult`](./processors#module_processors.ImageProcessorResult)> — An object containing the concatenated pixel values (and other metadata) of the preprocessed images.

#### `ImageProcessor.constructor(config)`

Create an image processor from a parsed `preprocessor_config.json`.

**Parameters**

- `config` ([`ImageProcessorConfig`](./processors#module_processors.ImageProcessorConfig)) — The configuration object.

#### `ImageProcessor.thumbnail(image, size, [resample])`

Resize the image to make a thumbnail. The image is resized so that no dimension is larger than any
corresponding dimension of the specified size.

**Parameters**

- `image` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The image to be resized.
- `size` (`{ height:number, width:number }`) — The size `{"height": h, "width": w}` to resize the image to.
- `resample` (`string` | `0` | `1` | `2` | `3` | `4` | `5`) _optional_ — defaults to `2` — The resampling filter to use.

**Returns:** `Promise`<[`RawImage`](./utils/image#module_utils/image.RawImage)> — The resized image.

#### `ImageProcessor.crop_margin(image, gray_threshold)`

Crops the margin of the image. Gray pixels are considered margin (i.e., pixels with a value below the threshold).

**Parameters**

- `image` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The image to be cropped.
- `gray_threshold` (`number`) — Value below which pixels are considered to be gray.

**Returns:** `Promise`<[`RawImage`](./utils/image#module_utils/image.RawImage)> — The cropped image.

#### `ImageProcessor.pad_image(pixelData, imgDims, padSize, options)`

Pad the image by a certain amount.

**Parameters**

- `pixelData` (`Float32Array`) — The pixel data to pad.
- `imgDims` (`number[]`) — The dimensions of the image (height, width, channels).
- `padSize` (`{ width:number; height:number }` | `number` | `'square'`) — The dimensions of the padded image.
- `options` (`Object`) — The options for padding.
  - `mode` (`'constant'` | `'symmetric'`) _optional_ — defaults to `'constant'` — The type of padding to add.
  - `center` (`boolean`) _optional_ — defaults to `false` — Whether to center the image.
  - `constant_values` (`number` | `number[]`) _optional_ — defaults to `0` — The constant value to use for padding.

**Returns:** `[Float32Array, number[]]` — The padded pixel data and image dimensions.

#### `ImageProcessor.rescale(pixelData)`

Rescale the image pixel values by `this.rescale_factor`.

**Parameters**

- `pixelData` (`Float32Array`) — The pixel data to rescale.

**Returns:** `void`

#### `ImageProcessor.get_resize_output_image_size(image, size)`

Find the target (width, height) dimension of the output image after
resizing given the input image and the desired size.

**Parameters**

- `image` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The image to resize.
- `size` (`any`) — The size to use for resizing the image.

**Returns:** `[number, number]` — The target (width, height) dimension of the output image after resizing.

#### `ImageProcessor.resize(image)`

Resizes the image.

**Parameters**

- `image` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The image to resize.

**Returns:** `Promise`<[`RawImage`](./utils/image#module_utils/image.RawImage)> — The resized image.

#### `ImageProcessor.preprocess(image, overrides)`

Preprocesses the given image.

**Parameters**

- `image` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The image to preprocess.
- `overrides` (`Object`) — The overrides for the preprocessing options.

**Returns:** `Promise`<[`PreprocessedImage`](./processors#module_processors.PreprocessedImage)> — The preprocessed image.

#### `ImageProcessor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the processor classes of the library from a pretrained model.

The processor class to instantiate is selected based on the `image_processor_type` (or `feature_extractor_type`; legacy)
property of the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model id* of a pretrained processor hosted inside a model repo on huggingface.co.
  Valid model ids can be located at the root-level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing processor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the processor.

**Returns:** `Promise`<[`ImageProcessor`](./processors#module_processors.ImageProcessor)> — A new image processor instance.

### AutoFeatureExtractor

Loads a feature extractor from a pretrained id. The concrete class is
selected from the `feature_extractor_type` in `preprocessor_config.json`.
Most commonly used for audio models.

```javascript
import { AutoFeatureExtractor, load_audio } from '@huggingface/transformers';

const extractor = await AutoFeatureExtractor.from_pretrained('onnx-community/whisper-tiny.en');
const audio = await load_audio('https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav', 16000);
const { input_features } = await extractor(audio);
```

#### `AutoFeatureExtractor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the feature extractor classes of the library from a pretrained model.

The feature extractor class to instantiate is selected based on the `feature_extractor_type` property of
the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model ID* of a pretrained feature extractor hosted inside a model repo on huggingface.co.
  Valid model IDs can be located at the root level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing feature_extractor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the feature_extractor.

**Returns:** `Promise`<[`FeatureExtractor`](./processors#module_processors.FeatureExtractor)> — A new feature extractor instance.

### AutoImageProcessor

Loads an image processor from a pretrained id. The concrete class is
selected from the `image_processor_type` in `preprocessor_config.json`.

```javascript
import { AutoImageProcessor, load_image } from '@huggingface/transformers';

const processor = await AutoImageProcessor.from_pretrained('Xenova/clip-vit-base-patch16');
const image = await load_image('https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/artemis.jpeg');
const { pixel_values } = await processor(image);
```

#### `AutoImageProcessor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the processor classes of the library from a pretrained model.

The processor class to instantiate is selected based on the `image_processor_type` (or `feature_extractor_type`; legacy)
property of the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model id* of a pretrained processor hosted inside a model repo on huggingface.co.
  Valid model ids can be located at the root-level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing processor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the processor.

**Returns:** `Promise`<[`ImageProcessor`](./processors#module_processors.ImageProcessor)> — A new image processor instance.

### AutoProcessor

Loads a processor from a pretrained id. Unlike `AutoImageProcessor` and
`AutoFeatureExtractor`, `AutoProcessor` returns a multi-modal [`Processor`](./processors#module_processors.Processor)
that bundles together a tokenizer, image processor, and/or feature extractor
— use it when a single model needs more than one.

**Example:** Load a Whisper processor (tokenizer + audio feature extractor).
```javascript
import { AutoProcessor } from '@huggingface/transformers';
const processor = await AutoProcessor.from_pretrained('onnx-community/whisper-tiny.en');
```

**Example:** Run an image through a CLIP processor.
```javascript
import { AutoProcessor, load_image } from '@huggingface/transformers';

const processor = await AutoProcessor.from_pretrained('Xenova/clip-vit-base-patch16');
const image = await load_image('https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/football-match.jpg');
const { pixel_values } = await processor(image);
```

#### `AutoProcessor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the processor classes of the library from a pretrained model.

The processor class to instantiate is selected based on the `image_processor_type` (or `feature_extractor_type`; legacy)
property of the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model ID* of a pretrained processor hosted inside a model repo on huggingface.co.
  Valid model IDs can be located at the root level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing processor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedProcessorOptions`](./processors#module_processors.PretrainedProcessorOptions)) — Additional options for loading the processor.

**Returns:** `Promise`<[`Processor`](./processors#module_processors.Processor)> — A new processor instance.

### Processor

Multi-modal preprocessor that delegates to the tokenizer, image processor,
and/or feature extractor required by a model.

#### `Processor(input, args)`

Calls the feature_extractor function with the given input.

**Parameters**

- `input` (`any`) — The input to extract features from.
- `args` (`...any`) — Additional arguments.

**Returns:** `Promise`<`any`> — A Promise that resolves with the extracted features.

#### `Processor.constructor(config, components, chat_template)`

Create a processor from parsed config and its component preprocessors.

**Parameters**

- `config` (`Object`) — Processor configuration.
- `components` (`Record`<`string`, `Object`>) — Loaded tokenizer, image processor, and/or feature extractor.
- `chat_template` (`string` | `null`) — Optional chat template loaded from the model repo.

#### `Processor.image_processor` : [`ImageProcessor`](./processors#module_processors.ImageProcessor) | `undefined`

The image processor used by this processor, if any.

#### `Processor.tokenizer` : [`PreTrainedTokenizer`](./tokenizers#module_tokenizers.PreTrainedTokenizer) | `undefined`

The tokenizer used by this processor, if any.

#### `Processor.feature_extractor` : [`FeatureExtractor`](./processors#module_processors.FeatureExtractor) | `undefined`

The feature extractor used by this processor, if any.

#### `Processor.apply_chat_template(messages, options)`

Delegates to the underlying tokenizer's `apply_chat_template`.

**Parameters**

- `messages` ([`Message`](./tokenizers#module_tokenizers.Message)[])
- `options` ([`ApplyChatTemplateOptions`](./tokenizers#module_tokenizers.ApplyChatTemplateOptions)<`TTokenize`, `TReturnTensor`, `TReturnDict`>)

**Returns:** [`ApplyChatTemplateReturn`](./tokenizers#module_tokenizers.ApplyChatTemplateReturn)<`TTokenize`, `TReturnTensor`, `TReturnDict`>

#### `Processor.batch_decode(batch, decode_args)`

Decode a batch of tokenized sequences via the underlying tokenizer.

**Parameters**

- `batch` (`number[][]` | [`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — List/Tensor of tokenized input sequences.
- `decode_args` (`Object`) — (Optional) Object with decoding arguments.

**Returns:** `string[]`

#### `Processor.decode(token_ids, [decode_args])`

Decode a single tokenized sequence via the underlying tokenizer.

**Parameters**

- `token_ids` (`number[]` | `bigint[]` | [`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — List/Tensor of token IDs to decode.
- `decode_args` (`Object`) _optional_ — defaults to `{}`
  - `skip_special_tokens` (`boolean`) _optional_ — defaults to `false` — If true, special tokens are removed from the output string.
  - `clean_up_tokenization_spaces` (`boolean`) _optional_ — defaults to `true` — If true, spaces before punctuation and abbreviated forms are removed.

**Returns:** `string`

#### `Processor.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the processor classes of the library from a pretrained model.

The processor class to instantiate is selected based on the `image_processor_type` (or `feature_extractor_type`; legacy)
property of the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model ID* of a pretrained processor hosted inside a model repo on huggingface.co.
  Valid model IDs can be located at the root level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing processor files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedProcessorOptions`](./processors#module_processors.PretrainedProcessorOptions)) — Additional options for loading the processor.

**Returns:** `Promise`<[`Processor`](./processors#module_processors.Processor)> — A new processor instance.

## Type Definitions

### HeightWidth

Named tuple to indicate the order we are using is (height x width),
even though the Graphics' industry standard is (width x height).

_Type:_ `[height: number, width: number]`

### ImageProcessorResult

**Properties**

- `pixel_values` ([`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — The pixel values of the batched preprocessed images.
- `original_sizes` ([`HeightWidth`](./processors#module_processors.HeightWidth)[]) — Array of two-dimensional tuples like [[480, 640]].
- `reshaped_input_sizes` ([`HeightWidth`](./processors#module_processors.HeightWidth)[]) — Array of two-dimensional tuples like [[1000, 1330]].

### ImageProcessorConfig

A configuration object used to create an image processor.

**Properties**

- `progress_callback` (`function`) _optional_ — defaults to `null` — If specified, this function is called during model construction with progress updates.
- `image_mean` (`number[]`) _optional_ — The mean values for image normalization.
- `image_std` (`number[]`) _optional_ — The standard deviation values for image normalization.
- `do_rescale` (`boolean`) _optional_ — Whether to rescale the image pixel values to the [0,1] range.
- `rescale_factor` (`number`) _optional_ — The factor to use for rescaling the image pixel values.
- `do_normalize` (`boolean`) _optional_ — Whether to normalize the image pixel values.
- `do_resize` (`boolean`) _optional_ — Whether to resize the image.
- `resample` (`number`) _optional_ — What method to use for resampling.
- `size` (`number` | `Object`) _optional_ — The size to resize the image to.
- `image_size` (`number` | `Object`) _optional_ — The size to resize the image to (same as `size`).
- `do_flip_channel_order` (`boolean`) _optional_ — defaults to `false` — Whether to flip the color channels from RGB to BGR.
  Can be overridden by the `do_flip_channel_order` parameter in the `preprocess` method.
- `do_center_crop` (`boolean`) _optional_ — Whether to center crop the image to the specified `crop_size`.
  Can be overridden by `do_center_crop` in the `preprocess` method.
- `do_thumbnail` (`boolean`) _optional_ — Whether to resize the image using thumbnail method.
- `keep_aspect_ratio` (`boolean`) _optional_ — If `true`, the image is resized to the largest possible size such that the aspect ratio is preserved.
  Can be overridden by `keep_aspect_ratio` in `preprocess`.
- `ensure_multiple_of` (`number`) _optional_ — If `do_resize` is `true`, the image is resized to a size that is a multiple of this value.
  Can be overridden by `ensure_multiple_of` in `preprocess`.
- `mean` (`number[]`) _optional_ — The mean values for image normalization (same as `image_mean`).
- `std` (`number[]`) _optional_ — The standard deviation values for image normalization (same as `image_std`).

### PreprocessedImage

**Properties**

- `original_size` ([`HeightWidth`](./processors#module_processors.HeightWidth)) — The original size of the image.
- `reshaped_input_size` ([`HeightWidth`](./processors#module_processors.HeightWidth)) — The reshaped input size of the image.
- `pixel_values` ([`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — The pixel values of the preprocessed image.

### ProcessorProperties

Additional processor-specific properties.

### PretrainedProcessorOptions

_Type:_ [`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions) & [`ProcessorProperties`](./processors#module_processors.ProcessorProperties)
