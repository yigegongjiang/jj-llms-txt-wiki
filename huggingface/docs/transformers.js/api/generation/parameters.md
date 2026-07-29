# generation/parameters

* [generation/parameters](#module_generation/parameters)
    * [`~GenerationFunctionParametersBase`](#module_generation/parameters..GenerationFunctionParametersBase) : Object
    * [`~GenerationFunctionParameters`](#module_generation/parameters..GenerationFunctionParameters) : GenerationFunctionParametersBase | Partial.&lt;GenerationConfig&gt;

* * *

## `generation/parameters~GenerationFunctionParametersBase` : Object

**Kind**: inner typedef of [generation/parameters](#module_generation/parameters)  
**Properties**

  
    
      NameTypeDefaultDescription
    
  
  

    [inputs]Tensor(Tensor of varying shape depending on the modality, optional):
The sequence used as a prompt for the generation or as model inputs to the encoder. If null the
method initializes it with bos_token_id and a batch size of 1. For decoder-only models inputs
should be in the format of input_ids. For encoder-decoder models inputs can represent any of
input_ids, input_values, input_features, or pixel_values.

    
    [generation_config]GenerationConfig(GenerationConfig, optional):
The generation configuration to be used as base parametrization for the generation call.
**kwargs passed to generate matching the attributes of generation_config will override them.
If generation_config is not provided, the default will be used, which has the following loading
priority:

(1) from the generation_config.json model file, if it exists;
(2) from the model configuration. Please note that unspecified parameters will inherit [GenerationConfig]&#39;s
default values, whose documentation should be checked to parameterize generation.

    
    [logits_processor]LogitsProcessorList(LogitsProcessorList, optional):
Custom logits processors that complement the default logits processors built from arguments and
generation config. If a logit processor is passed that is already created with the arguments or a
generation config an error is thrown. This feature is intended for advanced users.

    
    [stopping_criteria]StoppingCriteria | Array | StoppingCriteriaList(StoppingCriteriaList, optional):
Custom stopping criteria that complements the default stopping criteria built from arguments and a
generation config. If a stopping criteria is passed that is already created with the arguments or a
generation config an error is thrown. This feature is intended for advanced users.

    
    [streamer]BaseStreamer(BaseStreamer, optional):
Streamer object that will be used to stream the generated sequences. Generated tokens are passed
through streamer.put(token_ids) and the streamer is responsible for any further processing.

    
    [decoder_input_ids]Array | Tensor(number[] or Tensor, optional):
If the model is an encoder-decoder model, this argument is used to pass the decoder_input_ids.

    
    [past_key_values]DynamicCache | null(DynamicCache, optional):
A cache object that stores previously computed key/value states. When provided, the model will
use these cached states to avoid recomputing them, significantly speeding up sequential generation.

      

* * *

## `generation/parameters~GenerationFunctionParameters` : GenerationFunctionParametersBase | Partial.&lt;GenerationConfig&gt;

**Kind**: inner typedef of [generation/parameters](#module_generation/parameters)  

* * *
