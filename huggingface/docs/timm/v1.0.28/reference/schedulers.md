# Learning Rate Schedulers

This page contains the API reference documentation for learning rate schedulers included in `timm`.

## Schedulers

### Factory functions[[timm.scheduler.create_scheduler]]

### Scheduler Classes[[timm.scheduler.CosineLRScheduler]]

Cosine decay with restarts.
This is described in the paper https://arxiv.org/abs/1608.03983.

Inspiration from
https://github.com/allenai/allennlp/blob/master/allennlp/training/learning_rate_schedulers/cosine.py

k-decay option based on `k-decay: A New Method For Learning Rate Schedule` - https://arxiv.org/abs/2004.05909

Decay the LR by a factor every time the validation loss plateaus.

Polynomial LR Scheduler w/ warmup, noise, and k-decay

k-decay option based on `k-decay: A New Method For Learning Rate Schedule` - https://arxiv.org/abs/2004.05909

Hyberbolic-Tangent decay with restarts.
This is described in the paper https://arxiv.org/abs/1806.01593
