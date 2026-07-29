# Installation

You can install Trackio either from PyPI or from source:

## PyPI

Install the library with pip or [uv](https://docs.astral.sh/uv/):

uv is a fast Rust-based Python package and project manager. Refer to [Installation](https://docs.astral.sh/uv/getting-started/installation/) for installation instructions.

```bash
uv pip install trackio
```

```bash
pip install trackio
```

## Source

You can also install the latest version from source. First clone the repo and then run the installation with `pip`:

```bash
git clone https://github.com/gradio-app/trackio.git
cd trackio/
```

```sh
uv pip install .
```

```sh
pip install .
```

If you want the development install you can replace the pip install with the following:

```sh
uv pip install -e .
```

```sh
pip install -e .
```

## Optional Dependencies

Trackio has optional dependencies for additional features:

**GPU Monitoring (NVIDIA)** - For logging NVIDIA GPU metrics (utilization, memory, temperature, etc.):

```bash
pip install trackio[gpu]
```

**System Monitoring (Apple Silicon)** - For logging CPU, memory, and system metrics on Apple M-series Macs:

```bash
pip install trackio[apple-gpu]
```

**TensorBoard Import** - For importing TensorBoard event files:

```bash
pip install trackio[tensorboard]
```
