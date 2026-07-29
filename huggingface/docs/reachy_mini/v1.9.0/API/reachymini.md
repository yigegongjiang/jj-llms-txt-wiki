# Reachy Mini

## Main class[[reachy_mini.ReachyMini]]

- **robot_name** -- Name of the robot, defaults to "reachy_mini".
- **host** -- Hostname or IP of the daemon. Defaults to "reachy-mini.local".
  In `"auto"` mode this is only used as a fallback when localhost
  is unreachable, so the default works out of the box for local
  development.
- **port** -- Port of the daemon's FastAPI server. Defaults to 8000.
- **connection_mode** -- Select how to connect to the daemon. Use
  *"localhost_only"* to restrict connections to daemons running on
  localhost, *"network"* to connect to a remote daemon at *host:port*,
  or *"auto"* (default) to try localhost first then fall back to
  *host:port*.
- **spawn_daemon** (bool) -- If True, will spawn a daemon to control the robot, defaults to False.
- **use_sim** (bool) -- If True and spawn_daemon is True, will spawn a simulated robot, defaults to True.
Reachy Mini class for controlling a simulated or real Reachy Mini robot.

Tell the daemon to re-acquire camera and audio hardware.

The SDK's media_manager is re-created with the original backend
auto-detection logic.

Idempotent: safe to call multiple times.

- **move** (Move) -- The Move object to be played.
- **play_frequency** (float) -- The frequency at which to evaluate the move (in Hz).
- **initial_goto_duration** (float) -- Duration for the initial goto to the starting position of the move (in seconds). If 0, no initial goto is performed.
- **sound** (bool) -- If True, play the sound associated with the move (if any).
Asynchronously play a Move.

Cancel the currently playing move.

This will cause any running play_move or async_play_move to stop
at the next iteration of the playback loop. Audio is also stopped.

Disable gravity compensation for the head motors.

- **ids** (List[str] | None) -- List of motor names to disable. If None, all motors will be disabled.
  Valid names match `src/reachy_mini/assets/config/hardware_config.yaml`:
  `body_rotation`, `stewart_1` … `stewart_6`, `right_antenna`, `left_antenna`.
Disable the motors.

Disable audio-reactive head wobbling and reset offsets to zero.

Enable gravity compensation for the head motors.

- **ids** (List[str] | None) -- List of motor names to enable. If None, all motors will be enabled.
  Valid names match `src/reachy_mini/assets/config/hardware_config.yaml`:
  `body_rotation`, `stewart_1` … `stewart_6`, `right_antenna`, `left_antenna`.
Enable the motors.

Enable audio-reactive head wobbling.

When enabled, audio played through `media.play_sound()` or
`media.push_audio_sample()` is analysed and converted into
subtle head movements that are composed with the current target
pose on the daemon side.

For LOCAL backend: wobbling runs on the SDK side; offsets are sent
over WebSocket.  For all backends the daemon is also told to enable
wobbling so that daemon-side sounds (wake-up, sleep, etc.) and
incoming WebRTC audio also produce head movement.

np.ndarrayA 4x4 matrix representing the current head pose.
Get the current head pose as a 4x4 matrix.

Get the current head pose as a 4x4 matrix.

tupleA tuple containing two lists:
- List of head joint positions (rad) (length 7).
- List of antennas joint positions (rad) (length 2).
Get the current joint positions of the head and antennas.

Get the current joint positions of the head and antennas (in rad)

listA list of antennas joint positions (rad) (length 2).
Get the present joint positions of the antennas.

Get the present joint positions of the antennas (in rad)

Return the latest face observed by daemon-side head tracking.

Put the robot to sleep by moving the head and antennas to a predefined sleep position.

