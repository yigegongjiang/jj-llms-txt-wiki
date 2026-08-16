# CLI

The `openenv` CLI provides a set of commands for building, validating, and pushing environments to Hugging Face Spaces or a custom Docker registry. For an end-to-end tutorial on building environments with OpenEnv, see the [building an environment](../getting_started/environment-builder) guide.

## `openenv init`[[openenv.cli.commands.init]]

#### openenv.cli.commands.init[[openenv.cli.commands.init]]

```python
openenv.cli.commands.init(env_name: Annotated[str, typer.Argument(help="Name of the environment to create (snake_case, e.g., 'my_env')")], output_dir: Annotated[str | None, typer.Option('--output-dir', '-o', help='Output directory (defaults to current working directory)')] = None)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/init.py#L402)

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

#### openenv.cli.commands.build[[openenv.cli.commands.build]]

```python
openenv.cli.commands.build(env_path: Annotated[str | None, typer.Argument(help='Path to the environment directory (default: current directory)')] = None, tag: Annotated[str | None, typer.Option('--tag', '-t', help='Docker image tag (default: openenv-<env_name>)')] = None, context: Annotated[str | None, typer.Option('--context', '-c', help='Build context path (default: <env_path>/server)')] = None, dockerfile: Annotated[str | None, typer.Option('--dockerfile', '-f', help='Path to Dockerfile (default: <context>/Dockerfile)')] = None, no_cache: Annotated[bool, typer.Option('--no-cache', help='Build without using cache')] = False, build_arg: Annotated[list[str] | None, typer.Option('--build-arg', help='Build arguments (can be used multiple times, format: KEY=VALUE)')] = None)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/build.py#L358)

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

#### openenv.cli.commands.validate[[openenv.cli.commands.validate]]

```python
openenv.cli.commands.validate(target: Annotated = None, url: Annotated = None, json_output: Annotated = False, timeout: Annotated = 5.0, verbose: Annotated = False)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/validate.py#L30)

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

#### openenv.cli.commands.push[[openenv.cli.commands.push]]

```python
openenv.cli.commands.push(directory: Annotated[str | None, typer.Argument(help='Directory containing the OpenEnv environment (default: current directory)')] = None, repo_id: Annotated[str | None, typer.Option('--repo-id', '-r', help="Repository ID as 'repo_name' or 'namespace/repo_name'. Defaults to 'username/env-name' from openenv.yaml.")] = None, base_image: Annotated[str | None, typer.Option('--base-image', '-b', help='Base Docker image to use (overrides Dockerfile FROM)')] = None, interface: Annotated[bool, typer.Option('--interface', help='Enable web interface (default: True if no registry specified)')] = None, no_interface: Annotated[bool, typer.Option('--no-interface', help='Disable web interface')] = False, registry: Annotated[str | None, typer.Option('--registry', help='Custom registry URL (e.g., docker.io/username). Disables web interface by default.')] = None, private: Annotated[bool, typer.Option('--private', help='Deploy the space as private')] = False, create_pr: Annotated[bool, typer.Option('--create-pr', help='Create a Pull Request instead of pushing to the default branch')] = False, exclude: Annotated[str | None, typer.Option('--exclude', help='Optional additional ignore file with newline-separated glob patterns to exclude from Hugging Face uploads')] = None, hardware: Annotated[str | None, typer.Option('--hardware', '-H', help='Request hardware for Hugging Face Space (e.g. t4-medium, cpu-basic). See HF docs for options.')] = None, count: Annotated[int, typer.Option('--count', '-n', help='Number of Space instances to deploy. Each gets a numeric suffix (e.g. env-1, env-2).', min=1)] = 1, env_vars: Annotated[list[str] | None, typer.Option('--env-var', '-e', help='Public Space variable as KEY=VALUE (repeatable). Overrides matching keys from openenv.yaml variables:.')] = None, secrets: Annotated[list[str] | None, typer.Option('--secret', help='Private Space secret as KEY=VALUE (repeatable). Value is never logged.')] = None)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/push.py#L510)

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

#### openenv.cli.commands.serve[[openenv.cli.commands.serve]]

```python
openenv.cli.commands.serve(env_path: Annotated[str | None, typer.Argument(help='Path to the environment directory (default: current directory)')] = None, port: Annotated[int, typer.Option('--port', '-p', help='Port to serve on')] = 8000, host: Annotated[str, typer.Option('--host', help='Host to bind to')] = '0.0.0.0', reload: Annotated[bool, typer.Option('--reload', help='Enable auto-reload on code changes')] = False)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/serve.py#L17)

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

#### openenv.cli.commands.fork[[openenv.cli.commands.fork]]

```python
openenv.cli.commands.fork(source_space: Annotated[str, typer.Argument(help="Source Space ID in format 'owner/space-name' (e.g. org/my-openenv-space)")], repo_id: Annotated[str | None, typer.Option('--repo-id', '-r', help='Target repo ID for the fork (default: created under your account with same name)')] = None, private: Annotated[bool, typer.Option('--private', help='Create the forked Space as private')] = False, set_env: Annotated[list[str], typer.Option('--set-env', '-e', help='Set Space variable (public). Can be repeated. Format: KEY=VALUE')] = [], set_secret: Annotated[list[str], typer.Option('--set-secret', '--secret', '-s', help='Set Space secret. Can be repeated. Format: KEY=VALUE')] = [], hardware: Annotated[str | None, typer.Option('--hardware', '-H', help='Request hardware (e.g. t4-medium, cpu-basic). See Hub docs for options.')] = None)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/fork.py#L58)

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

