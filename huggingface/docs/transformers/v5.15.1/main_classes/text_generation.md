# Generation

Each framework has a generate method for text generation implemented in their respective `GenerationMixin` class:

- PyTorch [generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate) is implemented in [GenerationMixin](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin).

You can parameterize the generate method with a [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig) class instance. Please refer to this class for the complete list of generation parameters, which control the behavior of the generation method.

To learn how to inspect a model's generation configuration, what are the defaults, how to change the parameters ad hoc,
and how to create and save a customized generation configuration, refer to the
[text generation strategies guide](../generation_strategies). The guide also explains how to use related features,
like token streaming.

## GenerationConfig[[transformers.GenerationConfig]]

#### transformers.GenerationConfig[[transformers.GenerationConfig]]

```python
transformers.GenerationConfig(**kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L100)

**Parameters that control the length of the output:**

max_length (`int`, *optional*) : `max_new_tokens` is recommended for controlling how many tokens the model generates. `max_length` remains for backward compatibility. 

max_new_tokens (`int`, *optional*) : The maximum numbers of tokens to generate, ignoring the number of tokens in the prompt.

min_length (`int`, *optional*) : The minimum length of the sequence to be generated. Corresponds to the length of the input prompt + `min_new_tokens`. Its effect is overridden by `min_new_tokens`, if also set.

min_new_tokens (`int`, *optional*) : The minimum numbers of tokens to generate, ignoring the number of tokens in the prompt.

early_stopping (`bool` or `str`, *optional*) : Controls the stopping condition for beam-based methods, like beam-search. It accepts the following values: `True`, where the generation stops as soon as there are `num_beams` complete candidates; `False`, where an heuristic is applied and the generation stops when is it very unlikely to find better candidates; `"never"`, where the beam search procedure only stops when there cannot be better candidates (canonical beam search algorithm).

max_time (`float`, *optional*) : The maximum amount of time you allow the computation to run for in seconds. generation will still finish the current pass after allocated time has been passed.

stop_strings (`str` or `list[str]`, *optional*) : A string or a list of strings that should terminate generation if the model outputs them.

**Parameters that control the generation strategy used:**

do_sample (`bool`) : Whether or not to use sampling ; use greedy decoding otherwise.

num_beams (`int`, *optional*) : Number of beams for beam search. 1 means no beam search.

use_mtp : (`bool`): Whether or not to use Multi-Token Prediction (MTP) if the model supports it.

**Parameters that control the cache:**

use_cache (`bool`) : Whether or not the model should use the past last key/values attentions (if applicable to the model) to speed up decoding.

cache_implementation (`str`, *optional*) : Name of the cache class that will be instantiated in `generate`, for faster decoding. Possible values are:  - `"dynamic"`: [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) - `"static"`: [StaticCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.StaticCache) - `"offloaded"`: `DynamicCache(offloaded=True)` - `"offloaded_static"`: `StaticCache(offloaded=True)` - `"quantized"`: [QuantizedCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.QuantizedCache)  If none is specified, we will use the default cache for the model (which is often [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache)). See our [cache documentation](https://huggingface.co/docs/transformers/en/kv_cache) for further information.

cache_config (`dict`, *optional*, default to `None`) : Arguments used in the key-value cache class can be passed in `cache_config`.

max_cache_len (`int`, *optional*) : Only used with static caches (`cache_implementation` set to `"static"` or `"offloaded_static"`). Pre-sizes the cache to this length instead of the current call's `max_length`. Set it once to the largest call you expect so that repeated `generate()` calls with a longer prompt or a larger `max_new_tokens` (up to this ceiling) reuse the same cache instead of triggering a reallocation and a `torch.compile` recompilation.

**Parameters for manipulation of the model output logits:**

temperature (`float`, *optional*) : The value used to module the next token probabilities. This value is set in a model's `generation_config.json` file. If it isn't set, the default value is 1.0

top_k (`int`, *optional*) : The number of highest probability vocabulary tokens to keep for top-k-filtering. This value is set in a model's `generation_config.json` file. If it isn't set, the default value is 50.

top_p (`float`, *optional*) : If set to float < 1, only the smallest set of most probable tokens with probabilities that add up to `top_p` or higher are kept for generation. This value is set in a model's `generation_config.json` file. If it isn't set, the default value is 1.0

min_p (`float`, *optional*) : Minimum token probability, which will be scaled by the probability of the most likely token. It must be a value between 0 and 1. Typical values are in the 0.01-0.2 range, comparably selective as setting `top_p` in the 0.99-0.8 range (use the opposite of normal `top_p` values).

top_h (`float`, *optional*) : Entropy budget scaling factor, which controls how much of the distribution’s entropy is preserved when sampling. Must be a value between 0 and 1. At each step, tokens are sorted by probability, and the smallest prefix of tokens is kept whose *renormalized* entropy is less than or equal to `top_h` times the entropy of the full distribution. Smaller values (e.g., 0.2–0.5) lead to more focused, deterministic outputs, while values closer to 1.0 allow more randomness and diversity. Typical values are in the 0.3–0.6 range.

typical_p (`float`, *optional*) : Local typicality measures how similar the conditional probability of predicting a target token next is to the expected conditional probability of predicting a random token next, given the partial text already generated. If set to float < 1, the smallest set of the most locally typical tokens with probabilities that add up to `typical_p` or higher are kept for generation. See [this paper](https://huggingface.co/papers/2202.00666) for more details.

epsilon_cutoff (`float`, *optional*) : If set to float strictly between 0 and 1, only tokens with a conditional probability greater than `epsilon_cutoff` will be sampled. In the paper, suggested values range from 3e-4 to 9e-4, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

eta_cutoff (`float`, *optional*) : Eta sampling is a hybrid of locally typical sampling and epsilon sampling. If set to float strictly between 0 and 1, a token is only considered if it is greater than either `eta_cutoff` or `sqrt(eta_cutoff) * exp(-entropy(softmax(next_token_logits)))`. The latter term is intuitively the expected next token probability, scaled by `sqrt(eta_cutoff)`. In the paper, suggested values range from 3e-4 to 2e-3, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://huggingface.co/papers/2210.15191) for more details.

repetition_penalty (`float`, *optional*) : The parameter for repetition penalty. 1.0 means no penalty. See [this paper](https://huggingface.co/papers/1909.05858) for more details.

encoder_repetition_penalty (`float`, *optional*) : The parameter for encoder_repetition_penalty. An exponential penalty on sequences that are not in the original input. 1.0 means no penalty.

length_penalty (`float`, *optional*) : Exponential penalty to the length that is used with beam-based generation. It is applied as an exponent to the sequence length, which in turn is used to divide the score of the sequence. Since the score is the log likelihood of the sequence (i.e. negative), `length_penalty` > 0.0 promotes longer sequences, while `length_penalty` < 0.0 encourages shorter sequences.

no_repeat_ngram_size (`int`, *optional*) : If set to int > 0, all ngrams of that size can only occur once.

bad_words_ids (`list[list[int]]`, *optional*) : List of list of token ids that are not allowed to be generated. Check [NoBadWordsLogitsProcessor](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.NoBadWordsLogitsProcessor) for further documentation and examples.

renormalize_logits (`bool`) : Whether to renormalize the logits after applying all the logits processors (including the custom ones). It's highly recommended to set this flag to `True` as the search algorithms suppose the score logits are normalized but some logit processors break the normalization.

forced_bos_token_id (`int`, *optional*, defaults to `model.config.forced_bos_token_id`) : The id of the token to force as the first generated token after the `decoder_start_token_id`. Useful for multilingual models like [mBART](../model_doc/mbart) where the first generated token needs to be the target language token.

forced_eos_token_id (`int` or list[int]`, *optional*, defaults to `model.config.forced_eos_token_id`) : The id of the token to force as the last generated token when `max_length` is reached. Optionally, use a list to set multiple *end-of-sequence* tokens.

