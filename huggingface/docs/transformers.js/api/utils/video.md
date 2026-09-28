# utils/video

Browser video loading helpers.

`load_video()` samples frames from a video source into `RawImage` frames so
vision-language models can consume short clips. Video decoding currently
relies on browser media APIs.

## Classes

### RawVideoFrame

A decoded video frame and its timestamp, in seconds.

#### `RawVideoFrame.constructor(image, timestamp)`

Create a video frame.

**Parameters**

- `image` ([`RawImage`](./image#module_utils/image.RawImage)) — The decoded image for this frame.
- `timestamp` (`number`) — The frame timestamp, in seconds.

### RawVideo

A sampled video represented as decoded frames plus total duration.

#### `RawVideo.constructor(frames, duration)`

Create a video from decoded frames.

**Parameters**

- `frames` ([`RawVideoFrame`](./video#module_utils/video.RawVideoFrame)[] | [`RawImage`](./image#module_utils/image.RawImage)[]) — Frames with timestamps, or images to space uniformly across `duration`.
- `duration` (`number`) — Duration in seconds.

#### `RawVideo.width` : `number`

Width of the video frames, in pixels.

#### `RawVideo.height` : `number`

Height of the video frames, in pixels.

#### `RawVideo.fps` : `number`

Effective sampled frame rate.

## Functions

### `load_video(src, [options])`

Load and sample frames from a video.

**Parameters**

- `src` (`string` | `Blob` | `HTMLVideoElement`) — The video to process.
- `options` (`Object`) _optional_ — Optional parameters.
  - `num_frames` (`number`) _optional_ — defaults to `null` — The number of frames to sample uniformly.
  - `fps` (`number`) _optional_ — defaults to `null` — The number of frames to sample per second.

**Returns:** `Promise`<[`RawVideo`](./video#module_utils/video.RawVideo)> — The loaded video.
