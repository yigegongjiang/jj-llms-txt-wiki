# Tools

## Motor Setup Tools[[reachy_mini.tools.setup_motor]]

#### reachy_mini.tools.setup_motor[[reachy_mini.tools.setup_motor]]

```python
reachy_mini.tools.setup_motor(motor_config: MotorConfig, serial_port: str, from_baudrate: int, target_baudrate: int, from_id: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L55)

Set up the motor with the given configuration.

#### reachy_mini.tools.setup_motor.lookup_for_motor[[reachy_mini.tools.setup_motor.lookup_for_motor]]

```python
reachy_mini.tools.setup_motor.lookup_for_motor(serial_port: str, id: int, baudrate: int, silent: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L144)

Check if a motor with the given ID is reachable on the specified serial port.

#### reachy_mini.tools.setup_motor.disable_torque[[reachy_mini.tools.setup_motor.disable_torque]]

```python
reachy_mini.tools.setup_motor.disable_torque(serial_port: str, id: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L165)

Disable the torque of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_baudrate[[reachy_mini.tools.setup_motor.change_baudrate]]

```python
reachy_mini.tools.setup_motor.change_baudrate(serial_port: str, id: int, base_baudrate: int, target_baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L177)

Change the baudrate of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_id[[reachy_mini.tools.setup_motor.change_id]]

```python
reachy_mini.tools.setup_motor.change_id(serial_port: str, current_id: int, new_id: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L191)

Change the ID of the motor with the given current ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_offset[[reachy_mini.tools.setup_motor.change_offset]]

```python
reachy_mini.tools.setup_motor.change_offset(serial_port: str, id: int, offset: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L203)

Change the offset of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_operating_mode[[reachy_mini.tools.setup_motor.change_operating_mode]]

```python
reachy_mini.tools.setup_motor.change_operating_mode(serial_port: str, id: int, operating_mode: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L215)

Change the operating mode of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_angle_limits[[reachy_mini.tools.setup_motor.change_angle_limits]]

```python
reachy_mini.tools.setup_motor.change_angle_limits(serial_port: str, id: int, angle_limit_min: int, angle_limit_max: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L233)

Change the angle limits of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_shutdown_error[[reachy_mini.tools.setup_motor.change_shutdown_error]]

```python
reachy_mini.tools.setup_motor.change_shutdown_error(serial_port: str, id: int, baudrate: int, shutdown_error: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L256)

Change the shutdown error of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.change_return_delay_time[[reachy_mini.tools.setup_motor.change_return_delay_time]]

```python
reachy_mini.tools.setup_motor.change_return_delay_time(serial_port: str, id: int, return_delay_time: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L274)

Change the return delay time of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.light_led_up[[reachy_mini.tools.setup_motor.light_led_up]]

```python
reachy_mini.tools.setup_motor.light_led_up(serial_port: str, id: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L292)

Light the LED of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.light_led_down[[reachy_mini.tools.setup_motor.light_led_down]]

```python
reachy_mini.tools.setup_motor.light_led_down(serial_port: str, id: int, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L311)

Light the LED of the motor with the given ID on the specified serial port.

#### reachy_mini.tools.setup_motor.check_configuration[[reachy_mini.tools.setup_motor.check_configuration]]

```python
reachy_mini.tools.setup_motor.check_configuration(motor_config: MotorConfig, serial_port: str, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/setup_motor.py#L329)

Check the configuration of the motor with the given ID on the specified serial port.

## Motor Scanning Tools[[reachy_mini.tools.scan_motors.scan]]

#### reachy_mini.tools.scan_motors.scan[[reachy_mini.tools.scan_motors.scan]]

```python
reachy_mini.tools.scan_motors.scan(port: str, baudrate: int)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/scan_motors.py#L48)

Scan the bus at the given baudrate and return detected IDs.

## Motor Reflashing Tools[[reachy_mini.tools.reflash_motors.reflash_motors_if_needed]]

#### reachy_mini.tools.reflash_motors.reflash_motors_if_needed[[reachy_mini.tools.reflash_motors.reflash_motors_if_needed]]

```python
reachy_mini.tools.reflash_motors.reflash_motors_if_needed(serialport: typing.Optional[str] = None, dont_light_up: bool = False)
```

[Source](https://github.com/pollen-robotics/reachy_mini/blob/v1.10.0/src/reachy_mini/tools/reflash_motors.py#L40)

Reflash Reachy Mini's motors.
