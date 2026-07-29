# Daemon

## Core Daemon Classes[[reachy_mini.daemon.daemon.Daemon]]

"}]}>

Daemon for simulated or real Reachy Mini robot.

Runs the server with the appropriate backend (Mujoco for simulation or RobotBackend for real hardware).

Re-acquire camera and audio hardware after a release.

Restarts the GstMediaServer pipeline and central signalling relay.
Idempotent: no-op if not currently released or no media server.

Release camera and audio hardware so clients can access them directly.

Stops the GstMediaServer pipeline and central signalling relay.
Idempotent: no-op if already released or no media server.

- **sim** (bool) -- If True, run in simulation mode using Mujoco. Defaults to None (uses the previous value).
- **mockup_sim** (bool) -- If True, run in lightweight simulation mode (no MuJoCo). Defaults to None (uses the previous value).
- **serialport** (str) -- Serial port for real motors. Defaults to None (uses the previous value).
- **scene** (str) -- Name of the scene to load in simulation mode ("empty" or "minimal"). Defaults to None (uses the previous value).
- **headless** (bool) -- If True, run Mujoco in headless mode (no GUI). Defaults to None (uses the previous value).
- **use_audio** (bool) -- If True, enable audio. Defaults to None (uses the previous value).
- **wake_up_on_start** (bool) -- If True, wake up Reachy Mini on start. Defaults to None (don't wake up).
- **goto_sleep_on_stop** (bool) -- If True, put Reachy Mini to sleep on stop. Defaults to None (don't go to sleep).DaemonStateThe current state of the daemon after attempting to restart it.
Restart the Reachy Mini daemon.

- **sim** (bool) -- If True, run in simulation mode using Mujoco. Defaults to False.
- **mockup_sim** (bool) -- If True, run in lightweight simulation mode (no MuJoCo). Defaults to False.
- **serialport** (str) -- Serial port for real motors. Defaults to "auto", which will try to find the port automatically.
- **scene** (str) -- Name of the scene to load in simulation mode ("empty" or "minimal"). Defaults to "empty".
- **wake_up_on_start** (bool) -- If True, wake up Reachy Mini on start. Defaults to True.
- **check_collision** (bool) -- If True, enable collision checking. Defaults to False.
- **kinematics_engine** (str) -- Kinematics engine to use. Defaults to "AnalyticalKinematics".
- **headless** (bool) -- If True, run Mujoco in headless mode (no GUI). Defaults to False.
- **use_audio** (bool) -- If True, enable audio. Defaults to True.
- **hardware_config_filepath** (str | None) -- Path to the hardware configuration YAML file. Defaults to None.
- **on_wake_up_callback** (Callable[[], None] | None) -- Fired once each time the robot finishes waking up. Defaults to None.DaemonStateThe current state of the daemon after attempting to start it.
Start the Reachy Mini daemon.

Get the current status of the Reachy Mini daemon.

- **goto_sleep_on_stop** (bool) -- If True, put Reachy Mini to sleep on stop. Defaults to True.DaemonStateThe current state of the daemon after attempting to stop it.
Stop the Reachy Mini daemon.

Note:
The relay releases its remote hold on `self.robot_app_lock` via
`relay.stop()`. A local-app hold is *not* force-released here
because the daemon is going down; the lock object dies with the
process. If restart-in-place is ever added, force-release the
`LOCAL_APP` state here before restart.

Enum representing the state of the Reachy Mini daemon.

"}]}>

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

Thread-safe lock coordinating local app and remote session access to the robot.

- `RuntimeError` -- If another local app already holds the lock. Caller
  should not start a second Python app concurrently.`RuntimeError`
Acquire the lock for a local Python app, evicting any remote session.

If a remote session is held, it is transitioned to `local_app`
atomically and the registered eviction handler is invoked (after
releasing the mutex) so the relay can notify the remote peer.

- **app_name** -- Optional name of the app expected to hold the lock.
  If provided and the current holder differs, logs a warning
  but still releases — this protects against stale releases
  after a rapid stop/start cycle.
Release the lock held by a local app.

Idempotent: if the lock is free or held by a remote session, this
is a no-op (with a warning). Safe to call from `monitor_process`
regardless of how the subprocess exited.

Release a remote-session hold. Idempotent.

Register (or clear) the coroutine invoked when a local acquire evicts a remote session.

The handler must be safe to call from the caller of
`acquire_local_evicting_remote` — typically the main asyncio loop.
Pass `None` to clear.

Return a snapshot of the current lock state.

True if the lock was acquired. False if another local app or a
remote session already holds it. Unlike
`acquire_local_evicting_remote`, this never evicts a remote
session.
Acquire the lock for a local Python app only if the slot is free.

True if the lock was acquired (state transitioned to
`remote_session`). False if a local app or another remote
session is already holding it — caller must refuse the
incoming session.
Attempt to acquire the lock for a remote WebRTC session.

Lock state enum. Values are stable strings suitable for serialization.

Snapshot of the lock state, suitable for JSON serialization.

Returned by `RobotAppLock.status` and by the
`GET /api/daemon/robot-app-lock-status` endpoint.

## Backend Classes

### Abstract Backend[[reachy_mini.io.protocol.MotorControlMode]]

Enum for motor control modes.

### Robot Backend[[reachy_mini.daemon.backend.robot.RobotBackend]]

Real robot backend for Reachy Mini.

Close the motor controller connection and release resources.

Calculate the currents necessary to compensate for gravity.

Disable the motors by turning the torque off.

Enable motor torque; pin all targets to present pose first to avoid a snap.

tupleA tuple containing two lists - the first list is for the head joint positions,
and the second list is for the antenna joint positions.
Get the current joint positions of the robot.

An ImuDataMsg, or None if IMU is not available.
Get current IMU data (accelerometer, gyroscope, quaternion, temperature).

listA list of joint positions for the antennas.
Get the current joint positions of the antennas.

listA list of joint positions for the head, including the body rotation.
Get the current joint positions of the head.

Get the current status of the robot backend.

Read hardware errors from the motor controller.

Run the control loop for the robot backend.

This method continuously updates the motor controller at a specified frequency.
It reads the joint positions, updates the motor controller, and publishes the joint positions.
It also handles errors and retries if the motor controller is not responding.

- **mode** (int) -- The operation mode for the antennas motors (0: torque control, 3: position control, 5: current-based position control).
- **mode** (int) -- The operation mode for the antennas motors.
  This could be a specific mode like position control, velocity control, or torque control.
Change the operation mode of the antennas motors.

Important:
This method does not work well with the current feetech motors, as they do not support torque control.
So the method disables the antennas when in torque control mode.

- **mode** (int) -- The operation mode for the head motors.
- **mode** (int) -- The operation mode for the head motors.
  This could be a specific mode like position control, velocity control, or torque control.
Change the operation mode of the head motors.

The operation modes can be:
0: torque control
3: position control
5: current-based position control.

Important:
This method does not work well with the current feetech motors (body rotation), as they do not support torque control.
So the method disables the antennas when in torque control mode.
The dynamixel motors used for the head do support torque control, so this method works as expected.

- **ids** (list[int]) -- List of motor IDs to set the torque state for.
- **on** (bool) -- True to enable torque, False to disable.
Set the torque state for specific motor names.

Status of the Robot Backend.

### MuJoCo Backend[[reachy_mini.daemon.backend.mujoco.MujocoBackend]]

Simulated Reachy Mini using MuJoCo.

np.ndarrayThe current head pose as a 4x4 transformation matrix.
Get the current head pose from the Mujoco simulation.

Get the current joint positions of the antennas.

Get the current joint positions of the head.

dictAn empty dictionary as the Mujoco backend does not have a specific status to report.
Get the status of the Mujoco backend.

Offline Rendering loop for the Mujoco simulation.

Capture the image from the virtual camera_name and send it over UDP to the port or over WebSocket to the ws_uri.

Run the Mujoco simulation with a viewer.

This method initializes the viewer and enters the main simulation loop.
It updates the joint positions at a rate and publishes the joint positions.

Set the motor torque state for specific motor names.

Status of the Mujoco backend.

### Mockup Simulation Backend[[reachy_mini.daemon.backend.mockup_sim.MockupSimBackend]]

Lightweight simulated Reachy Mini without MuJoCo.

This backend provides a simple simulation where target positions
are applied immediately without physics simulation.

Apps access webcam/microphone directly (not via UDP streaming).

Get the current joint positions of the antennas.

Get the current joint positions of the head.

Get the status of the backend.

Run the simulation loop.

In mockup-sim mode, target positions are applied immediately.

Set the motor torque state for specific motor names.

No-op in mockup-sim mode.

Status of the MockupSim backend.

## Daemon Utilities[[reachy_mini.daemon.utils.find_serial_port]]

- **wireless_version** (bool) -- Whether to look for the wireless version using the Raspberry Pi UART.
- **vid** (str) -- Vendor ID of the device. (eg. "1a86").
- **pid** (str) -- Product ID of the device. (eg. "55d3").
- **pi_uart** (str) -- Path to the Raspberry Pi UART device. (eg. "/dev/ttyAMA3").
Find the serial port for Reachy Mini based on VID and PID or the Raspberry Pi UART for the wireless version.

Get the IP address of a specific network interface (Linux and Windows).

## App

### Models[[reachy_mini.daemon.app.models.Matrix4x4Pose]]

Represent a 3D pose by its 4x4 transformation matrix (translation is expressed in meters).

Create a Matrix4x4 pose representation from a 4x4 pose array.

Convert the Matrix4x4Pose to a 4x4 numpy array.

Represent a 3D pose using position (x, y, z) in meters and orientation (roll, pitch, yaw) angles in radians.

Create an XYZRPYPose representation from a 4x4 pose array.

Convert the XYZRPYPose to a 4x4 numpy array.

Represent the full body including the head pose and the joints for antennas.

Direction of Arrival info from the microphone array.

Represent the full state of the robot including all joint positions and poses.

### Dependencies[[reachy_mini.daemon.app.dependencies.get_daemon]]

Get the daemon as request dependency.

Get the backend as request dependency.

Get the app manager as request dependency.

Get the backend as websocket dependency.

### Jobs[[reachy_mini.daemon.app.bg_job_register.JobStatus]]

Enum for job status.

Pydantic model for install job status.

Handler for background jobs.

Start a background job, with a custom logger and return its job_id.

Get the info of a job by its ID.

WebSocket endpoint to stream job logs in real time.

### Main Application[[reachy_mini.daemon.app.main.Args]]

Arguments for configuring the Reachy Mini daemon.

Create and configure the FastAPI application.

Run the FastAPI app with Uvicorn.

Run the FastAPI app with Uvicorn.

## App Routers

### Daemon Router[[reachy_mini.daemon.app.routers.daemon.start_daemon]]

, use_cache=True, scope=None)"}]}>

Start the daemon.

, use_cache=True, scope=None)"}]}>

