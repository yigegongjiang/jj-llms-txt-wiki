# generation/logits_process

* [generation/logits_process](#module_generation/logits_process)
    * [.LogitsProcessor](#module_generation/logits_process.LogitsProcessor)
        * *[`._call(input_ids, logits)`](#module_generation/logits_process.LogitsProcessor+_call)*
    * [.LogitsWarper](#module_generation/logits_process.LogitsWarper)
        * *[`._call(input_ids, logits)`](#module_generation/logits_process.LogitsWarper+_call)*
    * [.LogitsProcessorList](#module_generation/logits_process.LogitsProcessorList)
        * [`new LogitsProcessorList()`](#new_module_generation/logits_process.LogitsProcessorList_new)
        * [`.push(item)`](#module_generation/logits_process.LogitsProcessorList+push)
        * [`.extend(items)`](#module_generation/logits_process.LogitsProcessorList+extend)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.LogitsProcessorList+_call)
    * [.ForcedBOSTokenLogitsProcessor](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor)
        * [`new ForcedBOSTokenLogitsProcessor(bos_token_id)`](#new_module_generation/logits_process.ForcedBOSTokenLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.ForcedEOSTokenLogitsProcessor](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor)
        * [`new ForcedEOSTokenLogitsProcessor(max_length, eos_token_id)`](#new_module_generation/logits_process.ForcedEOSTokenLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor+_call)
    * [.SuppressTokensLogitsProcessor](#module_generation/logits_process.SuppressTokensLogitsProcessor)
        * [`new SuppressTokensLogitsProcessor(suppress_tokens)`](#new_module_generation/logits_process.SuppressTokensLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.SuppressTokensLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.SuppressTokensAtBeginLogitsProcessor](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor)
        * [`new SuppressTokensAtBeginLogitsProcessor(begin_suppress_tokens, begin_index)`](#new_module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.WhisperTimeStampLogitsProcessor](#module_generation/logits_process.WhisperTimeStampLogitsProcessor)
        * [`new WhisperTimeStampLogitsProcessor(generate_config, init_tokens)`](#new_module_generation/logits_process.WhisperTimeStampLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.WhisperTimeStampLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)
        * [`new NoRepeatNGramLogitsProcessor(no_repeat_ngram_size)`](#new_module_generation/logits_process.NoRepeatNGramLogitsProcessor_new)
        * [`.getNgrams(prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+getNgrams) ⇒ Map.&lt;string, Array&gt;
        * [`.getGeneratedNgrams(bannedNgrams, prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+getGeneratedNgrams) ⇒ Array
        * [`.calcBannedNgramTokens(prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+calcBannedNgramTokens) ⇒ Array
        * [`._call(input_ids, logits)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.RepetitionPenaltyLogitsProcessor](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor)
        * [`new RepetitionPenaltyLogitsProcessor(penalty)`](#new_module_generation/logits_process.RepetitionPenaltyLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.MinLengthLogitsProcessor](#module_generation/logits_process.MinLengthLogitsProcessor)
        * [`new MinLengthLogitsProcessor(min_length, eos_token_id)`](#new_module_generation/logits_process.MinLengthLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.MinLengthLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.MinNewTokensLengthLogitsProcessor](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor)
        * [`new MinNewTokensLengthLogitsProcessor(prompt_length_to_skip, min_new_tokens, eos_token_id)`](#new_module_generation/logits_process.MinNewTokensLengthLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.NoBadWordsLogitsProcessor](#module_generation/logits_process.NoBadWordsLogitsProcessor)
        * [`new NoBadWordsLogitsProcessor(bad_words_ids, eos_token_id)`](#new_module_generation/logits_process.NoBadWordsLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.NoBadWordsLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.ClassifierFreeGuidanceLogitsProcessor](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor)
        * [`new ClassifierFreeGuidanceLogitsProcessor(guidance_scale)`](#new_module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor+_call) ⇒ [Tensor](#Tensor)
    * [.TemperatureLogitsWarper](#module_generation/logits_process.TemperatureLogitsWarper)
        * [`new TemperatureLogitsWarper(temperature)`](#new_module_generation/logits_process.TemperatureLogitsWarper_new)
        * [`._call(input_ids, logits)`](#module_generation/logits_process.TemperatureLogitsWarper+_call) ⇒ [Tensor](#Tensor)
    * [.TopPLogitsWarper](#module_generation/logits_process.TopPLogitsWarper)
        * [`new TopPLogitsWarper(top_p, options)`](#new_module_generation/logits_process.TopPLogitsWarper_new)
    * [.TopKLogitsWarper](#module_generation/logits_process.TopKLogitsWarper)
        * [`new TopKLogitsWarper(top_k, options)`](#new_module_generation/logits_process.TopKLogitsWarper_new)

* * *

## generation/logits_process.LogitsProcessor

Abstract base class for all logit processors that can be applied during generation.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* * *

### *`logitsProcessor._call(input_ids, logits)`*

Apply the processor to the input logits.

**Kind**: instance abstract method of [LogitsProcessor](#module_generation/logits_process.LogitsProcessor)  
**Throws**:

- Error Throws an error if `_call` is not implemented in the subclass.

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input ids.

    
    logitsTensorThe logits to process.

      

* * *

## generation/logits_process.LogitsWarper

Abstract base class for all logit warpers that can be applied during generation with multinomial sampling.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* * *

### *`logitsWarper._call(input_ids, logits)`*

Apply the processor to the input logits.

**Kind**: instance abstract method of [LogitsWarper](#module_generation/logits_process.LogitsWarper)  
**Throws**:

- Error Throws an error if `_call` is not implemented in the subclass.

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input ids.

    
    logitsTensorThe logits to process.

      

* * *

## generation/logits_process.LogitsProcessorList

A class representing a list of logits processors. A logits processor is a function that modifies the logits
output of a language model. This class provides methods for adding new processors and applying all processors to a
batch of logits.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.LogitsProcessorList](#module_generation/logits_process.LogitsProcessorList)
    * [`new LogitsProcessorList()`](#new_module_generation/logits_process.LogitsProcessorList_new)
    * [`.push(item)`](#module_generation/logits_process.LogitsProcessorList+push)
    * [`.extend(items)`](#module_generation/logits_process.LogitsProcessorList+extend)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.LogitsProcessorList+_call)

* * *

### `new LogitsProcessorList()`

Constructs a new instance of `LogitsProcessorList`.

* * *

### `logitsProcessorList.push(item)`

Adds a new logits processor to the list.

**Kind**: instance method of [LogitsProcessorList](#module_generation/logits_process.LogitsProcessorList)  

  
    
      ParamTypeDescription
    
  
  

    itemLogitsProcessorThe logits processor function to add.

      

* * *

### `logitsProcessorList.extend(items)`

Adds multiple logits processors to the list.

**Kind**: instance method of [LogitsProcessorList](#module_generation/logits_process.LogitsProcessorList)  

  
    
      ParamTypeDescription
    
  
  

    itemsArrayThe logits processor functions to add.

      

* * *

### `logitsProcessorList._call(input_ids, logits)`

Applies all logits processors in the list to a batch of logits, modifying them in-place.

**Kind**: instance method of [LogitsProcessorList](#module_generation/logits_process.LogitsProcessorList)  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs for the language model.

    
    logitsTensor
      

* * *

## generation/logits_process.ForcedBOSTokenLogitsProcessor

A LogitsProcessor that forces a BOS token at the beginning of the generated sequence.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.ForcedBOSTokenLogitsProcessor](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor)
    * [`new ForcedBOSTokenLogitsProcessor(bos_token_id)`](#new_module_generation/logits_process.ForcedBOSTokenLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new ForcedBOSTokenLogitsProcessor(bos_token_id)`

Create a ForcedBOSTokenLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    bos_token_idnumberThe ID of the beginning-of-sequence token to be forced.

      

* * *

### `forcedBOSTokenLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply the BOS token forcing to the logits.

**Kind**: instance method of [ForcedBOSTokenLogitsProcessor](#module_generation/logits_process.ForcedBOSTokenLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The logits with BOS token forcing.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.ForcedEOSTokenLogitsProcessor

A logits processor that enforces the specified token as the last generated token when `max_length` is reached.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.ForcedEOSTokenLogitsProcessor](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor)
    * [`new ForcedEOSTokenLogitsProcessor(max_length, eos_token_id)`](#new_module_generation/logits_process.ForcedEOSTokenLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor+_call)

* * *

### `new ForcedEOSTokenLogitsProcessor(max_length, eos_token_id)`

Create a ForcedEOSTokenLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    max_lengthnumberThe maximum length of the sequence to be generated.

    
    eos_token_idnumber | ArrayThe id(s) of the end-of-sequence token.

      

* * *

### `forcedEOSTokenLogitsProcessor._call(input_ids, logits)`

Apply the processor to input_ids and logits.

**Kind**: instance method of [ForcedEOSTokenLogitsProcessor](#module_generation/logits_process.ForcedEOSTokenLogitsProcessor)  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input ids.

    
    logitsTensorThe logits tensor.

      

* * *

## generation/logits_process.SuppressTokensLogitsProcessor

A LogitsProcessor that suppresses a list of tokens throughout generation.
Sets their log probs to `-inf` so that they are not generated.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.SuppressTokensLogitsProcessor](#module_generation/logits_process.SuppressTokensLogitsProcessor)
    * [`new SuppressTokensLogitsProcessor(suppress_tokens)`](#new_module_generation/logits_process.SuppressTokensLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.SuppressTokensLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new SuppressTokensLogitsProcessor(suppress_tokens)`

Create a SuppressTokensLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    suppress_tokensArrayThe IDs of the tokens to suppress.

      

* * *

### `suppressTokensLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Suppress the specified tokens by setting their logits to -Infinity.

**Kind**: instance method of [SuppressTokensLogitsProcessor](#module_generation/logits_process.SuppressTokensLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The modified logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.SuppressTokensAtBeginLogitsProcessor

A LogitsProcessor that suppresses a list of tokens as soon as the `generate` function starts
generating using `begin_index` tokens. This should ensure that the tokens defined by
`begin_suppress_tokens` at not sampled at the begining of the generation.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.SuppressTokensAtBeginLogitsProcessor](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor)
    * [`new SuppressTokensAtBeginLogitsProcessor(begin_suppress_tokens, begin_index)`](#new_module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new SuppressTokensAtBeginLogitsProcessor(begin_suppress_tokens, begin_index)`

Create a SuppressTokensAtBeginLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    begin_suppress_tokensArrayThe IDs of the tokens to suppress.

    
    begin_indexnumberThe number of tokens to generate before suppressing tokens.

      

* * *

### `suppressTokensAtBeginLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply the BOS token forcing to the logits.

**Kind**: instance method of [SuppressTokensAtBeginLogitsProcessor](#module_generation/logits_process.SuppressTokensAtBeginLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The logits with BOS token forcing.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.WhisperTimeStampLogitsProcessor

A LogitsProcessor that handles adding timestamps to generated text.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.WhisperTimeStampLogitsProcessor](#module_generation/logits_process.WhisperTimeStampLogitsProcessor)
    * [`new WhisperTimeStampLogitsProcessor(generate_config, init_tokens)`](#new_module_generation/logits_process.WhisperTimeStampLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.WhisperTimeStampLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new WhisperTimeStampLogitsProcessor(generate_config, init_tokens)`

Constructs a new WhisperTimeStampLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    generate_configWhisperGenerationConfigThe config object passed to the generate() method of a transformer model.

    
    init_tokensArrayThe initial tokens of the input sequence.

      

* * *

### `whisperTimeStampLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Modify the logits to handle timestamp tokens.

**Kind**: instance method of [WhisperTimeStampLogitsProcessor](#module_generation/logits_process.WhisperTimeStampLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The modified logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input sequence of tokens.

    
    logitsTensorThe logits output by the model.

      

* * *

## generation/logits_process.NoRepeatNGramLogitsProcessor

A logits processor that disallows ngrams of a certain size to be repeated.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)
    * [`new NoRepeatNGramLogitsProcessor(no_repeat_ngram_size)`](#new_module_generation/logits_process.NoRepeatNGramLogitsProcessor_new)
    * [`.getNgrams(prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+getNgrams) ⇒ Map.&lt;string, Array&gt;
    * [`.getGeneratedNgrams(bannedNgrams, prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+getGeneratedNgrams) ⇒ Array
    * [`.calcBannedNgramTokens(prevInputIds)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+calcBannedNgramTokens) ⇒ Array
    * [`._call(input_ids, logits)`](#module_generation/logits_process.NoRepeatNGramLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new NoRepeatNGramLogitsProcessor(no_repeat_ngram_size)`

Create a NoRepeatNGramLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    no_repeat_ngram_sizenumberThe no-repeat-ngram size. All ngrams of this size can only occur once.

      

* * *

### `noRepeatNGramLogitsProcessor.getNgrams(prevInputIds)` ⇒ Map.&lt;string, Array&gt;

Generate n-grams from a sequence of token ids.

**Kind**: instance method of [NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)  
**Returns**: Map.&lt;string, Array&gt; - Map of generated n-grams  

  
    
      ParamTypeDescription
    
  
  

    prevInputIdsArrayList of previous input ids

      

* * *

### `noRepeatNGramLogitsProcessor.getGeneratedNgrams(bannedNgrams, prevInputIds)` ⇒ Array

Generate n-grams from a sequence of token ids.

**Kind**: instance method of [NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)  
**Returns**: Array - Map of generated n-grams  

  
    
      ParamTypeDescription
    
  
  

    bannedNgramsMap.&lt;string, Array&gt;Map of banned n-grams

    
    prevInputIdsArrayList of previous input ids

      

* * *

### `noRepeatNGramLogitsProcessor.calcBannedNgramTokens(prevInputIds)` ⇒ Array

Calculate banned n-gram tokens

**Kind**: instance method of [NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)  
**Returns**: Array - Map of generated n-grams  

  
    
      ParamTypeDescription
    
  
  

    prevInputIdsArrayList of previous input ids

      

* * *

### `noRepeatNGramLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply the no-repeat-ngram processor to the logits.

**Kind**: instance method of [NoRepeatNGramLogitsProcessor](#module_generation/logits_process.NoRepeatNGramLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The logits with no-repeat-ngram processing.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.RepetitionPenaltyLogitsProcessor

A logits processor that prevents the repetition of previous tokens through a penalty.
This penalty is applied at most once per token. Note that, for decoder-only models like most LLMs,
the considered tokens include the prompt.

In the original [paper](https://huggingface.co/papers/1909.05858), the authors suggest the use of a
penalty of around 1.2 to achieve a good balance between truthful generation and lack of repetition.
To penalize and reduce repetition, use `penalty` values above 1.0, where a higher value penalizes
more strongly. To reward and encourage repetition, use `penalty` values between 0.0 and 1.0, where
a lower value rewards more strongly.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.RepetitionPenaltyLogitsProcessor](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor)
    * [`new RepetitionPenaltyLogitsProcessor(penalty)`](#new_module_generation/logits_process.RepetitionPenaltyLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new RepetitionPenaltyLogitsProcessor(penalty)`

Create a RepetitionPenaltyLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    penaltynumberThe parameter for repetition penalty.

1.0 means no penalty. Above 1.0 penalizes previously generated tokens.
Between 0.0 and 1.0 rewards previously generated tokens.

      

* * *

### `repetitionPenaltyLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply the repetition penalty to the logits.

**Kind**: instance method of [RepetitionPenaltyLogitsProcessor](#module_generation/logits_process.RepetitionPenaltyLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The logits with repetition penalty processing.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.MinLengthLogitsProcessor

A logits processor that enforces a minimum number of tokens.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.MinLengthLogitsProcessor](#module_generation/logits_process.MinLengthLogitsProcessor)
    * [`new MinLengthLogitsProcessor(min_length, eos_token_id)`](#new_module_generation/logits_process.MinLengthLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.MinLengthLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new MinLengthLogitsProcessor(min_length, eos_token_id)`

Create a MinLengthLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    min_lengthnumberThe minimum length below which the score of eos_token_id is set to negative infinity.

    
    eos_token_idnumber | ArrayThe ID/IDs of the end-of-sequence token.

      

* * *

### `minLengthLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply logit processor.

**Kind**: instance method of [MinLengthLogitsProcessor](#module_generation/logits_process.MinLengthLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The processed logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.MinNewTokensLengthLogitsProcessor

A logits processor that enforces a minimum number of new tokens.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.MinNewTokensLengthLogitsProcessor](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor)
    * [`new MinNewTokensLengthLogitsProcessor(prompt_length_to_skip, min_new_tokens, eos_token_id)`](#new_module_generation/logits_process.MinNewTokensLengthLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new MinNewTokensLengthLogitsProcessor(prompt_length_to_skip, min_new_tokens, eos_token_id)`

Create a MinNewTokensLengthLogitsProcessor.

  
    
      ParamTypeDescription
    
  
  

    prompt_length_to_skipnumberThe input tokens length.

    
    min_new_tokensnumberThe minimum new tokens length below which the score of eos_token_id is set to negative infinity.

    
    eos_token_idnumber | ArrayThe ID/IDs of the end-of-sequence token.

      

* * *

### `minNewTokensLengthLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply logit processor.

**Kind**: instance method of [MinNewTokensLengthLogitsProcessor](#module_generation/logits_process.MinNewTokensLengthLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The processed logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.NoBadWordsLogitsProcessor

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.NoBadWordsLogitsProcessor](#module_generation/logits_process.NoBadWordsLogitsProcessor)
    * [`new NoBadWordsLogitsProcessor(bad_words_ids, eos_token_id)`](#new_module_generation/logits_process.NoBadWordsLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.NoBadWordsLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new NoBadWordsLogitsProcessor(bad_words_ids, eos_token_id)`

Create a `NoBadWordsLogitsProcessor`.

  
    
      ParamTypeDescription
    
  
  

    bad_words_idsArrayList of list of token ids that are not allowed to be generated.

    
    eos_token_idnumber | ArrayThe id of the end-of-sequence token. Optionally, use a list to set multiple end-of-sequence tokens.

      

* * *

### `noBadWordsLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply logit processor.

**Kind**: instance method of [NoBadWordsLogitsProcessor](#module_generation/logits_process.NoBadWordsLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The processed logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.ClassifierFreeGuidanceLogitsProcessor

[`LogitsProcessor`] for classifier free guidance (CFG). The scores are split over the batch dimension,
where the first half correspond to the conditional logits (predicted from the input prompt) and the second half
correspond to the unconditional logits (predicted from an empty or 'null' prompt). The processor computes a
weighted average across the conditional and unconditional logits, parameterised by the `guidance_scale`.

See [the paper](https://huggingface.co/papers/2306.05284) for more information.

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.ClassifierFreeGuidanceLogitsProcessor](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor)
    * [`new ClassifierFreeGuidanceLogitsProcessor(guidance_scale)`](#new_module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor+_call) ⇒ [Tensor](#Tensor)

* * *

### `new ClassifierFreeGuidanceLogitsProcessor(guidance_scale)`

Create a `ClassifierFreeGuidanceLogitsProcessor`.

  
    
      ParamTypeDescription
    
  
  

    guidance_scalenumberThe guidance scale for classifier free guidance (CFG). CFG is enabled by setting guidance_scale &gt; 1.
Higher guidance scale encourages the model to generate samples that are more closely linked to the input
prompt, usually at the expense of poorer quality.

      

* * *

### `classifierFreeGuidanceLogitsProcessor._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply logit processor.

**Kind**: instance method of [ClassifierFreeGuidanceLogitsProcessor](#module_generation/logits_process.ClassifierFreeGuidanceLogitsProcessor)  
**Returns**: [Tensor](#Tensor) - The processed logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.TemperatureLogitsWarper

[`LogitsWarper`] for temperature (exponential scaling output probability distribution), which effectively means
that it can control the randomness of the predicted tokens. Often used together with [`TopPLogitsWarper`] and [`TopKLogitsWarper`].

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* [.TemperatureLogitsWarper](#module_generation/logits_process.TemperatureLogitsWarper)
    * [`new TemperatureLogitsWarper(temperature)`](#new_module_generation/logits_process.TemperatureLogitsWarper_new)
    * [`._call(input_ids, logits)`](#module_generation/logits_process.TemperatureLogitsWarper+_call) ⇒ [Tensor](#Tensor)

* * *

### `new TemperatureLogitsWarper(temperature)`

Create a `TemperatureLogitsWarper`.

  
    
      ParamTypeDescription
    
  
  

    temperaturenumberStrictly positive float value used to modulate the logits distribution.
A value smaller than 1 decreases randomness (and vice versa), with 0 being equivalent to shifting
all probability mass to the most likely token.

      

* * *

### `temperatureLogitsWarper._call(input_ids, logits)` ⇒ [Tensor](#Tensor)

Apply logit warper.

**Kind**: instance method of [TemperatureLogitsWarper](#module_generation/logits_process.TemperatureLogitsWarper)  
**Returns**: [Tensor](#Tensor) - The processed logits.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArrayThe input IDs.

    
    logitsTensorThe logits.

      

* * *

## generation/logits_process.TopPLogitsWarper

[`LogitsWarper`] that performs top-p, i.e. restricting to top tokens summing to prob_cut_off generation/logits_process](#module_generation/logits_process)  

* * *

### `new TopPLogitsWarper(top_p, options)`

Create a `TopPLogitsWarper`.

  
    
      ParamTypeDefaultDescription
    
  
  

    top_pnumberIf set to &lt; 1, only the smallest set of most probable tokens with
probabilities that add up to top_p or higher are kept for generation.

    
    optionsObjectAdditional options for the top-p sampling.

    
    [options.filter_value]number-InfinityAll filtered values will be set to this float value.

    
    [options.min_tokens_to_keep]number1Minimum number of tokens that cannot be filtered.

      

* * *

## generation/logits_process.TopKLogitsWarper

[`LogitsWarper`] that performs top-k, i.e. restricting to the k highest probability elements.
Often used together with [`TemperatureLogitsWarper`] and [`TopPLogitsWarper`].

**Kind**: static class of [generation/logits_process](#module_generation/logits_process)  

* * *

### `new TopKLogitsWarper(top_k, options)`

Create a `TopKLogitsWarper`.

  
    
      ParamTypeDefaultDescription
    
  
  

    top_knumberIf set to &gt; 0, only the top top_k tokens are kept for generation.

    
    optionsObjectAdditional options for the top-k sampling.

    
    [options.filter_value]number-InfinityAll filtered values will be set to this float value.

    
    [options.min_tokens_to_keep]number1Minimum number of tokens that cannot be filtered.

      

* * *