#### openenv.cli.commands.skills.skills_add[[openenv.cli.commands.skills.skills_add]]

```python
openenv.cli.commands.skills.skills_add(claude: Annotated[bool, typer.Option('--claude', help='Install for Claude.')] = False, codex: Annotated[bool, typer.Option('--codex', help='Install for Codex.')] = False, cursor: Annotated[bool, typer.Option('--cursor', help='Install for Cursor.')] = False, opencode: Annotated[bool, typer.Option('--opencode', help='Install for OpenCode.')] = False, global_: Annotated[bool, typer.Option('--global', '-g', help='Install globally (user-level) instead of in the current project directory.')] = False, dest: Annotated[Path | None, typer.Option(help='Install into a custom destination (skills directory path).')] = None, force: Annotated[bool, typer.Option('--force', help='Overwrite existing skills in the destination.')] = False)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/skills.py#L143)

Install OpenEnv CLI skill for AI assistants.

#### openenv.cli.commands.skills.skills_preview[[openenv.cli.commands.skills.skills_preview]]

```python
openenv.cli.commands.skills.skills_preview()
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/commands/skills.py#L137)

Print generated SKILL.md content.

# API Reference

## Entry point[[openenv.cli.__main__.main]]

#### openenv.cli.__main__.main[[openenv.cli.__main__.main]]

```python
openenv.cli.__main__.main()
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/__main__.py#L66)

Main entry point for the CLI.

## CLI helpers[[openenv.cli._cli_utils.validate_env_structure]]

#### openenv.cli._cli_utils.validate_env_structure[[openenv.cli._cli_utils.validate_env_structure]]

```python
openenv.cli._cli_utils.validate_env_structure(env_dir: Path, strict: bool = False)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_cli_utils.py#L30)

**Parameters:**

env_dir (`Path`) : Path to the environment directory.

strict (`bool`, *optional*, defaults to `False`) : If `True`, enforce all optional requirements.

**Returns:**

`list` of validation warnings (empty if all checks pass).

**Raises:** ``FileNotFoundError``

- ``FileNotFoundError`` -- If required files are missing.

Validate that the directory follows OpenEnv environment structure.

## Validation utilities[[openenv.cli._validation.validate_running_environment]]

#### openenv.cli._validation.validate_running_environment[[openenv.cli._validation.validate_running_environment]]

```python
openenv.cli._validation.validate_running_environment(base_url: str, timeout_s: float = 5.0)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_validation.py#L99)

Validate a running OpenEnv server against runtime API standards.

The returned JSON report contains an overall pass/fail result and
per-criterion outcomes that can be consumed in CI.

#### openenv.cli._validation.validate_multi_mode_deployment[[openenv.cli._validation.validate_multi_mode_deployment]]

```python
openenv.cli._validation.validate_multi_mode_deployment(env_path: Path)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_validation.py#L505)

**Returns:**

`tuple` of `(is_valid, issues)` where `is_valid` is a `bool` and `issues` is a
`list` of issue strings found during validation.

Validate that an environment is ready for multi-mode deployment.

Checks:
1. pyproject.toml exists
2. uv.lock exists
3. pyproject.toml has [project.scripts] with server entry point
4. server/app.py has a main() function
5. Required dependencies are present

#### openenv.cli._validation.get_deployment_modes[[openenv.cli._validation.get_deployment_modes]]

```python
openenv.cli._validation.get_deployment_modes(env_path: Path)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_validation.py#L581)

**Returns:**

`dict` mapping deployment mode names to whether they are supported.

Check which deployment modes are supported by the environment.

#### openenv.cli._validation.format_validation_report[[openenv.cli._validation.format_validation_report]]

```python
openenv.cli._validation.format_validation_report(env_name: str, is_valid: bool, issues: list)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_validation.py#L610)

**Returns:** `str`

formatted validation report.

Format a validation report for display.

#### openenv.cli._validation.build_local_validation_json_report[[openenv.cli._validation.build_local_validation_json_report]]

```python
openenv.cli._validation.build_local_validation_json_report(env_name: str, env_path: Path, is_valid: bool, issues: list, deployment_modes: dict[str, bool] | None = None)
```

[Source](https://github.com/huggingface/openenv/blob/main/openenv/cli/_validation.py#L628)

Build a JSON report for local environment validation.
