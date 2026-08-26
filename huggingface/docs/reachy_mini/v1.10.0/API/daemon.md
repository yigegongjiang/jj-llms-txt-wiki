# Daemon

## Core Daemon Classes[[reachy_mini.daemon.daemon.Daemon]]

#### reachy_mini.daemon.daemon.Daemon[[reachy_mini.daemon.daemon.Daemon]]

```python
reachy_mini.daemon.daemon.Daemon(log_level: str = 'INFO', robot_name: str = 'reachy_mini', wireless_version: bool = False, desktop_app_daemon: bool = False, no_media: bool = False, sim_mode: SimulationMode = <SimulationMode.NONE: 'none'>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L40)

Daemon for simulated or real Reachy Mini robot.

Runs the server with the appropriate backend (Mujoco for simulation or RobotBackend for real hardware).

#### acquire_media[[reachy_mini.daemon.daemon.Daemon.acquire_media]]

```python
acquire_media()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L155)

Re-acquire camera and audio hardware after a release.

Restarts the GstMediaServer pipeline and central signalling relay.
Idempotent: no-op if not currently released or no media server.

#### apply_robot_name[[reachy_mini.daemon.daemon.Daemon.apply_robot_name]]

```python
apply_robot_name(name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L295)

Apply a new robot name to the live daemon without a restart.

Refreshes the in-memory name and the daemon status, then nudges the
central relay so its next heartbeat advertises the new label. The
persistent store (`utils/robot_name`) is written by the caller
(the `set_robot_name` command handler); this only updates the
live/advertised copies. mDNS re-registration is wired separately in
the app lifespan, which owns the `MdnsServiceRegistration`.

Safe to call from the backend's command thread: it only mutates
attributes and calls the relay's thread-safe name setter.

#### release_media[[reachy_mini.daemon.daemon.Daemon.release_media]]

```python
release_media()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L139)

Release camera and audio hardware so clients can access them directly.

Stops the GstMediaServer pipeline and central signalling relay.
Idempotent: no-op if already released or no media server.

#### restart[[reachy_mini.daemon.daemon.Daemon.restart]]

```python
restart(sim: typing.Optional[bool] = None, mockup_sim: typing.Optional[bool] = None, serialport: typing.Optional[str] = None, scene: typing.Optional[str] = None, headless: typing.Optional[bool] = None, use_audio: typing.Optional[bool] = None, wake_up_on_start: typing.Optional[bool] = None, goto_sleep_on_stop: typing.Optional[bool] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L608)

**Parameters:**

sim (bool) : If True, run in simulation mode using Mujoco. Defaults to None (uses the previous value).

mockup_sim (bool) : If True, run in lightweight simulation mode (no MuJoCo). Defaults to None (uses the previous value).

serialport (str) : Serial port for real motors. Defaults to None (uses the previous value).

scene (str) : Name of the scene to load in simulation mode ("empty" or "minimal"). Defaults to None (uses the previous value).

headless (bool) : If True, run Mujoco in headless mode (no GUI). Defaults to None (uses the previous value).

use_audio (bool) : If True, enable audio. Defaults to None (uses the previous value).

wake_up_on_start (bool) : If True, wake up Reachy Mini on start. Defaults to None (don't wake up).

goto_sleep_on_stop (bool) : If True, put Reachy Mini to sleep on stop. Defaults to None (don't go to sleep).

**Returns:** `DaemonState`

The current state of the daemon after attempting to restart it.

Restart the Reachy Mini daemon.

#### start[[reachy_mini.daemon.daemon.Daemon.start]]

```python
start(sim: bool = False, mockup_sim: bool = False, serialport: str = 'auto', scene: str = 'empty', wake_up_on_start: bool = True, check_collision: bool = False, kinematics_engine: str = 'AnalyticalKinematics', headless: bool = False, use_audio: bool = True, hardware_config_filepath: str | None = None, on_wake_up_callback: collections.abc.Callable[[], None] | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L322)

**Parameters:**

sim (bool) : If True, run in simulation mode using Mujoco. Defaults to False.

mockup_sim (bool) : If True, run in lightweight simulation mode (no MuJoCo). Defaults to False.

serialport (str) : Serial port for real motors. Defaults to "auto", which will try to find the port automatically.

scene (str) : Name of the scene to load in simulation mode ("empty" or "minimal"). Defaults to "empty".

wake_up_on_start (bool) : If True, wake up Reachy Mini on start. Defaults to True.

check_collision (bool) : If True, enable collision checking. Defaults to False.

kinematics_engine (str) : Kinematics engine to use. Defaults to "AnalyticalKinematics".

headless (bool) : If True, run Mujoco in headless mode (no GUI). Defaults to False.

use_audio (bool) : If True, enable audio. Defaults to True.

hardware_config_filepath (str | None) : Path to the hardware configuration YAML file. Defaults to None.

on_wake_up_callback (Callable[[], None] | None) : Fired once each time the robot finishes waking up. Defaults to None.

**Returns:** `DaemonState`

The current state of the daemon after attempting to start it.

Start the Reachy Mini daemon.

#### status[[reachy_mini.daemon.daemon.Daemon.status]]

