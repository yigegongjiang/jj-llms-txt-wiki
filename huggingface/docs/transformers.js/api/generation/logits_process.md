# generation/logits_process

Logits processors applied during token generation.

A `LogitsProcessor` rewrites the probability distribution over the next token —
suppressing specific ids, forcing certain tokens at the start or end,
penalising repetition, and so on. `LogitsProcessorList` composes many
processors; pass one via the `logits_processor` argument of `generate()`.

## On this page

**Classes** — [`LogitsProcessor`](#module_generation/logits_process.LogitsProcessor) · [`LogitsWarper`](#module_generation/logits_process.LogitsWarper) · [`LogitsProcessorList`](#module_generation/logits_process.LogitsProcessorList) · [`ForcedBOSTokenLogitsProcessor`](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor) · [`ForcedEOSTokenLogitsProcessor`](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor) · [`SuppressTokensLogitsProcessor`](#module_generation/logits_process.SuppressTokensLogitsProcessor) · [`SuppressTokensAtBeginLogitsProcessor`](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor) · [`WhisperTimeStampLogitsProcessor`](#module_generation/logits_process.WhisperTimeStampLogitsProcessor) · [`NoRepeatNGramLogitsProcessor`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor) · [`RepetitionPenaltyLogitsProcessor`](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor) · [`MinLengthLogitsProcessor`](#module_generation/logits_process.MinLengthLogitsProcessor) · [`MinNewTokensLengthLogitsProcessor`](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor) · [`NoBadWordsLogitsProcessor`](#module_generation/logits_process.NoBadWordsLogitsProcessor) · [`ClassifierFreeGuidanceLogitsProcessor`](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor) · [`TemperatureLogitsWarper`](#module_generation/logits_process.TemperatureLogitsWarper) · [`TopPLogitsWarper`](#module_generation/logits_process.TopPLogitsWarper) · [`TopKLogitsWarper`](#module_generation/logits_process.TopKLogitsWarper)

## Classes

### LogitsProcessor

Abstract base class for all logit processors that can be applied during generation.

#### `LogitsProcessor(input_ids, logits)`

Apply the processor to the input logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits to process.

**Throws**

- `Error` — Throws an error if `_call` is not implemented in the subclass.

### LogitsWarper

Abstract base class for all logit warpers that can be applied during generation with multinomial sampling.

#### `LogitsWarper(input_ids, logits)`

Apply the processor to the input logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits to process.

**Throws**

- `Error` — Throws an error if `_call` is not implemented in the subclass.

### LogitsProcessorList

A class representing a list of logits processors. A logits processor is a function that modifies the logits
output of a language model. This class provides methods for adding new processors and applying all processors to a
batch of logits.

#### `LogitsProcessorList(input_ids, logits)`

Applies all logits processors in the list to a batch of logits, modifying them in-place.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs for the language model.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor))

#### `LogitsProcessorList.constructor()`

Constructs a new instance of `LogitsProcessorList`.

#### `LogitsProcessorList.push(item)`

Adds a new logits processor to the list.

**Parameters**

