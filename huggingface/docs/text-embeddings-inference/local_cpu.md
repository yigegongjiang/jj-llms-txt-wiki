# Using TEI locally with CPU

You can install `text-embeddings-inference` locally to run it on your own machine. Here are the step-by-step instructions for installation:

## Step 1: Install Rust

[Install Rust](https://rustup.rs/) on your machine by run the following in your terminal, then following the instructions:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Step 2: Install necessary packages

Depending on your machine's architecture, run one of the following commands:

### For x86 Machines

```shell
cargo install --path router -F mkl
```

### For M1 or M2 Machines

```shell
cargo install --path router -F metal
```

## Step 3: Launch Text Embeddings Inference

Once the installation is successfully complete, you can launch Text Embeddings Inference on CPU with the following command:

```shell
model=Qwen/Qwen3-Embedding-0.6B

text-embeddings-router --model-id $model --port 8080
```

In some cases, you might also need the OpenSSL libraries and gcc installed. On Linux machines, run the following command:

```shell
sudo apt-get install libssl-dev gcc -y
```

Now you are ready to use `text-embeddings-inference` locally on your machine.
If you want to run TEI locally with a GPU, check out the [Using TEI locally with GPU](local_gpu) page.
