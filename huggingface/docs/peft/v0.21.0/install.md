# Installation

Before you start, you will need to setup your environment, install the appropriate packages, and configure 🤗 PEFT. 🤗 PEFT is tested on **Python 3.9+**.

🤗 PEFT is available on PyPI, as well as GitHub:

## PyPI

To install 🤗 PEFT from PyPI:

```bash
pip install peft
```

## CUDA Windows Arm64

To install PEFT with PyTorch CUDA on Windows Arm64, such as for NVIDIA RTX Spark laptops, install PyTorch from the NVIDIA PyPI index. These devices require NVIDIA's Windows Arm64 builds of PyTorch, which are not available on the default PyPI index or the standard PyTorch wheel index.

Run the command below to check if your system detects an NVIDIA GPU.

```bash
nvidia-smi
```

Install PyTorch from the NVIDIA PyPI index, then install PEFT.

```bash
pip install torch --index-url https://pypi.nvidia.com/nvtorch_oot_nightly
pip install peft
```

## Source

New features that haven't been released yet are added every day, which also means there may be some bugs. To try them out, install from the GitHub repository:

```bash
pip install git+https://github.com/huggingface/peft
```

If you wish to play with the source code and see live results as you run the code, an editable version
can be installed from a locally-cloned version of the repository:

```bash
git clone https://github.com/huggingface/peft
cd peft
pip install -e ".[test]"
```

If you're planning to contribute to PEFT, follow the [contributing guide](developer_guides/contributing#installation) instead, which also covers forking, adding the upstream remote, and creating a working branch.
