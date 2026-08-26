# Media

## Media Manager[[reachy_mini.media.media_manager.MediaManager]]

#### reachy_mini.media.media_manager.MediaManager[[reachy_mini.media.media_manager.MediaManager]]

```python
reachy_mini.media.media_manager.MediaManager(backend: MediaBackend = <MediaBackend.LOCAL: 'local'>, log_level: str = 'INFO', signalling_host: str = 'localhost', camera_specs: Optional[CameraSpecs] = None, daemon_url: str = '')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L105)

**Parameters:**

logger : Logger instance for media-related messages.

backend : The selected media backend (after deprecation resolution).

camera : Camera device instance, or `None`.

audio : Audio device instance, or `None`.

Media Manager for handling camera and audio devices.

This class provides a unified interface for managing both camera and audio
devices across different backends.  It handles initialization,
configuration, and cleanup of media resources.

#### close[[reachy_mini.media.media_manager.MediaManager.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L179)

Close the media manager and release resources.

#### disable_wobbling[[reachy_mini.media.media_manager.MediaManager.disable_wobbling]]

```python
disable_wobbling()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L408)

Disable head wobbling.

#### enable_wobbling[[reachy_mini.media.media_manager.MediaManager.enable_wobbling]]

```python
enable_wobbling(callback: Callable[[SpeechOffsets], None])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L387)

**Parameters:**

callback : Called with `(x_m, y_m, z_m, roll_rad, pitch_rad, yaw_rad)` for each movement hop.

Enable head wobbling driven by audio playback.

Only supported with the LOCAL backend (GStreamerAudio).

#### get_DoA[[reachy_mini.media.media_manager.MediaManager.get_DoA]]

```python
get_DoA()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L418)

**Returns:**

A tuple `(angle_radians, speech_detected)`, or `None` if the
audio system is not available.

Get the Direction of Arrival (DoA) from the microphone array.

#### get_audio_sample[[reachy_mini.media.media_manager.MediaManager.get_audio_sample]]

```python
get_audio_sample()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L287)

**Returns:**

The recorded audio sample, or `None` if no data is available.

Get an audio sample from the audio device.

#### get_frame[[reachy_mini.media.media_manager.MediaManager.get_frame]]

```python
get_frame()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L243)

**Returns:**

The captured BGR frame as a numpy array with shape
`(height, width, 3)`, or `None` if the camera is not
available.

Get a frame from the camera.

#### get_frame_jpeg[[reachy_mini.media.media_manager.MediaManager.get_frame_jpeg]]

```python
get_frame_jpeg()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L257)

Get the current camera frame as JPEG bytes, or `None` if unavailable.

#### get_input_audio_samplerate[[reachy_mini.media.media_manager.MediaManager.get_input_audio_samplerate]]

```python
get_input_audio_samplerate()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L299)

Get the input samplerate of the audio device.

#### get_input_channels[[reachy_mini.media.media_manager.MediaManager.get_input_channels]]

```python
get_input_channels()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L313)

Get the number of input channels of the audio device.

#### get_output_audio_samplerate[[reachy_mini.media.media_manager.MediaManager.get_output_audio_samplerate]]

```python
get_output_audio_samplerate()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L306)

Get the output samplerate of the audio device.

#### get_output_channels[[reachy_mini.media.media_manager.MediaManager.get_output_channels]]

```python
get_output_channels()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L320)

Get the number of output channels of the audio device.

#### play_sound[[reachy_mini.media.media_manager.MediaManager.play_sound]]

```python
play_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L264)

**Parameters:**

sound_file : Path to the sound file to play.

Play a sound file.

Note:
If the audio backend is not initialised, a warning is logged
and the call is silently ignored.

#### push_audio_sample[[reachy_mini.media.media_manager.MediaManager.push_audio_sample]]

```python
push_audio_sample(data: npt.NDArray[np.float32])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L341)

**Parameters:**

data : Audio samples as a float32 array.  Shape should be `(num_samples,)` for mono or `(num_samples, channels)` for multi-channel.  The manager adapts the data to match the output device's channel count before forwarding it.

Push audio data to the output device.

#### start_playing[[reachy_mini.media.media_manager.MediaManager.start_playing]]

```python
start_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L334)

Start playing audio.

#### start_recording[[reachy_mini.media.media_manager.MediaManager.start_recording]]

```python
start_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L280)

Start recording audio.

#### stop_playing[[reachy_mini.media.media_manager.MediaManager.stop_playing]]

```python
stop_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L380)

Stop playing audio.

#### stop_recording[[reachy_mini.media.media_manager.MediaManager.stop_recording]]

```python
stop_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_manager.py#L327)

Stop recording audio.

## Audio[[reachy_mini.media.audio_base.AudioBase]]

#### reachy_mini.media.audio_base.AudioBase[[reachy_mini.media.audio_base.AudioBase]]

```python
reachy_mini.media.audio_base.AudioBase(log_level: str = 'INFO')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L103)

**Parameters:**

SAMPLE_RATE : Default sample rate (16 000 Hz — ReSpeaker hardware).

