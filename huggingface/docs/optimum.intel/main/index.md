# 🤗 Optimum Intel

🤗 Optimum Intel is the interface between the 🤗 Transformers, Diffusers, Sentence Transformers and timm libraries and the different tools and libraries provided by [OpenVINO](https://docs.openvino.ai) to accelerate end-to-end pipelines on Intel architectures.

[OpenVINO](https://docs.openvino.ai) is an open-source toolkit that enables high performance inference capabilities for Intel CPUs, GPUs, and special DL inference accelerators ([see](https://docs.openvino.ai/2024/about-openvino/compatibility-and-support/supported-devices.html) the full list of supported devices). It is supplied with a set of tools to optimize your models with compression techniques such as quantization, pruning and knowledge distillation. Optimum Intel provides a simple interface to optimize your Transformers and Diffusers models, convert them to the OpenVINO Intermediate Representation (IR) format and run inference using OpenVINO Runtime.