"}, {"name": "body_yaw", "val": ": float | None = 0.0"}]}>
- **head** (Optional[np.ndarray]) -- 4x4 pose matrix representing the target head pose.
- **antennas** (Optional[Union[np.ndarray, List[float]]]) -- 1D array with two elements representing the angles of the antennas in radians.
- **duration** (float) -- Duration of the movement in seconds.
- **method** (InterpolationTechnique) -- Interpolation method to use ("linear", "minjerk", "ease_in_out", "cartoon"). Default is "minjerk".
- **body_yaw** (float | None) -- Body yaw angle in radians. Use None to keep the current yaw.- ``ValueError`` -- If neither head nor antennas are provided, or if duration is not positive.</raises><raisederrors>``ValueError``
Go to a target head pose and/or antennas position using task space interpolation, in "duration" seconds.

- **u** (int) -- Horizontal coordinate in image frame.
- **v** (int) -- Vertical coordinate in image frame.
- **duration** (float) -- Duration of the movement in seconds. If 0, the head will snap to the position immediately.
- **perform_movement** (bool) -- If True, perform the movement. If False, only calculate and return the pose.np.ndarrayThe calculated head pose as a 4x4 matrix.- ``ValueError`` -- If duration is negative.</raises><raisederrors>``ValueError``
Make the robot head look at a point defined by a pixel position (u,v).

Pixels are counted from the image top-left corner: u to the right, v down.

- **x** (float) -- X coordinate in meters.
- **y** (float) -- Y coordinate in meters.
- **z** (float) -- Z coordinate in meters.
- **duration** (float) -- Duration of the movement in seconds. If 0, the head will snap to the position immediately.
- **perform_movement** (bool) -- If True, perform the movement. If False, only calculate and return the pose.np.ndarrayThe calculated head pose as a 4x4 matrix.- ``ValueError`` -- If duration is negative.</raises><raisederrors>``ValueError``
Look at a specific point in 3D space in Reachy Mini's reference frame.

The frame sits at the neutral head origin: x forward, y left, z up.

- **move** (Move) -- The Move object to be played.
- **play_frequency** (float) -- The frequency at which to evaluate the move (in Hz).
- **initial_goto_duration** (float) -- Duration for the initial goto to the starting position of the move (in seconds). If 0, no initial goto is performed.
- **sound** (bool) -- If True, play the sound associated with the move (if any).
Asynchronously play a Move.

Tell the daemon to release camera and audio hardware.

After calling this, the camera and microphone are available for direct
access via OpenCV / sounddevice / etc.  The SDK's media_manager is
switched to NO_MEDIA.

Idempotent: safe to call multiple times.

- **enabled** (bool) -- Whether automatic body yaw is enabled.
Enable or disable automatic body yaw.

- **head** (Optional[np.ndarray]) -- 4x4 pose matrix representing the head pose.
- **antennas** (Optional[Union[np.ndarray, List[float]]]) -- 1D array with two elements representing the angles of the antennas in radians.
- **body_yaw** (Optional[float]) -- Body yaw angle in radians.- `ValueError` -- If neither head nor antennas are provided, or if the shape of head is not (4, 4), or if antennas is not a 1D array with two elements.`ValueError`
Set the target pose of the head and/or the target position of the antennas.

Note:
*enable_motors()* pins all targets to the present pose before flipping
torque on, so the pattern `set_target(X); enable_motors()` no longer
drives the robot to `X`. Call `set_target` *after* `enable_motors`.

Set the target joint positions of the antennas.

- **body_yaw** (float) -- The yaw angle of the body in radians.
Set the target body yaw.

- **pose** (np.ndarray) -- A 4x4 matrix representing the desired head pose.
- **body_yaw** (float) -- The yaw angle of the body, used to adjust the head pose.- ``ValueError`` -- If the shape of the pose is not (4, 4).</raises><raisederrors>``ValueError``
Set the head pose to a specific 4x4 matrix.

- **weight** -- Blend factor in `[0, 1]`. `1` lets tracking fully own the
  head orientation; intermediate values let app motion show through
  while biasing toward the face; `0` pauses detection (freeing the
  head and CPU) without tearing the tracker down, for cheap on/off.
Enable daemon-side visual head tracking.

Start recording data.

Disable daemon-side visual head tracking.

Stop recording data and return the recorded data.

Wake up the robot - go to the initial head position and play the wake up emote and sound.
