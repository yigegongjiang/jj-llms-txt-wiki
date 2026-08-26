# Reachy Mini

## Main class[[reachy_mini.ReachyMini]]

#### reachy_mini.ReachyMini[[reachy_mini.ReachyMini]]

```python
reachy_mini.ReachyMini(robot_name: str = 'reachy_mini', host: str = 'reachy-mini.local', port: int = 8000, connection_mode: typing.Literal['auto', 'localhost_only', 'network'] = 'auto', spawn_daemon: bool = False, use_sim: bool = False, timeout: float = 5.0, automatic_body_yaw: bool = True, log_level: str = 'INFO', media_backend: str = 'default', localhost_only: typing.Optional[bool] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L83)

**Parameters:**

robot_name : Name of the robot, defaults to "reachy_mini". A non-default name is validated locally or resolved via mDNS.

host : Hostname or IP of the daemon. Defaults to "reachy-mini.local". In `"auto"` mode this is only used as a fallback when localhost is unreachable. For a non-default *robot_name* it is also tried as a last resort when mDNS discovery does not locate the robot.

port : Port of the daemon's FastAPI server. Defaults to 8000.

connection_mode : Select how to connect to the daemon. Use *"localhost_only"* to restrict connections to daemons running on localhost, *"network"* to connect to a remote daemon at *host:port*, or *"auto"* (default) to try localhost first then fall back to *host:port*.

spawn_daemon (bool) : If True, will spawn a daemon to control the robot, defaults to False.

use_sim (bool) : If True and spawn_daemon is True, will spawn a simulated robot, defaults to True.

Reachy Mini class for controlling a simulated or real Reachy Mini robot.

#### acquire_media[[reachy_mini.ReachyMini.acquire_media]]

```python
acquire_media()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L222)

Tell the daemon to re-acquire camera and audio hardware.

The SDK's media_manager is re-created with the original backend
auto-detection logic.

Idempotent: safe to call multiple times.

#### async_play_move[[reachy_mini.ReachyMini.async_play_move]]

```python
async_play_move(move: Move, play_frequency: float = 100.0, initial_goto_duration: float = 0.0, sound: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1115)

**Parameters:**

move (Move) : The Move object to be played.

play_frequency (float) : The frequency at which to evaluate the move (in Hz).

initial_goto_duration (float) : Duration for the initial goto to the starting position of the move (in seconds). If 0, no initial goto is performed.

sound (bool) : If True, play the sound associated with the move (if any).

Asynchronously play a Move.

#### cancel_move[[reachy_mini.ReachyMini.cancel_move]]

```python
cancel_move()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1105)

Cancel the currently playing move.

This will cause any running play_move or async_play_move to stop
at the next iteration of the playback loop. Audio is also stopped.

#### disable_gravity_compensation[[reachy_mini.ReachyMini.disable_gravity_compensation]]

```python
disable_gravity_compensation()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1092)

Disable gravity compensation for the head motors.

#### disable_motors[[reachy_mini.ReachyMini.disable_motors]]

```python
disable_motors(ids: typing.Optional[typing.List[str]] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1074)

**Parameters:**

ids (List[str] | None) : List of motor names to disable. If None, all motors will be disabled. Valid names match `src/reachy_mini/assets/config/hardware_config.yaml`: `body_rotation`, `stewart_1` … `stewart_6`, `right_antenna`, `left_antenna`.

Disable the motors.

#### disable_wobbling[[reachy_mini.ReachyMini.disable_wobbling]]

```python
disable_wobbling()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L269)

Disable audio-reactive head wobbling and reset offsets to zero.

#### enable_gravity_compensation[[reachy_mini.ReachyMini.enable_gravity_compensation]]

```python
enable_gravity_compensation()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1088)

Enable gravity compensation for the head motors.

#### enable_motors[[reachy_mini.ReachyMini.enable_motors]]

```python
enable_motors(ids: typing.Optional[typing.List[str]] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1063)

**Parameters:**

ids (List[str] | None) : List of motor names to enable. If None, all motors will be enabled. Valid names match `src/reachy_mini/assets/config/hardware_config.yaml`: `body_rotation`, `stewart_1` … `stewart_6`, `right_antenna`, `left_antenna`.

Enable the motors.

#### enable_wobbling[[reachy_mini.ReachyMini.enable_wobbling]]

```python
enable_wobbling()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L242)

