# generation/configuration_utils

* [generation/configuration_utils](#module_generation/configuration_utils)
    * [.GenerationConfig](#module_generation/configuration_utils.GenerationConfig)
        * [`new GenerationConfig(config)`](#new_module_generation/configuration_utils.GenerationConfig_new)
        * [`.max_length`](#module_generation/configuration_utils.GenerationConfig+max_length) : number
        * [`.max_new_tokens`](#module_generation/configuration_utils.GenerationConfig+max_new_tokens) : number
        * [`.min_length`](#module_generation/configuration_utils.GenerationConfig+min_length) : number
        * [`.min_new_tokens`](#module_generation/configuration_utils.GenerationConfig+min_new_tokens) : number
        * [`.early_stopping`](#module_generation/configuration_utils.GenerationConfig+early_stopping) : boolean | &quot;never&quot;
        * [`.max_time`](#module_generation/configuration_utils.GenerationConfig+max_time) : number
        * [`.do_sample`](#module_generation/configuration_utils.GenerationConfig+do_sample) : boolean
        * [`.num_beams`](#module_generation/configuration_utils.GenerationConfig+num_beams) : number
        * [`.num_beam_groups`](#module_generation/configuration_utils.GenerationConfig+num_beam_groups) : number
        * [`.penalty_alpha`](#module_generation/configuration_utils.GenerationConfig+penalty_alpha) : number
        * [`.use_cache`](#module_generation/configuration_utils.GenerationConfig+use_cache) : boolean
        * [`.temperature`](#module_generation/configuration_utils.GenerationConfig+temperature) : number
        * [`.top_k`](#module_generation/configuration_utils.GenerationConfig+top_k) : number
        * [`.top_p`](#module_generation/configuration_utils.GenerationConfig+top_p) : number
        * [`.typical_p`](#module_generation/configuration_utils.GenerationConfig+typical_p) : number
        * [`.epsilon_cutoff`](#module_generation/configuration_utils.GenerationConfig+epsilon_cutoff) : number
        * [`.eta_cutoff`](#module_generation/configuration_utils.GenerationConfig+eta_cutoff) : number
        * [`.diversity_penalty`](#module_generation/configuration_utils.GenerationConfig+diversity_penalty) : number
        * [`.repetition_penalty`](#module_generation/configuration_utils.GenerationConfig+repetition_penalty) : number
        * [`.encoder_repetition_penalty`](#module_generation/configuration_utils.GenerationConfig+encoder_repetition_penalty) : number
        * [`.length_penalty`](#module_generation/configuration_utils.GenerationConfig+length_penalty) : number
        * [`.no_repeat_ngram_size`](#module_generation/configuration_utils.GenerationConfig+no_repeat_ngram_size) : number
        * [`.bad_words_ids`](#module_generation/configuration_utils.GenerationConfig+bad_words_ids) : Array
        * [`.force_words_ids`](#module_generation/configuration_utils.GenerationConfig+force_words_ids) : Array | Array
        * [`.renormalize_logits`](#module_generation/configuration_utils.GenerationConfig+renormalize_logits) : boolean
        * [`.constraints`](#module_generation/configuration_utils.GenerationConfig+constraints) : Array
        * [`.forced_bos_token_id`](#module_generation/configuration_utils.GenerationConfig+forced_bos_token_id) : number
        * [`.forced_eos_token_id`](#module_generation/configuration_utils.GenerationConfig+forced_eos_token_id) : number | Array
        * [`.remove_invalid_values`](#module_generation/configuration_utils.GenerationConfig+remove_invalid_values) : boolean
        * [`.exponential_decay_length_penalty`](#module_generation/configuration_utils.GenerationConfig+exponential_decay_length_penalty) : Array
        * [`.suppress_tokens`](#module_generation/configuration_utils.GenerationConfig+suppress_tokens) : Array
        * [`.streamer`](#module_generation/configuration_utils.GenerationConfig+streamer) : TextStreamer
        * [`.begin_suppress_tokens`](#module_generation/configuration_utils.GenerationConfig+begin_suppress_tokens) : Array
        * [`.forced_decoder_ids`](#module_generation/configuration_utils.GenerationConfig+forced_decoder_ids) : Array
        * [`.guidance_scale`](#module_generation/configuration_utils.GenerationConfig+guidance_scale) : number
        * [`.num_return_sequences`](#module_generation/configuration_utils.GenerationConfig+num_return_sequences) : number
        * [`.output_attentions`](#module_generation/configuration_utils.GenerationConfig+output_attentions) : boolean
        * [`.output_hidden_states`](#module_generation/configuration_utils.GenerationConfig+output_hidden_states) : boolean
        * [`.output_scores`](#module_generation/configuration_utils.GenerationConfig+output_scores) : boolean
        * [`.return_dict_in_generate`](#module_generation/configuration_utils.GenerationConfig+return_dict_in_generate) : boolean
        * [`.pad_token_id`](#module_generation/configuration_utils.GenerationConfig+pad_token_id) : number
        * [`.bos_token_id`](#module_generation/configuration_utils.GenerationConfig+bos_token_id) : number
        * [`.eos_token_id`](#module_generation/configuration_utils.GenerationConfig+eos_token_id) : number | Array
        * [`.encoder_no_repeat_ngram_size`](#module_generation/configuration_utils.GenerationConfig+encoder_no_repeat_ngram_size) : number
        * [`.decoder_start_token_id`](#module_generation/configuration_utils.GenerationConfig+decoder_start_token_id) : number
        * [`.generation_kwargs`](#module_generation/configuration_utils.GenerationConfig+generation_kwargs) : Object

* * *

## generation/configuration_utils.GenerationConfig

Class that holds a configuration for a generation task.

**Kind**: static class of [generation/configuration_utils](#module_generation/configuration_utils)  

* [.GenerationConfig](#module_generation/configuration_utils.GenerationConfig)
    * [`new GenerationConfig(config)`](#new_module_generation/configuration_utils.GenerationConfig_new)
    * [`.max_length`](#module_generation/configuration_utils.GenerationConfig+max_length) : number
    * [`.max_new_tokens`](#module_generation/configuration_utils.GenerationConfig+max_new_tokens) : number
    * [`.min_length`](#module_generation/configuration_utils.GenerationConfig+min_length) : number
    * [`.min_new_tokens`](#module_generation/configuration_utils.GenerationConfig+min_new_tokens) : number
    * [`.early_stopping`](#module_generation/configuration_utils.GenerationConfig+early_stopping) : boolean | &quot;never&quot;
    * [`.max_time`](#module_generation/configuration_utils.GenerationConfig+max_time) : number
    * [`.do_sample`](#module_generation/configuration_utils.GenerationConfig+do_sample) : boolean
    * [`.num_beams`](#module_generation/configuration_utils.GenerationConfig+num_beams) : number
    * [`.num_beam_groups`](#module_generation/configuration_utils.GenerationConfig+num_beam_groups) : number
    * [`.penalty_alpha`](#module_generation/configuration_utils.GenerationConfig+penalty_alpha) : number
    * [`.use_cache`](#module_generation/configuration_utils.GenerationConfig+use_cache) : boolean
    * [`.temperature`](#module_generation/configuration_utils.GenerationConfig+temperature) : number
    * [`.top_k`](#module_generation/configuration_utils.GenerationConfig+top_k) : number
    * [`.top_p`](#module_generation/configuration_utils.GenerationConfig+top_p) : number
    * [`.typical_p`](#module_generation/configuration_utils.GenerationConfig+typical_p) : number
    * [`.epsilon_cutoff`](#module_generation/configuration_utils.GenerationConfig+epsilon_cutoff) : number
    * [`.eta_cutoff`](#module_generation/configuration_utils.GenerationConfig+eta_cutoff) : number
    * [`.diversity_penalty`](#module_generation/configuration_utils.GenerationConfig+diversity_penalty) : number
    * [`.repetition_penalty`](#module_generation/configuration_utils.GenerationConfig+repetition_penalty) : number
    * [`.encoder_repetition_penalty`](#module_generation/configuration_utils.GenerationConfig+encoder_repetition_penalty) : number
    * [`.length_penalty`](#module_generation/configuration_utils.GenerationConfig+length_penalty) : number
    * [`.no_repeat_ngram_size`](#module_generation/configuration_utils.GenerationConfig+no_repeat_ngram_size) : number
    * [`.bad_words_ids`](#module_generation/configuration_utils.GenerationConfig+bad_words_ids) : Array
    * [`.force_words_ids`](#module_generation/configuration_utils.GenerationConfig+force_words_ids) : Array | Array
    * [`.renormalize_logits`](#module_generation/configuration_utils.GenerationConfig+renormalize_logits) : boolean
    * [`.constraints`](#module_generation/configuration_utils.GenerationConfig+constraints) : Array
    * [`.forced_bos_token_id`](#module_generation/configuration_utils.GenerationConfig+forced_bos_token_id) : number
    * [`.forced_eos_token_id`](#module_generation/configuration_utils.GenerationConfig+forced_eos_token_id) : number | Array
    * [`.remove_invalid_values`](#module_generation/configuration_utils.GenerationConfig+remove_invalid_values) : boolean
    * [`.exponential_decay_length_penalty`](#module_generation/configuration_utils.GenerationConfig+exponential_decay_length_penalty) : Array
    * [`.suppress_tokens`](#module_generation/configuration_utils.GenerationConfig+suppress_tokens) : Array
    * [`.streamer`](#module_generation/configuration_utils.GenerationConfig+streamer) : TextStreamer
    * [`.begin_suppress_tokens`](#module_generation/configuration_utils.GenerationConfig+begin_suppress_tokens) : Array
    * [`.forced_decoder_ids`](#module_generation/configuration_utils.GenerationConfig+forced_decoder_ids) : Array
    * [`.guidance_scale`](#module_generation/configuration_utils.GenerationConfig+guidance_scale) : number
    * [`.num_return_sequences`](#module_generation/configuration_utils.GenerationConfig+num_return_sequences) : number
    * [`.output_attentions`](#module_generation/configuration_utils.GenerationConfig+output_attentions) : boolean
    * [`.output_hidden_states`](#module_generation/configuration_utils.GenerationConfig+output_hidden_states) : boolean
    * [`.output_scores`](#module_generation/configuration_utils.GenerationConfig+output_scores) : boolean
    * [`.return_dict_in_generate`](#module_generation/configuration_utils.GenerationConfig+return_dict_in_generate) : boolean
    * [`.pad_token_id`](#module_generation/configuration_utils.GenerationConfig+pad_token_id) : number
    * [`.bos_token_id`](#module_generation/configuration_utils.GenerationConfig+bos_token_id) : number
    * [`.eos_token_id`](#module_generation/configuration_utils.GenerationConfig+eos_token_id) : number | Array
    * [`.encoder_no_repeat_ngram_size`](#module_generation/configuration_utils.GenerationConfig+encoder_no_repeat_ngram_size) : number
    * [`.decoder_start_token_id`](#module_generation/configuration_utils.GenerationConfig+decoder_start_token_id) : number
    * [`.generation_kwargs`](#module_generation/configuration_utils.GenerationConfig+generation_kwargs) : Object

* * *

### `new GenerationConfig(config)`

  
    
      ParamType
    
  
  

    configGenerationConfig | PretrainedConfig
      

* * *

### `generationConfig.max_length` : number

The maximum length the generated tokens can have.
Corresponds to the length of the input prompt + `max_new_tokens`.
Its effect is overridden by `max_new_tokens`, if also set.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 20  

* * *

### `generationConfig.max_new_tokens` : number

The maximum numbers of tokens to generate, ignoring the number of tokens in the prompt.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.min_length` : number

The minimum length of the sequence to be generated.
Corresponds to the length of the input prompt + `min_new_tokens`.
Its effect is overridden by `min_new_tokens`, if also set.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0  

* * *

### `generationConfig.min_new_tokens` : number

The minimum numbers of tokens to generate, ignoring the number of tokens in the prompt.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.early_stopping` : boolean | &quot;never&quot;

Controls the stopping condition for beam-based methods, like beam-search. It accepts the following values:
- `true`, where the generation stops as soon as there are `num_beams` complete candidates;
- `false`, where an heuristic is applied and the generation stops when is it very unlikely to find better candidates;
- `"never"`, where the beam search procedure only stops when there cannot be better candidates (canonical beam search algorithm).

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.max_time` : number

The maximum amount of time you allow the computation to run for in seconds.
Generation will still finish the current pass after allocated time has been passed.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.do_sample` : boolean

Whether or not to use sampling; use greedy decoding otherwise.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.num_beams` : number

Number of beams for beam search. 1 means no beam search.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1  

* * *

### `generationConfig.num_beam_groups` : number

Number of groups to divide `num_beams` into in order to ensure diversity among different groups of beams.
See [this paper](https://huggingface.co/papers/1610.02424) for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1  

* * *

### `generationConfig.penalty_alpha` : number

The values balance the model confidence and the degeneration penalty in contrastive search decoding.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.use_cache` : boolean

Whether or not the model should use the past last key/values attentions (if applicable to the model) to speed up decoding.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: true  

* * *

### `generationConfig.temperature` : number

The value used to modulate the next token probabilities.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.top_k` : number

The number of highest probability vocabulary tokens to keep for top-k-filtering.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 50  

* * *

### `generationConfig.top_p` : number

If set to float GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.typical_p` : number

Local typicality measures how similar the conditional probability of predicting a target token next is to the expected conditional probability of predicting a random token next, given the partial text already generated.
If set to float GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.epsilon_cutoff` : number

If set to float strictly between 0 and 1, only tokens with a conditional probability greater than `epsilon_cutoff` will be sampled.
In the paper, suggested values range from 3e-4 to 9e-4, depending on the size of the model.
See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0.0  

* * *

### `generationConfig.eta_cutoff` : number

Eta sampling is a hybrid of locally typical sampling and epsilon sampling.
If set to float strictly between 0 and 1, a token is only considered if it is greater than either `eta_cutoff` or `sqrt(eta_cutoff) * exp(-entropy(softmax(next_token_logits)))`.
The latter term is intuitively the expected next token probability, scaled by `sqrt(eta_cutoff)`. In the paper, suggested values range from 3e-4 to 2e-3, depending on the size of the model.
See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0.0  

* * *

### `generationConfig.diversity_penalty` : number

This value is subtracted from a beam's score if it generates a token same as any beam from other group at a particular time.
Note that `diversity_penalty` is only effective if `group beam search` is enabled.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0.0  

* * *

### `generationConfig.repetition_penalty` : number

The parameter for repetition penalty. 1.0 means no penalty.
See [this paper](https://huggingface.co/papers/1909.05858) for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.encoder_repetition_penalty` : number

The paramater for encoder_repetition_penalty.
An exponential penalty on sequences that are not in the original input.
1.0 means no penalty.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.length_penalty` : number

Exponential penalty to the length that is used with beam-based generation.
It is applied as an exponent to the sequence length, which in turn is used to divide the score of the sequence.
Since the score is the log likelihood of the sequence (i.e. negative), `length_penalty` > 0.0 promotes longer sequences, while `length_penalty` GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1.0  

* * *

### `generationConfig.no_repeat_ngram_size` : number

If set to int > 0, all ngrams of that size can only occur once.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0  

* * *

### `generationConfig.bad_words_ids` : Array

List of token ids that are not allowed to be generated.
In order to get the token ids of the words that should not appear in the generated text, use
`tokenizer(bad_words, { add_prefix_space: true, add_special_tokens: false }).input_ids`.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.force_words_ids` : Array | Array

List of token ids that must be generated.
If given a `number[][]`, this is treated as a simple list of words that must be included, the opposite to `bad_words_ids`.
If given `number[][][]`, this triggers a [disjunctive constraint](https://github.com/huggingface/transformers/issues/14081), where one can allow different forms of each word.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.renormalize_logits` : boolean

Whether to renormalize the logits after applying all the logits processors or warpers (including the custom ones).
It's highly recommended to set this flag to `true` as the search algorithms suppose the score logits are normalized but some logit processors or warpers break the normalization.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.constraints` : Array

Custom constraints that can be added to the generation to ensure that the output will contain the use of certain tokens as defined by `Constraint` objects, in the most sensible way possible.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.forced_bos_token_id` : number

The id of the token to force as the first generated token after the `decoder_start_token_id`.
Useful for multilingual models like mBART where the first generated token needs to be the target language token.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.forced_eos_token_id` : number | Array

The id of the token to force as the last generated token when `max_length` is reached.
Optionally, use a list to set multiple *end-of-sequence* tokens.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.remove_invalid_values` : boolean

Whether to remove possible *nan* and *inf* outputs of the model to prevent the generation method to crash. Note that using `remove_invalid_values` can slow down generation.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  

* * *

### `generationConfig.exponential_decay_length_penalty` : Array

This Tuple adds an exponentially increasing length penalty, after a certain amount of tokens have been generated.
The tuple shall consist of: `(start_index, decay_factor)` where `start_index` indicates where penalty starts and `decay_factor` represents the factor of exponential decay.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.suppress_tokens` : Array

A list of tokens that will be suppressed at generation.
The `SuppressTokens` logit processor will set their log probs to `-inf` so that they are not sampled.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.streamer` : TextStreamer

A streamer that will be used to stream the generation.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.begin_suppress_tokens` : Array

A list of tokens that will be suppressed at the beginning of the generation.
The `SuppressBeginTokens` logit processor will set their log probs to `-inf` so that they are not sampled.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.forced_decoder_ids` : Array

A list of pairs of integers which indicates a mapping from generation indices to token indices that will be forced before sampling.
For example, `[[1, 123]]` means the second generated token will always be a token of index 123.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.guidance_scale` : number

The guidance scale for classifier free guidance (CFG). CFG is enabled by setting `guidance_scale > 1`.
Higher guidance scale encourages the model to generate samples that are more closely linked to the input
prompt, usually at the expense of poorer quality.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.num_return_sequences` : number

The number of independently computed returned sequences for each element in the batch.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 1  

* * *

### `generationConfig.output_attentions` : boolean

Whether or not to return the attentions tensors of all attention layers.
See `attentions` under returned tensors for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.output_hidden_states` : boolean

Whether or not to return the hidden states of all layers.
See `hidden_states` under returned tensors for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.output_scores` : boolean

Whether or not to return the prediction scores.
See `scores` under returned tensors for more details.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.return_dict_in_generate` : boolean

Whether or not to return a `ModelOutput` instead of a plain tuple.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: false  

* * *

### `generationConfig.pad_token_id` : number

The id of the *padding* token.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.bos_token_id` : number

The id of the *beginning-of-sequence* token.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.eos_token_id` : number | Array

The id of the *end-of-sequence* token.
Optionally, use a list to set multiple *end-of-sequence* tokens.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.encoder_no_repeat_ngram_size` : number

If set to int > 0, all ngrams of that size that occur in the `encoder_input_ids` cannot occur in the `decoder_input_ids`.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: 0  

* * *

### `generationConfig.decoder_start_token_id` : number

If an encoder-decoder model starts decoding with a different token than *bos*, the id of that token.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: null  

* * *

### `generationConfig.generation_kwargs` : Object

Additional generation kwargs will be forwarded to the `generate` function of the model.
Kwargs that are not present in `generate`'s signature will be used in the model forward pass.

**Kind**: instance property of [GenerationConfig](#module_generation/configuration_utils.GenerationConfig)  
**Default**: {}  

* * *