remove_invalid_values (`bool`) : Whether to remove possible *nan* and *inf* outputs of the model to prevent the generation method to crash. Note that using `remove_invalid_values` can slow down generation.

exponential_decay_length_penalty (`tuple(int, float)`, *optional*) : This Tuple adds an exponentially increasing length penalty, after a certain amount of tokens have been generated. The tuple shall consist of: `(start_index, decay_factor)` where `start_index` indicates where penalty starts and `decay_factor` represents the factor of exponential decay

suppress_tokens (`list[int]`, *optional*) : A list of tokens that will be suppressed at generation. The `SuppressTokens` logit processor will set their log probs to `-inf` so that they are not sampled.

begin_suppress_tokens  (`list[int]`, *optional*) : A list of tokens that will be suppressed at the beginning of the generation. The `SuppressBeginTokens` logit processor will set their log probs to `-inf` so that they are not sampled.

sequence_bias (`dict[tuple[int], float]`, *optional*)) : Dictionary that maps a sequence of tokens to its bias term. Positive biases increase the odds of the sequence being selected, while negative biases do the opposite. Check [SequenceBiasLogitsProcessor](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.SequenceBiasLogitsProcessor) for further documentation and examples.

token_healing (`bool`) : Heal tail tokens of prompts by replacing them with their appropriate extensions. This enhances the quality of completions for prompts affected by greedy tokenization bias.

guidance_scale (`float`, *optional*) : The guidance scale for classifier free guidance (CFG). CFG is enabled by setting `guidance_scale > 1`. Higher guidance scale encourages the model to generate samples that are more closely linked to the input prompt, usually at the expense of poorer quality.

watermarking_config (`BaseWatermarkingConfig` or `dict`, *optional*) : Arguments used to watermark the model outputs by adding a small bias to randomly selected set of "green" tokens. See the docs of [SynthIDTextWatermarkingConfig](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.SynthIDTextWatermarkingConfig) and [WatermarkingConfig](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.WatermarkingConfig) for more details. If passed as `Dict`, it will be converted to a `WatermarkingConfig` internally.

**Parameters that define the output variables of generate:**

num_return_sequences (`int`, *optional*) : The number of independently computed returned sequences for each element in the batch.

output_attentions (`bool`) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more details.

output_hidden_states (`bool`) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more details.

output_scores (`bool`) : Whether or not to return the prediction scores. See `scores` under returned tensors for more details.

output_logits (`bool`) : Whether or not to return the unprocessed prediction logit scores. See `logits` under returned tensors for more details.

return_dict_in_generate (`bool`) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput), as opposed to returning exclusively the generated sequence. This flag must be set to `True` to return the generation cache (when `use_cache` is `True`) or optional outputs (see flags starting with `output_`)

**Special tokens that can be used at generation time:**

pad_token_id (`int`, *optional*) : The id of the *padding* token.

bos_token_id (`int`, *optional*) : The id of the *beginning-of-sequence* token.

eos_token_id (`Union[int, list[int]]`, *optional*) : The id of the *end-of-sequence* token. Optionally, use a list to set multiple *end-of-sequence* tokens.

**Generation parameters exclusive to encoder-decoder models:**

encoder_no_repeat_ngram_size (`int`, *optional*) : If set to int > 0, all ngrams of that size that occur in the `encoder_input_ids` cannot occur in the `decoder_input_ids`.

decoder_start_token_id (`int` or `list[int]`, *optional*) : If an encoder-decoder model starts decoding with a different token than *bos*, the id of that token or a list of length `batch_size`. Indicating a list enables different start ids for each element in the batch (e.g. multilingual models with different target languages in one batch)

