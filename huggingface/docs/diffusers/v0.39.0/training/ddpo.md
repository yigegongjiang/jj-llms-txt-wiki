# Reinforcement learning training with DDPO

You can fine-tune Stable Diffusion on a reward function via reinforcement learning with the 🤗 TRL library and 🤗 Diffusers. This is done with the Denoising Diffusion Policy Optimization (DDPO) algorithm introduced by Black et al. in [Training Diffusion Models with Reinforcement Learning](https://huggingface.co/papers/2305.13301), which is implemented in 🤗 TRL with the `DDPOTrainer`.

For more information, check out the `DDPOTrainer` API reference and the [Finetune Stable Diffusion Models with DDPO via TRL](https://huggingface.co/blog/trl-ddpo) blog post.
