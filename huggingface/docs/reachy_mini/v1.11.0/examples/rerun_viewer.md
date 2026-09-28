# Rerun Viewer

This example shows how to use the Rerun utility to log and visualize Reachy Mini's state in real-time. The robot will be in compliant mode with gravity compensation, making it easy to move around while visualizing its configuration.

Requirements:
- Install with: `pip install reachy-mini[rerun,placo_kinematics]`
- Start the daemon with: `reachy-mini-daemon --kinematics-engine Placo`

```python

import logging
import time

from reachy_mini import ReachyMini
from reachy_mini.utils.rerun import Rerun

def main() -> None:
    """Log and visualize Reachy Mini's state using Rerun."""
    logging.basicConfig(
        level=logging.DEBUG, format="%(asctime)s [%(levelname)s] %(message)s"
    )

    with ReachyMini(log_level="DEBUG") as mini:
        try:
            mini.enable_gravity_compensation()
            rerun = Rerun(mini)
            rerun.start()

            print("Reachy Mini is now compliant. Press Ctrl+C to exit.")
            while True:
                # do nothing, just keep the program running
                time.sleep(0.02)

        except KeyboardInterrupt:
            pass
        finally:
            mini.disable_gravity_compensation()
            rerun.stop()
            print("Exiting... Reachy Mini is stiff again.")

if __name__ == "__main__":
    main()
```
