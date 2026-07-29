# utils/image

Helper module for image processing.

These functions and classes are only used internally,
meaning an end-user shouldn't need to access anything here.

* [utils/image](#module_utils/image)
    * _static_
        * [.RawImage](#module_utils/image.RawImage)
            * [`new RawImage(data, width, height, channels)`](#new_module_utils/image.RawImage_new)
            * _instance_
                * [`.size`](#module_utils/image.RawImage+size) ⇒ Array
                * [`.grayscale()`](#module_utils/image.RawImage+grayscale) ⇒ [RawImage](#RawImage)
                * [`.rgb()`](#module_utils/image.RawImage+rgb) ⇒ [RawImage](#RawImage)
                * [`.rgba()`](#module_utils/image.RawImage+rgba) ⇒ [RawImage](#RawImage)
                * [`.putAlpha(mask)`](#module_utils/image.RawImage+putAlpha) ⇒ [RawImage](#RawImage)
                * [`.resize(width, height, options)`](#module_utils/image.RawImage+resize) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
                * [`.split()`](#module_utils/image.RawImage+split) ⇒ Array
                * [`.clone()`](#module_utils/image.RawImage+clone) ⇒ [RawImage](#RawImage)
                * [`.convert(numChannels)`](#module_utils/image.RawImage+convert) ⇒ [RawImage](#RawImage)
                * [`.save(path)`](#module_utils/image.RawImage+save) ⇒ Promise.&lt;void&gt;
                * [`.toSharp()`](#module_utils/image.RawImage+toSharp) ⇒ Sharp
            * _static_
                * [`.read(input)`](#module_utils/image.RawImage.read) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
                * [`.fromCanvas(canvas)`](#module_utils/image.RawImage.fromCanvas) ⇒ [RawImage](#RawImage)
                * [`.fromURL(url)`](#module_utils/image.RawImage.fromURL) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
                * [`.fromBlob(blob)`](#module_utils/image.RawImage.fromBlob) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
                * [`.fromTensor(tensor)`](#module_utils/image.RawImage.fromTensor)
        * [`.load_image`](#module_utils/image.load_image)
    * _inner_
        * [`~CONTENT_TYPE_MAP`](#module_utils/image..CONTENT_TYPE_MAP)

* * *

## utils/image.RawImage

**Kind**: static class of [utils/image](#module_utils/image)  

* [.RawImage](#module_utils/image.RawImage)
    * [`new RawImage(data, width, height, channels)`](#new_module_utils/image.RawImage_new)
    * _instance_
        * [`.size`](#module_utils/image.RawImage+size) ⇒ Array
        * [`.grayscale()`](#module_utils/image.RawImage+grayscale) ⇒ [RawImage](#RawImage)
        * [`.rgb()`](#module_utils/image.RawImage+rgb) ⇒ [RawImage](#RawImage)
        * [`.rgba()`](#module_utils/image.RawImage+rgba) ⇒ [RawImage](#RawImage)
        * [`.putAlpha(mask)`](#module_utils/image.RawImage+putAlpha) ⇒ [RawImage](#RawImage)
        * [`.resize(width, height, options)`](#module_utils/image.RawImage+resize) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
        * [`.split()`](#module_utils/image.RawImage+split) ⇒ Array
        * [`.clone()`](#module_utils/image.RawImage+clone) ⇒ [RawImage](#RawImage)
        * [`.convert(numChannels)`](#module_utils/image.RawImage+convert) ⇒ [RawImage](#RawImage)
        * [`.save(path)`](#module_utils/image.RawImage+save) ⇒ Promise.&lt;void&gt;
        * [`.toSharp()`](#module_utils/image.RawImage+toSharp) ⇒ Sharp
    * _static_
        * [`.read(input)`](#module_utils/image.RawImage.read) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
        * [`.fromCanvas(canvas)`](#module_utils/image.RawImage.fromCanvas) ⇒ [RawImage](#RawImage)
        * [`.fromURL(url)`](#module_utils/image.RawImage.fromURL) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
        * [`.fromBlob(blob)`](#module_utils/image.RawImage.fromBlob) ⇒ [Promise.&lt;RawImage&gt;](#RawImage)
        * [`.fromTensor(tensor)`](#module_utils/image.RawImage.fromTensor)

* * *

### `new RawImage(data, width, height, channels)`

Create a new `RawImage` object.

  
    
      ParamTypeDescription
    
  
  

    dataUint8ClampedArray | Uint8ArrayThe pixel data.

    
    widthnumberThe width of the image.

    
    heightnumberThe height of the image.

    
    channels1 | 2 | 3 | 4The number of channels.

      

* * *

### `rawImage.size` ⇒ Array

Returns the size of the image (width, height).

**Kind**: instance property of [RawImage](#module_utils/image.RawImage)  
**Returns**: Array - The size of the image (width, height).  

* * *

### `rawImage.grayscale()` ⇒ [RawImage](#RawImage)

Convert the image to grayscale format.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - `this` to support chaining.  

* * *

### `rawImage.rgb()` ⇒ [RawImage](#RawImage)

Convert the image to RGB format.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - `this` to support chaining.  

* * *

### `rawImage.rgba()` ⇒ [RawImage](#RawImage)

Convert the image to RGBA format.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - `this` to support chaining.  

* * *

### `rawImage.putAlpha(mask)` ⇒ [RawImage](#RawImage)

Apply an alpha mask to the image. Operates in place.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - The masked image.  
**Throws**:

- Error If the mask is not the same size as the image.
- Error If the image does not have 4 channels.
- Error If the mask is not a single channel.

  
    
      ParamTypeDescription
    
  
  

    maskRawImageThe mask to apply. It should have a single channel.

      

* * *

### `rawImage.resize(width, height, options)` ⇒ [Promise.&lt;RawImage&gt;](#RawImage)

Resize the image to the given dimensions. This method uses the canvas API to perform the resizing.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [Promise.&lt;RawImage&gt;](#RawImage) - `this` to support chaining.  

  
    
      ParamTypeDescription
    
  
  

    widthnumberThe width of the new image. null or -1 will preserve the aspect ratio.

    
    heightnumberThe height of the new image. null or -1 will preserve the aspect ratio.

    
    optionsObjectAdditional options for resizing.

    
    [options.resample]0 | 1 | 2 | 3 | 4 | 5 | stringThe resampling method to use.

      

* * *

### `rawImage.split()` ⇒ Array

Split this image into individual bands. This method returns an array of individual image bands from an image.
For example, splitting an "RGB" image creates three new images each containing a copy of one of the original bands (red, green, blue).

Inspired by PIL's `Image.split()` [function](https://pillow.readthedocs.io/en/latest/reference/Image.html#PIL.Image.Image.split).

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: Array - An array containing bands.  

* * *

### `rawImage.clone()` ⇒ [RawImage](#RawImage)

Clone the image

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - The cloned image  

* * *

### `rawImage.convert(numChannels)` ⇒ [RawImage](#RawImage)

Helper method for converting image to have a certain number of channels

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - `this` to support chaining.  

  
    
      ParamTypeDescription
    
  
  

    numChannelsnumberThe number of channels. Must be 1, 3, or 4.

      

* * *

### `rawImage.save(path)` ⇒ Promise.&lt;void&gt;

Save the image to the given path.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  

  
    
      ParamTypeDescription
    
  
  

    pathstringThe path to save the image to.

      

* * *

### `rawImage.toSharp()` ⇒ Sharp

Convert the image to a Sharp instance.

**Kind**: instance method of [RawImage](#module_utils/image.RawImage)  
**Returns**: Sharp - The Sharp instance.  

* * *

### `RawImage.read(input)` ⇒ [Promise.&lt;RawImage&gt;](#RawImage)

Helper method for reading an image from a variety of input types.

**Kind**: static method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [Promise.&lt;RawImage&gt;](#RawImage) - The image object.

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

  
    
      ParamType
    
  
  

    inputRawImage | string | URL | Blob | HTMLCanvasElement | OffscreenCanvas
      

* * *

### `RawImage.fromCanvas(canvas)` ⇒ [RawImage](#RawImage)

Read an image from a canvas.

**Kind**: static method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [RawImage](#RawImage) - The image object.  

  
    
      ParamTypeDescription
    
  
  

    canvasHTMLCanvasElement | OffscreenCanvasThe canvas to read the image from.

      

* * *

### `RawImage.fromURL(url)` ⇒ [Promise.&lt;RawImage&gt;](#RawImage)

Read an image from a URL or file path.

**Kind**: static method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [Promise.&lt;RawImage&gt;](#RawImage) - The image object.  

  
    
      ParamTypeDescription
    
  
  

    urlstring | URLThe URL or file path to read the image from.

      

* * *

### `RawImage.fromBlob(blob)` ⇒ [Promise.&lt;RawImage&gt;](#RawImage)

Helper method to create a new Image from a blob.

**Kind**: static method of [RawImage](#module_utils/image.RawImage)  
**Returns**: [Promise.&lt;RawImage&gt;](#RawImage) - The image object.  

  
    
      ParamTypeDescription
    
  
  

    blobBlobThe blob to read the image from.

      

* * *

### `RawImage.fromTensor(tensor)`

Helper method to create a new Image from a tensor

**Kind**: static method of [RawImage](#module_utils/image.RawImage)  

  
    
      ParamType
    
  
  

    tensorTensor
      

* * *

## `utils/image.load_image`

Helper function to load an image from a URL, path, etc.

**Kind**: static constant of [utils/image](#module_utils/image)  

* * *

## `utils/image~CONTENT_TYPE_MAP`

Mapping from file extensions to MIME types.

**Kind**: inner constant of [utils/image](#module_utils/image)  

* * *