```python
status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L811)

Get the current status of the Reachy Mini daemon.

#### stop[[reachy_mini.daemon.daemon.Daemon.stop]]

```python
stop(goto_sleep_on_stop: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/daemon.py#L499)

**Parameters:**

goto_sleep_on_stop (bool) : If True, put Reachy Mini to sleep on stop. Defaults to True.

**Returns:** `DaemonState`

The current state of the daemon after attempting to stop it.

Stop the Reachy Mini daemon.

Note:
The relay releases its remote hold on `self.robot_app_lock` via
`relay.stop()`. A local-app hold is *not* force-released here
because the daemon is going down; the lock object dies with the
process. If restart-in-place is ever added, force-release the
`LOCAL_APP` state here before restart.

#### reachy_mini.io.protocol.DaemonState[[reachy_mini.io.protocol.DaemonState]]

```python
reachy_mini.io.protocol.DaemonState(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L51)

Enum representing the state of the Reachy Mini daemon.

#### reachy_mini.io.protocol.DaemonStatus[[reachy_mini.io.protocol.DaemonStatus]]

```python
reachy_mini.io.protocol.DaemonStatus(type: typing.Literal['daemon_status'] = 'daemon_status', robot_name: str, state: DaemonState, wireless_version: bool, desktop_app_daemon: bool, simulation_enabled: typing.Optional[bool], mockup_sim_enabled: typing.Optional[bool], no_media: bool = False, media_released: bool = False, camera_specs_name: str = '', backend_status: typing.Union[reachy_mini.io.protocol.RobotBackendStatus, reachy_mini.io.protocol.MujocoBackendStatus, reachy_mini.io.protocol.MockupSimBackendStatus, NoneType], error: typing.Optional[str] = None, wlan_ip: typing.Optional[str] = None, version: typing.Optional[str] = None, hardware_id: typing.Optional[str] = None, face_target: FaceTarget = <factory>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L147)

Status of the Reachy Mini daemon.

## Robot App Lock

The robot *app lock* is the daemon's single source of truth for which
managed app currently holds the robot. It serializes the two managed
entry points — local Python apps launched by :class:`AppManager` and
remote WebRTC clients routed through the central signaling relay.

It does **not** gate every code path that can drive the robot: SDK
clients talking to the daemon directly over LAN/WebSocket bypass it.
The name uses "app" deliberately to reflect that narrower scope.

### Concurrency model

Two managed entry points can open a session with the robot:

1. **Local path** — a Python app launched via
   ``POST /api/apps/start``. Runs as a subprocess of the daemon,
   talks to the backend directly.
2. **Remote path** — a browser client authenticated through the
   HuggingFace central signaling server, routed to the robot over
   WebRTC. Handled by ``CentralSignalingRelay`` in its own thread.

Without coordination, both paths can grab the robot at the same time
and fight over motor commands, camera, and audio. :class:`RobotAppLock`
prevents that with three mutually exclusive states:

- ``free`` — no managed app holds the slot.
- ``local_app(name)`` — a Python app is running.
- ``remote_session(name)`` — a remote WebRTC client is connected.

**Acquire rules:**

- The local path uses :meth:`RobotAppLock.acquire_local_evicting_remote`.
  If a remote session is active, the lock is transitioned atomically
  to ``local_app`` and the relay is asked to send ``endSession`` to
  the remote peer and to local GStreamer so the existing WebRTC
  connection tears down cleanly. If another Python app already holds
  the lock, the acquire raises ``RuntimeError``.
- The remote path uses :meth:`RobotAppLock.try_acquire_remote`. This
  fails fast (returns ``False``) whenever the lock is not ``free`` —
  incoming remote sessions are refused with
  ``{"type": "endSession", "reason": "robot_busy_local_app"}``.

**Release rules:**

- :meth:`RobotAppLock.release_local` is called from the subprocess
  monitor's ``finally`` block, so clean exits, crashes, ``SIGKILL``,
  OOM and task cancellation all release the lock.
- :meth:`RobotAppLock.release_remote` is called from every
  ``endSession`` handler (both directions), from
  ``_close_connections`` on disconnect/reconnect, and from the relay's
  ``stop()``. All release calls are idempotent — they no-op if the
  lock is not in the corresponding state.

**Cross-thread considerations:** the lock state is guarded by a
``threading.Lock``. The eviction callback is registered by the relay
and invoked by AppManager from the main asyncio loop, but dispatches
the actual session tear-down onto the relay's own event loop via
``asyncio.run_coroutine_threadsafe``. AppManager awaits the tear-down
before spawning the Python subprocess, so the remote peer has
released its media handles before the local app opens them.

### API reference[[reachy_mini.daemon.robot_app_lock.RobotAppLock]]

#### reachy_mini.daemon.robot_app_lock.RobotAppLock[[reachy_mini.daemon.robot_app_lock.RobotAppLock]]

```python
reachy_mini.daemon.robot_app_lock.RobotAppLock()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L64)

Thread-safe lock coordinating local app and remote session access to the robot.

#### acquire_local_evicting_remote[[reachy_mini.daemon.robot_app_lock.RobotAppLock.acquire_local_evicting_remote]]

```python
acquire_local_evicting_remote(app_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L203)

**Raises:** `RuntimeError`

- `RuntimeError` -- If another local app already holds the lock. Caller
  should not start a second Python app concurrently.

Acquire the lock for a local Python app, evicting any remote session.

If a remote session is held, it is transitioned to `local_app`
atomically and the registered eviction handler is invoked (after
releasing the mutex) so the relay can notify the remote peer.

#### acquire_local_keeping_remote[[reachy_mini.daemon.robot_app_lock.RobotAppLock.acquire_local_keeping_remote]]

```python
acquire_local_keeping_remote(app_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L246)

**Raises:** `RuntimeError``

- `RuntimeError` -- If another local app already holds the lock.

Acquire the local-app slot **without** evicting a remote session.

Used when the local app is started *by* the connected remote client
(e.g. the mobile app launching a conversation it will then drive): the
client is a controller, not a competitor for the robot, so its WebRTC
session must stay up. Transitions `remote_session`/``free` -> `local_app` and does **not** invoke the eviction handler.

#### release_local[[reachy_mini.daemon.robot_app_lock.RobotAppLock.release_local]]

```python
release_local(app_name: Optional[str] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L273)

**Parameters:**

app_name : Optional name of the app expected to hold the lock. If provided and the current holder differs, logs a warning but still releases — this protects against stale releases after a rapid stop/start cycle.

Release the lock held by a local app.

Idempotent: if the lock is free or held by a remote session, this
is a no-op (with a warning). Safe to call from `monitor_process`
regardless of how the subprocess exited.

#### release_remote[[reachy_mini.daemon.robot_app_lock.RobotAppLock.release_remote]]

```python
release_remote(expect_handoff: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L337)

**Parameters:**

expect_handoff : True when the client ended its session on purpose (a relayed `endSession`) rather than vanishing. The two read very differently downstream: a deliberate release usually means another session is on its way in - the mobile app dropping its own session so an app's iframe can take the slot - whereas a drop means nobody is coming back. Passed to the FREE handler so the daemon can wait longer before reclaiming the robot.

Release a remote-session hold. Idempotent.

#### set_on_became_free_handler[[reachy_mini.daemon.robot_app_lock.RobotAppLock.set_on_became_free_handler]]

```python
set_on_became_free_handler(handler: Optional[Callable[[bool], None]])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L119)

Register (or clear) the callback fired when the slot becomes FREE.

Fired once per FREE transition, from either `release_local` or
`release_remote`, *after* the internal mutex is released. The handler
receives `expect_handoff`: True when the previous owner released the
slot on purpose for someone else to pick it up (see
`release_remote`). It must return promptly and must not call back
into this lock. Pass `None` to clear.

#### set_on_remote_acquired_handler[[reachy_mini.daemon.robot_app_lock.RobotAppLock.set_on_remote_acquired_handler]]

