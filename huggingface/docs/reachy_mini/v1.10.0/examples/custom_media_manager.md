# Custom Media Manager

This example demonstrates how to deactivate the built-in media manager and access the camera and microphone directly using OpenCV and sounddevice.

**Why?** The daemon normally owns the camera and audio hardware. If you need raw access (e.g. custom OpenCV pipelines, sounddevice recording, or a third-party vision library), you must first tell the daemon to release the hardware. See [Media Architecture - Disabling Media](../SDK/media-architecture#disabling-media--direct-hardware-access) for details.

**How it works:**
1. Connects with `media_backend="no_media"` — this automatically tells the daemon to release camera and audio hardware
2. Uses OpenCV to capture a frame directly from the camera
3. Uses sounddevice to record audio from the microphone
4. On exit, the daemon automatically re-acquires the hardware

> **💡 Tip:** Robot control (head, antennas, body) keeps working normally while media is released. Only camera and audio are affected.

**Requirements:**
```bash
pip install opencv-python sounddevice soundfile
```

**Usage:**
```bash
python custom_media_manager.py
```

```python

import sys
import time

try:
    import cv2
except ImportError:
    print("Error: OpenCV is required. Install with: pip install opencv-python")
    sys.exit(1)

try:
    import sounddevice as sd
    import soundfile as sf
except ImportError:
    print("Error: sounddevice + soundfile are required.")
    print("Install with: pip install sounddevice soundfile")
    sys.exit(1)

import numpy as np

from reachy_mini import ReachyMini

def main() -> None:
    """Capture a frame with OpenCV and record audio with sounddevice."""
    # media_backend="no_media" automatically releases daemon media hardware
    with ReachyMini(media_backend="no_media") as mini:
        print(f"Connected. media_released={mini.media_released}")

        # --- OpenCV camera capture ---
        print("\nOpening camera with OpenCV...")
        cap = cv2.VideoCapture(0)
        if not cap.isOpened():
            print("Could not open camera — is it plugged in?")
        else:
            # Let the camera auto-adjust
            for _ in range(10):
                cap.read()

            ret, frame = cap.read()
            if ret:
                cv2.imwrite("release_media_frame.jpg", frame)
                print(f"Saved frame: release_media_frame.jpg ({frame.shape})")
            else:
                print("Failed to capture frame.")
            cap.release()

        # --- sounddevice audio recording ---
        duration = 2.0  # seconds
        samplerate = 44100
        print(f"\nRecording {duration}s of audio at {samplerate} Hz...")
        try:
            audio = sd.rec(
                int(duration * samplerate),
                samplerate=samplerate,
                channels=1,
                dtype=np.int16,
            )
            sd.wait()
            sf.write("release_media_audio.wav", audio, samplerate)
            print("Saved audio: release_media_audio.wav")
        except Exception as e:
            print(f"Audio recording failed: {e}")

        # The robot is still controllable while media is released
        print("\nWiggling antennas to prove robot control still works...")
        mini.goto_target(antennas=[0.3, -0.3], duration=0.5)
        time.sleep(0.2)
        mini.goto_target(antennas=[0.0, 0.0], duration=0.5)

    # __exit__ automatically calls acquire_media() → daemon reclaims hardware
    print("\nDone. Daemon media re-acquired on exit.")

if __name__ == "__main__":
    main()
```