CHANNELS : Number of audio channels (2 — stereo).

GAP_RESET_NS : PTS-continuity threshold for `_compute_pts`. If the gap between the next expected PTS and the appsrc's current running-time exceeds this value, we treat it as a new utterance and re-anchor to running-time.

Abstract audio backend.

#### apply_audio_config[[reachy_mini.media.audio_base.AudioBase.apply_audio_config]]

```python
apply_audio_config(config: Sequence, verify: bool = True, write_settle_seconds: float = 0.1)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L232)

**Parameters:**

config : Sequence of `(parameter_name, values)` pairs to write.

verify : When true, read each parameter back after writing it.

write_settle_seconds : Delay after each write before readback.

**Returns:**

True when all parameters were written and verified successfully.
False when the ReSpeaker audio board is unavailable or a parameter
write/readback fails.

Apply caller-provided audio control parameters to the ReSpeaker.

This opens a short-lived ReSpeaker USB handle, writes each parameter in
`config`, and optionally verifies the written values. The SDK does
not provide default values for these parameters; callers should pass the
values tuned for their own app.

#### cleanup[[reachy_mini.media.audio_base.AudioBase.cleanup]]

```python
cleanup()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L270)

Release shared resources (DoA USB device).

#### clear_output_buffer[[reachy_mini.media.audio_base.AudioBase.clear_output_buffer]]

```python
clear_output_buffer()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L309)

Use `clear_player` instead. Deprecated; does nothing.

#### clear_player[[reachy_mini.media.audio_base.AudioBase.clear_player]]

```python
clear_player()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L318)

Drop any queued playback audio immediately (barge-in).

#### get_DoA[[reachy_mini.media.audio_base.AudioBase.get_DoA]]

```python
get_DoA()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L222)

**Returns:**

A tuple `(angle_radians, speech_detected)` or `None`
if the device is unavailable.

Get the Direction of Arrival (DoA) from the ReSpeaker.

#### get_audio_sample[[reachy_mini.media.audio_base.AudioBase.get_audio_sample]]

```python
get_audio_sample()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L174)

**Returns:**

A float32 array of shape `(num_samples, 2)` (stereo), or
`None` if no data is available yet.

Pull the next recorded audio chunk.

#### get_input_audio_samplerate[[reachy_mini.media.audio_base.AudioBase.get_input_audio_samplerate]]

```python
get_input_audio_samplerate()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L190)

Input sample rate in Hz (16 000).

#### get_input_channels[[reachy_mini.media.audio_base.AudioBase.get_input_channels]]

```python
get_input_channels()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L198)

Return the number of input channels (2 — stereo).

#### get_output_audio_samplerate[[reachy_mini.media.audio_base.AudioBase.get_output_audio_samplerate]]

```python
get_output_audio_samplerate()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L194)

Output sample rate in Hz (16 000).

#### get_output_channels[[reachy_mini.media.audio_base.AudioBase.get_output_channels]]

```python
get_output_channels()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L202)

Return the number of output channels (2 — stereo).

#### play_sound[[reachy_mini.media.audio_base.AudioBase.play_sound]]

```python
play_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L323)

Play a sound file.

#### push_audio_sample[[reachy_mini.media.audio_base.AudioBase.push_audio_sample]]

```python
push_audio_sample(data: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L294)

**Parameters:**

data : Audio samples as a float32 array.

Push audio data to the output appsrc.

#### set_max_output_buffers[[reachy_mini.media.audio_base.AudioBase.set_max_output_buffers]]

```python
set_max_output_buffers(max_buffers: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L206)

**Parameters:**

max_buffers : Maximum number of buffers to queue.

Limit the number of queued playback buffers.

#### start_playing[[reachy_mini.media.audio_base.AudioBase.start_playing]]

```python
start_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L284)

Start the playback pipeline.

#### start_recording[[reachy_mini.media.audio_base.AudioBase.start_recording]]

```python
start_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L274)

Start capturing audio from the microphone.

#### stop_playing[[reachy_mini.media.audio_base.AudioBase.stop_playing]]

```python
stop_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L289)

Stop the playback pipeline.

#### stop_recording[[reachy_mini.media.audio_base.AudioBase.stop_recording]]

```python
stop_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_base.py#L279)

Stop the recording pipeline.

#### reachy_mini.media.audio_gstreamer.GStreamerAudio[[reachy_mini.media.audio_gstreamer.GStreamerAudio]]

```python
reachy_mini.media.audio_gstreamer.GStreamerAudio(log_level: str = 'INFO')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L88)

Audio implementation using GStreamer.

Extends `AudioBase` with a GStreamer-specific helper:

- `clear_player()`: flush the playback appsrc immediately via GStreamer
  flush events, dropping any queued audio.

(`clear_output_buffer()` is deprecated and does nothing; use
`clear_player()` instead.)

#### cleanup[[reachy_mini.media.audio_gstreamer.GStreamerAudio.cleanup]]

