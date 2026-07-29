# Installation

To install the latest release of 🤗 Optimum Intel with the corresponding required dependencies, you can do respectively:

```bash
python -m pip install --upgrade-strategy eager "optimum-intel[openvino]"
```

The `--upgrade-strategy eager` option is needed to ensure `optimum-intel` is upgraded to the latest version.

We recommend creating a [virtual environment](https://packaging.python.org/en/latest/guides/installing-using-pip-and-virtual-environments/#creating-a-virtual-environment) and upgrading pip with :
```bash
python -m pip install --upgrade pip
```

Optimum Intel is a fast-moving project, and you may want to install from source with the following command:

```bash
python -m pip install "optimum-intel[openvino]"@git+https://github.com/huggingface/optimum-intel.git
```
