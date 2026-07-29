# Take Picture

This example demonstrates how to capture a single frame from Reachy Mini's camera and save it as an image file.

Run with:
```bash
python take_picture.py --backend [default|local|webrtc]
```

The captured image will be saved as `reachy_mini_picture.jpg` in the current directory.

```python

import argparse
import sys
import time

try:
    import cv2
except ImportError:
    print("Error: OpenCV is required for this example but not installed.")
    print("Install it with: pip install reachy_mini[opencv]")
    sys.exit(1)

from reachy_mini import ReachyMini

def main(backend: str) -> None:
    """Get a frame and take a picture."""
    with ReachyMini(media_backend=backend) as reachy_mini:
        frame = reachy_mini.media.get_frame()
        start_time = time.time()
        while frame is None:
            if time.time() - start_time > 20:
                print("Timeout: Failed to grab frame within 20 seconds.")
                sys.exit(1)
            print("Failed to grab frame. Retrying...")
            frame = reachy_mini.media.get_frame()
            time.sleep(1)

        cv2.imwrite("reachy_mini_picture.jpg", frame)
        print("Saved frame as reachy_mini_picture.jpg")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Take a picture using Reachy Mini camera."
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
