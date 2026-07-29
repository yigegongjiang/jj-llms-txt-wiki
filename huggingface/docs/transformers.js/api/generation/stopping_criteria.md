# generation/stopping_criteria

* [generation/stopping_criteria](#module_generation/stopping_criteria)
    * [.StoppingCriteria](#module_generation/stopping_criteria.StoppingCriteria)
        * [`._call(input_ids, scores)`](#module_generation/stopping_criteria.StoppingCriteria+_call) ⇒ Array
    * [.StoppingCriteriaList](#module_generation/stopping_criteria.StoppingCriteriaList)
        * [`new StoppingCriteriaList()`](#new_module_generation/stopping_criteria.StoppingCriteriaList_new)
        * [`.push(item)`](#module_generation/stopping_criteria.StoppingCriteriaList+push)
        * [`.extend(items)`](#module_generation/stopping_criteria.StoppingCriteriaList+extend)
    * [.MaxLengthCriteria](#module_generation/stopping_criteria.MaxLengthCriteria)
        * [`new MaxLengthCriteria(max_length, [max_position_embeddings])`](#new_module_generation/stopping_criteria.MaxLengthCriteria_new)
    * [.EosTokenCriteria](#module_generation/stopping_criteria.EosTokenCriteria)
        * [`new EosTokenCriteria(eos_token_id)`](#new_module_generation/stopping_criteria.EosTokenCriteria_new)
        * [`._call(input_ids, scores)`](#module_generation/stopping_criteria.EosTokenCriteria+_call) ⇒ Array
    * [.InterruptableStoppingCriteria](#module_generation/stopping_criteria.InterruptableStoppingCriteria)

* * *

## generation/stopping_criteria.StoppingCriteria

Abstract base class for all stopping criteria that can be applied during generation.

**Kind**: static class of [generation/stopping_criteria](#module_generation/stopping_criteria)  

* * *

### `stoppingCriteria._call(input_ids, scores)` ⇒ Array

**Kind**: instance method of [StoppingCriteria](#module_generation/stopping_criteria.StoppingCriteria)  
**Returns**: Array - A list of booleans indicating whether each sequence should be stopped.  

  
    
      ParamTypeDescription
    
  
  

    input_idsArray(number[][] of shape (batch_size, sequence_length)):
Indices of input sequence tokens in the vocabulary.

    
    scoresArrayscores (number[][] of shape (batch_size, config.vocab_size)):
Prediction scores of a language modeling head. These can be scores for each vocabulary token before SoftMax
or scores for each vocabulary token after SoftMax.

      

* * *

## generation/stopping_criteria.StoppingCriteriaList

**Kind**: static class of [generation/stopping_criteria](#module_generation/stopping_criteria)  

* [.StoppingCriteriaList](#module_generation/stopping_criteria.StoppingCriteriaList)
    * [`new StoppingCriteriaList()`](#new_module_generation/stopping_criteria.StoppingCriteriaList_new)
    * [`.push(item)`](#module_generation/stopping_criteria.StoppingCriteriaList+push)
    * [`.extend(items)`](#module_generation/stopping_criteria.StoppingCriteriaList+extend)

* * *

### `new StoppingCriteriaList()`

Constructs a new instance of `StoppingCriteriaList`.

* * *

### `stoppingCriteriaList.push(item)`

Adds a new stopping criterion to the list.

**Kind**: instance method of [StoppingCriteriaList](#module_generation/stopping_criteria.StoppingCriteriaList)  

  
    
      ParamTypeDescription
    
  
  

    itemStoppingCriteriaThe stopping criterion to add.

      

* * *

### `stoppingCriteriaList.extend(items)`

Adds multiple stopping criteria to the list.

**Kind**: instance method of [StoppingCriteriaList](#module_generation/stopping_criteria.StoppingCriteriaList)  

  
    
      ParamTypeDescription
    
  
  

    itemsStoppingCriteria | StoppingCriteriaList | ArrayThe stopping criteria to add.

      

* * *

## generation/stopping_criteria.MaxLengthCriteria

This class can be used to stop generation whenever the full generated number of tokens exceeds `max_length`.
Keep in mind for decoder-only type of transformers, this will include the initial prompted tokens.

**Kind**: static class of [generation/stopping_criteria](#module_generation/stopping_criteria)  

* * *

### `new MaxLengthCriteria(max_length, [max_position_embeddings])`

  
    
      ParamTypeDefaultDescription
    
  
  

    max_lengthnumberThe maximum length that the output sequence can have in number of tokens.

    
    [max_position_embeddings]numberThe maximum model length, as defined by the model&#39;s config.max_position_embeddings attribute.

      

* * *

## generation/stopping_criteria.EosTokenCriteria

This class can be used to stop generation whenever the "end-of-sequence" token is generated.
By default, it uses the `model.generation_config.eos_token_id`.

**Kind**: static class of [generation/stopping_criteria](#module_generation/stopping_criteria)  

* [.EosTokenCriteria](#module_generation/stopping_criteria.EosTokenCriteria)
    * [`new EosTokenCriteria(eos_token_id)`](#new_module_generation/stopping_criteria.EosTokenCriteria_new)
    * [`._call(input_ids, scores)`](#module_generation/stopping_criteria.EosTokenCriteria+_call) ⇒ Array

* * *

### `new EosTokenCriteria(eos_token_id)`

  
    
      ParamTypeDescription
    
  
  

    eos_token_idnumber | ArrayThe id of the end-of-sequence token.
Optionally, use a list to set multiple end-of-sequence tokens.

      

* * *

### `eosTokenCriteria._call(input_ids, scores)` ⇒ Array

**Kind**: instance method of [EosTokenCriteria](#module_generation/stopping_criteria.EosTokenCriteria)  

  
    
      ParamType
    
  
  

    input_idsArray
    
    scoresArray
      

* * *

## generation/stopping_criteria.InterruptableStoppingCriteria

This class can be used to stop generation whenever the user interrupts the process.

**Kind**: static class of [generation/stopping_criteria](#module_generation/stopping_criteria)  

* * *