```python
set_on_remote_acquired_handler(handler: Optional[Callable[[], None]])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L133)

Register (or clear) the callback fired when a remote session acquires the slot.

Fired from `try_acquire_remote` after a successful acquisition,
*after* the internal mutex is released. It must return promptly and
must not call back into this lock. Pass `None` to clear.

#### set_remote_eviction_handler[[reachy_mini.daemon.robot_app_lock.RobotAppLock.set_remote_eviction_handler]]

```python
set_remote_eviction_handler(handler: Optional[Callable[[], Awaitable[None]]])
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L108)

Register (or clear) the coroutine invoked when a local acquire evicts a remote session.

The handler must be safe to call from the caller of
`acquire_local_evicting_remote` — typically the main asyncio loop.
Pass `None` to clear.

#### status[[reachy_mini.daemon.robot_app_lock.RobotAppLock.status]]

```python
status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L170)

Return a snapshot of the current lock state.

#### try_acquire_local[[reachy_mini.daemon.robot_app_lock.RobotAppLock.try_acquire_local]]

```python
try_acquire_local(app_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L179)

**Returns:**

True if the lock was acquired. False if another local app or a
remote session already holds it. Unlike
`acquire_local_evicting_remote`, this never evicts a remote
session.

Acquire the lock for a local Python app only if the slot is free.

#### try_acquire_remote[[reachy_mini.daemon.robot_app_lock.RobotAppLock.try_acquire_remote]]

```python
try_acquire_remote(app_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L311)

**Returns:**

True if the lock was acquired (state transitioned to
`remote_session`). False if a local app or another remote
session is already holding it — caller must refuse the
incoming session.

Attempt to acquire the lock for a remote WebRTC session.

#### reachy_mini.daemon.robot_app_lock.RobotAppLockState[[reachy_mini.daemon.robot_app_lock.RobotAppLockState]]

```python
reachy_mini.daemon.robot_app_lock.RobotAppLockState(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L45)

Lock state enum. Values are stable strings suitable for serialization.

#### reachy_mini.daemon.robot_app_lock.RobotAppLockStatus[[reachy_mini.daemon.robot_app_lock.RobotAppLockStatus]]

```python
reachy_mini.daemon.robot_app_lock.RobotAppLockStatus(state: RobotAppLockState, holder_name: typing.Optional[str] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/robot_app_lock.py#L53)

Snapshot of the lock state, suitable for JSON serialization.

Returned by `RobotAppLock.status` and by the
`GET /api/daemon/robot-app-lock-status` endpoint.

## Backend Classes

### Abstract Backend[[reachy_mini.io.protocol.MotorControlMode]]

#### reachy_mini.io.protocol.MotorControlMode[[reachy_mini.io.protocol.MotorControlMode]]

```python
reachy_mini.io.protocol.MotorControlMode(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L43)

Enum for motor control modes.

### Robot Backend[[reachy_mini.daemon.backend.robot.RobotBackend]]

#### reachy_mini.daemon.backend.robot.RobotBackend[[reachy_mini.daemon.backend.robot.RobotBackend]]

```python
reachy_mini.daemon.backend.robot.RobotBackend(serialport: str, log_level: str = 'INFO', check_collision: bool = False, kinematics_engine: str = 'AnalyticalKinematics', hardware_error_check_frequency: float = 1.0, use_audio: bool = True, wireless_version: bool = False, hardware_config_filepath: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L32)

Real robot backend for Reachy Mini.

#### close[[reachy_mini.daemon.backend.robot.RobotBackend.close]]

```python
close()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L337)

Close the motor controller connection and release resources.

#### compensate_head_gravity[[reachy_mini.daemon.backend.robot.RobotBackend.compensate_head_gravity]]

```python
compensate_head_gravity()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L576)

Calculate the currents necessary to compensate for gravity.

#### disable_motors[[reachy_mini.daemon.backend.robot.RobotBackend.disable_motors]]

```python
disable_motors()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L378)

Disable the motors by turning the torque off.

#### enable_motors[[reachy_mini.daemon.backend.robot.RobotBackend.enable_motors]]

```python
enable_motors()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L352)

Enable motor torque; pin all targets to present pose first to avoid a snap.

#### get_all_joint_positions[[reachy_mini.daemon.backend.robot.RobotBackend.get_all_joint_positions]]

```python
get_all_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L480)

**Returns:** `tuple`

A tuple containing two lists - the first list is for the head joint positions,
and the second list is for the antenna joint positions.

Get the current joint positions of the robot.

#### get_imu_data[[reachy_mini.daemon.backend.robot.RobotBackend.get_imu_data]]

```python
get_imu_data()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L519)

**Returns:**

An ImuDataMsg, or None if the IMU is missing or the cache is
older than `IMU_CACHE_FRESH_S`.

Return the latest IMU reading cached by the control loop.

#### get_present_antenna_joint_positions[[reachy_mini.daemon.backend.robot.RobotBackend.get_present_antenna_joint_positions]]

```python
get_present_antenna_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L508)

**Returns:** `list`

A list of joint positions for the antennas.

Get the current joint positions of the antennas.

#### get_present_head_joint_positions[[reachy_mini.daemon.backend.robot.RobotBackend.get_present_head_joint_positions]]

```python
get_present_head_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L497)

**Returns:** `list`

A list of joint positions for the head, including the body rotation.

Get the current joint positions of the head.

#### get_status[[reachy_mini.daemon.backend.robot.RobotBackend.get_status]]

```python
get_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L344)

Get the current status of the robot backend.

#### read_hardware_errors[[reachy_mini.daemon.backend.robot.RobotBackend.read_hardware_errors]]

```python
read_hardware_errors()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L685)

Read hardware errors from the motor controller.

#### run[[reachy_mini.daemon.backend.robot.RobotBackend.run]]

```python
run()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L154)

Run the control loop for the robot backend.

This method continuously updates the motor controller at a specified frequency.
It reads the joint positions, updates the motor controller, and publishes the joint positions.
It also handles errors and retries if the motor controller is not responding.

#### set_antennas_operation_mode[[reachy_mini.daemon.backend.robot.RobotBackend.set_antennas_operation_mode]]

```python
set_antennas_operation_mode(mode: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L440)

**Parameters:**

mode (int) : The operation mode for the antennas motors (0: torque control, 3: position control, 5: current-based position control).

mode (int) : The operation mode for the antennas motors. This could be a specific mode like position control, velocity control, or torque control.

Change the operation mode of the antennas motors.

Important:
This method does not work well with the current feetech motors, as they do not support torque control.
So the method disables the antennas when in torque control mode.

#### set_head_operation_mode[[reachy_mini.daemon.backend.robot.RobotBackend.set_head_operation_mode]]

```python
set_head_operation_mode(mode: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L385)

**Parameters:**

mode (int) : The operation mode for the head motors.

