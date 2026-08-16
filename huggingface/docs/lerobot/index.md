# LeRobot

  
    <img
      alt="LeRobot, Hugging Face Robotics Library"
      src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/lerobot/lerobot-logo-thumbnail.png"
    >
  

**State-of-the-art machine learning for real-world robotics**

🤗 LeRobot provides a hardware-agnostic, Python-native interface for controlling real robots - from affordable arms like the SO-ARM101 to full humanoids. Plus the tools to record, store, and share the datasets they generate. Every dataset uses the standardized **LeRobotDataset** format (synchronized video + action/state data) and can be streamed directly from the [Hugging Face Hub](https://huggingface.co/lerobot).

🤗 On top of that data, LeRobot implements state-of-the-art policies - from lightweight imitation-learning models like ACT to large vision-language-action models like π₀ and SmolVLA - all trainable, shareable, and deployable with the same handful of CLI commands.

The goal: lower the barrier to entry for robotics, so that everyone can contribute to, and benefit from, shared datasets and pretrained models.

  
    
  
  
    
  
  
    
  

  

## How It Works

**Teleoperate → Record → Train → Deploy**

1. **Teleoperate** - control the robot yourself (with a leader arm, keyboard, or phone) so it can learn from your movements.
2. **Record** - each demonstration is saved as a dataset: synchronized camera video plus the actions you took.
3. **Train** - a policy (the neural network that will control the robot) learns to imitate your demonstrations.
4. **Deploy** - run the trained policy on the robot and watch it complete the task on its own.

## Get Started

New here? [Install LeRobot](./installation), then pick your path:

  
    🔧 I have a robot
    
      LeRobot supports a wide range of arms and mobile robots. Popular picks:
    
    
      
        SO-101 - our flagship, low-cost arm
      
      
        LeKiwi - a mobile base with an arm on top
      
      
        Koch v1.1 - a long-time community favorite
      
      
        or find yours under Robots in the sidebar
      
    
    
      Once it's assembled and calibrated, record a dataset and train your first
      policy with the imitation learning tutorial - or
      skip the CLI entirely with LeLab, a browser GUI for
      the same workflow.
    
  
  
    💻 No hardware yet
    
      You can still train and evaluate policies without owning a robot:
    
    
      
        train on an existing
        
          LeRobot dataset
        
        from the Hub
      
      
        evaluate in simulation, against benchmarks like
        LIBERO or Meta-World
      
      
        try the free Colab notebooks - nothing to
        install
      
    
  
  
    🤝 I want to contribute
    
      Start with the Contributing guide, then
      add a new policy or
      bring your own hardware.
    
  

## Explore the Docs

  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./cheat-sheet"
  >
    📋 Cheat Sheet
    
      Every LeRobot CLI command, copy-paste ready.
    
  
  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./hardware_guide"
  >
    🖥️ Compute & Hardware Guide
    
      Which policy fits your GPU, and how long training takes.
    
  
  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./lerobot-dataset-v3"
  >
    🗂️ LeRobotDataset
    
      Load, stream, and visualize robot datasets from the Hub.
    
  
  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./lelab"
  >
    🖼 LeLab
    
      A browser GUI for calibrating, recording, and training - no CLI required.
    
  
  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./act"
  >
    🧠 Policies
    
      Start with ACT, our recommended first policy - or browse SmolVLA, π₀, and
      more in the sidebar.
    
  
  <a
    class="!no-underline border dark:border-gray-700 rounded-lg p-4 shadow hover:shadow-lg"
    href="./envhub"
  >
    🎮 Simulation & Benchmarks
    
      Train and evaluate in simulated environments before touching real
      hardware.
    
  

## Common Problems

Running into issues? A few of the most frequent ones:

- **Blurry or unusable camera footage** - lighting matters more than resolution. See the [Cameras](./cameras) guide.
- **Build or install errors** (`cmake`, `ffmpeg`, CUDA) - see the Troubleshooting section of the [Installation guide](./installation#troubleshooting).
- **Not sure which policy fits your GPU** - check the [Compute & Hardware Guide](./hardware_guide).
- **Still stuck?** Ask on [Discord](https://discord.gg/s3KuuzsPFb) - the community (and the LeRobot team) is there to help.