Enable audio-reactive head wobbling.

When enabled, audio played through `media.play_sound()` or
`media.push_audio_sample()` is analysed and converted into
subtle head movements that are composed with the current target
pose on the daemon side.

For LOCAL backend: wobbling runs on the SDK side; offsets are sent
over WebSocket.  For all backends the daemon is also told to enable
wobbling so that daemon-side sounds (wake-up, sleep, etc.) and
incoming WebRTC audio also produce head movement.

#### get_current_head_pose[[reachy_mini.ReachyMini.get_current_head_pose]]

```python
get_current_head_pose()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L950)

**Returns:** `np.ndarray`

A 4x4 matrix representing the current head pose.

Get the current head pose as a 4x4 matrix.

Get the current head pose as a 4x4 matrix.

#### get_current_joint_positions[[reachy_mini.ReachyMini.get_current_joint_positions]]

```python
get_current_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L926)

**Returns:** `tuple`

A tuple containing two lists:
- List of head joint positions (rad) (length 7).
- List of antennas joint positions (rad) (length 2).

Get the current joint positions of the head and antennas.

Get the current joint positions of the head and antennas (in rad)

#### get_present_antenna_joint_positions[[reachy_mini.ReachyMini.get_present_antenna_joint_positions]]

```python
get_present_antenna_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L939)

**Returns:** `list`

A list of antennas joint positions (rad) (length 2).

Get the present joint positions of the antennas.

Get the present joint positions of the antennas (in rad)

#### get_tracked_face[[reachy_mini.ReachyMini.get_tracked_face]]

```python
get_tracked_face(wait: bool = True, timeout: float = 5.0)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L293)

Return the latest face observed by daemon-side head tracking.

#### goto_sleep[[reachy_mini.ReachyMini.goto_sleep]]

```python
goto_sleep()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L740)

Put the robot to sleep by moving the head and antennas to a predefined sleep position.

#### goto_target[[reachy_mini.ReachyMini.goto_target]]

```python
goto_target(head: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None, antennas: typing.Union[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]], typing.List[float], NoneType] = None, duration: float = 0.5, method: InterpolationTechnique = <InterpolationTechnique.MIN_JERK: 'minjerk'>, body_yaw: float | None = 0.0)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L672)

**Parameters:**

head (Optional[np.ndarray]) : 4x4 pose matrix representing the target head pose.

antennas (Optional[Union[np.ndarray, List[float]]]) : 1D array with two elements representing the angles of the antennas in radians.

duration (float) : Duration of the movement in seconds.

method (InterpolationTechnique) : Interpolation method to use ("linear", "minjerk", "ease_in_out", "cartoon"). Default is "minjerk".

body_yaw (float | None) : Body yaw angle in radians. Use None to keep the current yaw.

**Raises:** ``ValueError``

- ``ValueError`` -- If neither head nor antennas are provided, or if duration is not positive.

Go to a target head pose and/or antennas position using task space interpolation, in "duration" seconds.

#### look_at_image[[reachy_mini.ReachyMini.look_at_image]]

```python
look_at_image(u: int, v: int, duration: float = 1.0, perform_movement: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L772)

**Parameters:**

u (int) : Horizontal coordinate in image frame.

v (int) : Vertical coordinate in image frame.

duration (float) : Duration of the movement in seconds. If 0, the head will snap to the position immediately.

perform_movement (bool) : If True, perform the movement. If False, only calculate and return the pose.

**Returns:** `np.ndarray`

The calculated head pose as a 4x4 matrix.

**Raises:** ``ValueError``

- ``ValueError`` -- If duration is negative.

Make the robot head look at a point defined by a pixel position (u,v).

Pixels are counted from the image top-left corner: u to the right, v down.

#### look_at_world[[reachy_mini.ReachyMini.look_at_world]]

```python
look_at_world(x: float, y: float, z: float, duration: float = 1.0, perform_movement: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L830)

**Parameters:**

x (float) : X coordinate in meters.

y (float) : Y coordinate in meters.

z (float) : Z coordinate in meters.

duration (float) : Duration of the movement in seconds. If 0, the head will snap to the position immediately.

perform_movement (bool) : If True, perform the movement. If False, only calculate and return the pose.

**Returns:** `np.ndarray`

The calculated head pose as a 4x4 matrix.

**Raises:** ``ValueError``

