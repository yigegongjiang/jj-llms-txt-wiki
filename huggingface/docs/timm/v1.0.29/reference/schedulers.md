# Learning Rate Schedulers

This page contains the API reference documentation for learning rate schedulers included in `timm`.

## Schedulers

### Factory functions[[timm.scheduler.create_scheduler]]

#### timm.scheduler.create_scheduler[[timm.scheduler.create_scheduler]]

```python
timm.scheduler.create_scheduler(args, optimizer: Optimizer, updates_per_epoch: int = 0)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/scheduler_factory.py#L51)

#### timm.scheduler.create_scheduler_v2[[timm.scheduler.create_scheduler_v2]]

```python
timm.scheduler.create_scheduler_v2(optimizer: Optimizer, sched: str = 'cosine', num_epochs: int = 300, decay_epochs: int = 90, decay_milestones: typing.List[int] = (90, 180, 270), cooldown_epochs: int = 0, patience_epochs: int = 10, decay_rate: float = 0.1, min_lr: float = 0.0, warmup_lr: float = 1e-05, warmup_epochs: int = 0, warmup_prefix: bool = False, noise: typing.Union[float, typing.List[float]] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, cycle_mul: float = 1.0, cycle_decay: float = 0.1, cycle_limit: int = 1, k_decay: float = 1.0, plateau_mode: str = 'max', step_on_epochs: bool = True, updates_per_epoch: int = 0)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/scheduler_factory.py#L63)

### Scheduler Classes[[timm.scheduler.CosineLRScheduler]]

#### timm.scheduler.CosineLRScheduler[[timm.scheduler.CosineLRScheduler]]

```python
timm.scheduler.CosineLRScheduler(optimizer: Optimizer, t_initial: int, lr_min: float = 0.0, cycle_mul: float = 1.0, cycle_decay: float = 1.0, cycle_limit: int = 1, warmup_t: int = 0, warmup_lr_init: float = 0.0, warmup_prefix: bool = False, t_in_epochs: bool = True, noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, k_decay: float = 1.0, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/cosine_lr.py#L19)

Cosine decay with restarts.
This is described in the paper https://arxiv.org/abs/1608.03983.

Inspiration from
https://github.com/allenai/allennlp/blob/master/allennlp/training/learning_rate_schedulers/cosine.py

k-decay option based on `k-decay: A New Method For Learning Rate Schedule` - https://arxiv.org/abs/2004.05909

#### timm.scheduler.MultiStepLRScheduler[[timm.scheduler.MultiStepLRScheduler]]

```python
timm.scheduler.MultiStepLRScheduler(optimizer: Optimizer, decay_t: typing.List[int], decay_rate: float = 1.0, warmup_t: int = 0, warmup_lr_init: float = 0.0, warmup_prefix: bool = True, t_in_epochs: bool = True, noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/multistep_lr.py#L10)

#### timm.scheduler.PlateauLRScheduler[[timm.scheduler.PlateauLRScheduler]]

```python
timm.scheduler.PlateauLRScheduler(optimizer: Optimizer, decay_rate: float = 0.1, patience_t: int = 10, threshold: float = 0.0001, cooldown_t: int = 0, warmup_t: int = 0, warmup_lr_init: float = 0.0, lr_min: float = 0.0, mode: str = 'max', noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_type: str = 'normal', noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: typing.Optional[int] = None, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/plateau_lr.py#L13)

Decay the LR by a factor every time the validation loss plateaus.

#### timm.scheduler.PolyLRScheduler[[timm.scheduler.PolyLRScheduler]]

```python
timm.scheduler.PolyLRScheduler(optimizer: Optimizer, t_initial: int, power: float = 0.5, lr_min: float = 0.0, cycle_mul: float = 1.0, cycle_decay: float = 1.0, cycle_limit: int = 1, warmup_t: int = 0, warmup_lr_init: float = 0.0, warmup_prefix: bool = False, t_in_epochs: bool = True, noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, k_decay: float = 1.0, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/poly_lr.py#L19)

Polynomial LR Scheduler w/ warmup, noise, and k-decay

k-decay option based on `k-decay: A New Method For Learning Rate Schedule` - https://arxiv.org/abs/2004.05909

#### timm.scheduler.StepLRScheduler[[timm.scheduler.StepLRScheduler]]

```python
timm.scheduler.StepLRScheduler(optimizer: Optimizer, decay_t: float, decay_rate: float = 1.0, warmup_t: int = 0, warmup_lr_init: float = 0.0, warmup_prefix: bool = True, t_in_epochs: bool = True, noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/step_lr.py#L15)

#### timm.scheduler.TanhLRScheduler[[timm.scheduler.TanhLRScheduler]]

```python
timm.scheduler.TanhLRScheduler(optimizer: Optimizer, t_initial: int, lb: float = -7.0, ub: float = 3.0, lr_min: float = 0.0, cycle_mul: float = 1.0, cycle_decay: float = 1.0, cycle_limit: int = 1, warmup_t: int = 0, warmup_lr_init: float = 0.0, warmup_prefix: bool = False, t_in_epochs: bool = True, noise_range_t: typing.Union[typing.List[int], typing.Tuple[int, int], int, NoneType] = None, noise_pct: float = 0.67, noise_std: float = 1.0, noise_seed: int = 42, initialize: bool = True)
```

[Source](https://github.com/huggingface/pytorch-image-models/blob/v1.0.29/timm/scheduler/tanh_lr.py#L19)

Hyberbolic-Tangent decay with restarts.
This is described in the paper https://arxiv.org/abs/1806.01593