mode (int) : The operation mode for the head motors. This could be a specific mode like position control, velocity control, or torque control.

Change the operation mode of the head motors.

The operation modes can be:
0: torque control
3: position control
5: current-based position control.

Important:
This method does not work well with the current feetech motors (body rotation), as they do not support torque control.
So the method disables the antennas when in torque control mode.
The dynamixel motors used for the head do support torque control, so this method works as expected.

#### set_motor_torque_ids[[reachy_mini.daemon.backend.robot.RobotBackend.set_motor_torque_ids]]

```python
set_motor_torque_ids(ids: list, on: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/robot/backend.py#L646)

**Parameters:**

ids (list[int]) : List of motor IDs to set the torque state for.

on (bool) : True to enable torque, False to disable.

Set the torque state for specific motor names.

#### reachy_mini.io.protocol.RobotBackendStatus[[reachy_mini.io.protocol.RobotBackendStatus]]

```python
reachy_mini.io.protocol.RobotBackendStatus(ready: bool, motor_control_mode: MotorControlMode, last_alive: float | None, control_loop_stats: dict, error: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L67)

Status of the Robot Backend.

### MuJoCo Backend[[reachy_mini.daemon.backend.mujoco.MujocoBackend]]

#### reachy_mini.daemon.backend.mujoco.MujocoBackend[[reachy_mini.daemon.backend.mujoco.MujocoBackend]]

```python
reachy_mini.daemon.backend.mujoco.MujocoBackend(scene: str = 'empty', check_collision: bool = False, kinematics_engine: str = 'AnalyticalKinematics', headless: bool = False, use_audio: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L42)

Simulated Reachy Mini using MuJoCo.

#### get_mj_present_head_pose[[reachy_mini.daemon.backend.mujoco.MujocoBackend.get_mj_present_head_pose]]

```python
get_mj_present_head_pose()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L294)

**Returns:** `np.ndarray`

The current head pose as a 4x4 transformation matrix.

Get the current head pose from the Mujoco simulation.

#### get_present_antenna_joint_positions[[reachy_mini.daemon.backend.mujoco.MujocoBackend.get_present_antenna_joint_positions]]

```python
get_present_antenna_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L328)

Get the current joint positions of the antennas.

#### get_present_head_joint_positions[[reachy_mini.daemon.backend.mujoco.MujocoBackend.get_present_head_joint_positions]]

```python
get_present_head_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L319)

Get the current joint positions of the head.

#### get_status[[reachy_mini.daemon.backend.mujoco.MujocoBackend.get_status]]

```python
get_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L310)

**Returns:** `dict`

An empty dictionary as the Mujoco backend does not have a specific status to report.

Get the status of the Mujoco backend.

#### rendering_loop[[reachy_mini.daemon.backend.mujoco.MujocoBackend.rendering_loop]]

```python
rendering_loop(camera_name: str, port: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L137)

Offline Rendering loop for the Mujoco simulation.

Capture the image from the virtual camera_name and send it over UDP to the port or over WebSocket to the ws_uri.

#### run[[reachy_mini.daemon.backend.mujoco.MujocoBackend.run]]

```python
run()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L162)

Run the Mujoco simulation with a viewer.

This method initializes the viewer and enters the main simulation loop.
It updates the joint positions at a rate and publishes the joint positions.

#### set_motor_torque_ids[[reachy_mini.daemon.backend.mujoco.MujocoBackend.set_motor_torque_ids]]

```python
set_motor_torque_ids(ids: list, on: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mujoco/backend.py#L345)

Set the motor torque state for specific motor names.

#### reachy_mini.io.protocol.MujocoBackendStatus[[reachy_mini.io.protocol.MujocoBackendStatus]]

```python
reachy_mini.io.protocol.MujocoBackendStatus(motor_control_mode: MotorControlMode, error: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L77)

Status of the Mujoco backend.

### Mockup Simulation Backend[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend]]

#### reachy_mini.daemon.backend.mockup_sim.MockupSimBackend[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend]]

```python
reachy_mini.daemon.backend.mockup_sim.MockupSimBackend(check_collision: bool = False, kinematics_engine: str = 'AnalyticalKinematics', use_audio: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L26)

Lightweight simulated Reachy Mini without MuJoCo.

This backend provides a simple simulation where target positions
are applied immediately without physics simulation.

Apps access webcam/microphone directly (not via UDP streaming).

#### get_present_antenna_joint_positions[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend.get_present_antenna_joint_positions]]

```python
get_present_antenna_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L149)

Get the current joint positions of the antennas.

#### get_present_head_joint_positions[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend.get_present_head_joint_positions]]

```python
get_present_head_joint_positions()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L143)

Get the current joint positions of the head.

#### get_status[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend.get_status]]

```python
get_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L139)

Get the status of the backend.

#### run[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend.run]]

```python
run()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L73)

Run the simulation loop.

In mockup-sim mode, target positions are applied immediately.

#### set_motor_torque_ids[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend.set_motor_torque_ids]]

```python
set_motor_torque_ids(ids: list, on: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/backend/mockup_sim/backend.py#L163)

Set the motor torque state for specific motor names.

No-op in mockup-sim mode.

#### reachy_mini.io.protocol.MockupSimBackendStatus[[reachy_mini.io.protocol.MockupSimBackendStatus]]

```python
reachy_mini.io.protocol.MockupSimBackendStatus(motor_control_mode: MotorControlMode, error: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L84)

Status of the MockupSim backend.

## Daemon Utilities[[reachy_mini.daemon.utils.find_serial_port]]

#### reachy_mini.daemon.utils.find_serial_port[[reachy_mini.daemon.utils.find_serial_port]]

```python
reachy_mini.daemon.utils.find_serial_port(wireless_version: bool = False, vid: str = '1a86', pid: str = '55d3', pi_uart: str = '/dev/ttyAMA3')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/utils.py#L145)

**Parameters:**

wireless_version (bool) : Whether to look for the wireless version using the Raspberry Pi UART.

vid (str) : Vendor ID of the device. (eg. "1a86").

pid (str) : Product ID of the device. (eg. "55d3").

pi_uart (str) : Path to the Raspberry Pi UART device. (eg. "/dev/ttyAMA3").

Find the serial port for Reachy Mini based on VID and PID or the Raspberry Pi UART for the wireless version.

#### reachy_mini.daemon.utils.get_ip_address[[reachy_mini.daemon.utils.get_ip_address]]

```python
reachy_mini.daemon.utils.get_ip_address(ifname: str = 'wlan0')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/utils.py#L173)

Get the IP address of a specific network interface (Linux and Windows).

## App

### Models[[reachy_mini.daemon.app.models.Matrix4x4Pose]]