```python
cleanup()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L631)

Release all resources (pipelines, USB devices).

#### clear_player[[reachy_mini.media.audio_gstreamer.GStreamerAudio.clear_player]]

```python
clear_player()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L506)

Flush the player's appsrc to drop any queued audio immediately.

#### delete_sound[[reachy_mini.media.audio_gstreamer.GStreamerAudio.delete_sound]]

```python
delete_sound(filename: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L592)

**Returns:**

Always `False`.

No-op for the local backend.

#### disable_wobbling[[reachy_mini.media.audio_gstreamer.GStreamerAudio.disable_wobbling]]

```python
disable_wobbling()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L624)

Disable head wobbling.

#### enable_wobbling[[reachy_mini.media.audio_gstreamer.GStreamerAudio.enable_wobbling]]

```python
enable_wobbling(callback: Callable)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L611)

**Parameters:**

callback : Called with `(x_m, y_m, z_m, roll_rad, pitch_rad, yaw_rad)` for each movement hop.

Enable head wobbling driven by audio playback.

#### list_sounds[[reachy_mini.media.audio_gstreamer.GStreamerAudio.list_sounds]]

```python
list_sounds()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L583)

**Returns:**

An empty list.

No-op for the local backend.

#### play_sound[[reachy_mini.media.audio_gstreamer.GStreamerAudio.play_sound]]

```python
play_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L522)

**Parameters:**

sound_file : Absolute path **or** filename relative to the built-in assets directory.

**Raises:** `FileNotFoundError`

- `FileNotFoundError` -- If the file cannot be found.

Play a sound file through the Reachy Mini Audio card.

The file is played via a GStreamer `playbin` routed to the same
audio sink used by the push-based playback pipeline.  When the head
wobbler is enabled the audio is also forked to it via a tee.

#### start_playing[[reachy_mini.media.audio_gstreamer.GStreamerAudio.start_playing]]

```python
start_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L488)

Start the playback pipeline so `push_audio_sample` can feed data.

#### upload_sound[[reachy_mini.media.audio_gstreamer.GStreamerAudio.upload_sound]]

```python
upload_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_gstreamer.py#L574)

**Returns:**

The unchanged *sound_file* path.

No-op for the local backend — the file is already accessible.

### Audio Utils Functions[[reachy_mini.media.audio_utils.get_respeaker_card_number]]

#### reachy_mini.media.audio_utils.get_respeaker_card_number[[reachy_mini.media.audio_utils.get_respeaker_card_number]]

```python
reachy_mini.media.audio_utils.get_respeaker_card_number()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_utils.py#L97)

**Returns:** `int`

The card number of the detected ReSpeaker/Reachy Mini Audio device.
Returns 0 if no specific device is found (uses default sound card),
or -1 if there's an error running the detection command.

Return the card number of the ReSpeaker sound card, or 0 if not found.

Note:
This function runs 'arecord -l' to list available audio capture devices
and processes the output to find Reachy Mini Audio or ReSpeaker devices.
It's primarily used on Linux systems with ALSA audio configuration.

The function returns:
- Positive integer: Card number of detected Reachy Mini Audio device
- 0: No Reachy Mini Audio device found, using default sound card
- -1: Error occurred while trying to detect audio devices

Example:
```python
card_num = get_respeaker_card_number()
if card_num > 0:
    print(f"Using Reachy Mini Audio card {card_num}")
elif card_num == 0:
    print("Using default sound card")
else:
    print("Error detecting audio devices")
```

#### reachy_mini.media.audio_utils.has_reachymini_asoundrc[[reachy_mini.media.audio_utils.has_reachymini_asoundrc]]

```python
reachy_mini.media.audio_utils.has_reachymini_asoundrc()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_utils.py#L140)

**Returns:** `bool`

True if ~/.asoundrc exists and contains the required Reachy Mini
audio configuration entries, False otherwise.

Check if ~/.asoundrc exists and contains both reachymini_audio_sink and reachymini_audio_src.

Note:
This function checks for the presence of the ALSA configuration file
~/.asoundrc and verifies that it contains the necessary configuration
entries for Reachy Mini audio devices (reachymini_audio_sink and
reachymini_audio_src). These entries are required for proper audio
routing and device management.

Example:
```python
if has_reachymini_asoundrc():
    print("Reachy Mini audio configuration is properly set up")
else:
    print("Need to configure Reachy Mini audio devices")
    write_asoundrc_to_home()  # Create the configuration
```

#### reachy_mini.media.audio_utils.check_reachymini_asoundrc[[reachy_mini.media.audio_utils.check_reachymini_asoundrc]]

```python
reachy_mini.media.audio_utils.check_reachymini_asoundrc()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_utils.py#L171)

Check if ~/.asoundrc exists and is correctly configured for Reachy Mini Audio.

#### reachy_mini.media.audio_utils.write_asoundrc_to_home[[reachy_mini.media.audio_utils.write_asoundrc_to_home]]

