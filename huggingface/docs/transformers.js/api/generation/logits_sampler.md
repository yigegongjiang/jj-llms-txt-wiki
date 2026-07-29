# generation/logits_sampler

* [generation/logits_sampler](#module_generation/logits_sampler)
    * _static_
        * [.LogitsSampler](#module_generation/logits_sampler.LogitsSampler)
            * [`new LogitsSampler(generation_config)`](#new_module_generation/logits_sampler.LogitsSampler_new)
            * _instance_
                * [`._call(logits)`](#module_generation/logits_sampler.LogitsSampler+_call) ⇒ Promise.&lt;Array&gt;
                * [`.sample(logits)`](#module_generation/logits_sampler.LogitsSampler+sample) ⇒ Promise.&lt;Array&gt;
                * [`.getLogits(logits, index)`](#module_generation/logits_sampler.LogitsSampler+getLogits) ⇒ Float32Array
                * [`.randomSelect(probabilities)`](#module_generation/logits_sampler.LogitsSampler+randomSelect) ⇒ number
            * _static_
                * [`.getSampler(generation_config)`](#module_generation/logits_sampler.LogitsSampler.getSampler) ⇒ LogitsSampler
    * _inner_
        * [~GreedySampler](#module_generation/logits_sampler..GreedySampler)
            * [`.sample(logits)`](#module_generation/logits_sampler..GreedySampler+sample) ⇒ Promise.&lt;Array&gt;
        * [~MultinomialSampler](#module_generation/logits_sampler..MultinomialSampler)
            * [`.sample(logits)`](#module_generation/logits_sampler..MultinomialSampler+sample) ⇒ Promise.&lt;Array&gt;
        * [~BeamSearchSampler](#module_generation/logits_sampler..BeamSearchSampler)
            * [`.sample(logits)`](#module_generation/logits_sampler..BeamSearchSampler+sample) ⇒ Promise.&lt;Array&gt;

* * *

## generation/logits_sampler.LogitsSampler

Sampler is a base class for all sampling methods used for text generation.

**Kind**: static class of [generation/logits_sampler](#module_generation/logits_sampler)  

* [.LogitsSampler](#module_generation/logits_sampler.LogitsSampler)
    * [`new LogitsSampler(generation_config)`](#new_module_generation/logits_sampler.LogitsSampler_new)
    * _instance_
        * [`._call(logits)`](#module_generation/logits_sampler.LogitsSampler+_call) ⇒ Promise.&lt;Array&gt;
        * [`.sample(logits)`](#module_generation/logits_sampler.LogitsSampler+sample) ⇒ Promise.&lt;Array&gt;
        * [`.getLogits(logits, index)`](#module_generation/logits_sampler.LogitsSampler+getLogits) ⇒ Float32Array
        * [`.randomSelect(probabilities)`](#module_generation/logits_sampler.LogitsSampler+randomSelect) ⇒ number
    * _static_
        * [`.getSampler(generation_config)`](#module_generation/logits_sampler.LogitsSampler.getSampler) ⇒ LogitsSampler

* * *

### `new LogitsSampler(generation_config)`

Creates a new Sampler object with the specified generation config.

  
    
      ParamTypeDescription
    
  
  

    generation_configGenerationConfigThe generation config.

      

* * *

### `logitsSampler._call(logits)` ⇒ Promise.&lt;Array&gt;

Executes the sampler, using the specified logits.

**Kind**: instance method of [LogitsSampler](#module_generation/logits_sampler.LogitsSampler)  

  
    
      ParamType
    
  
  

    logitsTensor
      

* * *

### `logitsSampler.sample(logits)` ⇒ Promise.&lt;Array&gt;

Abstract method for sampling the logits.

**Kind**: instance method of [LogitsSampler](#module_generation/logits_sampler.LogitsSampler)  
**Throws**:

- Error If not implemented in subclass.

  
    
      ParamType
    
  
  

    logitsTensor
      

* * *

### `logitsSampler.getLogits(logits, index)` ⇒ Float32Array

Returns the specified logits as an array, with temperature applied.

**Kind**: instance method of [LogitsSampler](#module_generation/logits_sampler.LogitsSampler)  

  
    
      ParamType
    
  
  

    logitsTensor
    
    indexnumber
      

* * *

### `logitsSampler.randomSelect(probabilities)` ⇒ number

Selects an item randomly based on the specified probabilities.

**Kind**: instance method of [LogitsSampler](#module_generation/logits_sampler.LogitsSampler)  
**Returns**: number - The index of the selected item.  

  
    
      ParamTypeDescription
    
  
  

    probabilitiesFloat32ArrayAn array of probabilities to use for selection.

      

* * *

### `LogitsSampler.getSampler(generation_config)` ⇒ LogitsSampler

Returns a Sampler object based on the specified options.

**Kind**: static method of [LogitsSampler](#module_generation/logits_sampler.LogitsSampler)  
**Returns**: LogitsSampler - A Sampler object.  

  
    
      ParamTypeDescription
    
  
  

    generation_configGenerationConfigAn object containing options for the sampler.

      

* * *

## generation/logits_sampler~GreedySampler

Class representing a Greedy Sampler.

**Kind**: inner class of [generation/logits_sampler](#module_generation/logits_sampler)  

* * *

### `greedySampler.sample(logits)` ⇒ Promise.&lt;Array&gt;

Sample the maximum probability of a given logits tensor.

**Kind**: instance method of [GreedySampler](#module_generation/logits_sampler..GreedySampler)  
**Returns**: Promise.&lt;Array&gt; - An array with a single tuple, containing the index of the maximum value and a meaningless score (since this is a greedy search).  

  
    
      ParamType
    
  
  

    logitsTensor
      

* * *

## generation/logits_sampler~MultinomialSampler

Class representing a MultinomialSampler.

**Kind**: inner class of [generation/logits_sampler](#module_generation/logits_sampler)  

* * *

### `multinomialSampler.sample(logits)` ⇒ Promise.&lt;Array&gt;

Sample from the logits.

**Kind**: instance method of [MultinomialSampler](#module_generation/logits_sampler..MultinomialSampler)  

  
    
      ParamType
    
  
  

    logitsTensor
      

* * *

## generation/logits_sampler~BeamSearchSampler

Class representing a BeamSearchSampler.

**Kind**: inner class of [generation/logits_sampler](#module_generation/logits_sampler)  

* * *

### `beamSearchSampler.sample(logits)` ⇒ Promise.&lt;Array&gt;

Sample from the logits.

**Kind**: instance method of [BeamSearchSampler](#module_generation/logits_sampler..BeamSearchSampler)  

  
    
      ParamType
    
  
  

    logitsTensor
      

* * *
