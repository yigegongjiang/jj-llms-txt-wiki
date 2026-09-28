# transformers

Public API of `@huggingface/transformers`. Everything re-exported from
this file is considered stable — other imports are internal and may change.

**Start here**
- [`pipeline()`](./pipelines#module_pipelines.pipeline) — the one-call entry point for every task.
- [Environment](./env) — `env` fields and `LogLevel` enum.

**Model loading**
- [Pipelines](./pipelines) — task-specific pipeline classes (`TextGenerationPipeline`, etc.).
- [Models](./models) — `AutoModel*` classes (one per task).
- [Tokenizers](./tokenizers) — `AutoTokenizer`, chat templates, `Message`.
- [Processors](./processors) — `AutoProcessor`, `AutoImageProcessor`, `AutoFeatureExtractor`.
- [Configs](./configs) — `AutoConfig` / `PretrainedConfig`.

**Generation**
- [Generation config](./generation/configuration_utils) — sampling and beam-search parameters.
- [Generation parameters](./generation/parameters) — full shape of `generate()` arguments.
- [Logits processors](./generation/logits_process) — modify next-token probabilities.
- [Stopping criteria](./generation/stopping_criteria) — control when generation halts.
- [Streamers](./generation/streamers) — receive tokens as they're produced.

**Data types and I/O**
- [Tensors](./utils/tensor) — `Tensor`, shape ops, math, I/O.
- [Images](./utils/image) — `RawImage`, `load_image()`.
- [Audio](./utils/audio) — `RawAudio`, `load_audio()`.
- [Video](./utils/video) — `RawVideo`, `RawVideoFrame`, `load_video()` _(experimental)_.

**Utilities**
- [Hub options](./utils/hub) — shared `from_pretrained()` option shapes.
- [Maths](./utils/maths) — `softmax`, `cos_sim`, typed-array helpers.
- [Model registry](./utils/model_registry) — inspect or clear the model cache.
- [Random](./utils/random) — seedable MT19937 PRNG matching Python's `random`.

## Type Definitions

### DeviceType

_Type:_ `keyof typeof DEVICE_TYPES`

### DataType

_Type:_ `keyof typeof DATA_TYPES`
