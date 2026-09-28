# utils/image

Image I/O and manipulation.

`RawImage` wraps a raw pixel buffer with width, height, and channel metadata;
use `load_image()` to decode from paths, URLs, or Blobs, and the instance
methods to resize, convert, and save.

## Classes

### RawImage

Represents an image stored as a raw pixel buffer.

```javascript
import { RawImage } from '@huggingface/transformers';
const image = await RawImage.read('https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/artemis.jpeg');
console.log(image.width, image.height, image.channels);
```

#### `RawImage.constructor(data, width, height, channels)`

Create a new `RawImage` object.

**Parameters**

- `data` (`Uint8ClampedArray` | `Uint8Array`) — The pixel data.
- `width` (`number`) — The width of the image.
- `height` (`number`) — The height of the image.
- `channels` (`1` | `2` | `3` | `4`) — The number of channels.

#### `RawImage.size` : `[number, number]`

Returns the size of the image (width, height).

#### `RawImage.read(input)`

Helper method for reading an image from a variety of input types.

**Parameters**

- `input` ([`RawImage`](./image#module_utils/image.RawImage) | `string` | `URL` | `Blob` | `HTMLCanvasElement` | `OffscreenCanvas`)

**Returns:** `Promise`<[`RawImage`](./image#module_utils/image.RawImage)> — The image object.

**Example:** Read image from a URL.
```javascript
let image = await RawImage.read('https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/football-match.jpg');
// RawImage {
//   "data": Uint8ClampedArray [ 25, 25, 25, 19, 19, 19, ... ],
//   "width": 800,
//   "height": 533,
//   "channels": 3
// }
```

#### `RawImage.fromCanvas(canvas)`

Read an image from a canvas.

**Parameters**

- `canvas` (`HTMLCanvasElement` | `OffscreenCanvas`) — The canvas to read the image from.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — The image object.

#### `RawImage.fromURL(url)`

Read an image from a URL or file path.

**Parameters**

- `url` (`string` | `URL`) — The URL or file path to read the image from.

**Returns:** `Promise`<[`RawImage`](./image#module_utils/image.RawImage)> — The image object.

#### `RawImage.fromBlob(blob)`

Helper method to create a new Image from a blob.

**Parameters**

- `blob` (`Blob`) — The blob to read the image from.

**Returns:** `Promise`<[`RawImage`](./image#module_utils/image.RawImage)> — The image object.

#### `RawImage.fromTensor(tensor, [channel_format])`

Helper method to create a new Image from a tensor

**Parameters**

- `tensor` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The 3D tensor containing the image data. Must be of type `uint8`.
- `channel_format` (`'CHW'` | `'HWC'`) _optional_ — defaults to `'CHW'` — The dimension ordering of the tensor.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — The image created from the tensor.

**Throws**

- `Error` — If the tensor does not have 3 dimensions, or the channel format,
tensor type, or number of channels is unsupported.

#### `RawImage.grayscale()`

Convert the image to grayscale format.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — `this` to support chaining.

#### `RawImage.rgb()`

Convert the image to RGB format.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — `this` to support chaining.

#### `RawImage.rgba()`

Convert the image to RGBA format.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — `this` to support chaining.

#### `RawImage.putAlpha(mask)`

Apply an alpha mask to the image. Operates in place.

**Parameters**

- `mask` ([`RawImage`](./image#module_utils/image.RawImage)) — The mask to apply. It should have a single channel.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — The masked image.

**Throws**

- `Error` — If the mask is not the same size as the image.
- `Error` — If the image does not have 4 channels.
- `Error` — If the mask is not a single channel.

#### `RawImage.resize(width, height, options)`

Resize the image to the given dimensions. This method uses the canvas API to perform the resizing.

**Parameters**

- `width` (`number`) — The width of the new image. `null` or `-1` will preserve the aspect ratio.
- `height` (`number`) — The height of the new image. `null` or `-1` will preserve the aspect ratio.
- `options` (`Object`) — Additional options for resizing.
  - `resample` (`0` | `1` | `2` | `3` | `4` | `5` | `string`) _optional_ — The resampling method to use.

**Returns:** `Promise`<[`RawImage`](./image#module_utils/image.RawImage)> — `this` to support chaining.

#### `RawImage.split()`

Split this image into individual bands. This method returns an array of individual image bands from an image.
For example, splitting an "RGB" image creates three new images each containing a copy of one of the original bands (red, green, blue).

Inspired by PIL's `Image.split()` [function](https://pillow.readthedocs.io/en/latest/reference/Image.html#PIL.Image.Image.split).

**Returns:** [`RawImage`](./image#module_utils/image.RawImage)[] — An array containing bands.

#### `RawImage.clone()`

Clone the image

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — The cloned image

#### `RawImage.convert(numChannels)`

Helper method for converting image to have a certain number of channels

**Parameters**

- `numChannels` (`number`) — The number of channels. Must be 1, 3, or 4.

**Returns:** [`RawImage`](./image#module_utils/image.RawImage) — `this` to support chaining.

#### `RawImage.save(path)`

Save the image to the given path.

**Parameters**

- `path` (`string`) — The path to save the image to.

**Returns:** `Promise`<`void`>

#### `RawImage.toSharp()`

Convert the image to a Sharp instance.

**Returns:** `Sharp` — The Sharp instance.

## Constants

### `load_image` : `typeof RawImage.read`

Load an image from a URL, file path, `Blob`, `HTMLCanvasElement`, or
`OffscreenCanvas`. Equivalent to `RawImage.read`.