Stop the daemon, optionally putting the robot to sleep.

, use_cache=True, scope=None)"}]}>

Restart the daemon.

, use_cache=True, scope=None)"}]}>

Get the current status of the daemon.

, use_cache=True, scope=None)"}]}>

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

, use_cache=True, scope=None)"}]}>
- **use_pose_matrix** (bool) -- Whether to use the pose matrix representation (4x4 flattened) or the translation + Euler angles representation (x, y, z, roll, pitch, yaw).
- **backend** (Backend) -- The backend instance.AnyPoseThe present head pose.
Get the present head pose.

, use_cache=True, scope=None)"}]}>

Get the present body yaw (in radians).

, use_cache=True, scope=None)"}]}>

Get the present antenna joint positions (in radians) - (left, right).

, use_cache=True, scope=None)"}]}>

Get the Direction of Arrival from the microphone array.

Returns the angle in radians (0=left, π/2=front, π=right) and speech detection status.
Returns None if the audio device is not available.

, use_cache=True, scope=None)"}]}>

Get the full robot state, with optional fields.

, use_cache=True, scope=None)"}]}>

WebSocket endpoint to stream the full state of the robot.

### Motors Router[[reachy_mini.daemon.app.routers.motors.get_motor_status]]

, use_cache=True, scope=None)"}]}>

