# OpenEnv: Agentic Execution Environments

  
    A unified framework for building, deploying, and interacting with isolated execution environments for agentic reinforcement learning—powered by simple, Gymnasium-style APIs.
  

Training RL agents—especially in agentic settings like code generation, web browsing, or game playing—requires environments that are:

  
    
      Gymnasium-Style APIs
      Familiar step(), reset(), and state() interface for seamless integration with existing RL frameworks.
    
    
      Container-First Design
      Package environments as containers for consistent, reproducible deployments across any infrastructure.
    
    
      HTTP-Native
      Deploy environments as HTTP services for distributed training and remote execution.
    
    
      Secure Isolation
      Run untrusted agent code safely with sandboxed execution environments.
    
    
      Rich Environment Library
      Pre-built environments for games, coding, web browsing, and more.
    
    
      CLI Tools
      Powerful command-line interface for environment management and deployment.
    
  

## Getting Started

New to OpenEnv? Follow our recommended learning path:

1. **[Getting Started Series](tutorials/index)** — A 5-part series covering what OpenEnv is, how to use and build environments, and how to contribute. No GPU required.

2. **[Build Your Own Environment](getting_started/environment-builder)** — The complete reference guide for creating, packaging, and deploying custom environments with Docker and Hugging Face Hub.

3. **[Simulation vs Production Mode](guides/simulation-vs-production)** — Understand when to use the training loop, when to expose MCP directly, and how tools behave in each mode.

4. **[MCP Environment Lifecycle](guides/mcp-environment-lifecycle)** — Understand how MCP tools fit into the OpenEnv step loop, when `step_async()` is used, and when to use `call_tool()` versus `step(...)`.

5. **[Explore Environments](environments)** — Browse pre-built environments for games, coding, web browsing, and more.

## How Can I Contribute?

We welcome contributions from the community! OpenEnv is openly governed by a technical committee that includes Meta-PyTorch, Reflection, Unsloth, Modal, Prime Intellect, Nvidia, Mercor, Fleet AI, Microsoft, Hugging Face, and RadixArk. The committee coordinates project direction, major technical decisions, RFCs, and release planning through the public repository.

If you find a bug, have a feature request, or want to contribute a new environment, please open an issue or submit a pull request. The repository is hosted on GitHub at [huggingface/OpenEnv](https://github.com/huggingface/OpenEnv). For the changelog, see [GitHub Releases](https://github.com/huggingface/OpenEnv/releases).

> [!WARNING]
> OpenEnv is currently in an experimental stage. You should expect bugs, incomplete features, and APIs that may change in future versions. The project welcomes bug fixes, but significant changes should be discussed before implementation so the technical committee and community can coordinate scope, compatibility, and release timing. Signal your intention to contribute in the issue tracker by filing a new issue or claiming an existing one.