**Generation parameters exclusive to assistant generation:**

is_assistant (`bool`) : Whether the model is an assistant (draft) model.

num_assistant_tokens (`int`, *optional*) : Defines the number of _speculative tokens_ that shall be generated by the assistant model before being checked by the target model at each iteration. Higher values for `num_assistant_tokens` make the generation more _speculative_ : If the assistant model is performant larger speed-ups can be reached, if the assistant model requires lots of corrections, lower speed-ups are reached.

num_assistant_tokens_schedule (`str`, *optional*) : Defines the schedule at which max assistant tokens shall be changed during inference. - `"heuristic"`: When all speculative tokens are correct, increase `num_assistant_tokens` by 2 else reduce by 1. `num_assistant_tokens` value is persistent over multiple generation calls with the same assistant model. - `"heuristic_transient"`: Same as `"heuristic"` but `num_assistant_tokens` is reset to its initial value after each generation call. - `"constant"`: `num_assistant_tokens` stays unchanged during generation

assistant_confidence_threshold (`float`, *optional*) : The confidence threshold for the assistant model. If the assistant model's confidence in its prediction for the current token is lower than this threshold, the assistant model stops the current token generation iteration, even if the number of _speculative tokens_ (defined by `num_assistant_tokens`) is not yet reached. The assistant's confidence threshold is adjusted throughout the speculative iterations to reduce the number of unnecessary draft and target forward passes, biased towards avoiding false negatives. `assistant_confidence_threshold` value is persistent over multiple generation calls with the same assistant model. It is an unsupervised version of the dynamic speculation lookahead from Dynamic Speculation Lookahead Accelerates Speculative Decoding of Large Language Models .

prompt_lookup_num_tokens (`int`, *optional*) : The number of tokens to be output as candidate tokens.

max_matching_ngram_size (`int`, *optional*) : The maximum ngram size to be considered for matching in the prompt. Default to 2 if not provided.

assistant_early_exit(`int`, *optional*) : If set to a positive integer, early exit of the model will be used as an assistant. Can only be used with models that support early exit (i.e. models where logits from intermediate layers can be interpreted by the LM head).