#### reachy_mini.daemon.app.models.Matrix4x4Pose[[reachy_mini.daemon.app.models.Matrix4x4Pose]]

```python
reachy_mini.daemon.app.models.Matrix4x4Pose(m: tuple)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L14)

Represent a 3D pose by its 4x4 transformation matrix (translation is expressed in meters).

#### from_pose_array[[reachy_mini.daemon.app.models.Matrix4x4Pose.from_pose_array]]

```python
from_pose_array(arr: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L36)

Create a Matrix4x4 pose representation from a 4x4 pose array.

#### to_pose_array[[reachy_mini.daemon.app.models.Matrix4x4Pose.to_pose_array]]

```python
to_pose_array()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L63)

Convert the Matrix4x4Pose to a 4x4 numpy array.

#### reachy_mini.daemon.app.models.XYZRPYPose[[reachy_mini.daemon.app.models.XYZRPYPose]]

```python
reachy_mini.daemon.app.models.XYZRPYPose(x: float = 0.0, y: float = 0.0, z: float = 0.0, roll: float = 0.0, pitch: float = 0.0, yaw: float = 0.0)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L68)

Represent a 3D pose using position (x, y, z) in meters and orientation (roll, pitch, yaw) angles in radians.

#### from_pose_array[[reachy_mini.daemon.app.models.XYZRPYPose.from_pose_array]]

```python
from_pose_array(arr: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L78)

Create an XYZRPYPose representation from a 4x4 pose array.

#### to_pose_array[[reachy_mini.daemon.app.models.XYZRPYPose.to_pose_array]]

```python
to_pose_array()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L95)

Convert the XYZRPYPose to a 4x4 numpy array.

#### reachy_mini.daemon.app.models.FullBodyTarget[[reachy_mini.daemon.app.models.FullBodyTarget]]

```python
reachy_mini.daemon.app.models.FullBodyTarget(target_head_pose: reachy_mini.daemon.app.models.XYZRPYPose | reachy_mini.daemon.app.models.Matrix4x4Pose | None = None, target_antennas: tuple[float, float] | None = None, target_body_yaw: float | None = None, timestamp: datetime.datetime | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L116)

Represent the full body including the head pose and the joints for antennas.

#### reachy_mini.io.protocol.DoaSnapshot[[reachy_mini.io.protocol.DoaSnapshot]]

```python
reachy_mini.io.protocol.DoaSnapshot(angle: float, speech_detected: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/io/protocol.py#L101)

Sound Direction of Arrival reading (ReSpeaker mic array).

`angle` is in radians: 0 = left, π/2 = front, π = right.

#### reachy_mini.daemon.app.models.FullState[[reachy_mini.daemon.app.models.FullState]]

```python
reachy_mini.daemon.app.models.FullState(control_mode: reachy_mini.io.protocol.MotorControlMode | None = None, head_pose: reachy_mini.daemon.app.models.XYZRPYPose | reachy_mini.daemon.app.models.Matrix4x4Pose | None = None, head_joints: list[float] | None = None, body_yaw: float | None = None, antennas_position: list[float] | None = None, timestamp: datetime.datetime | None = None, passive_joints: list[float] | None = None, doa: reachy_mini.io.protocol.DoaSnapshot | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/models.py#L149)

Represent the full state of the robot including all joint positions and poses.

### Dependencies[[reachy_mini.daemon.app.dependencies.get_daemon]]

#### reachy_mini.daemon.app.dependencies.get_daemon[[reachy_mini.daemon.app.dependencies.get_daemon]]

```python
reachy_mini.daemon.app.dependencies.get_daemon(request: Request)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/dependencies.py#L10)

Get the daemon as request dependency.

#### reachy_mini.daemon.app.dependencies.get_backend[[reachy_mini.daemon.app.dependencies.get_backend]]

```python
reachy_mini.daemon.app.dependencies.get_backend(request: Request)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/dependencies.py#L16)

Get the backend as request dependency.

#### reachy_mini.daemon.app.dependencies.get_app_manager[[reachy_mini.daemon.app.dependencies.get_app_manager]]

```python
reachy_mini.daemon.app.dependencies.get_app_manager(request: Request)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/dependencies.py#L27)

Get the app manager as request dependency.

#### reachy_mini.daemon.app.dependencies.ws_get_backend[[reachy_mini.daemon.app.dependencies.ws_get_backend]]

```python
reachy_mini.daemon.app.dependencies.ws_get_backend(websocket: WebSocket)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/dependencies.py#L33)

Get the backend as websocket dependency.

### Jobs[[reachy_mini.daemon.app.bg_job_register.JobStatus]]

#### reachy_mini.daemon.app.bg_job_register.JobStatus[[reachy_mini.daemon.app.bg_job_register.JobStatus]]

```python
reachy_mini.daemon.app.bg_job_register.JobStatus(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L15)

Enum for job status.

#### reachy_mini.daemon.app.bg_job_register.JobInfo[[reachy_mini.daemon.app.bg_job_register.JobInfo]]

```python
reachy_mini.daemon.app.bg_job_register.JobInfo(command: str, status: JobStatus, logs: list)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L24)

Pydantic model for install job status.

#### reachy_mini.daemon.app.bg_job_register.JobHandler[[reachy_mini.daemon.app.bg_job_register.JobHandler]]

```python
reachy_mini.daemon.app.bg_job_register.JobHandler(uuid: str, info: JobInfo, new_log_evt: dict)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L33)

Handler for background jobs.

#### reachy_mini.daemon.app.bg_job_register.run_command[[reachy_mini.daemon.app.bg_job_register.run_command]]

```python
reachy_mini.daemon.app.bg_job_register.run_command(command: str, coro_func: typing.Callable[..., typing.Awaitable[NoneType]], *args: Any)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L44)

Start a background job, with a custom logger and return its job_id.

#### reachy_mini.daemon.app.bg_job_register.get_info[[reachy_mini.daemon.app.bg_job_register.get_info]]

```python
reachy_mini.daemon.app.bg_job_register.get_info(job_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L93)

Get the info of a job by its ID.

#### reachy_mini.daemon.app.bg_job_register.ws_poll_info[[reachy_mini.daemon.app.bg_job_register.ws_poll_info]]

