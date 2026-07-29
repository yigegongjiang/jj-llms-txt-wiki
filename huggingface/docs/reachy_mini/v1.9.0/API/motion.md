# Motion

## Base Classes[[reachy_mini.motion.move.Move]]

Abstract base class for defining a move on the ReachyMini robot.

- **t** -- The time at which to evaluate the move (in seconds). It will always be between 0 and duration.headThe head position (4x4 homogeneous matrix).
antennas: The antennas positions (rad).
body_yaw: The body yaw angle (rad).
Evaluate the move at time t, typically called at a high-frequency (eg. 100Hz).

## Goto Moves[[reachy_mini.motion.goto.GotoMove]]

A goto move to a target head pose and/or antennas position.

Evaluate the goto at time t.

## Recorded Moves[[reachy_mini.motion.recorded_move.RecordedMove]]

Represent a recorded move.

headThe head position (4x4 homogeneous matrix).
antennas: The antennas positions (rad).
body_yaw: The body yaw angle (rad).
Evaluate the move at time t.

Load a library of recorded moves from a HuggingFace dataset.

Uses local cache only to avoid blocking network calls during playback.
The dataset should be pre-downloaded at daemon startup via preload_default_datasets().
If not cached, falls back to network download (which may cause delays).

Get a recorded move by name.

List all moves in the loaded library.

Populate recorded moves and sounds.
