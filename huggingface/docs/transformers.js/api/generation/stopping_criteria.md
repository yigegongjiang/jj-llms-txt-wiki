# generation/stopping_criteria

Stopping criteria for controlling when generation halts.

Each criterion returns one boolean per sequence in the batch, indicating which sequences
should stop. Combine criteria with `StoppingCriteriaList` and pass it to `generate()`
as the `stopping_criteria` argument.

## Classes

### StoppingCriteria

Abstract base class for all stopping criteria that can be applied during generation.

#### `StoppingCriteria(input_ids, scores)`

**Parameters**

- `input_ids` (`number[][]`) — (`number[][]` of shape `(batch_size, sequence_length)`):
  Indices of input sequence tokens in the vocabulary.
- `scores` (`number[][]`) — (`number[][]` of shape `(batch_size, config.vocab_size)`):
  Prediction scores of a language modeling head. These can be scores for each vocabulary token before SoftMax
  or scores for each vocabulary token after SoftMax.

**Returns:** `boolean[]` — A list of booleans indicating whether each sequence should be stopped.

### StoppingCriteriaList

A list of `StoppingCriteria` that stops generation when any one of them returns `true`.

#### `StoppingCriteriaList.constructor()`

Constructs a new instance of `StoppingCriteriaList`.

#### `StoppingCriteriaList.push(item)`

Adds a new stopping criterion to the list.

**Parameters**

- `item` ([`StoppingCriteria`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteria)) — The stopping criterion to add.

#### `StoppingCriteriaList.extend(items)`

Adds multiple stopping criteria to the list.

**Parameters**

- `items` ([`StoppingCriteria`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteria) | [`StoppingCriteriaList`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteriaList) | [`StoppingCriteria`](./stopping_criteria#module_generation/stopping_criteria.StoppingCriteria)[]) — The stopping criteria to add.

### MaxLengthCriteria

Stops generation whenever the generated sequence length reaches `max_length`.
For decoder-only models, this includes the initial prompt tokens.

#### `MaxLengthCriteria.constructor(max_length, [max_position_embeddings])`

**Parameters**

- `max_length` (`number`) — The maximum length that the output sequence can have in number of tokens.
- `max_position_embeddings` (`number`) _optional_ — defaults to `null` — The maximum model length, as defined by the model's `config.max_position_embeddings` attribute.

### EosTokenCriteria

Stops generation whenever an "end-of-sequence" token is generated.
By default, it uses the `model.generation_config.eos_token_id`.

#### `EosTokenCriteria(input_ids, scores)`

**Parameters**

- `input_ids` (`number[][]`)
- `scores` (`number[][]`)

**Returns:** `boolean[]`

#### `EosTokenCriteria.constructor(eos_token_id)`

**Parameters**

- `eos_token_id` (`number` | `number[]`) — The ID of the *end-of-sequence* token.
  Optionally, use a list to set multiple *end-of-sequence* tokens.

### InterruptableStoppingCriteria

Stops generation whenever the user interrupts the process.

#### `InterruptableStoppingCriteria(input_ids, scores)`

**Parameters**

- `input_ids` (`number[][]`) — (`number[][]` of shape `(batch_size, sequence_length)`):
  Indices of input sequence tokens in the vocabulary.
- `scores` (`number[][]`) — (`number[][]` of shape `(batch_size, config.vocab_size)`):
  Prediction scores of a language modeling head.

**Returns:** `boolean[]` — A list of booleans indicating whether each sequence should be stopped.

#### `InterruptableStoppingCriteria.constructor()`

Constructs a new instance of `InterruptableStoppingCriteria`.

#### `InterruptableStoppingCriteria.interrupt()`

Interrupts generation, stopping every sequence on the next call.

#### `InterruptableStoppingCriteria.reset()`

Clears a previous interruption, allowing generation to continue.
