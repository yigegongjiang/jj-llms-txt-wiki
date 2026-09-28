# generation/configuration_utils

Configuration for text generation.

`GenerationConfig` holds the parameters that control `generate()`.

## Classes

### GenerationConfig

Class that holds a configuration for a generation task.

#### `GenerationConfig.max_length` : `number`

The maximum length the generated tokens can have.
Corresponds to the length of the input prompt + `max_new_tokens`.
Its effect is overridden by `max_new_tokens`, if also set.

**Default:** `20`

#### `GenerationConfig.max_new_tokens` : `number`

The maximum number of tokens to generate, ignoring the number of tokens in the prompt.

**Default:** `null`

#### `GenerationConfig.min_length` : `number`

The minimum length of the sequence to be generated.
Corresponds to the length of the input prompt + `min_new_tokens`.
Its effect is overridden by `min_new_tokens`, if also set.

**Default:** `0`

#### `GenerationConfig.min_new_tokens` : `number`

The minimum number of tokens to generate, ignoring the number of tokens in the prompt.

**Default:** `null`

#### `GenerationConfig.early_stopping` : `boolean` | `'never'`

Controls the stopping condition for beam-based methods, like beam-search. It accepts the following values:
- `true`, where the generation stops as soon as there are `num_beams` complete candidates;
- `false`, where a heuristic is applied and the generation stops when it is very unlikely to find better candidates;
- `"never"`, where the beam search procedure only stops when there cannot be better candidates (canonical beam search algorithm).

**Default:** `false`

#### `GenerationConfig.max_time` : `number`

The maximum time, in seconds, allowed for generation.
Generation will still finish the current pass after the allocated time has passed.

**Default:** `null`

#### `GenerationConfig.do_sample` : `boolean`

Whether to use sampling; use greedy decoding otherwise.

**Default:** `false`

#### `GenerationConfig.num_beams` : `number`

Number of beams for beam search. 1 means no beam search.

**Default:** `1`

#### `GenerationConfig.num_beam_groups` : `number`