Get the current status of the motors.

, use_cache=True, scope=None)"}]}>

Set the motor control mode.

### Move Router[[reachy_mini.daemon.app.routers.move.get_running_moves]]

Get a list of currently running move tasks.

, use_cache=True, scope=None)"}]}>

Request a movement to a specific target.

, use_cache=True, scope=None)"}]}>

Request the robot to wake up.

, use_cache=True, scope=None)"}]}>

Request the robot to go to sleep.

List available recorded moves in a dataset.

, use_cache=True, scope=None)"}]}>

Request the robot to play a predefined recorded move from a dataset.

Stop a running move task.

, use_cache=True, scope=None)"}]}>

POST route to set a single FullBodyTarget.

WebSocket route to stream move updates.

### Apps Router[[reachy_mini.daemon.app.routers.apps.list_available_apps]]

, use_cache=True, scope=None)"}]}>

List available apps (including not installed).

, use_cache=True, scope=None)"}]}>

List all available apps (including not installed).

, use_cache=True, scope=None)"}]}>

Install a new app by its info (background, returns job_id).

, use_cache=True, scope=None)"}]}>

Remove an installed app by its name (background, returns job_id).

Get status/logs for a job.

WebSocket route to stream live job status/logs for a job, sending updates as soon as new logs are available.

