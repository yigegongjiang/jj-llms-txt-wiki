# 📦 Installation Guide

> **Welcome to Reachy Mini!** This guide will help you install the Python SDK and daemon to start controlling your robot.

| 🐧 **Linux** | 🍎 **macOS** | 🪟 **Windows** |
|:---:|:---:|:---:|
| ✅ Supported | ✅ Supported | ✅ Supported |

**Need help?** Feel free to open an [issue](https://github.com/pollen-robotics/reachy_mini/issues) if you encounter any problem.

## First time using the command line? 🖥️

Click here if you're new to using a terminal/command line

A **command line** (also called terminal or command prompt) is a text-based interface where you can type commands to interact with your computer. Don't worry—it's simpler than it looks!

**How to open the command line:**
* **Windows:** Press `Win + R`, type `cmd` or `powershell`, and press Enter
* **macOS:** Press `Cmd + Space`, type `Terminal`, and press Enter  
* **Linux:** Press `Ctrl + Alt + T` or search for "Terminal" in your applications

**Basic tips:**
* Type commands exactly as shown in the instructions
* Press `Enter` after typing each command to run it
* You can copy and paste commands (right-click to paste in most command line interfaces)

> [!TIP]
> _Don't be intimidated!_ The command line is just another way to give instructions to your computer. Follow the commands step by step, and you'll be controlling your Reachy Mini in no time!

## 1. 📋 Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| 🐍 **Python** | 3.10 - 3.12 | Run Reachy Mini SDK |
| 📂 **Git** | Latest | Download source code and apps |
| 📦 **Git LFS** | Latest | Download model assets |

### 🐍 Install Python

We'll use `uv` - a fast Python package manager that makes installation simple!

#### Step 1: Install uv

In your terminal, run:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

In your terminal, run:
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

**✅ Verify installation:**

Once the installation is completed, close your terminal and open a new one. You can check if everything went well with :
```bash
uv --version
```

#### Step 2: Install Python

In your terminal, run:
```bash
uv python install 3.12 --default
```

> [!TIP]
> We recommend Python 3.12 as it's the latest supported version for Reachy Mini.

### 📂 Install Git and Git LFS

#### Install Git and Git LFS

In your terminal, run:
```bash
sudo apt install git git-lfs
```

#### 1. Install Homebrew (if not already installed)

In your terminal, run:
```zsh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

For Apple Silicon (M1, M2, etc.), you will also be prompted to run:

```zsh
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"
```

✅ Verify Homebrew:

Once the installation is completed you can check if it went fine with:
```zsh
brew --version
```

#### 2. Install Git and Git LFS 

In your terminal, run:
```zsh
brew install git git-lfs
```

#### Download and install Git for Windows

Download and install Git for Windows:  
https://git-scm.com/install/windows

**✅ Finalize installation:**

Finally, Git LFS then needs to be initialized with the command :

```bash
git lfs install
```

## 2. 🏠 Set up a Virtual Environment

> **Why use a virtual environment?** It keeps your Reachy Mini installation isolated and prevents conflicts with other Python projects. Modern Python development requires this!

### Create the environment

In your terminal, run:
```bash
uv venv reachy_mini_env --python 3.12
```

### Activate the environment

In your terminal, run:
```bash
source reachy_mini_env/bin/activate
```

> [!WARNING]
> _First-time setup:_ Before you can activate your virtual environment, Windows needs permission to run scripts. You only need to do this once!

**Step 1:** Open terminal as Administrator
- Press `Win + R`, type `powershell`
- Right-click on "Windows PowerShell" and select "Run as administrator"

**Step 2:** Enable script execution

In the administrator terminal, run:
```powershell
powershell Set-ExecutionPolicy RemoteSigned
```

**Step 3:** Close the administrator terminal and open a regular terminal

Now you can activate your virtual environment by running:
```powershell
reachy_mini_env\Scripts\activate
```

> **✅ Success indicator:** You should see `(reachy_mini_env)` at the start of your command line prompt!

## 3. 🚀 Install Reachy Mini

Choose your installation method:

> **Recommended for most users** - Just want to control your robot? This is for you!

In your terminal, run:
```bash
uv pip install "reachy-mini"
```

If you want to use the simulation mode, you need to add the `mujoco` extra:
```bash
uv pip install "reachy-mini[mujoco]"
```

> [!TIP]
> The post installation of gstreamer is due to an [issue](https://github.com/pypi/support/issues/8847#issuecomment-3899714506) with PyPi and should be solved in the future.

🐧 Linux users: additional steps required

**GStreamer** must be installed manually on Linux:

[![GStreamer Installation Guide](https://img.shields.io/badge/📖-GStreamer%20Installation%20Guide-blue?style=for-the-badge)](gstreamer-installation)

**USB permissions** — needed for the USB connection to Reachy Mini:

```bash
echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="55d3", MODE="0666", GROUP="dialout"
SUBSYSTEM=="usb", ATTRS{idVendor}=="38fb", ATTRS{idProduct}=="1001", MODE="0666", GROUP="dialout"' \
| sudo tee /etc/udev/rules.d/99-reachy-mini.rules

sudo udevadm control --reload-rules && sudo udevadm trigger
sudo usermod -aG dialout $USER
```

> [!WARNING]
> Log out and log back in for the changes to take effect!

> **For developers** - Want to modify the SDK or contribute? Choose this option!

In your terminal, run:
```bash
git clone https://github.com/pollen-robotics/reachy_mini && cd reachy_mini
uv sync
```

If you want to use the simulation mode, you need to add the `mujoco` extra:
```bash
uv sync --extra mujoco
```

🐧 Linux users: additional steps required

**GStreamer** must be installed manually on Linux:

[![GStreamer Installation Guide](https://img.shields.io/badge/📖-GStreamer%20Installation%20Guide-blue?style=for-the-badge)](gstreamer-installation)

**USB permissions** — needed for the USB connection to Reachy Mini:

```bash
echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="55d3", MODE="0666", GROUP="dialout"
SUBSYSTEM=="usb", ATTRS{idVendor}=="38fb", ATTRS{idProduct}=="1001", MODE="0666", GROUP="dialout"' \
| sudo tee /etc/udev/rules.d/99-reachy-mini.rules

sudo udevadm control --reload-rules && sudo udevadm trigger
sudo usermod -aG dialout $USER
```

> [!WARNING]
> Log out and log back in for the changes to take effect!

## 🎉 Congratulations!

You've successfully installed Reachy Mini! Your robot is ready to come to life.

## ❓ Troubleshooting
Encountering an issue? 👉 **[Check the Troubleshooting & FAQ Guide](../troubleshooting)**

## Next Steps
* **[Quickstart Guide](quickstart)**: Run your first behavior on Reachy Mini
* **[Python SDK](python-sdk)**: Learn to move, see, speak, and hear.
* **[AI Integrations](integration)**: Connect LLMs, build Apps, and publish to Hugging Face.
* **[Core Concepts](core-concept)**: Architecture, coordinate systems, and safety limits.