- `item` ([`LogitsProcessor`](./logits_process#module_generation/logits_process.LogitsProcessor)) — The logits processor function to add.

#### `LogitsProcessorList.extend(items)`

Adds multiple logits processors to the list.

**Parameters**

- `items` ([`LogitsProcessor`](./logits_process#module_generation/logits_process.LogitsProcessor)[]) — The logits processor functions to add.

### ForcedBOSTokenLogitsProcessor

A LogitsProcessor that forces a BOS token at the beginning of the generated sequence.

#### `ForcedBOSTokenLogitsProcessor(input_ids, logits)`

Apply the BOS token forcing to the logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The logits with BOS token forcing.

#### `ForcedBOSTokenLogitsProcessor.constructor(bos_token_id)`

Create a ForcedBOSTokenLogitsProcessor.

**Parameters**

- `bos_token_id` (`number`) — The ID of the beginning-of-sequence token to be forced.

### ForcedEOSTokenLogitsProcessor

A logits processor that enforces the specified token as the last generated token when `max_length` is reached.

#### `ForcedEOSTokenLogitsProcessor(input_ids, logits)`

Apply the processor to input_ids and logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits tensor.

#### `ForcedEOSTokenLogitsProcessor.constructor(max_length, eos_token_id)`

Create a ForcedEOSTokenLogitsProcessor.

**Parameters**

- `max_length` (`number`) — The maximum length of the sequence to be generated.
- `eos_token_id` (`number` | `number[]`) — The ID or IDs of the *end-of-sequence* token.

### SuppressTokensLogitsProcessor

A LogitsProcessor that suppresses a list of tokens throughout generation.
Sets their log probabilities to `-inf` so that they are not generated.

#### `SuppressTokensLogitsProcessor(input_ids, logits)`

Suppress the specified tokens by setting their logits to -Infinity.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The modified logits.

#### `SuppressTokensLogitsProcessor.constructor(suppress_tokens)`

Create a SuppressTokensLogitsProcessor.

**Parameters**

- `suppress_tokens` (`number[]`) — The IDs of the tokens to suppress.

### SuppressTokensAtBeginLogitsProcessor

A LogitsProcessor that suppresses a list of tokens as soon as the `generate` function starts
generating using `begin_index` tokens. This should ensure that the tokens defined by
`begin_suppress_tokens` at not sampled at the begining of the generation.

#### `SuppressTokensAtBeginLogitsProcessor(input_ids, logits)`

Apply the BOS token forcing to the logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The logits with BOS token forcing.

#### `SuppressTokensAtBeginLogitsProcessor.constructor(begin_suppress_tokens, begin_index)`

Create a SuppressTokensAtBeginLogitsProcessor.

**Parameters**

- `begin_suppress_tokens` (`number[]`) — The IDs of the tokens to suppress.
- `begin_index` (`number`) — The number of tokens to generate before suppressing tokens.

### WhisperTimeStampLogitsProcessor

A LogitsProcessor that handles adding timestamps to generated text.

#### `WhisperTimeStampLogitsProcessor(input_ids, logits)`

Modify the logits to handle timestamp tokens.

**Parameters**

- `input_ids` (`bigint[][]`) — The input sequence of tokens.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits output by the model.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The modified logits.

#### `WhisperTimeStampLogitsProcessor.constructor(generate_config, init_tokens)`

Constructs a new WhisperTimeStampLogitsProcessor.

**Parameters**

- `generate_config` (`WhisperGenerationConfig`) — The config object passed to the `generate()` method of a transformer model.
- `init_tokens` (`number[]`) — The initial tokens of the input sequence.

### NoRepeatNGramLogitsProcessor

A logits processor that disallows repeated n-grams of a certain size.

#### `NoRepeatNGramLogitsProcessor(input_ids, logits)`

Apply the no-repeat n-gram processor to the logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The logits with no-repeat n-gram processing.

#### `NoRepeatNGramLogitsProcessor.constructor(no_repeat_ngram_size)`

Create a NoRepeatNGramLogitsProcessor.

**Parameters**

- `no_repeat_ngram_size` (`number`) — The no-repeat n-gram size. All n-grams of this size can only occur once.

#### `NoRepeatNGramLogitsProcessor.getNgrams(prevInputIds)`

Generate n-grams from a sequence of token IDs.

**Parameters**

- `prevInputIds` (`bigint[]`) — List of previous input IDs.

**Returns:** `Map`<`string`, `number[]`> — Map of generated n-grams

#### `NoRepeatNGramLogitsProcessor.getGeneratedNgrams(bannedNgrams, prevInputIds)`

Generate n-grams from a sequence of token IDs.

**Parameters**

- `bannedNgrams` (`Map`<`string`, `number[]`>) — Map of banned n-grams
- `prevInputIds` (`bigint[]`) — List of previous input IDs.

**Returns:** `number[]` — Map of generated n-grams

#### `NoRepeatNGramLogitsProcessor.calcBannedNgramTokens(prevInputIds)`

Calculate banned n-gram tokens

**Parameters**

- `prevInputIds` (`bigint[]`) — List of previous input IDs.

**Returns:** `number[]` — Map of generated n-grams

### RepetitionPenaltyLogitsProcessor

A logits processor that prevents the repetition of previous tokens through a penalty.
This penalty is applied at most once per token. Note that, for decoder-only models like most LLMs,
the considered tokens include the prompt.

In the original [paper](https://huggingface.co/papers/1909.05858), the authors suggest the use of a
penalty of around 1.2 to achieve a good balance between truthful generation and lack of repetition.
To penalize and reduce repetition, use `penalty` values above 1.0, where a higher value penalizes
more strongly. To reward and encourage repetition, use `penalty` values between 0.0 and 1.0, where
a lower value rewards more strongly.

#### `RepetitionPenaltyLogitsProcessor(input_ids, logits)`

Apply the repetition penalty to the logits.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The logits with repetition penalty processing.

#### `RepetitionPenaltyLogitsProcessor.constructor(penalty)`

Create a RepetitionPenaltyLogitsProcessor.

**Parameters**

- `penalty` (`number`) — Penalty applied to repeated tokens.
  - 1.0 means no penalty. Above 1.0 penalizes previously generated tokens.
  - Between 0.0 and 1.0 rewards previously generated tokens.

### MinLengthLogitsProcessor

A logits processor that enforces a minimum number of tokens.

#### `MinLengthLogitsProcessor(input_ids, logits)`

Apply logit processor.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The processed logits.

#### `MinLengthLogitsProcessor.constructor(min_length, eos_token_id)`

Create a MinLengthLogitsProcessor.

**Parameters**

- `min_length` (`number`) — The minimum length below which the score of `eos_token_id` is set to negative infinity.
- `eos_token_id` (`number` | `number[]`) — The ID or IDs of the end-of-sequence token.

### MinNewTokensLengthLogitsProcessor

A logits processor that enforces a minimum number of new tokens.

#### `MinNewTokensLengthLogitsProcessor(input_ids, logits)`

Apply logit processor.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The processed logits.

#### `MinNewTokensLengthLogitsProcessor.constructor(prompt_length_to_skip, min_new_tokens, eos_token_id)`

Create a MinNewTokensLengthLogitsProcessor.

**Parameters**

- `prompt_length_to_skip` (`number`) — The input tokens length.
- `min_new_tokens` (`number`) — The minimum *new* tokens length below which the score of `eos_token_id` is set to negative infinity.
- `eos_token_id` (`number` | `number[]`) — The ID or IDs of the end-of-sequence token.

### NoBadWordsLogitsProcessor

LogitsProcessor that enforces that specified sequences will never be selected.

#### `NoBadWordsLogitsProcessor(input_ids, logits)`

Apply logit processor.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The processed logits.

#### `NoBadWordsLogitsProcessor.constructor(bad_words_ids, eos_token_id)`

Create a `NoBadWordsLogitsProcessor`.

**Parameters**

- `bad_words_ids` (`number[][]`) — List of token ID sequences that are not allowed to be generated.
- `eos_token_id` (`number` | `number[]`) — The ID of the *end-of-sequence* token. Optionally, use a list to set multiple *end-of-sequence* tokens.

### ClassifierFreeGuidanceLogitsProcessor

[`LogitsProcessor`](./logits_process#module_generation/logits_process.LogitsProcessor) for classifier-free guidance (CFG). The scores are split over the batch dimension,
where the first half correspond to the conditional logits (predicted from the input prompt) and the second half
correspond to the unconditional logits (predicted from an empty or 'null' prompt). The processor computes a
weighted average across the conditional and unconditional logits, parameterised by the `guidance_scale`.

See [the paper](https://huggingface.co/papers/2306.05284) for more information.

#### `ClassifierFreeGuidanceLogitsProcessor(input_ids, logits)`

Apply logit processor.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The processed logits.

#### `ClassifierFreeGuidanceLogitsProcessor.constructor(guidance_scale)`

Create a `ClassifierFreeGuidanceLogitsProcessor`.

**Parameters**

- `guidance_scale` (`number`) — The guidance scale for classifier-free guidance (CFG). CFG is enabled by setting `guidance_scale > 1`.
  Higher guidance scale encourages the model to generate samples that are more closely tied to the input
  prompt, usually at the expense of poorer quality.

### TemperatureLogitsWarper

[`LogitsWarper`](./logits_process#module_generation/logits_process.LogitsWarper) for temperature (exponential scaling output probability distribution), which effectively means
that it can control the randomness of the predicted tokens. Often used together with [`TopPLogitsWarper`](./logits_process#module_generation/logits_process.TopPLogitsWarper) and [`TopKLogitsWarper`](./logits_process#module_generation/logits_process.TopKLogitsWarper).

#### `TemperatureLogitsWarper(input_ids, logits)`

Apply logit warper.

**Parameters**

- `input_ids` (`bigint[][]`) — The input IDs.
- `logits` ([`Tensor`](../utils/tensor#module_utils/tensor.Tensor)) — The logits.

**Returns:** [`Tensor`](../utils/tensor#module_utils/tensor.Tensor) — The processed logits.

#### `TemperatureLogitsWarper.constructor(temperature)`

Create a `TemperatureLogitsWarper`.

**Parameters**

- `temperature` (`number`) — Strictly positive float value used to modulate the logits distribution.
  A value smaller than `1` decreases randomness (and vice versa), with `0` being equivalent to shifting
  all probability mass to the most likely token.

### TopPLogitsWarper

[`LogitsWarper`](./logits_process#module_generation/logits_process.LogitsWarper) that performs top-p, i.e. restricting to top tokens summing to prob_cut_off <= prob_cut_off.
Often used together with [`TemperatureLogitsWarper`](./logits_process#module_generation/logits_process.TemperatureLogitsWarper) and [`TopKLogitsWarper`](./logits_process#module_generation/logits_process.TopKLogitsWarper).

#### `TopPLogitsWarper.constructor(top_p, options)`

Create a `TopPLogitsWarper`.

**Parameters**

- `top_p` (`number`) — If set to < 1, only the smallest set of most probable tokens with
  probabilities that add up to `top_p` or higher are kept for generation.
- `options` (`Object`) — Additional options for the top-p sampling.
  - `filter_value` (`number`) _optional_ — defaults to `-Infinity` — All filtered values will be set to this float value.
  - `min_tokens_to_keep` (`number`) _optional_ — defaults to `1` — Minimum number of tokens that cannot be filtered.

### TopKLogitsWarper

[`LogitsWarper`](./logits_process#module_generation/logits_process.LogitsWarper) that performs top-k, i.e. restricting to the k highest probability elements.
Often used together with [`TemperatureLogitsWarper`](./logits_process#module_generation/logits_process.TemperatureLogitsWarper) and [`TopPLogitsWarper`](./logits_process#module_generation/logits_process.TopPLogitsWarper).

#### `TopKLogitsWarper.constructor(top_k, options)`

Create a `TopKLogitsWarper`.

**Parameters**

- `top_k` (`number`) — If set to > 0, only the top `top_k` tokens are kept for generation.
- `options` (`Object`) — Additional options for the top-k sampling.
  - `filter_value` (`number`) _optional_ — defaults to `-Infinity` — All filtered values will be set to this float value.
  - `min_tokens_to_keep` (`number`) _optional_ — defaults to `1` — Minimum number of tokens that cannot be filtered.
