# generation/parameters

## Type Definitions

### GenerationFunctionParametersBase

**Properties**

- `inputs` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) _optional_ — defaults to `null` — The sequence used as a prompt for generation or as model inputs to the encoder. If `null`, the
  method initializes it with `bos_token_id` and a batch size of 1. For decoder-only models `inputs`
  should be in the format of `input_ids`. For encoder-decoder models *inputs* can represent any of
  `input_ids`, `input_values`, `input_features`, or `pixel_values`.
- `generation_config` ([`GenerationConfig`](./configuration_utils#module_generation/configuration_utils.GenerationConfig)) _optional_ — defaults to `null` — The generation configuration to use as the base parameters for the generation call.
  `**kwargs` passed to generate that match attributes of `generation_config` will override them.
  If `generation_config` is not provided, the default is used, with the following loading
  priority:
  - (1) from the `generation_config.json` model file, if it exists;
  - (2) from the model configuration. Unspecified parameters inherit the default values documented by `GenerationConfig`.
- `logits_processor` ([`LogitsProcessorList`](./logits_process#module_generation/logits_process.LogitsProcessorList)) _optional_ — defaults to `null` — Custom logits processors that complement the default logits processors built from arguments and
  the generation config. If a logits processor duplicates one already created from the arguments or
  generation config, an error is thrown. This feature is intended for advanced users.
- `stopping_criteria` ([`StoppingCriteria`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteria) | [`StoppingCriteria`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteria)[] | [`StoppingCriteriaList`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteriaList)) _optional_ — defaults to `null` — Custom stopping criteria that complement the default stopping criteria built from arguments and the
  generation config. If a stopping criterion duplicates one already created from the arguments or
  generation config, an error is thrown. This feature is intended for advanced users.
- `streamer` ([`BaseStreamer`](./streamers#module_generation/streamers.BaseStreamer)) _optional_ — defaults to `null` — Streamer object used to stream the generated sequences. Generated tokens are passed
  through `streamer.put(token_ids)` and the streamer is responsible for any further processing.
- `decoder_input_ids` (`number[]` | [`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) _optional_ — defaults to `null` — If the model is an encoder-decoder model, this argument is used to pass the `decoder_input_ids`.
- `past_key_values` ([`DynamicCache`](./cache#module_generation/cache.DynamicCache) | `null`) _optional_ — defaults to `null` — A cache object that stores previously computed key/value states. When provided, the model will
  use these cached states to avoid recomputing them, significantly speeding up sequential generation.

### GenerationFunctionParameters

_Type:_ [`GenerationFunctionParametersBase`](./parameters#module_generation/parameters.GenerationFunctionParametersBase) & `Partial`<[`GenerationConfig`](./configuration_utils#module_generation/configuration_utils.GenerationConfig)> & `{ [key: string]: unknown }`
