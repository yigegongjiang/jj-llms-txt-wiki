# CLI

The `openenv` CLI provides a set of commands for building, validating, and pushing environments to Hugging Face Spaces or a custom Docker registry. For an end-to-end tutorial on building environments with OpenEnv, see the [building an environment](../getting_started/environment-builder) guide.

## `openenv init`[[openenv.cli.commands.init]]

Initialize a new OpenEnv environment.

Creates a new directory with the environment name and generates all necessary
files based on the OpenEnv template structure.

Examples:

```bash
$ openenv init my_game_env
$ openenv init my_env --output-dir /path/to/projects
```

## `openenv import`

Import a supported third-party source environment into a generated OpenEnv
wrapper package. The command detects the source format from the directory
contents, so ORS/OpenReward and Prime Intellect Verifiers sources do not
require `--type` in the common case.

The generated wrapper vendors the source tree into the package and includes
vendored files as package data, so non-secret fixture/data files are available to
the environment server at runtime. The importer carries portable dependencies
from source `pyproject.toml` and `requirements.txt` files into the generated
environment, skips VCS/cache/build directories and common secret file patterns
such as `.env`, `secrets.yaml`, and private key files, and excludes compiled
binary artifacts; review the generated `vendor/` directory before publishing a
wrapper.

```bash
openenv import path/to/source --name my_env --output-dir ./envs
openenv import path/to/source --name my_env --output-dir ./envs --env-class MyEnv
```

```{eval-rst}
.. automodule:: openenv.cli.commands.import_env
   :members:
   :undoc-members:
   :show-inheritance:
```

## `openenv build`[[openenv.cli.commands.build]]

)')] = None"}, {"name": "context", "val": ": Annotated[str | None, typer.Option('--context', '-c', help='Build context path (default: /server)')] = None"}, {"name": "dockerfile", "val": ": Annotated[str | None, typer.Option('--dockerfile', '-f', help='Path to Dockerfile (default: /Dockerfile)')] = None"}, {"name": "no_cache", "val": ": Annotated[bool, typer.Option('--no-cache', help='Build without using cache')] = False"}, {"name": "build_arg", "val": ": Annotated[list[str] | None, typer.Option('--build-arg', help='Build arguments (can be used multiple times, format: KEY=VALUE)')] = None"}]}>

Build Docker images for OpenEnv environments.

This command builds Docker images using the environment's pyproject.toml
and uv for dependency management. Run from the environment root directory.

Examples:

```bash
# Build from environment root (recommended)
$ cd my_env
$ openenv build

# Build with custom tag
$ openenv build -t my-custom-tag

# Build without cache
$ openenv build --no-cache

# Build with custom build arguments
$ openenv build --build-arg VERSION=1.0 --build-arg ENV=prod

# Build from different directory
$ openenv build envs/echo_env
```

## `openenv validate`[[openenv.cli.commands.validate]]

Validate local environments and running OpenEnv servers.

Local validation checks if an environment is properly configured with:
- Required files (pyproject.toml, openenv.yaml, server/app.py, etc.)
- Docker deployment support
- uv run server capability
- python -m module execution

Runtime validation checks if a live OpenEnv server conforms to the
versioned runtime API contract and returns a criteria-based JSON report.

Examples:

```bash
# Validate current directory (recommended)
$ cd my_env
$ openenv validate

# Validate a running environment and return JSON criteria
$ openenv validate --url http://localhost:8000
$ openenv validate https://my-env.hf.space

# Validate with detailed output
$ openenv validate --verbose

# Validate specific environment
$ openenv validate envs/echo_env
```

## `openenv push`[[openenv.cli.commands.push]]

Push an OpenEnv environment to Hugging Face Spaces or a custom Docker registry.

This command:
1. Validates that the directory is an OpenEnv environment (openenv.yaml present)
2. Builds and pushes to Hugging Face Spaces or custom Docker registry
3. Optionally enables web interface for deployment

The web interface is enabled by default when pushing to HuggingFace Spaces,
but disabled by default when pushing to a custom Docker registry.

Examples:

