# Installation

🤗 Optimum can be installed using `pip` as follows:

```bash
python -m pip install optimum
```

If you'd like to use the accelerator-specific features of 🤗 Optimum, you can install the required dependencies according to the table below:

| Accelerator                                                                                                            | Installation                                                      |
|:-----------------------------------------------------------------------------------------------------------------------|:------------------------------------------------------------------|
| [ONNX Runtime](https://huggingface.co/docs/optimum/onnxruntime/overview)                                               | `pip install --upgrade --upgrade-strategy eager optimum[onnxruntime]`       |
| [OpenVINO](https://huggingface.co/docs/optimum/intel/index)                                                            | `pip install --upgrade --upgrade-strategy eager optimum[openvino]`          |
| [NVIDIA TensorRT-LLM](https://huggingface.co/docs/optimum/main/en/nvidia_overview)                                     | `docker run -it --gpus all --ipc host huggingface/optimum-nvidia`           |
| [AMD Instinct GPUs and Ryzen AI NPU](https://huggingface.co/docs/optimum/amd/index)                                    | `pip install --upgrade --upgrade-strategy eager optimum[amd]`               |
| [AWS Trainum & Inferentia](https://huggingface.co/docs/optimum-neuron/index)                                           | `pip install --upgrade --upgrade-strategy eager optimum[neuronx]`           |
| [Habana Gaudi Processor (HPU)](https://huggingface.co/docs/optimum/habana/index)                                       | `pip install --upgrade --upgrade-strategy eager optimum[habana]`            |
| [FuriosaAI](https://huggingface.co/docs/optimum/furiosa/index)                                                         | `pip install --upgrade --upgrade-strategy eager optimum[furiosa]`           |

The `--upgrade --upgrade-strategy eager` option is needed to ensure the different packages are upgraded to the latest possible version.

If you'd like to play with the examples or need the bleeding edge of the code and can't wait for a new release, you can install the base library from source as follows:

```bash
python -m pip install git+https://github.com/huggingface/optimum.git
```

For the accelerator-specific features, you can install them by appending `optimum[accelerator_type]` to the `pip` command, e.g.

```bash
python -m pip install optimum[onnxruntime]@git+https://github.com/huggingface/optimum.git
```
