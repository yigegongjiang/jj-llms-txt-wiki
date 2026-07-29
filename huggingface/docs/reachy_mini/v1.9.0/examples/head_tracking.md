# Head Tracking

This example enables daemon-side head tracking: Reachy Mini turns its head to follow the closest face, aiming at the nose. Detection runs inside the daemon (YuNet on ONNX Runtime), so the script only toggles tracking and polls the latest face target.

Run with:
```bash
python head_tracking.py
```

`start_head_tracking(weight=...)` accepts a blend factor: `1.0` lets tracking own the head, `0.0` pauses detection (freeing the head and CPU) without stopping the tracker — useful to hand the head back to an application between conversation turns.

```python

from reachy_mini import ReachyMini

with ReachyMini() as mini:
    mini.start_head_tracking()
    try:
        while True:
            # Waits for the next daemon status broadcast, so the loop runs at ~1 Hz.
            face = mini.get_tracked_face()
            if face.detected:
                print(f"Face at x={face.x:+.2f}, y={face.y:+.2f}")
            else:
                print("No face detected")
    except KeyboardInterrupt:
        pass
    finally:
        mini.stop_head_tracking()
```
