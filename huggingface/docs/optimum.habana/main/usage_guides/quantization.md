# Quantization

Intel® Gaudi® offers several possibilities to make inference faster. For examples of FP8 and UINT4 for Inference, see the
[text-generation](/examples/text-generation) example.

This guide provides the steps required to enable FP8 and UINT4 precision on your Intel® Gaudi® AI
accelerator using the Intel® Neural Compressor (INC) package.

## Run Inference Using FP8

When running inference on large language models (LLMs), high memory usage is often the bottleneck. Therefore
using FP8 data type for inference on large language models halves the required memory bandwidth. In addition,
FP8 compute is twice as fast as BF16 compute, so even compute-bound workloads, such as offline inference on
large batch sizes benefit.

References to [Run Inference Using FP8](https://docs.habana.ai/en/latest/PyTorch/Inference_on_PyTorch/Inference_Using_FP8.html)
section on [Intel® Gaudi® AI Accelerator Documentation](https://docs.habana.ai/en/latest/index.html).

## Run Inference Using UINT4

When running inference on large language models (LLMs), high memory usage is often the bottleneck. Therefore,
using UINT4 data type for inference on large language models halves the required memory bandwidth compared to
running inference in FP8.

References to [Run Inference Using UINT4](https://docs.habana.ai/en/latest/PyTorch/Inference_on_PyTorch/Inference_Using_UINT4.html)
section on [Intel® Gaudi® AI Accelerator Documentation](https://docs.habana.ai/en/latest/index.html).