```bash
# Push to HuggingFace Spaces from current directory (web interface enabled)
$ cd my_env
$ openenv push

# Push to HuggingFace repo and open a Pull Request
$ openenv push my-org/my-env --create-pr
$ openenv push --repo-id my-org/my-env --create-pr

# Push to HuggingFace without web interface
$ openenv push --no-interface

# Push to Docker Hub
$ openenv push --registry docker.io/myuser

# Push to GitHub Container Registry
$ openenv push --registry ghcr.io/myorg

# Push to custom registry with web interface
$ openenv push --registry myregistry.io/path1/path2 --interface

# Push to specific HuggingFace repo
$ openenv push --repo-id my-org/my-env

# Push privately with custom base image
$ openenv push --private --base-image ghcr.io/huggingface/openenv-base:latest

# Push with GPU hardware
$ openenv push --hardware t4-medium

# Set a public Space variable (overrides openenv.yaml variables:)
$ openenv push -e OPENSPIEL_GAME=tic_tac_toe -e MAX_STEPS=100

# Set a private Space secret (value never logged)
$ openenv push --secret OPENAI_API_KEY=sk-...
```

## `openenv serve`[[openenv.cli.commands.serve]]

Local serving is not implemented in the CLI yet. This command exits non-zero
and prints alternative ways to run an environment server.

Serve an OpenEnv environment locally.

TODO: This command is currently not implemented and has been deferred for later.

Planned functionality:
- Run environment server locally without Docker
- Support multiple deployment modes (local, notebook, cluster)
- Auto-reload for development
- Integration with environment's [project.scripts] entry point

For now, use Docker-based serving:
1. Build the environment: openenv build
2. Run the container: docker run -p 8000:8000 

Or use uv directly:
uv run --project . server --port 8000

## `openenv fork`[[openenv.cli.commands.fork]]

Fork (duplicate) a Hugging Face Space to your account using the Hub API.

Uses the Hugging Face duplicate_space API. You can set environment variables
and secrets, and request hardware/storage/sleep time at creation time.

Examples:

```bash
$ openenv fork owner/source-space
$ openenv fork owner/source-space --private
$ openenv fork owner/source-space --repo-id myuser/my-fork
$ openenv fork owner/source-space --set-env MODEL_ID=user/model --set-secret HF_TOKEN=hf_xxx
$ openenv fork owner/source-space --hardware t4-medium
```

## `openenv skills`[[openenv.cli.commands.skills.skills_add]]

Installs an `openenv-cli` skill into your AI assistant's skills directory so
it knows the `openenv` CLI is available and what each command does. Supports
Claude Code, Cursor, Codex, and OpenCode.

**Install for a single assistant (project-local):**

```bash
openenv skills add --claude    # → .claude/skills/openenv-cli/
openenv skills add --cursor    # → .cursor/skills/openenv-cli/
openenv skills add --codex     # → .codex/skills/openenv-cli/
openenv skills add --opencode  # → .opencode/skills/openenv-cli/
```

Multiple flags can be combined — `openenv skills add --claude --cursor` installs
for both at once. The skill file is written to a central location
(`.agents/skills/openenv-cli/`) and each agent directory gets a symlink, so
there is only one copy to update.

**Install globally (user-level, across all projects):**

```bash
openenv skills add --claude --global  # → ~/.claude/skills/openenv-cli/
```

**Overwrite an existing installation** (e.g. after upgrading `openenv`):

```bash
openenv skills add --claude --force
```

**Preview the skill content without installing:**

```bash
openenv skills preview
```

**Install to a custom path** (for non-standard agent setups):

```bash
openenv skills add --dest /path/to/my-agent/skills/
```

Install OpenEnv CLI skill for AI assistants.

Print generated SKILL.md content.

# API Reference

## Entry point[[openenv.cli.__main__.main]]

Main entry point for the CLI.

## CLI helpers[[openenv.cli._cli_utils.validate_env_structure]]

- **env_dir** (`Path`) --
  Path to the environment directory.
- **strict** (`bool`, *optional*, defaults to `False`) --
  If `True`, enforce all optional requirements.`list` of validation warnings (empty if all checks pass).- ``FileNotFoundError`` -- If required files are missing.</raises><raisederrors>``FileNotFoundError``

Validate that the directory follows OpenEnv environment structure.

## Validation utilities[[openenv.cli._validation.validate_running_environment]]

Validate a running OpenEnv server against runtime API standards.

The returned JSON report contains an overall pass/fail result and
per-criterion outcomes that can be consumed in CI.

`tuple` of `(is_valid, issues)` where `is_valid` is a `bool` and `issues` is a
`list` of issue strings found during validation.

Validate that an environment is ready for multi-mode deployment.

Checks:
1. pyproject.toml exists
2. uv.lock exists
3. pyproject.toml has [project.scripts] with server entry point
4. server/app.py has a main() function
5. Required dependencies are present

`dict` mapping deployment mode names to whether they are supported.

Check which deployment modes are supported by the environment.

`str`formatted validation report.

Format a validation report for display.

Build a JSON report for local environment validation.
