# Using TEI locally with Metal

You can install `text-embeddings-inference` locally to run it on your own Mac with Metal support.

## Homebrew (Apple Silicon)

On Apple Silicon (M1/M2/M3/M4), you can install a prebuilt binary via Homebrew:

```shell
brew install text-embeddings-inference
```

Then launch Text Embeddings Inference:

```shell
model=Qwen/Qwen3-Embedding-0.6B

text-embeddings-router --model-id $model --port 8080
```

## Build from source

Alternatively, you can build from source. Here are the step-by-step instructions:

## Step 1: Install Rust

[Install Rust](https://rustup.rs/) on your machine by run the following in your terminal, then following the instructions:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Step 2: Install with Metal support

```shell
cargo install --path router -F metal
```

## Step 3: Launch Text Embeddings Inference

Once the installation is successfully complete, you can launch Text Embeddings Inference with Metal with the following command:

```shell
model=Qwen/Qwen3-Embedding-0.6B

text-embeddings-router --model-id $model --port 8080
```

Now you are ready to use `text-embeddings-inference` locally on your machine.
