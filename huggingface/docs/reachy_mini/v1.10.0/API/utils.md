# Utils

## Interpolation Functions[[reachy_mini.utils.interpolation.minimum_jerk]]

#### reachy_mini.utils.interpolation.minimum_jerk[[reachy_mini.utils.interpolation.minimum_jerk]]

```python
reachy_mini.utils.interpolation.minimum_jerk(starting_position: ndarray, goal_position: ndarray, duration: float, starting_velocity: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None, starting_acceleration: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None, final_velocity: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None, final_acceleration: typing.Optional[numpy.ndarray[tuple[typing.Any, ...], numpy.dtype[numpy.float64]]] = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L13)

Compute the minimum jerk interpolation function from starting position to goal position.

#### reachy_mini.utils.interpolation.linear_pose_interpolation[[reachy_mini.utils.interpolation.linear_pose_interpolation]]

```python
reachy_mini.utils.interpolation.linear_pose_interpolation(start_pose: ndarray, target_pose: ndarray, t: float, yaw_as_scalar: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L58)

Linearly interpolate between two poses in 6D space.

Use `yaw_as_scalar` to interpolate yaw as a signed scalar Euler angle instead of along the SO(3) geodesic.
This keeps the path through the front rather than taking the shortest 3D rotation through +-180° (the back).

#### reachy_mini.utils.interpolation.time_trajectory[[reachy_mini.utils.interpolation.time_trajectory]]

```python
reachy_mini.utils.interpolation.time_trajectory(t: float, method: InterpolationTechnique = <InterpolationTechnique.MIN_JERK: 'minjerk'>)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L116)

Compute the time trajectory value based on the specified interpolation method.

#### reachy_mini.utils.interpolation.delta_angle_between_mat_rot[[reachy_mini.utils.interpolation.delta_angle_between_mat_rot]]

```python
reachy_mini.utils.interpolation.delta_angle_between_mat_rot(P: ndarray, Q: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L155)

**Parameters:**

P : A 3x3 rotation matrix.

Q : Another 3x3 rotation matrix.

**Returns:**

The angle in radians between the two rotations.

Compute the angle (in radians) between two 3x3 rotation matrices `P` and `Q`.

This is equivalent to the angular distance in axis-angle space.
It is computed via the trace of the relative rotation matrix.

References:
- https://math.stackexchange.com/questions/2113634/comparing-two-rotation-matrices
- http://www.boris-belousov.net/2016/12/01/quat-dist/

#### reachy_mini.utils.interpolation.distance_between_poses[[reachy_mini.utils.interpolation.distance_between_poses]]

```python
reachy_mini.utils.interpolation.distance_between_poses(pose1: ndarray, pose2: ndarray)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L181)

**Parameters:**

pose1 : A 4x4 homogeneous transformation matrix representing the first pose.

pose2 : A 4x4 homogeneous transformation matrix representing the second pose.

**Returns:** `A tuple of`

- translation distance in meters,
- angular distance in radians,
- unhinged distance in magic-mm (translation in mm + rotation in degrees).

Compute three types of distance between two 4x4 homogeneous transformation matrices.

The result combines translation (in mm) and rotation (in degrees) using an arbitrary but
emotionally satisfying equivalence: 1 degree ≈ 1 mm.

#### reachy_mini.utils.interpolation.compose_world_offset[[reachy_mini.utils.interpolation.compose_world_offset]]

```python
reachy_mini.utils.interpolation.compose_world_offset(T_abs: ndarray, T_off_world: ndarray, reorthonormalize: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L207)

Compose an absolute world-frame pose with a world-frame offset.

- translations add in world:       t_final = t_abs + t_off
- rotations compose in world:      R_final = R_off @ R_abs
This rotates the frame in place (about its own origin) by a rotation
defined in world axes, and shifts it by a world translation.

Parameters
----------
T_abs : (4,4) ndarray
Absolute pose in world frame.
T_off_world : (4,4) ndarray
Offset transform specified in world axes (dx,dy,dz in world; dR about world axes).
reorthonormalize : bool
If True, SVD-orthonormalize the resulting rotation to fight drift.

Returns
-------
T_final : (4,4) ndarray
Resulting pose in world frame.

#### reachy_mini.utils.interpolation.InterpolationTechnique[[reachy_mini.utils.interpolation.InterpolationTechnique]]

