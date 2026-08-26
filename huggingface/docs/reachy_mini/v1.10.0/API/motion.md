# Motion

## Base Classes[[reachy_mini.motion.move.Move]]

#### reachy_mini.motion.move.Move[[reachy_mini.motion.move.Move]]

```python
reachy_mini.motion.move.Move()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/move.py#L11)

Abstract base class for defining a move on the ReachyMini robot.

#### evaluate[[reachy_mini.motion.move.Move.evaluate]]

```python
evaluate(t: float)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/move.py#L25)

**Parameters:**

t : The time at which to evaluate the move (in seconds). It will always be between 0 and duration.

**Returns:** `head`

The head position (4x4 homogeneous matrix).
antennas: The antennas positions (rad).
body_yaw: The body yaw angle (rad).

Evaluate the move at time t, typically called at a high-frequency (eg. 100Hz).

## Goto Moves[[reachy_mini.motion.goto.GotoMove]]

#### reachy_mini.motion.goto.GotoMove[[reachy_mini.motion.goto.GotoMove]]

```python
reachy_mini.motion.goto.GotoMove(start_head_pose: ndarray, target_head_pose: numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]] | None, start_antennas: ndarray, target_antennas: numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]] | None, start_body_yaw: float, target_body_yaw: float | None, duration: float, method: InterpolationTechnique)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/goto.py#L15)

A goto move to a target head pose and/or antennas position.

#### evaluate[[reachy_mini.motion.goto.GotoMove.evaluate]]

```python
evaluate(t: float)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/goto.py#L51)

Evaluate the goto at time t.

## Recorded Moves[[reachy_mini.motion.recorded_move.RecordedMove]]

#### reachy_mini.motion.recorded_move.RecordedMove[[reachy_mini.motion.recorded_move.RecordedMove]]

```python
reachy_mini.motion.recorded_move.RecordedMove(move: typing.Dict[str, typing.Any], sound_path: typing.Optional[pathlib.Path] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L82)

Represent a recorded move.

#### evaluate[[reachy_mini.motion.recorded_move.RecordedMove.evaluate]]

```python
evaluate(t: float)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L110)

**Returns:** `head`

The head position (4x4 homogeneous matrix).
antennas: The antennas positions (rad).
body_yaw: The body yaw angle (rad).

Evaluate the move at time t.

#### reachy_mini.motion.recorded_move.RecordedMoves[[reachy_mini.motion.recorded_move.RecordedMoves]]

```python
reachy_mini.motion.recorded_move.RecordedMoves(hf_dataset_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L168)

Load a library of recorded moves from a HuggingFace dataset.

Uses local cache only to avoid blocking network calls during playback.
The dataset should be pre-downloaded at daemon startup via preload_default_datasets().
If not cached, falls back to network download (which may cause delays).

#### get[[reachy_mini.motion.recorded_move.RecordedMoves.get]]

```python
get(move_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L224)

Get a recorded move by name.

#### list_moves[[reachy_mini.motion.recorded_move.RecordedMoves.list_moves]]

```python
list_moves()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L233)

List all moves in the loaded library.

#### process[[reachy_mini.motion.recorded_move.RecordedMoves.process]]

```python
process()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/motion/recorded_move.py#L201)

Populate recorded moves and sounds.