, use_cache=True, scope=None)"}]}>

Start an app by its name.

, use_cache=True, scope=None)"}]}>

Restart the currently running app.

, use_cache=True, scope=None)"}]}>

Stop the currently running app.

, use_cache=True, scope=None)"}]}>

Get the status of the currently running app, if any.

, use_cache=True, scope=None)"}]}>

Install a private HuggingFace space.

Requires HF token to be stored via /api/hf-auth/save-token first.

### Update Router[[reachy_mini.daemon.app.routers.update.available]]

Check if an update is available for Reachy Mini Wireless.

Start the update process for Reachy Mini Wireless version.

Get the info of an update job.

WebSocket endpoint to stream update logs in real time.

### Cache Router[[reachy_mini.daemon.app.routers.cache.clear_huggingface_cache]]

Clear HuggingFace cache directory.

Remove applications virtual environment directory.

### Kinematics Router[[reachy_mini.daemon.app.routers.kinematics.get_kinematics_info]]

, use_cache=True, scope=None)"}]}>

Get the current information of the kinematics.

, use_cache=True, scope=None)"}]}>

Get the URDF representation of the robot.

Get the path to an STL asset file.

### Volume Router[[reachy_mini.daemon.app.routers.volume.get_volume]]

Get the current output volume level.

Set the output volume level and play a test sound.

, use_cache=True, scope=None)"}]}>

Play a test sound.

Get the current microphone input volume level.

Set the microphone input volume level.

### Logs Router[[reachy_mini.daemon.app.routers.logs.websocket_daemon_logs]]

WebSocket endpoint to stream journalctl logs for reachy-mini-daemon service in real time.

### HF Auth Router[[reachy_mini.daemon.app.routers.hf_auth.save_token]]

Save HuggingFace token after validation.

Check if user is authenticated with HuggingFace.

Delete stored HuggingFace token.