```python
reachy_mini.utils.interpolation.InterpolationTechnique(value, names = None, module = None, qualname = None, type = None, start = 1, boundary = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/interpolation.py#L107)

Enumeration of interpolation techniques.

## Hardware Configuration[[reachy_mini.utils.hardware_config.parser.MotorConfig]]

#### reachy_mini.utils.hardware_config.parser.MotorConfig[[reachy_mini.utils.hardware_config.parser.MotorConfig]]

```python
reachy_mini.utils.hardware_config.parser.MotorConfig(id: int, offset: int, angle_limit_min: int, angle_limit_max: int, return_delay_time: int, shutdown_error: int, operating_mode: int, pid: tuple[int, int, int] | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/hardware_config/parser.py#L9)

Motor configuration.

#### reachy_mini.utils.hardware_config.parser.SerialConfig[[reachy_mini.utils.hardware_config.parser.SerialConfig]]

```python
reachy_mini.utils.hardware_config.parser.SerialConfig(baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/hardware_config/parser.py#L23)

Serial configuration.

#### reachy_mini.utils.hardware_config.parser.ReachyMiniConfig[[reachy_mini.utils.hardware_config.parser.ReachyMiniConfig]]

```python
reachy_mini.utils.hardware_config.parser.ReachyMiniConfig(version: str, serial: SerialConfig, motors: dict)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/hardware_config/parser.py#L30)

Reachy Mini configuration.

#### reachy_mini.utils.hardware_config.parser.parse_yaml_config[[reachy_mini.utils.hardware_config.parser.parse_yaml_config]]

```python
reachy_mini.utils.hardware_config.parser.parse_yaml_config(filename: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/hardware_config/parser.py#L38)

Parse the YAML configuration file and return a ReachyMiniConfig.

## Rerun Visualization[[reachy_mini.utils.rerun.Rerun]]

#### reachy_mini.utils.rerun.Rerun[[reachy_mini.utils.rerun.Rerun]]

```python
reachy_mini.utils.rerun.Rerun(reachymini: ReachyMini, app_id: str = 'reachy_mini_rerun', spawn: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L27)

Rerun logging for Reachy Mini.

#### log_camera[[reachy_mini.utils.rerun.Rerun.log_camera]]

```python
log_camera()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L130)

Log the camera image to Rerun.

#### log_movements[[reachy_mini.utils.rerun.Rerun.log_movements]]

```python
log_movements()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L186)

Log the movement data to Rerun.

#### set_absolute_path_to_urdf[[reachy_mini.utils.rerun.Rerun.set_absolute_path_to_urdf]]

```python
set_absolute_path_to_urdf(urdf_path: str, abs_path: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L95)

Set the absolute paths in the URDF file. Rerun cannot read the "package://" paths.

#### start[[reachy_mini.utils.rerun.Rerun.start]]

```python
start()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L105)

Start the Rerun logging threads.

#### stop[[reachy_mini.utils.rerun.Rerun.stop]]

```python
stop()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/rerun.py#L111)

Stop the Rerun logging threads.

## Wireless Version Utilities[[reachy_mini.utils.wireless_version.utils.call_logger_wrapper]]

#### reachy_mini.utils.wireless_version.utils.call_logger_wrapper[[reachy_mini.utils.wireless_version.utils.call_logger_wrapper]]

```python
reachy_mini.utils.wireless_version.utils.call_logger_wrapper(command: str, logger: Logger, env: dict[str, str] | None = None, ok_returncodes: tuple = (0,))
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/utils.py#L117)

**Parameters:**

command : Shell command string.

logger : logger object with .info and .error methods

env : Optional environment variables dict. If None, inherits current environment.

ok_returncodes : Exit codes treated as success. Only the daemon restart needs more than `(0,)` - see `update_reachy_mini`.

**Raises:** `RuntimeError`

- `RuntimeError` -- If the command exits with a code outside *ok_returncodes*.

Run a shell command asynchronously, streaming stdout and stderr to logger in real time.

#### reachy_mini.utils.wireless_version.update.update_reachy_mini[[reachy_mini.utils.wireless_version.update.update_reachy_mini]]

```python
reachy_mini.utils.wireless_version.update.update_reachy_mini(logger: Logger, pre_release: bool = False, git_ref: str | None = None)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/update.py#L9)

**Parameters:**

logger : Logger for streaming output.

pre_release : If True, install pre-release from PyPI (ignored if git_ref set).

git_ref : If set, install from this GitHub tag/branch instead of PyPI.

**Raises:** ``RuntimeError``

- ``RuntimeError`` -- If the daemon venv install or the restart command
  fails. An apps_venv failure is deliberately non-fatal.

Update reachy_mini package and restart daemon.

#### reachy_mini.utils.wireless_version.startup_check.check_and_fix_venvs_ownership[[reachy_mini.utils.wireless_version.startup_check.check_and_fix_venvs_ownership]]

```python
reachy_mini.utils.wireless_version.startup_check.check_and_fix_venvs_ownership(venvs_path: str = '/venvs')
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/startup_check.py#L22)