Number of groups to divide `num_beams` into to encourage diversity among different groups of beams.
See [this paper](https://huggingface.co/papers/1610.02424) for more details.

**Default:** `1`

#### `GenerationConfig.penalty_alpha` : `number`

Balance model confidence against the degeneration penalty during contrastive search decoding.

**Default:** `null`

#### `GenerationConfig.use_cache` : `boolean`

Whether the model should reuse past key/value states, when supported, to speed up decoding.

**Default:** `true`

#### `GenerationConfig.temperature` : `number`

The value used to modulate the next token probabilities.

**Default:** `1.0`

#### `GenerationConfig.top_k` : `number`

The number of highest-probability vocabulary tokens to keep for top-k filtering.

**Default:** `50`

#### `GenerationConfig.top_p` : `number`

If set to a float below 1, only the smallest set of most probable tokens with probabilities that add up to `top_p` or higher are kept for generation.

**Default:** `1.0`

#### `GenerationConfig.typical_p` : `number`

Local typicality measures how similar the conditional probability of predicting a target token next is to the expected conditional probability of predicting a random token next, given the partial text already generated.
If set to a float below 1, the smallest set of the most locally typical tokens with probabilities that add up to `typical_p` or higher are kept for generation.
See [this paper](https://huggingface.co/papers/2202.00666) for more details.

**Default:** `1.0`

#### `GenerationConfig.epsilon_cutoff` : `number`

If set to a float strictly between 0 and 1, only tokens with a conditional probability greater than `epsilon_cutoff` will be sampled.
In the paper, suggested values range from 3e-4 to 9e-4, depending on the size of the model.
See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

**Default:** `0.0`

#### `GenerationConfig.eta_cutoff` : `number`

Eta sampling is a hybrid of locally typical sampling and epsilon sampling.
If set to a float strictly between 0 and 1, a token is only considered if it is greater than either `eta_cutoff` or `sqrt(eta_cutoff) * exp(-entropy(softmax(next_token_logits)))`.
The latter term is intuitively the expected next token probability, scaled by `sqrt(eta_cutoff)`. In the paper, suggested values range from 3e-4 to 2e-3, depending on the size of the model.
See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

**Default:** `0.0`

#### `GenerationConfig.diversity_penalty` : `number`

This value is subtracted from a beam's score if it generates the same token as any beam from another group at a particular time.
Note that `diversity_penalty` is only effective if `group beam search` is enabled.

**Default:** `0.0`

#### `GenerationConfig.repetition_penalty` : `number`

Penalty applied to repeated tokens. 1.0 means no penalty.
See [this paper](https://huggingface.co/papers/1909.05858) for more details.

**Default:** `1.0`

#### `GenerationConfig.encoder_repetition_penalty` : `number`

Penalty applied to sequences that are not in the original input.
1.0 means no penalty.

**Default:** `1.0`

#### `GenerationConfig.length_penalty` : `number`

Exponential penalty applied to sequence length during beam-based generation.
It is applied as an exponent to the sequence length, which in turn is used to divide the score of the sequence.
Since the score is the log likelihood of the sequence (i.e. negative), `length_penalty` > 0.0 promotes longer sequences, while `length_penalty` < 0.0 encourages shorter sequences.

**Default:** `1.0`

#### `GenerationConfig.no_repeat_ngram_size` : `number`

If set to an integer greater than 0, all n-grams of that size can only occur once.

**Default:** `0`

#### `GenerationConfig.bad_words_ids` : `number[][]`

List of token IDs that are not allowed to be generated.
In order to get the token IDs of the words that should not appear in the generated text, use
`tokenizer(bad_words, { add_prefix_space: true, add_special_tokens: false }).input_ids`.

**Default:** `null`

#### `GenerationConfig.force_words_ids` : `number[][]` | `number[][][]`

List of token IDs that must be generated.
If given a `number[][]`, this is treated as a simple list of words that must be included, the opposite of `bad_words_ids`.
If given `number[][][]`, this triggers a [disjunctive constraint](https://github.com/huggingface/transformers/issues/14081), which allows different forms of each word.

**Default:** `null`

#### `GenerationConfig.renormalize_logits` : `boolean`

Whether to renormalize the logits after applying all the logits processors or warpers (including the custom ones).
It's highly recommended to set this flag to `true` because search algorithms assume the score logits are normalized, but some logit processors or warpers break the normalization.

**Default:** `false`

#### `GenerationConfig.constraints` : `Object[]`

Custom constraints that guide generation to include certain tokens as defined by `Constraint` objects.

**Default:** `null`

#### `GenerationConfig.forced_bos_token_id` : `number`

The ID of the token to force as the first generated token after the `decoder_start_token_id`.
Useful for multilingual models like mBART where the first generated token needs to be the target language token.

**Default:** `null`

#### `GenerationConfig.forced_eos_token_id` : `number` | `number[]`

The ID of the token to force as the last generated token when `max_length` is reached.
Optionally, use a list to set multiple *end-of-sequence* tokens.

**Default:** `null`

#### `GenerationConfig.remove_invalid_values` : `boolean`

Whether to remove possible *nan* and *inf* outputs of the model to prevent the generation method from crashing. Note that using `remove_invalid_values` can slow down generation.

**Default:** `false`

#### `GenerationConfig.exponential_decay_length_penalty` : `[number, number]`

This tuple adds an exponentially increasing length penalty after a certain number of tokens have been generated.
The tuple consists of: `(start_index, decay_factor)` where `start_index` indicates where the penalty starts and `decay_factor` represents the factor of exponential decay.

**Default:** `null`

#### `GenerationConfig.suppress_tokens` : `number[]`

A list of tokens to suppress during generation.
The `SuppressTokens` logit processor sets their log probabilities to `-inf` so that they are not sampled.

**Default:** `null`

#### `GenerationConfig.streamer` : [`TextStreamer`](./streamers#module_generation/streamers.TextStreamer)

Streamer used to yield generated text incrementally.

**Default:** `null`

#### `GenerationConfig.begin_suppress_tokens` : `number[]`

A list of tokens to suppress at the beginning of the generation.
The `SuppressBeginTokens` logit processor sets their log probabilities to `-inf` so that they are not sampled.

**Default:** `null`

#### `GenerationConfig.forced_decoder_ids` : `[number, number][]`

A list of integer pairs that maps generation indices to token indices that will be forced before sampling.
For example, `[[1, 123]]` means the second generated token will always be a token of index 123.

**Default:** `null`

#### `GenerationConfig.guidance_scale` : `number`

The guidance scale for classifier-free guidance (CFG). CFG is enabled by setting `guidance_scale > 1`.
Higher guidance scale encourages the model to generate samples that are more closely tied to the input
prompt, usually at the expense of poorer quality.

**Default:** `null`

#### `GenerationConfig.num_return_sequences` : `number`

The number of independently computed returned sequences for each element in the batch.

**Default:** `1`

#### `GenerationConfig.output_attentions` : `boolean`

Whether to return attention tensors from all attention layers.
See `attentions` under returned tensors for more details.

**Default:** `false`

#### `GenerationConfig.output_hidden_states` : `boolean`

Whether to return the hidden states of all layers.
See `hidden_states` under returned tensors for more details.

**Default:** `false`

#### `GenerationConfig.output_scores` : `boolean`

Whether to return the prediction scores.
See `scores` under returned tensors for more details.

**Default:** `false`

#### `GenerationConfig.return_dict_in_generate` : `boolean`

Whether to return a `ModelOutput` instead of a plain tuple.

**Default:** `false`

#### `GenerationConfig.pad_token_id` : `number`

The ID of the *padding* token.

**Default:** `null`

#### `GenerationConfig.bos_token_id` : `number`

The ID of the *beginning-of-sequence* token.

**Default:** `null`

#### `GenerationConfig.eos_token_id` : `number` | `number[]`

The ID of the *end-of-sequence* token.
Optionally, use a list to set multiple *end-of-sequence* tokens.

**Default:** `null`

#### `GenerationConfig.encoder_no_repeat_ngram_size` : `number`

If set to an integer greater than 0, all n-grams of that size that occur in the `encoder_input_ids` cannot occur in the `decoder_input_ids`.

**Default:** `0`

#### `GenerationConfig.decoder_start_token_id` : `number`

If an encoder-decoder model starts decoding with a token other than *bos*, the ID of that token.

**Default:** `null`

#### `GenerationConfig.generation_kwargs` : `Object`

Additional generation kwargs forwarded to the model's `generate` function.
Kwargs that are not present in `generate`'s signature are used in the model forward pass.

**Default:** `{}`

#### `GenerationConfig.constructor(config)`

**Parameters**

- `config` ([`GenerationConfig`](./configuration_utils#module_generation/configuration_utils.GenerationConfig) | [`PretrainedConfig`](../configs#module_configs.PretrainedConfig))
