# Sound Recording

This example demonstrates how to record audio from Reachy Mini's microphone array and save it to a WAV file. The script records for 5 seconds and saves the audio to `recorded_audio.wav`.

**How it works:**
1. Starts audio recording using `start_recording()`
2. Continuously retrieves audio samples using `get_audio_sample()`
3. Collects samples until the desired duration is reached
4. Stops recording and concatenates all samples
5. Saves the audio data to a WAV file using `soundfile`

**Features:**
- Configurable recording duration (default: 5 seconds)
- Automatic sample rate detection
- Timeout protection to prevent infinite loops
- Support for different media backends

**Usage:**
```bash
python sound_record.py --backend [default|local|webrtc]
```

The recorded audio will be saved to `recorded_audio.wav` in the current directory.

```python

import argparse
import time

import numpy as np

from reachy_mini import ReachyMini
from reachy_mini.media.audio_utils import save_audio_to_wav

TIMEOUT = 1
DURATION = 5  # seconds
OUTPUT_FILE = "recorded_audio.wav"

def main(backend: str) -> None:
    """Record audio for 5 seconds and save to a WAV file."""
    with ReachyMini(log_level="INFO", media_backend=backend) as mini:
        audio_samples = []
        mini.media.start_recording()

        # Wait to actually get an audio sample
        print("Waiting for the microphone to be ready...")
        start_time = time.time()
        while (
            mini.media.get_audio_sample() is None and time.time() - start_time < TIMEOUT
        ):
            time.sleep(0.005)

        if time.time() - start_time >= TIMEOUT:
            print(f"Timeout: the microphone did not respond in {TIMEOUT} seconds.")
            return

        print(f"Recording for {DURATION} seconds...")

        start_time = time.time()
        while time.time() - start_time < DURATION:
            sample = mini.media.get_audio_sample()
            if sample is not None:
                audio_samples.append(sample)

        mini.media.stop_recording()

        # Concatenate all samples and save
        if audio_samples:
            audio_data = np.concatenate(audio_samples, axis=0)
            samplerate = mini.media.get_input_audio_samplerate()
            save_audio_to_wav(audio_data, samplerate, OUTPUT_FILE)
            print(f"Audio saved to {OUTPUT_FILE}")
        else:
            print("No audio data recorded.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Records audio from Reachy Mini's microphone."
    )
    parser.add_argument(
        "--backend",
        type=str,
        choices=["default", "local", "webrtc"],
        default="default",
        help="Media backend to use.",
    )

    args = parser.parse_args()
    main(backend=args.backend)
```
