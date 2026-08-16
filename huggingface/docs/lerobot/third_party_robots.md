# Third-Party Robots & Teleoperators

The LeRobot ecosystem extends far beyond its officially supported hardware. Thanks to LeRobot's plugin architecture, the community has built integrations for a wide range of robot arms and teleoperation devices — from industrial manipulators to affordable hobbyist platforms, VR headsets, haptic devices, and full arm-plus-teleoperator kits. This page showcases community-maintained integrations you can use for teleoperation, data collection, and policy deployment.

> [!IMPORTANT]
> These projects are developed and maintained by third parties. Please refer to each repository for installation instructions, hardware requirements, and support.

Drop-in plugins are auto-discovered by package name: LeRobot imports any installed package prefixed with `lerobot_robot_` or `lerobot_teleoperator_`. Once installed, reference the `type` the plugin registers (see its README — it may differ from the package name) directly from any LeRobot command:

```bash
pip install lerobot_robot_<name> lerobot_teleoperator_<name>

lerobot-record \
    --robot.type=<robot_name> \
    --teleop.type=<teleoperator_name> \
    --dataset.repo_id=${HF_USER}/my-dataset \
    --dataset.num_episodes=5
```

> [!TIP]
> ⚠️ marks projects that are forks/extensions of LeRobot. They may require custom setup rather than working with an unmodified install. All other entries are drop-in plugins.

## Industrial & Collaborative Arms

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot-robot-xarm
      Plugin for the xArm collaborative arm series from UFACTORY.
    
    
      lerobot_lebai
      Plugin for the six-axis collaborative arms from Lebai.
    
    
      LeFranX ⚠️
      LeRobot extension for the Franka research arm, paired with the RobotEra XHand hand for VR teleoperation.
    
  

#### Universal Robots UR5e

  
    
    
  
  
    ProjectDescription
  
  
    
      UR5e-LeRobot ⚠️
      LeRobot extension for the Universal Robots UR5e, with single-arm and bimanual support.
    
    
      lerobot_ur5e_auto ⚠️
      LeRobot extension for a mobile Universal Robots UR5e, adding automated recording at scale with minimal supervision.
    
    
      lerobot_robot_ur5e
      Plugin for the Universal Robots UR5e with a Robotiq gripper, over RTDE control.
    
  

## Research & Learning Arms

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot_trossen
      Plugin for the WidowX and ALOHA-style arms from Trossen Robotics.
    
  

#### AgileX Piper

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot_robot_piper (AgRobotics Research)
      Plugin for the AgileX Piper arm.
    
    
      lerobot_robot_piper (WeGo Robotics)
      Plugin for the AgileX Piper arm, with multi-arm teleoperation, safety limits, and GUI tools.
    
  

## Affordable & Hobbyist Arms

  
    
    
  
  
    ProjectDescription
  
  
    
      fashionstar-lerobot-robot-cello
      Plugin for the StarAI Cello 6+1 degrees of freedom robot arm from FashionStar.
    
    
      fashionstar-lerobot-robot-viola
      Plugin for the compact StarAI Viola 6+1 degrees of freedom robot arm from FashionStar.
    
  

## Service, Mobile & Utility Robots

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot-robot-ugo-pro
      Plugin for the ugo Pro dual-arm service robot from ugo.
    
    
      lerobot_robot_lekiwi_pincopen
      Plugin for a LeKiwi mobile manipulator with a PincOpen gripper.
    
    
      lerobot-robot-dummy
      Plugin simulating a robot for recording without hardware. Useful for debugging !
    
  

## Teleoperators

### VR & Motion Controllers

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot-teleoperator-teleop
      Plugin turning a phone or VR headset into a teleoperator via WebXR, wrapping the open-source teleop library.
    
    
      lerobot-teleoperator-spacemouse
      Plugin for the 3Dconnexion SpaceMouse, with inverse kinematics for SO-ARMS robots.
    
    
      vr-teleop-kit
      Plugin teleoperating arms from a Meta Quest (WebXR), relying on URDF descriptions for inverse kinematics.
    
    
      lerobot-teleoperator-pico4
      Plugin for the PICO 4 VR headset, with a companion controller-free hand-tracking variant.
    
  

### Leader Arms

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot_teleoperator_gello
      Plugin for the 7 degrees of freedom GELLO teleoperator.
    
    
      lerobot_teleoperator_yamactiveleader
      Plugin for the active YAM teleoperator from I2RT, a bilateral force-feedback arm.
    
    
      lerobot_teleoperator_omy
      Plugin for the OMY-L100 6 degrees of freedom teleoperator from ROBOTIS.
    
    
      lerobot-teleoperator-pipermate
      Plugin for the PiperMate teleoperator (FashionStar UART servos), driving the AgileX Piper arm.
    
  

### Haptic Devices

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot_teleoperator_inverse3
      Plugin for the Haply Inverse3 haptic device, adding force-feedback teleoperation.
    
    
      lerobot_teleoperator_omega7
      Plugin for the Force Dimension omega.7 haptic device, adding force-feedback teleoperation.
    
  

### Networked & Remote

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot-teleoperator-livekit
      Plugin receiving teleoperation commands over a LiveKit Portal (WebRTC) for remote control.
    
  

## Full Kits (Robot + Teleoperator)

  
    
    
  
  
    ProjectDescription
  
  
    
      lerobot-arx5
      Plugin for the ARX5 arm: lerobot-arx5 robot arm with its lerobot-teleoperator-arx5 teleoperator arm.
    
    
      Nextis-AIRA-3D
      Plugin for the 7 degrees of freedom arm from Nextis: robot arm aira_follower and teleoperator arm aira_leader.
    
    
      lerobot_yam
      Plugin suite for the YAM arm from I2RT: robot arm yam_follower and teleoperator arm yam_leader.
    
    
      trlc-dk1
      Plugin for the development kit from The Robot Learning Company: single and bimanual arms follower/teleoperator types.
    
    
      hex_lerobot_drivers
      Plugin suite for HEXFELLOW devices: robots, teleoperators, and cameras (see Cameras &amp; Sensors).
    
    
      lerobot-robot-nexarm-follower
      Plugin for the NexArm from Hiwonder: the robot arm and its matching teleoperator arm.
    
  

## ROS 2 Bridges

  
    
    
  
  
    ProjectDescription
  
  
    
      leros2
      Plugin bridging ROS 2 topics and actions to LeRobot robots and teleoperators.
    
    
      lerobot_robot_ros2_zenoh
      Plugin bridging ROS 2 robots to LeRobot over Zenoh pub/sub transport.
    
  

## Contributing

Built your own LeRobot hardware integration? The plugin system makes it straightforward to add new robots and teleoperators — check out the [Bring Your Own Hardware](./integrate_hardware) guide to get started, and share your project with the community!