```python
reachy_mini.media.audio_utils.write_asoundrc_to_home()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_utils.py#L190)

Write the .asoundrc file with Reachy Mini audio configuration to the user's home directory.

This function creates an ALSA configuration file (.asoundrc) in the user's home directory
that configures the ReSpeaker sound card for proper audio routing and multi-client support.
The configuration enables simultaneous audio input and output access, which is essential
for the Reachy Mini Wireless version's audio functionality.

The generated configuration includes:
- Default audio device settings pointing to the ReSpeaker sound card
- dmix plugin for multi-client audio output (reachymini_audio_sink)
- dsnoop plugin for multi-client audio input (reachymini_audio_src)
- Proper buffer and sample rate settings for optimal performance

Note:
This function automatically detects the ReSpeaker card number and creates a configuration
tailored to the detected hardware. It is primarily used for the Reachy Mini Wireless version.

The configuration file will be created at ~/.asoundrc and will overwrite any existing file
with the same name. Existing audio configurations should be backed up before calling this function.

### Audio Control Utils Functions[[reachy_mini.media.audio_control_utils.ReSpeaker]]

#### reachy_mini.media.audio_control_utils.ReSpeaker[[reachy_mini.media.audio_control_utils.ReSpeaker]]

```python
reachy_mini.media.audio_control_utils.ReSpeaker(dev: Device)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L181)

Class to interface with the ReSpeaker XVF3800 USB device.

#### apply_audio_config[[reachy_mini.media.audio_control_utils.ReSpeaker.apply_audio_config]]

```python
apply_audio_config(config: Sequence, verify: bool = True, write_settle_seconds: float = 0.1)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L331)

**Parameters:**

config : Parameter names and values to write.

verify : When true, read each parameter back after writing it.

write_settle_seconds : Delay after each write before readback.

**Returns:**

True when all parameters were written and verified successfully.

Apply a set of audio control parameters to the ReSpeaker.

#### close[[reachy_mini.media.audio_control_utils.ReSpeaker.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L459)

Close the interface.

#### read[[reachy_mini.media.audio_control_utils.ReSpeaker.read]]

```python
read(name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L249)

Read data from a specified parameter on the ReSpeaker device.

#### read_values[[reachy_mini.media.audio_control_utils.ReSpeaker.read_values]]

```python
read_values(name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L326)

Read a parameter and decode it into numeric values.

#### write[[reachy_mini.media.audio_control_utils.ReSpeaker.write]]

```python
write(name: str, data_list: Any)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L190)

Write data to a specified parameter on the ReSpeaker device.

#### reachy_mini.media.audio_control_utils.find[[reachy_mini.media.audio_control_utils.find]]

```python
reachy_mini.media.audio_control_utils.find(vid: int = 10374, pid: int = 26)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L464)

**Parameters:**

vid (int) : USB Vendor ID to search for. Default: 0x2886 (XMOS).

pid (int) : USB Product ID to search for. Default: 0x001A (XMOS XVF3800).

**Returns:** `ReSpeaker | None`

A ReSpeaker object if the device is found,
None otherwise.

Find and return the ReSpeaker USB device with the given Vendor ID and Product ID.

Note:
This function searches for USB devices with the specified Vendor ID
and Product ID using libusb backend. The default values target
XMOS XVF3800 devices used in ReSpeaker microphone arrays.

Example:
```python
from reachy_mini.media.audio_control_utils import find

# Find default ReSpeaker device
respeaker = find()
if respeaker is not None:
    print("Found ReSpeaker device")
    respeaker.close()

# Find specific device
custom_device = find(vid=0x1234, pid=0x5678)
```

#### reachy_mini.media.audio_control_utils.init_respeaker_usb[[reachy_mini.media.audio_control_utils.init_respeaker_usb]]

```python
reachy_mini.media.audio_control_utils.init_respeaker_usb()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/audio_control_utils.py#L502)

**Returns:** Optional[ReSpeaker]

A ReSpeaker object if a compatible device is found,
None otherwise.

Initialize the ReSpeaker USB device. Looks for both new and beta device IDs.

Note:
This function attempts to initialize a ReSpeaker microphone array by
searching for USB devices with known Vendor and Product IDs. It tries:
1. New Reachy Mini Audio firmware (0x38FB:0x1001) - preferred
2. Old ReSpeaker firmware (0x2886:0x001A) - with warning to update

The function handles USB backend errors gracefully and returns
None if no compatible device is found or if initialization fails.

Example:
```python
from reachy_mini.media.audio_control_utils import init_respeaker_usb

# Initialize ReSpeaker device
respeaker = init_respeaker_usb()
if respeaker is not None:
    print("ReSpeaker initialized successfully")
    # Use the device...
    doa = respeaker.read("DOA_VALUE_RADIANS")
    respeaker.close()
else:
    print("No ReSpeaker device found")
```

## Camera[[reachy_mini.media.camera_gstreamer.GStreamerCamera]]

#### reachy_mini.media.camera_gstreamer.GStreamerCamera[[reachy_mini.media.camera_gstreamer.GStreamerCamera]]

