# Using TEI locally with GPU

You can install `text-embeddings-inference` locally to run it on your own machine with a GPU.
To make sure that your hardware is supported, check out the [Supported models and hardware](supported_models) page.

## Step 1: CUDA and NVIDIA drivers

Make sure you have CUDA and the NVIDIA drivers installed - NVIDIA drivers on your device need to be compatible with CUDA version 12.2 or higher.

Add the NVIDIA binaries to your path:

```shell
export PATH=$PATH:/usr/local/cuda/bin
```

## Step 2: Install Rust

[Install Rust](https://rustup.rs/) on your machine by run the following in your terminal, then following the instructions:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Step 3: Install necessary packages

This step can take a while as we need to compile a lot of CUDA kernels.

### For Turing GPUs (T4, RTX 2000 series ... )

```shell
cargo install --path router -F candle-cuda-turing
```

### For Ampere, Ada Lovelace, Hopper, and Blackwell

```shell
cargo install --path router -F candle-cuda
```

## Step 4: Launch Text Embeddings Inference

You can now launch Text Embeddings Inference on GPU with:

```shell
model=Qwen/Qwen3-Embedding-0.6B

text-embeddings-router --model-id $model --dtype float16 --port 8080
```
