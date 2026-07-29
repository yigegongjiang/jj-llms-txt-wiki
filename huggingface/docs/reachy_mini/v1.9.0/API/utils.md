# Utils

## Interpolation Functions[[reachy_mini.utils.interpolation.minimum_jerk]]

Compute the minimum jerk interpolation function from starting position to goal position.

Linearly interpolate between two poses in 6D space.

Use `yaw_as_scalar` to interpolate yaw as a signed scalar Euler angle instead of along the SO(3) geodesic.
This keeps the path through the front rather than taking the shortest 3D rotation through +-180° (the back).

"}]}>

Compute the time trajectory value based on the specified interpolation method.

- **P** -- A 3x3 rotation matrix.
- **Q** -- Another 3x3 rotation matrix.The angle in radians between the two rotations.
Compute the angle (in radians) between two 3x3 rotation matrices `P` and `Q`.

This is equivalent to the angular distance in axis-angle space.
It is computed via the trace of the relative rotation matrix.

References:
- https://math.stackexchange.com/questions/2113634/comparing-two-rotation-matrices
- http://www.boris-belousov.net/2016/12/01/quat-dist/

- **pose1** -- A 4x4 homogeneous transformation matrix representing the first pose.
- **pose2** -- A 4x4 homogeneous transformation matrix representing the second pose.A tuple of- translation distance in meters,
- angular distance in radians,
- unhinged distance in magic-mm (translation in mm + rotation in degrees).
Compute three types of distance between two 4x4 homogeneous transformation matrices.

The result combines translation (in mm) and rotation (in degrees) using an arbitrary but
emotionally satisfying equivalence: 1 degree ≈ 1 mm.

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

Enumeration of interpolation techniques.

## Hardware Configuration[[reachy_mini.utils.hardware_config.parser.MotorConfig]]

Motor configuration.

Serial configuration.

Reachy Mini configuration.

Parse the YAML configuration file and return a ReachyMiniConfig.

## Rerun Visualization[[reachy_mini.utils.rerun.Rerun]]

Rerun logging for Reachy Mini.

Log the camera image to Rerun.

Log the movement data to Rerun.

Set the absolute paths in the URDF file. Rerun cannot read the "package://" paths.

Start the Rerun logging threads.

Stop the Rerun logging threads.

## Wireless Version Utilities[[reachy_mini.utils.wireless_version.utils.call_logger_wrapper]]

- **command** -- Shell command string.
- **logger** -- logger object with .info and .error methods
- **env** -- Optional environment variables dict. If None, inherits current environment.
Run a shell command asynchronously, streaming stdout and stderr to logger in real time.

- **logger** -- Logger for streaming output.
- **pre_release** -- If True, install pre-release from PyPI (ignored if git_ref set).
- **git_ref** -- If set, install from this GitHub tag/branch instead of PyPI.
Update reachy_mini package and restart daemon.

- **venvs_path** -- Path to the virtual environments directory (default: /venvs)
- **custom_logger** -- Optional logger to use instead of the module logger
For wireless units, check if files under venvs_path are owned by user pollen and fix if needed.

Check if bluetooth service needs updating and update if different.

Compares the source bluetooth_service.py with the installed version at
/bluetooth/bluetooth_service.py. If they differ, copies the new version
and restarts the bluetooth service. Also syncs the commands/ folder.

Check if wireless daemon service needs updating and update if different.

Compares the source reachy-mini-daemon.service with the installed version.
If they differ, copies the new version and reloads systemd.

Check if apps_venv SDK matches daemon install source and sync if needed.

Compares both version AND install source (PyPI vs git ref). If daemon was
installed from a git ref, apps_venv will be synced to the same ref.

Check if an update is available for the given package.

Get the latest version of a package from PyPI.

Get the currently installed version of a package.

## Core Utilities[[reachy_mini.utils.create_head_pose]]

- **x** (float) -- X coordinate of the position.
- **y** (float) -- Y coordinate of the position.
- **z** (float) -- Z coordinate of the position.
- **roll** (float) -- Roll angle
- **pitch** (float) -- Pitch angle
- **yaw** (float) -- Yaw angle
- **mm** (bool) -- If True, convert position from millimeters to meters.
- **degrees** (bool) -- If True, interpret roll, pitch, and yaw as degrees; otherwise as radians.np.ndarrayA 4x4 homogeneous transformation matrix representing the pose.
Create a homogeneous transformation matrix representing a pose in 6D space (position and orientation).

## URDF Parsing[[reachy_mini.utils.parse_urdf_for_kinematics.get_data]]

Generate the urdf_kinematics.json file.