**Parameters:**

venvs_path : Path to the virtual environments directory (default: /venvs)

For wireless units, check if files under venvs_path are owned by user pollen and fix if needed.

#### reachy_mini.utils.wireless_version.startup_check.check_and_update_bluetooth_service[[reachy_mini.utils.wireless_version.startup_check.check_and_update_bluetooth_service]]

```python
reachy_mini.utils.wireless_version.startup_check.check_and_update_bluetooth_service()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/startup_check.py#L82)

Check if bluetooth service needs updating and update if different.

Compares the source bluetooth_service.py with the installed version at
/bluetooth/bluetooth_service.py. If they differ, copies the new version
and restarts the bluetooth service. Also syncs the commands/ folder.

#### reachy_mini.utils.wireless_version.startup_check.check_and_update_wireless_launcher[[reachy_mini.utils.wireless_version.startup_check.check_and_update_wireless_launcher]]

```python
reachy_mini.utils.wireless_version.startup_check.check_and_update_wireless_launcher()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/startup_check.py#L192)

Check if wireless daemon service needs updating and update if different.

Compares the source reachy-mini-daemon.service with the installed version.
If they differ, copies the new version and reloads systemd.

#### reachy_mini.utils.wireless_version.startup_check.check_and_sync_apps_venv_sdk[[reachy_mini.utils.wireless_version.startup_check.check_and_sync_apps_venv_sdk]]

```python
reachy_mini.utils.wireless_version.startup_check.check_and_sync_apps_venv_sdk()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/startup_check.py#L388)

Check if apps_venv SDK matches daemon install source and sync if needed.

Compares both version AND install source (PyPI vs git ref). If daemon was
installed from a git ref, apps_venv will be synced to the same ref.

#### reachy_mini.utils.wireless_version.update_available.is_update_available[[reachy_mini.utils.wireless_version.update_available.is_update_available]]

```python
reachy_mini.utils.wireless_version.update_available.is_update_available(package_name: str, pre_release: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/update_available.py#L37)

Check if an update is available for the given package.

#### reachy_mini.utils.wireless_version.update_available.get_pypi_version[[reachy_mini.utils.wireless_version.update_available.get_pypi_version]]

```python
reachy_mini.utils.wireless_version.update_available.get_pypi_version(package_name: str, pre_release: bool)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/update_available.py#L48)

Get the latest version of a package from PyPI.

#### reachy_mini.utils.wireless_version.update_available.get_local_version[[reachy_mini.utils.wireless_version.update_available.get_local_version]]

```python
reachy_mini.utils.wireless_version.update_available.get_local_version(package_name: str)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/wireless_version/update_available.py#L73)

Get the currently installed version of a package.

## Core Utilities[[reachy_mini.utils.create_head_pose]]

#### reachy_mini.utils.create_head_pose[[reachy_mini.utils.create_head_pose]]

```python
reachy_mini.utils.create_head_pose(x: float = 0, y: float = 0, z: float = 0, roll: float = 0, pitch: float = 0, yaw: float = 0, mm: bool = False, degrees: bool = True)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/__init__.py#L13)

**Parameters:**

x (float) : X coordinate of the position.

y (float) : Y coordinate of the position.

z (float) : Z coordinate of the position.

roll (float) : Roll angle

pitch (float) : Pitch angle

yaw (float) : Yaw angle

mm (bool) : If True, convert position from millimeters to meters.

degrees (bool) : If True, interpret roll, pitch, and yaw as degrees; otherwise as radians.

**Returns:** `np.ndarray`

A 4x4 homogeneous transformation matrix representing the pose.

Create a homogeneous transformation matrix representing a pose in 6D space (position and orientation).

## URDF Parsing[[reachy_mini.utils.parse_urdf_for_kinematics.get_data]]

#### reachy_mini.utils.parse_urdf_for_kinematics.get_data[[reachy_mini.utils.parse_urdf_for_kinematics.get_data]]

```python
reachy_mini.utils.parse_urdf_for_kinematics.get_data()
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/utils/parse_urdf_for_kinematics.py#L17)

Generate the urdf_kinematics.json file.
