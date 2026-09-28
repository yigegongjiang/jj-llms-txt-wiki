# Expert parallelism

[Expert parallelism](https://huggingface.co/spaces/nanotron/ultrascale-playbook?section=expert_parallelism) is a parallelism strategy for [mixture-of-experts (MoE) models](https://huggingface.co/blog/moe). Each expert's feedforward layer lives on a different hardware accelerator. A router dispatches tokens to the appropriate experts and gathers the results. This approach scales models to far larger parameter counts without increasing computation cost because each token activates only a few experts.

## DistributedConfig[[transformers.DistributedConfig]]

Enable expert parallelism with the [DistributedConfig](/docs/transformers/v5.17.0/en/expert_parallelism#transformers.DistributedConfig) class and the `enable_expert_parallel` argument.

```py
import os

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from transformers.distributed.configuration_utils import DistributedConfig

distributed_config = DistributedConfig(
    tp_size=int(os.environ["WORLD_SIZE"]),
    enable_expert_parallel=True,
)

model = AutoModelForCausalLM.from_pretrained(
    "openai/gpt-oss-120b",
    distributed_config=distributed_config,
)
```

> [!TIP]
> Expert parallelism automatically enables [tensor parallelism](./perf_infer_gpu_multi) for attention layers.

This argument switches to the `ep_plan` (expert parallel plan) defined in each MoE model's config file. The `GroupedGemmParallel` class splits expert weights so each device loads only its local experts. The `ep_router` routes tokens to experts and an all-reduce operation combines their outputs.

Launch your inference script with [torchrun](https://pytorch.org/docs/stable/elastic/run.html) and specify how many devices to use. The number of devices must evenly divide the total number of experts.

```zsh
torchrun --nproc-per-node 8 your_script.py
```

#### transformers.DistributedConfig[[transformers.DistributedConfig]]

```python
transformers.DistributedConfig(tp_size: int | None = None, tp_plan: typing.Union[dict[str, str], typing.Literal['auto'], NoneType] = None, enable_sequence_parallel: bool = False, enable_expert_parallel: bool = False, fsdp_size: int | None = None, fsdp_cpu_offload: bool = False, fsdp_mixed_precision: bool = False, pp_size: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/distributed/configuration_utils.py#L22)

**Parameters:**

tp_size (*int*, *optional*) : Number of devices for tensor parallelism. If *None* and *tp_plan* is set, defaults to *WORLD_SIZE // (other_parallel_size)*. If *None* and no *tp_plan* is set, defaults to 1.

tp_plan (*dict[str, str]* or *"auto"*, *optional*) : Tensor parallel sharding plan. Pass *"auto"*, or leave as *None* when *tp_size* is set, to use the model's predefined *base_model_tp_plan*. Pass a dictionary to override the predefined plan.

enable_sequence_parallel (*bool*, *optional*, defaults to *False*) : Reserved for sequence parallelism. Not wired up yet.

enable_expert_parallel (*bool*, *optional*, defaults to *False*) : Route MoE models through the expert-parallel path (`base_model_ep_plan`).

fsdp_size (*int*, *optional*) : Number of devices for FSDP (data parallelism). If *None* and *tp_size* is set, defaults to 1.

fsdp_cpu_offload (*bool*, *optional*, defaults to *False*) : Whether to enable CPU offloading for FSDP2.

fsdp_mixed_precision (*bool*, *optional*, defaults to *False*) : Whether to enable mixed precision for FSDP2.

pp_size (*int*, *optional*) : Number of devices for pipeline parallelism. If *None* and another parallel mode is set, defaults to 1.

Configuration for native distributed inference and training with tensor, pipeline, or FSDP2 parallelism.
