# Serving private and gated models

If the model you wish to serve is behind gated access or resides in a private model repository on Hugging Face Hub,
you will need to have access to the model to serve it.

Once you have confirmed that you have access to the model:

- Navigate to your account's [Profile | Settings | Access Tokens page](https://huggingface.co/settings/tokens).
- Generate and copy a read token.

If you're the CLI, set the `HF_TOKEN` environment variable. For example:

```shell
export HF_TOKEN=
```

Alternatively, you can provide the token when deploying the model with Docker:

```shell
model=
volume=$PWD/data
token=

docker run --gpus all -e HF_TOKEN=$token -p 8080:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:cuda-1.9 --model-id $model
```
