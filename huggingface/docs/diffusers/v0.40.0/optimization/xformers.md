# xFormers

We recommend [xFormers](https://github.com/facebookresearch/xformers) for both inference and training. In our tests, the optimizations performed in the attention blocks allow for both faster speed and reduced memory consumption.

Install xFormers from `pip`:

```bash
pip install xformers
```

> [!TIP]
> The xFormers `pip` package requires the latest version of PyTorch. If you need to use a previous version of PyTorch, then we recommend [installing xFormers from the source](https://github.com/facebookresearch/xformers#installing-xformers).

After xFormers is installed, you can use it with [set_attention_backend()](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin.set_attention_backend) as shown in the [Attention backends](./attention_backends) guide.

> [!WARNING]
> According to this [issue](https://github.com/huggingface/diffusers/issues/2234#issuecomment-1416931212), xFormers `v0.0.16` cannot be used for training (fine-tune or DreamBooth) in some GPUs. If you observe this problem, please install a development version as indicated in the issue comments.