```python
reachy_mini.media.camera_gstreamer.GStreamerCamera(log_level: str = 'INFO', camera_specs: typing.Optional[reachy_mini.media.camera_constants.CameraSpecs] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_gstreamer.py#L76)

**Parameters:**

camera_specs : Camera specifications (resolutions, intrinsics, …).

Camera that reads BGR frames from the daemon's local IPC endpoint.

The WebRTC daemon exposes BGR camera frames via a local IPC mechanism:

- Linux / macOS: `unixfdsink` / `unixfdsrc` (Unix domain socket)
- Windows: `win32ipcvideosink` / `win32ipcvideosrc` (shared memory)

Since the daemon's IPC branch already converts to BGR, the reader
pipeline is simply `source → queue → appsink` with no extra
conversion.

#### close[[reachy_mini.media.camera_gstreamer.GStreamerCamera.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_gstreamer.py#L271)

Stop the pipeline and release resources.

#### open[[reachy_mini.media.camera_gstreamer.GStreamerCamera.open]]

```python
open()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_gstreamer.py#L240)

Start the GStreamer pipeline and begin receiving frames.

#### read[[reachy_mini.media.camera_gstreamer.GStreamerCamera.read]]

```python
read()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_gstreamer.py#L256)

**Returns:**

A NumPy array of shape `(height, width, 3)` in BGR order,
or `None` if no frame is available within the timeout.

Pull the latest BGR frame from the IPC endpoint.

### Camera Utils Functions[[reachy_mini.media.camera_utils.undistort_points]]

#### reachy_mini.media.camera_utils.undistort_points[[reachy_mini.media.camera_utils.undistort_points]]

```python
reachy_mini.media.camera_utils.undistort_points(u: float, v: float, K: ndarray, D: ndarray, max_iterations: int = 20, epsilon: float = 0.01)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_utils.py#L46)

**Parameters:**

u : Horizontal pixel coordinate.

v : Vertical pixel coordinate.

K : 3x3 camera intrinsic matrix [[fx, 0, cx], [0, fy, cy], [0, 0, 1]].

D : Distortion coefficients array. Supports lengths 0, 4, 5, 8, 12, or 14. Unused positions default to 0.

max_iterations : Maximum number of iterations (default 20).

epsilon : Convergence threshold in pixel reprojection error (default 0.01).

**Returns:** `Tuple (x_n, y_n)`

Normalized undistorted coordinates (on the z=1 plane).

Undistort a single pixel coordinate to normalized camera coordinates.

Pure numpy equivalent of cv2.undistortPoints(). Supports the OpenCV distortion
model with up to 12 coefficients (rational model + thin prism):
D = (k1, k2, p1, p2, k3, k4, k5, k6, s1, s2, s3, s4)

Also works with 5-coefficient models (k1, k2, p1, p2, k3) and zero-distortion.

The algorithm matches OpenCV's cvUndistortPointsInternal:
1. Remove camera intrinsics to get normalized distorted coordinates.
2. Iteratively solve for undistorted coordinates using a damped
   fixed-point method with adaptive step size.

Reference:
OpenCV distortion model and undistortPoints algorithm:
https://docs.opencv.org/4.x/d9/d0c/group__calib3d.html
https://github.com/opencv/opencv/blob/4.x/modules/calib3d/src/undistort.dispatch.cpp

#### reachy_mini.media.camera_utils.scale_intrinsics[[reachy_mini.media.camera_utils.scale_intrinsics]]

```python
reachy_mini.media.camera_utils.scale_intrinsics(K_original: ndarray, original_size: typing.Tuple[int, int], target_size: typing.Tuple[int, int], crop_scale: float)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_utils.py#L180)

**Parameters:**

K_original : Original 3x3 camera matrix

original_size : (width, height) of original calibration

target_size : (width, height) of target resolution

crop_scale : Scale factor due to digital zoom/crop (>1 means more zoomed in)

**Returns:** `K_scaled`

Adjusted camera matrix for target resolution

Scale camera intrinsics for a different resolution with cropping.

### Camera Constants[[reachy_mini.media.camera_constants.CameraResolution]]

#### reachy_mini.media.camera_constants.CameraResolution[[reachy_mini.media.camera_constants.CameraResolution]]

```python
reachy_mini.media.camera_constants.CameraResolution(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L37)

**Parameters:**

R1536x864at40fps : 1536x864 resolution at 40 fps

R1280x720at60fps : 1280x720 resolution at 60 fps (HD)

R1280x720at30fps : 1280x720 resolution at 30 fps (HD)

R1920x1080at30fps : 1920x1080 resolution at 30 fps (Full HD)

R1920x1080at60fps : 1920x1080 resolution at 60 fps (Full HD)

R2304x1296at30fps : 2304x1296 resolution at 30 fps

R1600x1200at30fps : 1600x1200 resolution at 30 fps

R3264x2448at30fps : 3264x2448 resolution at 30 fps

R3264x2448at10fps : 3264x2448 resolution at 10 fps

R3840x2592at30fps : 3840x2592 resolution at 30 fps

R3840x2592at10fps : 3840x2592 resolution at 10 fps