```python
reachy_mini.daemon.app.bg_job_register.ws_poll_info(websocket: WebSocket, job_uuid: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/bg_job_register.py#L103)

WebSocket endpoint to stream job logs in real time.

### Main Application[[reachy_mini.daemon.app.main.Args]]

#### reachy_mini.daemon.app.main.Args[[reachy_mini.daemon.app.main.Args]]

```python
reachy_mini.daemon.app.main.Args(log_level: str = 'INFO', log_file: str | None = None, wireless_version: bool = False, desktop_app_daemon: bool = False, serialport: str = 'auto', hardware_config_filepath: str | None = None, sim: bool = False, mockup_sim: bool = False, scene: str = 'empty', headless: bool = False, no_media: bool = False, kinematics_engine: str = 'AnalyticalKinematics', check_collision: bool = False, autostart: bool = True, timeout_health_check: float | None = None, wake_up_on_start: bool = True, goto_sleep_on_stop: bool = True, preload_datasets: bool = False, dataset_update_interval_hours: float = 24.0, robot_name: str = 'reachy_mini', fastapi_host: str | None = None, fastapi_port: int = 8000)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/main.py#L74)

Arguments for configuring the Reachy Mini daemon.

#### reachy_mini.daemon.app.main.create_app[[reachy_mini.daemon.app.main.create_app]]

```python
reachy_mini.daemon.app.main.create_app(args: Args, health_check_event: asyncio.locks.Event | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/main.py#L124)

Create and configure the FastAPI application.

#### reachy_mini.daemon.app.main.run_app[[reachy_mini.daemon.app.main.run_app]]

```python
reachy_mini.daemon.app.main.run_app(args: Args)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/main.py#L521)

Run the FastAPI app with Uvicorn.

#### reachy_mini.daemon.app.main[[reachy_mini.daemon.app.main]]

