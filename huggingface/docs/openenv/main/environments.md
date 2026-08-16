# Environments

> [!NOTE]
> The environments listed here may not reflect the latest additions. For the official OpenEnv collection, see the [OpenEnv organization on Hugging Face](https://huggingface.co/openenv). You may also find additional community environments tagged `agent-environment` on [Hugging Face Spaces](https://huggingface.co/spaces?category=agent-environment). The environments highlighted below are a curated selection.

The OpenEnv community has built a catalog of ready-to-run environments that cover deterministic smoke tests, full developer workflows, and multi-step reasoning challenges. Explore the surface area below and jump directly into the guides for each environment.

  
    
      Echo
      Minimal observation/action loop for verifying client integrations, CI pipelines, and onboarding flows in seconds.
      
        📄 Docs
        🤗 HF
      
    
    
      Coding
      Secure sandbox with filesystem access and evaluation hooks for executing generated code and building autonomous dev workflows.
      
        📄 Docs
        🤗 HF
      
    
    
      Jupyter
      Notebook-style coding environment backed by E2B with setup/verify hooks and a web UI for interactive runs.
      
        📄 Docs
      
    
    
      Terminus
      Terminal-first coding environment with high-contrast shell output and session controls for execute/verify/close flows.
      
        📄 Docs
      
    
    
      Coding Tools
      SETA-style multi-tool coding environment with shell, file editing, search, todos, and submit verification.
      
        📄 Docs
      
    
    
      Chat
      Message-driven loop tailored for conversational agents that need structured turns, safety rails, and message attribution.
      
        📄 Docs
        🤗 HF
      
    
    
      Atari
      Classic Arcade Learning Environment tasks packaged for fast benchmarking of reinforcement-learning style agents.
      
        📄 Docs
        🤗 HF
      
    
    
      OpenSpiel
      Multi-agent, game-theory workloads powered by DeepMind's OpenSpiel suite, ideal for search and self-play experiments.
      
        📄 Docs
        🤗 HF
      
    
    
      SUMO-RL
      Traffic control scenarios with SUMO simulators for agents that reason about continuous control and scheduling.
      
        📄 Docs
      
    
    
      FinRL
      Financial market simulations with portfolio APIs, perfect for RLHF strategies and algorithmic trading experiments.
      
        📄 Docs
      
    
    
      TextArena
      Multi-task text arena for language-game competitions such as Wordle, reasoning puzzles, and program synthesis.
      
        📄 Docs
        🤗 HF
      
    
    
      Git
      Teaches agents to navigate repositories, inspect diffs, and land changes via Git-native operations.
      
        📄 Docs
      
    
    
      DIPG Safety
      Safety-critical diagnostics from the DIPG benchmark, highlighting guardrails, adversarial prompts, and risk scoring.
      
        📄 Docs
        🤗 HF
      
    
    
      Snake
      Classic snake game environment for RL research with configurable grids, partial observability, and customizable rewards.
      
        📄 Docs
        🤗 HF
      
    
    
      Web Search
      Web search environment for RL research with configurable grids, partial observability, and customizable rewards.
      
        📄 Docs
        🤗 HF
      
    
    
      BrowserGym
      Browser automation environment for web agents with DOM interaction, navigation, and multi-step task completion.
      
        📄 Docs
        🤗 HF
      
    
    
      KernRL
      RL environment for GPU kernel optimization. Train LLM agents to write fast CUDA/Triton kernels that beat baseline implementations.
      
        📄 Docs
      
    
    
      Calendar
      Calendar tool-use environment exposing a Calendar Gym through the OpenEnv reset/step/state interface for scheduling agents.
      
        📄 Docs
      
    
    
      CARLA
      Embodied evaluation environment for testing LLM decision-making in a full 3D driving simulator with irreversible consequences and ethical trolley scenarios.
      
        📄 Docs
        🤗 HF
      
    
    
      Chess
      Chess RL environment powered by the moonfish engine with configurable opponents, position evaluation, and full chess rules.
      
        📄 Docs
      
    
    
      Connect4
      Classic Connect Four board game environment for training agents on turn-based strategy with a 6×7 grid.
      
        📄 Docs
      
    
    
      DM Control
      Generic OpenEnv wrapper for dm_control.suite, providing access to all MuJoCo-based continuous control tasks like cartpole, walker, and humanoid.
      
        📄 Docs
      
    
    
      FinQA
      Financial question-answering environment that evaluates LLMs on complex financial questions using tool calls on SEC 10-K filing data.
      
        📄 Docs
      
    
    
      Grid World
      Simple 5×5 grid world RL testbed and step-by-step guide for building new OpenEnv environments from scratch.
      
        📄 Docs
        🤗 HF
      
    
    
      Julia
      Julia code execution environment with test result tracking and reward calculation for RL training on Julia programming tasks.
      
        📄 Docs
      
    
    
      Maze
      Gridworld maze where agents navigate from start to exit while avoiding walls, with configurable 8×8 layouts.
      
        📄 Docs
      
    
    
      OpenApp
      Web application simulation wrapping the OpenApps framework and BrowserGym for training UI agents on calendar, todo, messenger, and maps apps.
      
        📄 Docs
      
    
    
      Reasoning Gym
      Integrates the Reasoning Gym library to provide single-step reasoning tasks with configurable datasets and scoring.
      
        📄 Docs
      
    
    
      REPL
      Python REPL environment for code execution tasks based on the Recursive Language Models paradigm with sandboxed execution and context loading.
      
        📄 Docs
      
    
    
      TB2
      OpenEnv wrapper for Terminal-Bench 2 tasks with local and Docker execution modes for terminal-based agent evaluation.
      
        📄 Docs
      
    
    
      Unity
      OpenEnv wrapper for Unity ML-Agents environments, providing access to Unity's RL environments through HTTP/WebSocket interfaces.
      
        📄 Docs
      
    
    
      Wildfire
      Autonomous wildfire-control simulation where agents contain spreading fires using water, firebreaks, and timing under dynamic conditions.
      
        📄 Docs
      
    
    
      Agent World Model
      AgentWorldModel-1K — 1,000 synthetic MCP tool-use environments with 10,000 tasks for large-scale agentic RL training.
      
        📄 Docs
        🤗 HF
      
    
    
      OpenCode
      opencode_env runs the OpenCode coding agent inside an isolated E2B sandbox against any OpenAI-compatible LLM endpoint, optionally capturing per-token logprobs.
      
        📄 Docs
      
    
    
      Pelican SVG
      pelican_svg_env scores blind SVG drawings of an animal riding a vehicle in three layers: a source gate against cheats, deterministic geometry checks and a vision judge. 30 subject-vehicle tasks, with eval and GRPO training examples included.
      
        📄 Docs
        🤗 HF
      
    
    
      Pi
      pi_env runs the Pi coding agent inside an isolated Hugging Face sandbox against any OpenAI-compatible LLM endpoint, optionally capturing per-token logprobs.
      
        📄 Docs
      
    
    
      Sophistry Bench Sprint
      sophistry_bench_sprint_env is a single-turn advocacy reward-hacking environment on QuALITY passages: the policy defends an assigned answer and the reward proxy peaks at 8 &lt;claim&gt; tags, with four weight-0 canaries that detect format hacking.
      
        📄 Docs
      
    
  

> [!TIP]
> Want to publish your own environment? Head over to the [Build Your Own Environment](getting_started/environment-builder) guide for a step-by-step walkthrough.

## Community Environments

  
    
      RLVE Gym
      A suite of 400 environments that procedurally generate reasoning problems for LM training with configurable difficulty.
      
        📄 Docs
        🤗 HF
      
    
    
      Reasoning Core
      Formally verifiable symbolic reasoning tasks across logic, mathematics, planning, syntax, and related procedural domains.
      
        📄 Docs
        🤗 HF