R3840x2160at30fps : 3840x2160 resolution at 30 fps (4K UHD)

R3840x2160at10fps : 3840x2160 resolution at 10 fps (4K UHD)

R3072x1728at10fps : 3072x1728 resolution at 10 fps

R4608x2592at10fps : 4608x2592 resolution at 10 fps

Base class for camera resolutions.

Enumeration of standardized camera resolutions and frame rates supported
by Reachy Mini cameras. Each enum value contains a tuple of (width, height, fps).

Note:
The enum values are tuples containing (width, height, frames_per_second, crop_factor).
Not all resolutions are supported by all camera models - check the specific
camera specifications for available resolutions.

Example:
```python
from reachy_mini.media.camera_constants import CameraResolution

# Get resolution information
res = CameraResolution.R1280x720at30fps
width, height, fps, crop_factor = res.value
print(f"Resolution: {width}x{height}@{fps}fps")

# Check if a resolution is supported by a camera
from reachy_mini.media.camera_constants import ReachyMiniLiteCamSpecs
res = CameraResolution.R1920x1080at60fps
if res in ReachyMiniLiteCamSpecs.available_resolutions:
    print("This resolution is supported")
```

#### reachy_mini.media.camera_constants.CameraSpecs[[reachy_mini.media.camera_constants.CameraSpecs]]

```python
reachy_mini.media.camera_constants.CameraSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L110)

**Parameters:**

name (str) : Human-readable name of the camera model.

available_resolutions (List[CameraResolution]) : List of supported resolutions and frame rates for this camera model.

default_resolution (CameraResolution) : Default resolution used when the camera is initialized.

vid (int) : USB Vendor ID for identifying this camera model.

pid (int) : USB Product ID for identifying this camera model.

K (npt.NDArray[np.float64]) : 3x3 camera intrinsic matrix containing focal lengths and principal point coordinates.

D (npt.NDArray[np.float64]) : 5-element array containing distortion coefficients (k1, k2, p1, p2, k3) for radial and tangential distortion.

Base camera specifications.

Dataclass containing specifications for a camera model, including supported
resolutions, calibration parameters, and USB identification information.

Note:
The intrinsic matrix K has the format:
[[fx,  0, cx],
[ 0, fy, cy],
[ 0,  0,  1]]

Where fx, fy are focal lengths in pixels, and cx, cy are the principal
point coordinates (typically near the image center).

Example:
```python
from reachy_mini.media.camera_constants import CameraSpecs

# Create a custom camera specification
custom_specs = CameraSpecs(
    name="custom_camera",
    available_resolutions=[CameraResolution.R1280x720at30fps],
    default_resolution=CameraResolution.R1280x720at30fps,
    vid=0x1234,
    pid=0x5678,
    K=np.array([[800, 0, 640], [0, 800, 360], [0, 0, 1]]),
    D=np.zeros(5)
)
```

#### reachy_mini.media.camera_constants.ArducamSpecs[[reachy_mini.media.camera_constants.ArducamSpecs]]

```python
reachy_mini.media.camera_constants.ArducamSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L180)

Arducam camera specifications.

#### reachy_mini.media.camera_constants.ReachyMiniLiteCamSpecs[[reachy_mini.media.camera_constants.ReachyMiniLiteCamSpecs]]

```python
reachy_mini.media.camera_constants.ReachyMiniLiteCamSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L199)

Reachy Mini Lite camera specifications.

#### reachy_mini.media.camera_constants.ReachyMiniWirelessCamSpecs[[reachy_mini.media.camera_constants.ReachyMiniWirelessCamSpecs]]

```python
reachy_mini.media.camera_constants.ReachyMiniWirelessCamSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L246)

Reachy Mini Wireless camera specifications.

#### reachy_mini.media.camera_constants.OlderRPiCamSpecs[[reachy_mini.media.camera_constants.OlderRPiCamSpecs]]

```python
reachy_mini.media.camera_constants.OlderRPiCamSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L264)

Older Raspberry Pi camera specifications. Keeping for compatibility.

#### reachy_mini.media.camera_constants.MujocoCameraSpecs[[reachy_mini.media.camera_constants.MujocoCameraSpecs]]

```python
reachy_mini.media.camera_constants.MujocoCameraSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L273)

Mujoco simulated camera specifications.

#### reachy_mini.media.camera_constants.GenericWebcamSpecs[[reachy_mini.media.camera_constants.GenericWebcamSpecs]]

```python
reachy_mini.media.camera_constants.GenericWebcamSpecs(name: str = '', available_resolutions: typing.List[reachy_mini.media.camera_constants.CameraResolution] = <factory>, default_resolution: CameraResolution = <CameraResolution.R1280x720at30fps: (1280, 720, 30, 1.0)>, K: ndarray = <factory>, D: ndarray = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/camera_constants.py#L301)

Generic webcam specifications (fallback for any webcam).

## WebRTC[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient]]

#### reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient]]

```python
reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient(log_level: str = 'INFO', peer_id: str = '', signaling_host: str = '', signaling_port: int = 8443, camera_specs: typing.Optional[reachy_mini.media.camera_constants.CameraSpecs] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L68)