- ``ValueError`` -- If duration is negative.

Look at a specific point in 3D space in Reachy Mini's reference frame.

The frame sits at the neutral head origin: x forward, y left, z up.

#### async_play_move[[reachy_mini.ReachyMini.play_move]]

```python
async_play_move(move: Move, play_frequency: float = 100.0, initial_goto_duration: float = 0.0, sound: bool = True)
```

**Parameters:**

move (Move) : The Move object to be played.

play_frequency (float) : The frequency at which to evaluate the move (in Hz).

initial_goto_duration (float) : Duration for the initial goto to the starting position of the move (in seconds). If 0, no initial goto is performed.

sound (bool) : If True, play the sound associated with the move (if any).

Asynchronously play a Move.

#### release_media[[reachy_mini.ReachyMini.release_media]]

```python
release_media()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L203)

Tell the daemon to release camera and audio hardware.

After calling this, the camera and microphone are available for direct
access via OpenCV / sounddevice / etc.  The SDK's media_manager is
switched to NO_MEDIA.

Idempotent: safe to call multiple times.

#### set_automatic_body_yaw[[reachy_mini.ReachyMini.set_automatic_body_yaw]]

```python
set_automatic_body_yaw(enabled: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1096)

**Parameters:**

enabled (bool) : Whether automatic body yaw is enabled.

Enable or disable automatic body yaw.

#### set_target[[reachy_mini.ReachyMini.set_target]]

```python
set_target(head: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None, antennas: typing.Union[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]], typing.List[float], NoneType] = None, body_yaw: typing.Optional[float] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L610)

**Parameters:**

head (Optional[np.ndarray]) : 4x4 pose matrix representing the head pose.

antennas (Optional[Union[np.ndarray, List[float]]]) : 1D array with two elements representing the angles of the antennas in radians.

body_yaw (Optional[float]) : Body yaw angle in radians.

**Raises:** `ValueError`

- `ValueError` -- If neither head nor antennas are provided, or if the shape of head is not (4, 4), or if antennas is not a 1D array with two elements.

Set the target pose of the head and/or the target position of the antennas.

Note:
*enable_motors()* pins all targets to the present pose before flipping
torque on, so the pattern `set_target(X); enable_motors()` no longer
drives the robot to `X`. Call `set_target` *after* `enable_motors`.

#### set_target_antenna_joint_positions[[reachy_mini.ReachyMini.set_target_antenna_joint_positions]]

```python
set_target_antenna_joint_positions(antennas: typing.List[float])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1015)

Set the target joint positions of the antennas.

#### set_target_body_yaw[[reachy_mini.ReachyMini.set_target_body_yaw]]

```python
set_target_body_yaw(body_yaw: float)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1019)

**Parameters:**

body_yaw (float) : The yaw angle of the body in radians.

Set the target body yaw.

#### set_target_head_pose[[reachy_mini.ReachyMini.set_target_head_pose]]

```python
set_target_head_pose(pose: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L995)

**Parameters:**

pose (np.ndarray) : A 4x4 matrix representing the desired head pose.

body_yaw (float) : The yaw angle of the body, used to adjust the head pose.

**Raises:** ``ValueError``

- ``ValueError`` -- If the shape of the pose is not (4, 4).

Set the head pose to a specific 4x4 matrix.

#### start_head_tracking[[reachy_mini.ReachyMini.start_head_tracking]]

```python
start_head_tracking(weight: float = 1.0)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L275)

**Parameters:**

weight : Blend factor in `[0, 1]`. `1` lets tracking fully own the head orientation; intermediate values let app motion show through while biasing toward the face; `0` pauses detection (freeing the head and CPU) without tearing the tracker down, for cheap on/off.

Enable daemon-side visual head tracking.

#### start_recording[[reachy_mini.ReachyMini.start_recording]]

```python
start_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1028)

Start recording data.

#### stop_head_tracking[[reachy_mini.ReachyMini.stop_head_tracking]]

```python
stop_head_tracking()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L288)

Disable daemon-side visual head tracking.

#### stop_recording[[reachy_mini.ReachyMini.stop_recording]]

```python
stop_recording()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L1033)

Stop recording data and return the recorded data.

#### wake_up[[reachy_mini.ReachyMini.wake_up]]

```python
wake_up()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/reachy_mini.py#L724)

Wake up the robot - go to the initial head position and play the wake up emote and sound.
