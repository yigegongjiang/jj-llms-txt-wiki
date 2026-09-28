# utils/audio

Audio I/O helpers.

Decode audio files and URLs into the `Float32Array` pipelines expect,
and wrap generated waveforms in `RawAudio` for playback or saving.

## Classes

### RawAudio

An audio buffer paired with its sampling rate.

```javascript
import { RawAudio } from '@huggingface/transformers';
const samples = new Float32Array(16000); // 1 second of silence @ 16 kHz
const audio = new RawAudio(samples, 16000);
const blob = audio.toBlob(); // WAV blob for upload or playback
await audio.save('out.wav'); // Saves the audio as a WAV file named 'out.wav'
```

#### `RawAudio.constructor(audio, sampling_rate)`

Create a new `RawAudio` object.

**Parameters**

- `audio` (`Float32Array` | `Float32Array[]`) — Audio data, either as a single `Float32Array` chunk or multiple `Float32Array` chunks.
- `sampling_rate` (`number`) — Sampling rate of the audio data

#### `RawAudio.data` : `Float32Array`

Get the audio data, accumulating all chunks if necessary.

#### `RawAudio.toBlob()`

Convert the audio to a blob.

**Returns:** `Blob`

#### `RawAudio.save(path)`

Save the audio to a wav file.

**Parameters**

- `path` (`string`)

**Returns:** `Promise`<`void`>

## Functions

### `load_audio(url, sampling_rate)`

Helper function to load audio from a path/URL.

**Parameters**

- `url` (`string` | `URL`) — The path/URL to load the audio from.
- `sampling_rate` (`number`) — The sampling rate to use when decoding the audio.

**Returns:** `Promise`<`Float32Array`> — The decoded audio as a `Float32Array`.