WebRTC client that provides both camera frames and audio.

Implements the same public API surface as `GStreamerCamera` (for
video) and `GStreamerAudio` (for audio) so that `MediaManager`
can assign the same instance to both its `camera` and `audio`
slots.

#### cleanup[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.cleanup]]

```python
cleanup()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L669)

Release all resources.

#### clear_player[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.clear_player]]

```python
clear_player()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L524)

Drop queued playback audio during barge-in.

Flushes the local audio *send* chain so any not-yet-sent samples
are dropped, then asks the daemon to flush the audio it has
already received and queued for the robot's speaker (where the
bulk of buffered audio actually sits).

#### close[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L349)

Stop the WebRTC pipeline.

#### delete_sound[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.delete_sound]]

```python
delete_sound(filename: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L637)

**Parameters:**

filename : Name of the file to delete (not a full path).

**Returns:**

`True` if the file was deleted, `False` otherwise.

Delete a sound file from the daemon's temporary sound directory.

#### get_DoA[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.get_DoA]]

```python
get_DoA()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L660)

**Returns:**

A tuple `(angle_radians, speech_detected)` or `None`.

Get the Direction of Arrival from the ReSpeaker.

#### list_sounds[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.list_sounds]]

```python
list_sounds()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L615)

**Returns:**

A list of filenames, or an empty list on error.

List sound files in the daemon's temporary sound directory.

#### open[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.open]]

```python
open()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L331)

Start the WebRTC pipeline (both video and audio).

#### play_sound[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.play_sound]]

```python
play_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L552)

**Parameters:**

sound_file : Absolute local path **or** asset filename (e.g. `"wake_up.wav"`).

Play a sound file on the robot's speaker via the daemon REST API.

If *sound_file* is a local path that exists on this machine the
file is uploaded to the daemon's temporary sound directory
(overwriting any previous upload with the same basename).
Otherwise the filename is sent as-is and the daemon resolves it
from its built-in assets or filesystem.

#### read[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.read]]

```python
read()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L335)

**Returns:**

A NumPy array of shape `(height, width, 3)` or `None`.

Pull the latest BGR video frame.

#### start_playing[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.start_playing]]

```python
start_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L507)

No-op — audio send chain is set up automatically on WebRTC connection.

#### start_recording[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.start_recording]]

```python
start_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L354)

No-op — recording starts automatically with `open()`.

#### stop_playing[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.stop_playing]]

```python
stop_playing()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L511)

Reset the PTS counter for the send chain and stop daemon-side sound.

#### stop_recording[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.stop_recording]]

```python
stop_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L358)

No-op — managed by `close()`.

#### upload_sound[[reachy_mini.media.webrtc_client_gstreamer.GstWebRTCClient.upload_sound]]

```python
upload_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/webrtc_client_gstreamer.py#L585)

**Parameters:**

sound_file : Local path to the sound file.

**Returns:**

The absolute path of the file on the daemon.

**Raises:** ``FileNotFoundError`` or ``requests.HTTPError``

- ``FileNotFoundError`` -- If *sound_file* does not exist locally.
- ``requests.HTTPError`` -- If the upload request fails.

Upload a local sound file to the daemon's temporary directory.

#### reachy_mini.media.media_server.GstMediaServer[[reachy_mini.media.media_server.GstMediaServer]]

```python
reachy_mini.media.media_server.GstMediaServer(log_level: str = 'INFO', sim_mode: SimulationMode = <SimulationMode.NONE: 'none'>, enable_turn: typing.Optional[bool] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L130)

**Parameters:**

camera_specs (CameraSpecs) : Specifications of the detected camera.

resized_K (npt.NDArray[np.float64]) : Camera intrinsic matrix for current resolution.

Daemon-side GStreamer media server.

Owns the camera and audio hardware and distributes media to consumers:

- **IPC branch** — raw BGR frames via `unixfdsink` / `win32ipcvideosink`
  for on-device applications (`GStreamerCamera` reads from this).
- **WebRTC branch** — encoded video + audio via `webrtcsink` for remote
  clients (`GstWebRTCClient` connects to this).
- **Sound playback** — `playbin` for playing WAV files on the speaker.

#### clear_incoming_audio[[reachy_mini.media.media_server.GstMediaServer.clear_incoming_audio]]

```python
clear_incoming_audio()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L720)

Flush queued/rendering audio in the incoming-audio playback pipeline.

Used for barge-in: drops audio already received from a WebRTC client
and queued for the robot's speaker so the robot stops speaking promptly.

The playback pipeline shares the sender clock + base-time, so incoming
buffer PTS live in that shared running-time; we flush with
`reset_time=False` to keep the timeline intact (`reset_time=True`
would strand future-stamped buffers and stall playback). The pad probe
keeps pushing new RTP buffers into the appsrc, which resume in sync.

#### close[[reachy_mini.media.media_server.GstMediaServer.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L322)

Release GStreamer resources (MainLoop, bus watch).

Reached from `__del__`, so it has to tolerate a half-built instance:
`__init__` raises before any of these attributes exist when the camera
resolution can't be determined, and a bare attribute access here would
bury that real error under an unraisable `AttributeError`.

#### disable_wobbling[[reachy_mini.media.media_server.GstMediaServer.disable_wobbling]]

```python
disable_wobbling()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1523)