assistant_lookbehind(`int`, *optional*) : If set to a positive integer, the re-encodeing process will additionally consider the last `assistant_lookbehind` assistant tokens to correctly align tokens. Can only be used with different tokenizers in speculative decoding. See this [blog](https://huggingface.co/blog/universal_assisted_generation) for more details.

target_lookbehind(`int`, *optional*) : If set to a positive integer, the re-encodeing process will additionally consider the last `target_lookbehind` target tokens to correctly align tokens. Can only be used with different tokenizers in speculative decoding. See this [blog](https://huggingface.co/blog/universal_assisted_generation) for more details.

assistant_ensemble_weight (`float`, *optional*) : Enables static ensemble verification in speculative decoding. If set to a value in `(0.0, 1.0)`, the verifier accepts tokens against the mixture `w * p_target + (1 - w) * q_draft` instead of `p_target`, trading a controlled distributional bias for a higher acceptance rate. Defaults to `None`, which keeps decoding lossless. Requires the assistant model to return logits, so it is not compatible with prompt lookup decoding.

speculation_type (`str`, *optional*) : The requested speculation type. Accepted values are `dflash`.

**Parameters related to performances and compilation:**

compile_config (CompileConfig, *optional*) : If using a compilable cache, this controls how `generate` will `compile` the forward pass for faster inference.

disable_compile (`bool`) : Whether to disable the automatic compilation of the forward pass. Automatic compilation happens when specific criteria are met, including using a compilable cache. Please open an issue if you find the need to use this flag.

Class that holds a configuration for a generation task. A `generate` call supports the following generation methods
for text-decoder, text-to-text, speech-to-text, and vision-to-text models:

- *greedy decoding* if `num_beams=1` and `do_sample=False`
- *multinomial sampling* if `num_beams=1` and `do_sample=True`
- *beam-search decoding* if `num_beams>1` and `do_sample=False`
- *beam-search multinomial sampling* if `num_beams>1` and `do_sample=True`
- *assisted decoding* if `assistant_model` or `prompt_lookup_num_tokens` is passed to `.generate()`

To learn more about decoding strategies refer to the [text generation strategies guide](../generation_strategies).

A large number of these flags control the logits or the stopping criteria of the generation. Make sure you check
the [generate-related classes](https://huggingface.co/docs/transformers/internal/generation_utils) for a full
description of the possible manipulations, as well as examples of their usage.

Note: the configuration fields that are still `None` will be overridden by `GenerationConfig._get_default_generation_params()`
during the generation loop. If you want to use different values for these fields, make sure to explicitly set them in the
generation config.

#### from_pretrained[[transformers.GenerationConfig.from_pretrained]]

```python
from_pretrained(pretrained_model_name: str | os.PathLike, config_file_name: str | os.PathLike | None = None, cache_dir: str | os.PathLike | None = None, force_download: bool = False, local_files_only: bool = False, token: str | bool | None = None, revision: str = 'main', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L931)

**Parameters:**

pretrained_model_name (`str` or `os.PathLike`) : This can be either:  - a string, the *model id* of a pretrained model configuration hosted inside a model repo on huggingface.co. - a path to a *directory* containing a configuration file saved using the [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig.save_pretrained) method, e.g., `./my_model_directory/`.

config_file_name (`str` or `os.PathLike`, *optional*, defaults to `"generation_config.json"`) : Name of the generation configuration JSON file to be loaded from `pretrained_model_name`.

cache_dir (`str` or `os.PathLike`, *optional*) : Path to a directory in which a downloaded pretrained model configuration should be cached if the standard cache should not be used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force to (re-)download the configuration files and override the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.

token (`str` or `bool`, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use the token generated when running `hf auth login` (stored in `~/.huggingface`).

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any identifier allowed by git.    To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.   

return_unused_kwargs (`bool`, *optional*, defaults to `False`) : If `False`, then this function returns just the final configuration object.  If `True`, then this functions returns a `Tuple(config, unused_kwargs)` where *unused_kwargs* is a dictionary consisting of the key/value pairs whose keys are not configuration attributes: i.e., the part of `kwargs` which has not been used to update `config` and is otherwise ignored.

subfolder (`str`, *optional*, defaults to `""`) : In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can specify the folder name here.

kwargs (`dict[str, Any]`, *optional*) : The values in kwargs of any keys which are configuration attributes will be used to override the loaded values. Behavior concerning key/value pairs whose keys are *not* configuration attributes is controlled by the `return_unused_kwargs` keyword parameter.

**Returns:** [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)

The configuration object instantiated from this pretrained model.

Instantiate a [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig) from a generation configuration file.

Examples:

```python
>>> from transformers import GenerationConfig

>>> # Download configuration from huggingface.co and cache.
>>> generation_config = GenerationConfig.from_pretrained("openai-community/gpt2")

>>> # E.g. config was saved using *save_pretrained('./test/saved_model/')*
>>> generation_config.save_pretrained("./test/saved_model/")
>>> generation_config = GenerationConfig.from_pretrained("./test/saved_model/")

>>> # You can also specify configuration names to your generation configuration file
>>> generation_config.save_pretrained("./test/saved_model/", config_file_name="my_configuration.json")
>>> generation_config = GenerationConfig.from_pretrained("./test/saved_model/", "my_configuration.json")

>>> # If you'd like to try a minor variation to an existing configuration, you can also pass generation
>>> # arguments to `.from_pretrained()`. Be mindful that typos and unused arguments will be ignored
>>> generation_config, unused_kwargs = GenerationConfig.from_pretrained(
...     "openai-community/gpt2", top_k=1, foo=False, do_sample=True, return_unused_kwargs=True
... )
>>> generation_config.top_k
1

>>> unused_kwargs
{'foo': False}
```

#### from_model_config[[transformers.GenerationConfig.from_model_config]]

```python
from_model_config(model_config: typing.Union[ForwardRef('PreTrainedConfig'), dict])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L1265)

**Parameters:**

model_config (`PreTrainedConfig | dict`) : The model config that will be used to instantiate the generation config.

**Returns:** [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)

The configuration object instantiated from those parameters.

Instantiates a [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig) from a [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig). This function is useful to convert legacy
[PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) objects, which may contain generation parameters, into a stand-alone [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig).

#### save_pretrained[[transformers.GenerationConfig.save_pretrained]]

```python
save_pretrained(save_directory: str | os.PathLike, config_file_name: str | os.PathLike | None = None, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L872)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the configuration JSON file will be saved (will be created if it does not exist).

config_file_name (`str` or `os.PathLike`, *optional*, defaults to `"generation_config.json"`) : Name of the generation configuration JSON file to be saved in `save_directory`.

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save a generation configuration object to the directory `save_directory`, so that it can be re-loaded using the
[from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig.from_pretrained) class method.

#### update[[transformers.GenerationConfig.update]]

```python
update(defaults_only = False, allow_custom_entries = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L1315)

**Parameters:**

defaults_only (`bool`, *optional*, defaults to `False`) : Whether to update all keys in config with `kwargs` or only those that are set to `None` (i.e. default value).

allow_custom_entries (`bool`, *optional*, defaults to `False`) : Whether to allow updating custom entries into the config with `kwargs` if not present in the current config.

kwargs (`dict[str, Any]`) : Dictionary of attributes to tentatively update this class.

**Returns:** `dict[str, Any]`

Dictionary containing all the key-value pairs that were not used to update the instance.

Updates attributes of this class instance with attributes from `kwargs` if they match existing attributes,
returning all the unused kwargs.

#### validate[[transformers.GenerationConfig.validate]]

```python
validate(strict = False, user_set_attributes: set[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L647)

**Parameters:**

strict (bool) : If True, raise an exception for any issues found. If False, only log issues.

user_set_attributes (set[str], *optional*) : Names of attributes the caller explicitly provided. When supplied, "minor issue" warnings about conflicting flag combinations (e.g. sampling-only flags set while `do_sample=False`) only fire if the conflicting flag is in this set -- avoiding noisy warnings when the value was inherited from a model's default `generation_config.json`. When `None`, all set attributes are considered user-set (backward-compatible behavior for direct `validate()` calls).

Validates the values of the attributes of the [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig) instance. Raises exceptions in the presence
of parameterization that can be detected as incorrect from the configuration instance alone.

Note that some parameters not validated here are best validated at generate runtime, as they may depend on
other inputs and/or the model, such as parameters related to the generation length.

#### get_generation_mode[[transformers.GenerationConfig.get_generation_mode]]

```python
get_generation_mode(assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/configuration_utils.py#L534)

**Parameters:**

assistant_model (`PreTrainedModel`, *optional*) : The assistant model to be used for assisted generation. If set, the generation mode will be assisted generation.

**Returns:** `GenerationMode`

The generation mode triggered by the instance.

Returns the generation mode triggered by the [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig) instance.

## GenerationMixin[[transformers.GenerationMixin]]

#### transformers.GenerationMixin[[transformers.GenerationMixin]]

```python
transformers.GenerationMixin()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/utils.py#L359)

A class containing all functions for auto-regressive text generation, to be used as a mixin in model classes.
Inheriting from this class causes the model to have special generation-related behavior, such as loading a
`GenerationConfig` at initialization time or ensuring `generate`-related tests are run in `transformers` CI.

A model class should inherit from `GenerationMixin` to enable calling methods like `generate`, or when it
has defined a custom `generate` method that relies on `GenerationMixin`, directly or indirectly, which
approximately shares the same interface to public methods like `generate`. Three examples:
- `LlamaForCausalLM` should inherit from `GenerationMixin` to enable calling `generate` and other public
  methods in the mixin;
- `BlipForQuestionAnswering` has a custom `generate` method that approximately shares the same interface as
  `GenerationMixin.generate` (it has a few extra arguments, and the same output). That function also calls
  `GenerationMixin.generate` indirectly, through an inner model. As such, `BlipForQuestionAnswering` should
  inherit from `GenerationMixin` to benefit from all generation-related automation in our codebase;
- `BarkModel` has a custom `generate` method and one of its inner models calls `GenerationMixin.generate`.
  However, its `generate` does not share the same interface as `GenerationMixin.generate`. In this case,
  `BarkModel` should NOT inherit from `GenerationMixin`, as it breaks the `generate` interface.

The class exposes [generate()](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationMixin.generate), which can be used for:
- *greedy decoding* if `num_beams=1` and `do_sample=False`
- *multinomial sampling* if `num_beams=1` and `do_sample=True`
- *beam-search decoding* if `num_beams>1` and `do_sample=False`
- *beam-search multinomial sampling* if `num_beams>1` and `do_sample=True`
- *assisted decoding* if `assistant_model` or `prompt_lookup_num_tokens` is passed to `.generate()`

To learn more about decoding strategies refer to the [text generation strategies guide](../generation_strategies).

#### generate[[transformers.GenerationMixin.generate]]

```python
generate(inputs: typing.Optional[torch.Tensor] = None, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, logits_processor: transformers.generation.logits_process.LogitsProcessorList | None = None, stopping_criteria: transformers.generation.stopping_criteria.StoppingCriteriaList | None = None, prefix_allowed_tokens_fn: collections.abc.Callable[[int, torch.Tensor], list[int]] | None = None, synced_gpus: bool | None = None, assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, negative_prompt_ids: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, custom_generate: str | collections.abc.Callable | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/utils.py#L2260)

**Parameters:**

inputs (`torch.Tensor` of varying shape depending on the modality, *optional*) : The sequence used as a prompt for the generation or as model inputs to the encoder. If `None` the method initializes it with `bos_token_id` and a batch size of 1. For decoder-only models `inputs` should be in the format of `input_ids`. For encoder-decoder models *inputs* can represent any of `input_ids`, `input_values`, `input_features`, or `pixel_values`.

generation_config ([GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : The generation configuration to be used as base parametrization for the generation call. `**kwargs` passed to generate matching the attributes of `generation_config` will override them. If `generation_config` is not provided, the default will be used, which has the following loading priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig)'s default values, whose documentation should be checked to parameterize generation.

logits_processor (`LogitsProcessorList`, *optional*) : Custom logits processors that complement the default logits processors built from arguments and generation config. If a logit processor is passed that is already created with the arguments or a generation config an error is thrown. This feature is intended for advanced users.

stopping_criteria (`StoppingCriteriaList`, *optional*) : Custom stopping criteria that complements the default stopping criteria built from arguments and a generation config. If a stopping criteria is passed that is already created with the arguments or a generation config an error is thrown. If your stopping criteria depends on the `scores` input, make sure you pass `return_dict_in_generate=True, output_scores=True` to `generate`. This feature is intended for advanced users.

prefix_allowed_tokens_fn (`Callable[[int, torch.Tensor], list[int]]`, *optional*) : If provided, this function constraints the beam search to allowed tokens only at each step. If not provided no constraint is applied. This function takes 2 arguments: the batch ID `batch_id` and `input_ids`. It has to return a list with the allowed tokens for the next generation step conditioned on the batch ID `batch_id` and the previously generated tokens `inputs_ids`. This argument is useful for constrained generation conditioned on the prefix, as described in [Autoregressive Entity Retrieval](https://huggingface.co/papers/2010.00904).

synced_gpus (`bool`, *optional*) : Whether to continue running the while loop until max_length. Unless overridden, this flag will be set to `True` if using `FullyShardedDataParallel` or DeepSpeed ZeRO Stage 3 with multiple GPUs to avoid deadlocking if one GPU finishes generating before other GPUs. Otherwise, defaults to `False`.

assistant_model (`PreTrainedModel`, *optional*) : An assistant model that can be used to accelerate generation. The assistant model must have the exact same tokenizer. The acceleration is achieved when forecasting candidate tokens with the assistant model is much faster than running generation with the model you're calling generate from. As such, the assistant model should be much smaller.

streamer (`BaseStreamer`, *optional*) : Streamer object that will be used to stream the generated sequences. Generated tokens are passed through `streamer.put(token_ids)` and the streamer is responsible for any further processing.

negative_prompt_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : The negative prompt needed for some processors such as CFG. The batch size must match the input batch size. This is an experimental feature, subject to breaking API changes in future versions.

negative_prompt_attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Attention_mask for `negative_prompt_ids`.

custom_generate (`str` or `Callable`, *optional*) : One of the following: - `str` (Hugging Face Hub repository name): runs the custom `generate` function defined at `custom_generate/generate.py` in that repository instead of the standard `generate` method. The repository fully replaces the generation logic, and the return type may differ. - `str` (local repository path): same as above but from a local path. Local directories also require `trust_remote_code=True` because the local `custom_generate/generate.py` is executed. - `Callable`: `generate` will perform the usual input preparation steps, then call the provided callable to run the decoding loop. For more information, see [the docs](../../generation_strategies#custom-generation-methods).

kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of `generation_config` and/or additional model-specific kwargs that will be forwarded to the `forward` function of the model. If the model is an encoder-decoder model, encoder specific kwargs should not be prefixed and decoder specific kwargs should be prefixed with *decoder_*.

**Returns:** [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) or `torch.LongTensor`

A [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) (if `return_dict_in_generate=True`
or when `config.return_dict_in_generate=True`) or a `torch.LongTensor`.

If the model is *not* an encoder-decoder model (`model.config.is_encoder_decoder=False`), the possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateDecoderOnlyOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateDecoderOnlyOutput),
- [GenerateBeamDecoderOnlyOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamDecoderOnlyOutput)

If the model is an encoder-decoder model (`model.config.is_encoder_decoder=True`), the possible
[ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) types are:

- [GenerateEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateEncoderDecoderOutput),
- [GenerateBeamEncoderDecoderOutput](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.generation.GenerateBeamEncoderDecoderOutput)

Generates sequences of token ids for models with a language modeling head.

Most generation-controlling parameters are set in `generation_config` which, if not passed, will be set to the
model's default generation configuration. You can override any `generation_config` by passing the corresponding
parameters to generate(), e.g. `.generate(inputs, num_beams=4, do_sample=True)`.

For an overview of generation strategies and code examples, check out the [following
guide](../generation_strategies).

#### compute_transition_scores[[transformers.GenerationMixin.compute_transition_scores]]

```python
compute_transition_scores(sequences: Tensor, scores: tuple, beam_indices: typing.Optional[torch.Tensor] = None, normalize_logits: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/utils.py#L1433)

**Parameters:**

sequences (`torch.LongTensor`) : The generated sequences. The second dimension (sequence_length) is either equal to `max_length` or shorter if all batches finished early due to the `eos_token_id`.

scores (`tuple(torch.FloatTensor)`) : Transition scores for each vocabulary token at each generation step. Beam transition scores consisting of log probabilities of tokens conditioned on log softmax of previously generated tokens in this beam. Tuple of `torch.FloatTensor` with up to `max_new_tokens` elements (one element for each generated token), with each tensor of shape `(batch_size*num_beams, config.vocab_size)`.

beam_indices (`torch.LongTensor`, *optional*) : Beam indices of generated token id at each generation step. `torch.LongTensor` of shape `(batch_size*num_return_sequences, sequence_length)`. Only required if a `num_beams>1` at generate-time.

normalize_logits (`bool`, *optional*, defaults to `False`) : Whether to normalize the logits (which, for legacy reasons, may be unnormalized).

**Returns:** `torch.Tensor`

A `torch.Tensor` of shape `(batch_size*num_return_sequences, sequence_length)` containing
the transition scores (logits)

Computes the transition scores of sequences given the generation scores (and beam indices, if beam search was
used). This is a convenient method to quickly obtain the scores of the selected tokens at generation time.

Examples:

```python
>>> from transformers import GPT2Tokenizer, AutoModelForCausalLM
>>> import numpy as np

>>> tokenizer = GPT2Tokenizer.from_pretrained("gpt2")
>>> model = AutoModelForCausalLM.from_pretrained("openai-community/gpt2")
>>> tokenizer.pad_token_id = tokenizer.eos_token_id
>>> inputs = tokenizer(["Today is"], return_tensors="pt")

>>> # Example 1: Print the scores for each token generated with Greedy Search
>>> outputs = model.generate(**inputs, max_new_tokens=5, return_dict_in_generate=True, output_scores=True)
>>> transition_scores = model.compute_transition_scores(
...     outputs.sequences, outputs.scores, normalize_logits=True
... )
>>> # input_length is the length of the input prompt for decoder-only models, like the GPT family, and 1 for
>>> # encoder-decoder models, like BART or T5.
>>> input_length = 1 if model.config.is_encoder_decoder else inputs.input_ids.shape[1]
>>> generated_tokens = outputs.sequences[:, input_length:]
>>> for tok, score in zip(generated_tokens[0], transition_scores[0]):
...     # | token | token string | log probability | probability
...     print(f"| {tok:5d} | {tokenizer.decode(tok):8s} | {score.numpy():.3f} | {np.exp(score.numpy()):.2%}")
|   262 |  the     | -1.414 | 24.33%
|  1110 |  day     | -2.609 | 7.36%
|   618 |  when    | -2.010 | 13.40%
|   356 |  we      | -1.859 | 15.58%
|   460 |  can     | -2.508 | 8.14%

>>> # Example 2: Reconstruct the sequence scores from Beam Search
>>> outputs = model.generate(
...     **inputs,
...     max_new_tokens=5,
...     num_beams=4,
...     num_return_sequences=4,
...     return_dict_in_generate=True,
...     output_scores=True,
... )
>>> transition_scores = model.compute_transition_scores(
...     outputs.sequences, outputs.scores, outputs.beam_indices, normalize_logits=False
... )
>>> # If you sum the generated tokens' scores and apply the length penalty, you'll get the sequence scores.
>>> # Tip 1: recomputing the scores is only guaranteed to match with `normalize_logits=False`. Depending on the
>>> # use case, you might want to recompute it with `normalize_logits=True`.
>>> # Tip 2: the output length does NOT include the input length
>>> output_length = np.sum(transition_scores.numpy() < 0, axis=1)
>>> length_penalty = model.generation_config.length_penalty
>>> reconstructed_scores = transition_scores.sum(axis=1) / (output_length**length_penalty)
>>> print(np.allclose(outputs.sequences_scores, reconstructed_scores))
True
```

## ContinuousMixin[[transformers.ContinuousMixin]]

#### transformers.ContinuousMixin[[transformers.ContinuousMixin]]

```python
transformers.ContinuousMixin()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1083)

Mixin class for models to add continuous batching capabilities. Continuous batching has three entry points:
- `init_continuous_batching`, which is the actual entry point for continuous batching
- `continuous_batching_context_manager`, which itself is a wrapper around `init_continuous_batching`
- `generate_batch`, which is really a wrapper around `continuous_batching_context_manager`

They are defined in this order. Any change made to any of those three entry points should be reflected in the other
two.

#### continuous_batching_context_manager[[transformers.ContinuousMixin.continuous_batching_context_manager]]

```python
continuous_batching_context_manager(generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, block: bool = True, timeout: float | None = None, continuous_batching_config: transformers.generation.configuration_utils.ContinuousBatchingConfig | None = None, persistent_manager: bool = False, warmup: bool = True, workload_hints: transformers.generation.continuous_batching.utils.WorkloadHints | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1161)

A context manager to safely use the continuous batching manager. Arguments are similar to the ones of
`init_continuous_batching`, except for:
- block: whether to block the thread when stopping the manager. Default is True.
- timeout: maximum time to wait for the thread to stop. Default is None (no timeout).
- warmup: whether to pre-capture CUDA graphs at the largest sizes before running. Default is True.

#### destroy_cached_continuous_batching_manager[[transformers.ContinuousMixin.destroy_cached_continuous_batching_manager]]

```python
destroy_cached_continuous_batching_manager()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1154)

Destroy the cached continuous batching manager and free GPU resources.

#### generate_batch[[transformers.ContinuousMixin.generate_batch]]

```python
generate_batch(inputs: list, generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, continuous_batching_config: transformers.generation.configuration_utils.ContinuousBatchingConfig | None = None, record_timestamps: bool = False, progress_bar: bool = True, persistent_manager: bool = False, warmup: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1201)

**Parameters:**

inputs : List of input token sequences (prompts)

generation_config : Optional generation configuration

continuous_batching_config : Optional continuous batching configuration

record_timestamps : If set to true, the requests will have a timestamp for each token generated

progress_bar : If set to true, a progress bar will be displayed

persistent_manager : whether to persist the manager after the generation is finished. Default is False.

warmup : whether to pre-capture CUDA graphs before processing requests. Default is True.

**Returns:** `dict[str, GenerationOutput]`

a dictionary of request ids to GenerationOutput objects

Generate sequences for a batch of prompts using continuous batching.

#### init_continuous_batching[[transformers.ContinuousMixin.init_continuous_batching]]

```python
init_continuous_batching(generation_config: transformers.generation.configuration_utils.GenerationConfig | None = None, continuous_batching_config: transformers.generation.configuration_utils.ContinuousBatchingConfig | None = None, workload_hints: transformers.generation.continuous_batching.utils.WorkloadHints | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L1095)

**Parameters:**

generation_config : An optional generation configuration, which may contain a CompileConfig object

continuous_batching_config : An optional continuous batching configuration

workload_hints : Optional WorkloadHints to help the continuous batching manager make better decisions for default values

**Returns:** `ContinuousBatchingManager`

The manager instance to add requests and retrieve results.

Initialize a manager for continuous batching inference.

## ContinuousBatchingManager[[transformers.ContinuousBatchingManager]]

#### transformers.ContinuousBatchingManager[[transformers.ContinuousBatchingManager]]

```python
transformers.ContinuousBatchingManager(model: ProtoPretrainedModel, generation_config: GenerationConfig, continuous_batching_config: ContinuousBatchingConfig, workload_hints: transformers.generation.continuous_batching.utils.WorkloadHints | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L553)

Manager for handling continuous batching of generation requests. It provides a user interface for submitting
generation requests, retrieving results, and managing the background generation thread. This class should not be
created directly, but through one of the following entry points (all methods of the `ContinuousMixin` mixin):
- `init_continuous_batching`
- `continuous_batching_context_manager`
- `generate_batch`

#### add_request[[transformers.ContinuousBatchingManager.add_request]]

```python
add_request(input_ids: list, request_id: str | None = None, max_new_tokens: int | None = None, streaming: bool = False, record_timestamps: bool = False, eos_token_id: int | list[int] | None = None, **logit_processor_kwargs: typing.Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L763)

**Parameters:**

input_ids : Input token IDs to use as prompt

request_id : Optional custom request ID (auto-generated if None)

max_new_tokens : Maximum number of new tokens to generate

streaming : Whether to stream tokens as they're generated

record_timestamps : Whether to record timestamps for each generated token

eos_token_id : End-of-sequence token ID(s)

logit_processor_kwargs : Keyword arguments for the logits processor.

**Returns:** `str | None`

The request ID if the process is a TP driver, None otherwise.

Add a new generation request to the queue. If the process is not a TP driver, this is a no-op.

#### add_requests[[transformers.ContinuousBatchingManager.add_requests]]

```python
add_requests(inputs: list, max_new_tokens: int | None = None, streaming: bool = False, record_timestamps: bool = False, **logit_processor_kwargs: typing.Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L820)

Utility function to batch `add_request` and return their IDs. Check its documentation for more details.

#### cancel_request[[transformers.ContinuousBatchingManager.cancel_request]]

```python
cancel_request(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L855)

Cancel a request by its ID. If this called from a process that is not a TP driver, it's a no-op: only TP
driver processes interact with the manager.

#### destroy[[transformers.ContinuousBatchingManager.destroy]]

```python
destroy()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L754)

Terminate the manager and release distributed resources. Safe to call multiple times. After calling this,
the manager cannot be restarted.

#### get_result[[transformers.ContinuousBatchingManager.get_result]]

```python
get_result(request_id: str | None = None, timeout: float | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L864)

Retrieve one result from the output queue. If an ID is provided, returns the first matching request. If a
timeout is provided, returns None after the timeout (in seconds).

#### is_running[[transformers.ContinuousBatchingManager.is_running]]

```python
is_running()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L675)

Returns True if the background generation thread has been started and is still alive.

#### join[[transformers.ContinuousBatchingManager.join]]

```python
join(stop_trigger_time: float, timeout: float | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L739)

Wait for the background thread to finish. Wait can be capped using the timeout argument (in seconds).

#### register_result_handler[[transformers.ContinuousBatchingManager.register_result_handler]]

```python
register_result_handler(request_id: str, callback: Callable)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L898)

**Parameters:**

request_id (*str*) : The request ID to receive outputs for.

callback (*callable*) : Called with a `GenerationOutput` for each result.

Register a callback for result delivery (streaming or non-streaming).

The callback is invoked on the event loop via `call_soon_threadsafe` each time a result is produced for this
request. For streaming requests, this happens on every token; for non-streaming, only on completion. The handler
is automatically cleaned up when the request finishes.

#### request_id_iter[[transformers.ContinuousBatchingManager.request_id_iter]]

```python
request_id_iter(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L885)

Iterate over results matching a specific request id (blocking).

Uses the shared output queue with requeue. For high-concurrency serving,
use `register_result_handler` instead.

#### start[[transformers.ContinuousBatchingManager.start]]

```python
start()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L679)

Start the background generation thread.

#### stop[[transformers.ContinuousBatchingManager.stop]]

```python
stop(block: bool = True, timeout: float | None = None, keep_for_next_session: bool = False, hard_stop: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L689)

Stop the background generation thread. If the `block` flag is set to True, then this method waits for the
thread to stop for a maximum time of `timeout` seconds (None means no timeout). If the `keep_for_next_session`
flag is set to True, then the manager is cached on the model for future use. If the `hard_stop` flag is set,
the background generation thread will be stopped immediately and pending requests will be failed.

#### switch_to_cb_friendly_attn[[transformers.ContinuousBatchingManager.switch_to_cb_friendly_attn]]

```python
switch_to_cb_friendly_attn(model: ProtoPretrainedModel)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L631)

Switch the attn implementation to one that is CB friendly: try to find a flash implementation if flash is
requested and, in any cases, switch to a paged implementation.

#### warmup[[transformers.ContinuousBatchingManager.warmup]]

```python
warmup()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/continuous_api.py#L665)

Pre-capture CUDA graphs for varlen and decode paths by running dummy batches. Initializes the batch
processor if not already done.

## Scheduler[[transformers.generation.Scheduler]]

#### transformers.generation.Scheduler[[transformers.generation.Scheduler]]

```python
transformers.generation.Scheduler(cache: PagedAttentionCache, safety_margin: float, max_requests_per_batch: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L22)

Abstract base class for scheduling requests in the continuous batch processor. Schedulers manage the lifecycle of
requests from when they are added to the waiting queue to when they are scheduled for processing. Different
schedulers implement different strategies for prioritizing and batching requests.

#### add_waiting_request[[transformers.generation.Scheduler.add_waiting_request]]

```python
add_waiting_request(state: RequestState)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L61)

Adds a request to the waiting list.

#### clear_cancelled_requests[[transformers.generation.Scheduler.clear_cancelled_requests]]

```python
clear_cancelled_requests()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L99)

Remove all cancelled requests from active and waiting queues.

#### finish_request[[transformers.generation.Scheduler.finish_request]]

```python
finish_request(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L81)

Completes processing of a request and frees its allocated cache blocks. This method is called
when a request has finished generation or encountered an error.

#### get_active_request_static_outputs[[transformers.generation.Scheduler.get_active_request_static_outputs]]

```python
get_active_request_static_outputs(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L88)

Gets generated tokens for an active request.

#### has_pending_requests[[transformers.generation.Scheduler.has_pending_requests]]

```python
has_pending_requests()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L77)

Checks if there are requests ready to be processed.

#### request_is_cancelled[[transformers.generation.Scheduler.request_is_cancelled]]

```python
request_is_cancelled(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L116)

Checks if a request has been cancelled or removed.

#### reset[[transformers.generation.Scheduler.reset]]

```python
reset()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L49)

Reset scheduler state for a new generation loop.

#### schedule_batch[[transformers.generation.Scheduler.schedule_batch]]

```python
schedule_batch(token_budget: int, cache_budget: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L66)

Schedules requests for the next batch based on available token and cache budgets. This method selects which
requests should be processed in the current batch, considering the budgets and the scheduler's prioritization
rules. The token_budget is the maximum number of tokens that can be processed in a batch, and the cache_budget
is the maximum number of KV cache entries that can be read in a batch.
Returns the list of scheduled requests in their "FutureRequestState" form, a boolean indicating if the decode
fast path can be used, the total number of query tokens and the maximum number of kv tokens read.

#### set_request_cancellation[[transformers.generation.Scheduler.set_request_cancellation]]

```python
set_request_cancellation(request_id: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L94)

Marks a request for cancellation.

## FIFOScheduler[[transformers.generation.FIFOScheduler]]

#### transformers.generation.FIFOScheduler[[transformers.generation.FIFOScheduler]]

```python
transformers.generation.FIFOScheduler(cache: PagedAttentionCache, safety_margin: float | None, max_requests_per_batch: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L331)

This scheduler processes requests in the order they arrive, meaning decoding requests has priority over
prefilling requests.

## PrefillFirstScheduler[[transformers.generation.PrefillFirstScheduler]]

#### transformers.generation.PrefillFirstScheduler[[transformers.generation.PrefillFirstScheduler]]

```python
transformers.generation.PrefillFirstScheduler(cache: PagedAttentionCache, safety_margin: float | None, max_requests_per_batch: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/generation/continuous_batching/scheduler.py#L380)

Scheduler that prioritizes split prefill requests over decoding requests. This scheduler ensures that split
prefill requests (which are continuations of partially processed prompts) are completed before processing new
decoding requests.