```python
reachy_mini.daemon.app.main()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/main.py#L654)

Run the FastAPI app with Uvicorn.

## App Routers

### Daemon Router[[reachy_mini.daemon.app.routers.daemon.start_daemon]]

#### reachy_mini.daemon.app.routers.daemon.start_daemon[[reachy_mini.daemon.app.routers.daemon.start_daemon]]

```python
reachy_mini.daemon.app.routers.daemon.start_daemon(request: Request, wake_up: bool, daemon: Daemon = Depends(dependency=<function get_daemon at 0x7fef0c7ffc40>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/daemon.py#L24)

Start the daemon.

#### reachy_mini.daemon.app.routers.daemon.stop_daemon[[reachy_mini.daemon.app.routers.daemon.stop_daemon]]

```python
reachy_mini.daemon.app.routers.daemon.stop_daemon(goto_sleep: bool, daemon: Daemon = Depends(dependency=<function get_daemon at 0x7fef0c7ffc40>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/daemon.py#L52)

Stop the daemon, optionally putting the robot to sleep.

#### reachy_mini.daemon.app.routers.daemon.restart_daemon[[reachy_mini.daemon.app.routers.daemon.restart_daemon]]

```python
reachy_mini.daemon.app.routers.daemon.restart_daemon(request: Request, daemon: Daemon = Depends(dependency=<function get_daemon at 0x7fef0c7ffc40>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/daemon.py#L68)

Restart the daemon.

#### reachy_mini.daemon.app.routers.daemon.get_daemon_status[[reachy_mini.daemon.app.routers.daemon.get_daemon_status]]

```python
reachy_mini.daemon.app.routers.daemon.get_daemon_status(daemon: Daemon = Depends(dependency=<function get_daemon at 0x7fef0c7ffc40>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/daemon.py#L84)

Get the current status of the daemon.

#### reachy_mini.daemon.app.routers.daemon.get_robot_app_lock_status[[reachy_mini.daemon.app.routers.daemon.get_robot_app_lock_status]]

```python
reachy_mini.daemon.app.routers.daemon.get_robot_app_lock_status(daemon: Daemon = Depends(dependency=<function get_daemon at 0x7fef0c7ffc40>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/daemon.py#L167)

Return the current state of the robot's managed-app lock.

The daemon's single source of truth for which managed app (if any)
currently holds the robot:

- `free`: no managed app holds the slot.
- `local_app`: a Python app launched via AppManager is running.
  `holder_name` is the app name.
- `remote_session`: a remote WebRTC client is connected via the
  central signaling relay. `holder_name` is a generic `"remote"`
  placeholder (the real consumer app name lives on the central
  server and is surfaced via its own `/api/robot-status`).

Note that SDK clients talking to the daemon directly bypass this
lock; it only reflects the two *managed* app entry points.

Intended for UI layers (desktop app, dashboard) that want to render
a busy/free indicator without trying to open a session.

### State Router[[reachy_mini.daemon.app.routers.state.get_head_pose]]

#### reachy_mini.daemon.app.routers.state.get_head_pose[[reachy_mini.daemon.app.routers.state.get_head_pose]]

```python
reachy_mini.daemon.app.routers.state.get_head_pose(use_pose_matrix: bool = False, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L24)

**Parameters:**

use_pose_matrix (bool) : Whether to use the pose matrix representation (4x4 flattened) or the translation + Euler angles representation (x, y, z, roll, pitch, yaw).

backend (Backend) : The backend instance.

**Returns:** `AnyPose`

The present head pose.

Get the present head pose.

#### reachy_mini.daemon.app.routers.state.get_body_yaw[[reachy_mini.daemon.app.routers.state.get_body_yaw]]

```python
reachy_mini.daemon.app.routers.state.get_body_yaw(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L42)

Get the present body yaw (in radians).

#### reachy_mini.daemon.app.routers.state.get_antenna_joint_positions[[reachy_mini.daemon.app.routers.state.get_antenna_joint_positions]]

```python
reachy_mini.daemon.app.routers.state.get_antenna_joint_positions(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L50)

Get the present antenna joint positions (in radians) - (left, right).

#### reachy_mini.daemon.app.routers.state.get_doa[[reachy_mini.daemon.app.routers.state.get_doa]]

```python
reachy_mini.daemon.app.routers.state.get_doa(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L60)

Get the Direction of Arrival from the microphone array.

Returns the angle in radians (0=left, π/2=front, π=right) and speech detection status.
Returns None if the audio device is not available or no reading has
landed yet (the first one lands within ~100 ms of the first request).

#### reachy_mini.daemon.app.routers.state.get_full_state[[reachy_mini.daemon.app.routers.state.get_full_state]]

```python
reachy_mini.daemon.app.routers.state.get_full_state(with_control_mode: bool = True, with_head_pose: bool = True, with_target_head_pose: bool = False, with_head_joints: bool = False, with_target_head_joints: bool = False, with_body_yaw: bool = True, with_target_body_yaw: bool = False, with_antenna_positions: bool = True, with_target_antenna_positions: bool = False, with_passive_joints: bool = False, with_doa: bool = False, use_pose_matrix: bool = False, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L73)

Get the full robot state, with optional fields.

#### reachy_mini.daemon.app.routers.state.ws_full_state[[reachy_mini.daemon.app.routers.state.ws_full_state]]

```python
reachy_mini.daemon.app.routers.state.ws_full_state(websocket: WebSocket, frequency: float = 10.0, with_head_pose: bool = True, with_target_head_pose: bool = False, with_head_joints: bool = False, with_target_head_joints: bool = False, with_body_yaw: bool = True, with_target_body_yaw: bool = False, with_antenna_positions: bool = True, with_target_antenna_positions: bool = False, with_passive_joints: bool = False, with_doa: bool = False, use_pose_matrix: bool = False, backend: Backend = Depends(dependency=<function ws_get_backend at 0x7fef0c7ffec0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/state.py#L127)

WebSocket endpoint to stream the full state of the robot.

### Motors Router[[reachy_mini.daemon.app.routers.motors.get_motor_status]]

#### reachy_mini.daemon.app.routers.motors.get_motor_status[[reachy_mini.daemon.app.routers.motors.get_motor_status]]

```python
reachy_mini.daemon.app.routers.motors.get_motor_status(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/motors.py#L29)

Get the current status of the motors.

#### reachy_mini.daemon.app.routers.motors.set_motor_mode[[reachy_mini.daemon.app.routers.motors.set_motor_mode]]

```python
reachy_mini.daemon.app.routers.motors.set_motor_mode(mode: MotorControlMode, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/motors.py#L35)

Set the motor control mode.

### Move Router[[reachy_mini.daemon.app.routers.move.get_running_moves]]

#### reachy_mini.daemon.app.routers.move.get_running_moves[[reachy_mini.daemon.app.routers.move.get_running_moves]]

```python
reachy_mini.daemon.app.routers.move.get_running_moves()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L131)

Get a list of currently running move tasks.

#### reachy_mini.daemon.app.routers.move.goto[[reachy_mini.daemon.app.routers.move.goto]]

```python
reachy_mini.daemon.app.routers.move.goto(goto_req: GotoModelRequest, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L137)

Request a movement to a specific target.

#### reachy_mini.daemon.app.routers.move.play_wake_up[[reachy_mini.daemon.app.routers.move.play_wake_up]]

```python
reachy_mini.daemon.app.routers.move.play_wake_up(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L152)

Request the robot to wake up.

#### reachy_mini.daemon.app.routers.move.play_goto_sleep[[reachy_mini.daemon.app.routers.move.play_goto_sleep]]

```python
reachy_mini.daemon.app.routers.move.play_goto_sleep(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L158)

Request the robot to go to sleep.

#### reachy_mini.daemon.app.routers.move.list_recorded_move_dataset[[reachy_mini.daemon.app.routers.move.list_recorded_move_dataset]]

```python
reachy_mini.daemon.app.routers.move.list_recorded_move_dataset(dataset_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L164)

List available recorded moves in a dataset.

#### reachy_mini.daemon.app.routers.move.play_recorded_move_dataset[[reachy_mini.daemon.app.routers.move.play_recorded_move_dataset]]

```python
reachy_mini.daemon.app.routers.move.play_recorded_move_dataset(dataset_name: str, move_name: str, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L177)

Request the robot to play a predefined recorded move from a dataset.

#### reachy_mini.daemon.app.routers.move.stop_move[[reachy_mini.daemon.app.routers.move.stop_move]]

```python
reachy_mini.daemon.app.routers.move.stop_move(uuid: MoveUUID)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L195)

Stop a running move task.

#### reachy_mini.daemon.app.routers.move.set_target[[reachy_mini.daemon.app.routers.move.set_target]]

```python
reachy_mini.daemon.app.routers.move.set_target(target: FullBodyTarget, backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L216)

POST route to set a single FullBodyTarget.

#### reachy_mini.daemon.app.routers.move.ws_move_updates[[reachy_mini.daemon.app.routers.move.ws_move_updates]]

```python
reachy_mini.daemon.app.routers.move.ws_move_updates(websocket: WebSocket)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/move.py#L201)

WebSocket route to stream move updates.

### Apps Router[[reachy_mini.daemon.app.routers.apps.list_available_apps]]

#### reachy_mini.daemon.app.routers.apps.list_available_apps[[reachy_mini.daemon.app.routers.apps.list_available_apps]]

```python
reachy_mini.daemon.app.routers.apps.list_available_apps(source_kind: SourceKind, app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L49)

List available apps (including not installed).

#### reachy_mini.daemon.app.routers.apps.list_all_available_apps[[reachy_mini.daemon.app.routers.apps.list_all_available_apps]]

```python
reachy_mini.daemon.app.routers.apps.list_all_available_apps(app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L58)

List all available apps (including not installed).

#### reachy_mini.daemon.app.routers.apps.install_app[[reachy_mini.daemon.app.routers.apps.install_app]]

```python
reachy_mini.daemon.app.routers.apps.install_app(app_info: AppInfo, app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L66)

Install a new app by its info (background, returns job_id).

#### reachy_mini.daemon.app.routers.apps.remove_app[[reachy_mini.daemon.app.routers.apps.remove_app]]

```python
reachy_mini.daemon.app.routers.apps.remove_app(app_name: str, request: Request, app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L91)

Remove an installed app by its name (background, returns job_id).

#### reachy_mini.daemon.app.routers.apps.job_status[[reachy_mini.daemon.app.routers.apps.job_status]]

```python
reachy_mini.daemon.app.routers.apps.job_status(job_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L114)

Get status/logs for a job.

#### reachy_mini.daemon.app.routers.apps.ws_apps_manager[[reachy_mini.daemon.app.routers.apps.ws_apps_manager]]

```python
reachy_mini.daemon.app.routers.apps.ws_apps_manager(websocket: WebSocket, job_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L124)

WebSocket route to stream live job status/logs for a job, sending updates as soon as new logs are available.

#### reachy_mini.daemon.app.routers.apps.start_app[[reachy_mini.daemon.app.routers.apps.start_app]]

```python
reachy_mini.daemon.app.routers.apps.start_app(app_name: str, app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L132)

Start an app by its name.

#### reachy_mini.daemon.app.routers.apps.restart_app[[reachy_mini.daemon.app.routers.apps.restart_app]]

```python
reachy_mini.daemon.app.routers.apps.restart_app(app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L156)

Restart the currently running app.

#### reachy_mini.daemon.app.routers.apps.stop_app[[reachy_mini.daemon.app.routers.apps.stop_app]]

```python
reachy_mini.daemon.app.routers.apps.stop_app(app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L167)

Stop the currently running app.

#### reachy_mini.daemon.app.routers.apps.current_app_status[[reachy_mini.daemon.app.routers.apps.current_app_status]]

```python
reachy_mini.daemon.app.routers.apps.current_app_status(app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L178)

Get the status of the currently running app, if any.

#### reachy_mini.daemon.app.routers.apps.install_private_space[[reachy_mini.daemon.app.routers.apps.install_private_space]]

```python
reachy_mini.daemon.app.routers.apps.install_private_space(request: PrivateSpaceInstallRequest, app_manager: AppManager = Depends(dependency=<function get_app_manager at 0x7fef0c7ffce0>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/apps.py#L232)

Install a private HuggingFace space.

Requires HF token to be stored via /api/hf-auth/save-token first.

### Update Router[[reachy_mini.daemon.app.routers.update.available]]

#### reachy_mini.daemon.app.routers.update.available[[reachy_mini.daemon.app.routers.update.available]]

```python
reachy_mini.daemon.app.routers.update.available(pre_release: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/update.py#L32)

Check if an update is available for Reachy Mini Wireless.

#### reachy_mini.daemon.app.routers.update.start_update[[reachy_mini.daemon.app.routers.update.start_update]]

```python
reachy_mini.daemon.app.routers.update.start_update(pre_release: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/update.py#L58)

Start the update process for Reachy Mini Wireless version.

#### reachy_mini.daemon.app.routers.update.get_update_info[[reachy_mini.daemon.app.routers.update.get_update_info]]

```python
reachy_mini.daemon.app.routers.update.get_update_info(job_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/update.py#L117)

Get the info of an update job.

#### reachy_mini.daemon.app.routers.update.websocket_logs[[reachy_mini.daemon.app.routers.update.websocket_logs]]

```python
reachy_mini.daemon.app.routers.update.websocket_logs(websocket: WebSocket, job_id: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/update.py#L126)

WebSocket endpoint to stream update logs in real time.

### Cache Router[[reachy_mini.daemon.app.routers.cache.clear_huggingface_cache]]

#### reachy_mini.daemon.app.routers.cache.clear_huggingface_cache[[reachy_mini.daemon.app.routers.cache.clear_huggingface_cache]]

```python
reachy_mini.daemon.app.routers.cache.clear_huggingface_cache()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/cache.py#L13)

Clear HuggingFace cache directory.

#### reachy_mini.daemon.app.routers.cache.reset_apps[[reachy_mini.daemon.app.routers.cache.reset_apps]]

```python
reachy_mini.daemon.app.routers.cache.reset_apps()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/cache.py#L32)

Remove applications virtual environment directory.

### Kinematics Router[[reachy_mini.daemon.app.routers.kinematics.get_kinematics_info]]

#### reachy_mini.daemon.app.routers.kinematics.get_kinematics_info[[reachy_mini.daemon.app.routers.kinematics.get_kinematics_info]]

```python
reachy_mini.daemon.app.routers.kinematics.get_kinematics_info(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/kinematics.py#L29)

Get the current information of the kinematics.

#### reachy_mini.daemon.app.routers.kinematics.get_urdf[[reachy_mini.daemon.app.routers.kinematics.get_urdf]]

```python
reachy_mini.daemon.app.routers.kinematics.get_urdf(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/kinematics.py#L42)

Get the URDF representation of the robot.

#### reachy_mini.daemon.app.routers.kinematics.get_stl_file[[reachy_mini.daemon.app.routers.kinematics.get_stl_file]]

```python
reachy_mini.daemon.app.routers.kinematics.get_stl_file(filename: Path)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/kinematics.py#L48)

Get the path to an STL asset file.

### Volume Router[[reachy_mini.daemon.app.routers.volume.get_volume]]

#### reachy_mini.daemon.app.routers.volume.get_volume[[reachy_mini.daemon.app.routers.volume.get_volume]]

```python
reachy_mini.daemon.app.routers.volume.get_volume()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/volume.py#L80)

Get the current output volume level.

#### reachy_mini.daemon.app.routers.volume.set_volume[[reachy_mini.daemon.app.routers.volume.set_volume]]

```python
reachy_mini.daemon.app.routers.volume.set_volume(volume_req: VolumeRequest, request: Request)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/volume.py#L89)

Set the output volume level and play a test sound.

#### reachy_mini.daemon.app.routers.volume.play_test_sound[[reachy_mini.daemon.app.routers.volume.play_test_sound]]

```python
reachy_mini.daemon.app.routers.volume.play_test_sound(backend: Backend = Depends(dependency=<function get_backend at 0x7fef0c7ffe20>, use_cache=True, scope=None))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/volume.py#L115)

Play a test sound.

#### reachy_mini.daemon.app.routers.volume.get_microphone_volume[[reachy_mini.daemon.app.routers.volume.get_microphone_volume]]

```python
reachy_mini.daemon.app.routers.volume.get_microphone_volume()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/volume.py#L144)

Get the current microphone input volume level.

#### reachy_mini.daemon.app.routers.volume.set_microphone_volume[[reachy_mini.daemon.app.routers.volume.set_microphone_volume]]

```python
reachy_mini.daemon.app.routers.volume.set_microphone_volume(volume_req: VolumeRequest)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/volume.py#L153)

Set the microphone input volume level.

### Logs Router[[reachy_mini.daemon.app.routers.logs.websocket_daemon_logs]]

#### reachy_mini.daemon.app.routers.logs.websocket_daemon_logs[[reachy_mini.daemon.app.routers.logs.websocket_daemon_logs]]

```python
reachy_mini.daemon.app.routers.logs.websocket_daemon_logs(websocket: WebSocket)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/logs.py#L15)

WebSocket endpoint to stream journalctl logs for reachy-mini-daemon service in real time.

### HF Auth Router[[reachy_mini.daemon.app.routers.hf_auth.save_token]]

#### reachy_mini.daemon.app.routers.hf_auth.save_token[[reachy_mini.daemon.app.routers.hf_auth.save_token]]

```python
reachy_mini.daemon.app.routers.hf_auth.save_token(request: TokenRequest)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/hf_auth.py#L46)

Save HuggingFace token after validation.

#### reachy_mini.daemon.app.routers.hf_auth.get_auth_status[[reachy_mini.daemon.app.routers.hf_auth.get_auth_status]]

```python
reachy_mini.daemon.app.routers.hf_auth.get_auth_status()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/hf_auth.py#L62)

Check if user is authenticated with HuggingFace.

#### reachy_mini.daemon.app.routers.hf_auth.delete_token[[reachy_mini.daemon.app.routers.hf_auth.delete_token]]

```python
reachy_mini.daemon.app.routers.hf_auth.delete_token()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/daemon/app/routers/hf_auth.py#L92)

Delete stored HuggingFace token.