Disable head wobbling.

#### enable_wobbling[[reachy_mini.media.media_server.GstMediaServer.enable_wobbling]]

```python
enable_wobbling(callback: typing.Callable[[tuple[float, float, float, float, float, float]], NoneType])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1510)

**Parameters:**

callback : Called with `(x_m, y_m, z_m, roll_rad, pitch_rad, yaw_rad)` for each movement hop.

Enable head wobbling driven by audio playback.

#### play_sound[[reachy_mini.media.media_server.GstMediaServer.play_sound]]

```python
play_sound(sound_file: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1316)

**Parameters:**

sound_file : Path to the sound file to play. If the file is not found at the given path, it is looked up in the assets directory.

Play a sound file on the robot's speaker.

Uses GStreamer's playbin element with a platform-aware audio sink.
This is used for daemon-side sounds (wake-up, sleep, etc.).

#### send_data_message[[reachy_mini.media.media_server.GstMediaServer.send_data_message]]

```python
send_data_message(peer_id: typing.Optional[str], message: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1806)

**Parameters:**

message : The string message to send

peer_id : If specified, send only to this peer. Otherwise broadcast to all.

Send a message to connected peers via data channel.

#### set_message_handler[[reachy_mini.media.media_server.GstMediaServer.set_message_handler]]

```python
set_message_handler(handler: typing.Callable[[str, str], NoneType])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1530)

**Parameters:**

handler : Callback function that receives (peer_id, message)

Set a callback for incoming data channel messages.

#### set_peer_disconnect_handler[[reachy_mini.media.media_server.GstMediaServer.set_peer_disconnect_handler]]

```python
set_peer_disconnect_handler(handler: typing.Callable[[str], NoneType])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1542)

Set a callback fired when a WebRTC peer disconnects.

The callback runs on the GStreamer/GLib thread (same context as
`_consumer_removed`) so consumers must hop back to their own
loop before touching shared state.

#### set_pose_provider[[reachy_mini.media.media_server.GstMediaServer.set_pose_provider]]

```python
set_pose_provider(provider: typing.Optional[typing.Callable[[], typing.Optional[str]]])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1853)

Register the callback that yields the JSON pose frame to push.

The backend passes `build_state_json` here. The provider is polled
on the GLib main loop at :attr:*POSE_PUSH_INTERVAL_MS*; returning
`None` skips a tick (e.g. before the first kinematics update).

Wiring a provider re-arms the timer if peers are already subscribed,
for the (unusual) case where the provider lands after the backend has
started accepting *subscribe_pose*.

#### set_pose_subscription[[reachy_mini.media.media_server.GstMediaServer.set_pose_subscription]]

```python
set_pose_subscription(peer_id: str, enabled: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1871)

Add/remove a peer from the pushed pose stream (see `_push_pose`).

Driven by the backend's `subscribe_pose`/`unsubscribe_pose` handling,
so this runs on the backend's asyncio thread rather than the GLib main
loop. Idempotent. Subscribing opens the peer's pose channel if it has
none yet and arms the push timer; the timer disarms itself once the
last subscriber leaves. The channel is kept open for the rest of the
session: unsubscribing only stops the frames.

#### set_session_failed_handler[[reachy_mini.media.media_server.GstMediaServer.set_session_failed_handler]]

```python
set_session_failed_handler(handler: typing.Callable[[str, str, typing.Dict[str, typing.Any]], NoneType])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1554)

**Parameters:**

handler : `(peer_id, reason, diagnostic_dict) -> None`. `reason` is one of `SESSION_FAILED_REASON_*`. `diagnostic_dict` carries the snapshot of the webrtcbin state at failure time, suitable for logs.

Set a callback fired when the negotiation watchdog gives up on a peer.

The callback runs on the GStreamer/GLib thread (or the
webrtcbin internal thread for *connection-state == failed*),
so consumers must hop back to their own loop before doing I/O.
Typical wiring is to forward to the central signaling relay
which converts the call into an `endSession` message for
the JS client.

#### start[[reachy_mini.media.media_server.GstMediaServer.start]]

```python
start()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1301)

Rebuild the pipeline from scratch and start it.

Rebuilding ensures a clean state after stop() released all hardware.

#### stop[[reachy_mini.media.media_server.GstMediaServer.stop]]

```python
stop()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1311)

Stop the pipeline and release all hardware (camera, audio).

#### stop_sound[[reachy_mini.media.media_server.GstMediaServer.stop_sound]]

```python
stop_sound()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/media/media_server.py#L1366)

Stop the currently playing sound file.

If no sound is currently playing this is a no-op.
