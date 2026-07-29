# tokenizers

Tokenization utilities

* [tokenizers](#module_tokenizers)
    * _static_
        * [.PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)
            * [`new PreTrainedTokenizer(tokenizerJSON, tokenizerConfig)`](#new_module_tokenizers.PreTrainedTokenizer_new)
            * _instance_
                * [`.convert_tokens_to_ids(tokens)`](#module_tokenizers.PreTrainedTokenizer+convert_tokens_to_ids) ⇒ any
                * [`._call(text, [options])`](#module_tokenizers.PreTrainedTokenizer+_call) ⇒ BatchEncoding.&lt;BatchEncodingItem.&lt;TText, TReturnTensor&gt;&gt;
                * [`._encode_text(text)`](#module_tokenizers.PreTrainedTokenizer+_encode_text) ⇒ Array | null
                * [`.tokenize(text, options)`](#module_tokenizers.PreTrainedTokenizer+tokenize) ⇒ Array
                * [`.encode(text, options)`](#module_tokenizers.PreTrainedTokenizer+encode) ⇒ Array
                * [`.batch_decode(batch, decode_args)`](#module_tokenizers.PreTrainedTokenizer+batch_decode) ⇒ Array
                * [`.decode(token_ids, [decode_args])`](#module_tokenizers.PreTrainedTokenizer+decode) ⇒ string
                * [`.decode_single(token_ids, decode_args)`](#module_tokenizers.PreTrainedTokenizer+decode_single) ⇒ string
                * [`.get_chat_template(options)`](#module_tokenizers.PreTrainedTokenizer+get_chat_template) ⇒ string
                * [`.apply_chat_template(conversation, [options])`](#module_tokenizers.PreTrainedTokenizer+apply_chat_template) ⇒ ApplyChatTemplateReturn.&lt;TTokenize, TReturnTensor, TReturnDict&gt;
            * _static_
                * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_tokenizers.PreTrainedTokenizer.from_pretrained) ⇒ Promise.&lt;PreTrainedTokenizer&gt;
        * [`.loadTokenizer(pretrained_model_name_or_path, options)`](#module_tokenizers.loadTokenizer) ⇒ Promise.&lt;Array&gt;
        * [`.prepareTensorForDecode(tensor)`](#module_tokenizers.prepareTensorForDecode) ⇒ Array
        * [`._build_translation_inputs(self, raw_inputs, tokenizer_options, generate_kwargs)`](#module_tokenizers._build_translation_inputs) ⇒ Object
    * _inner_
        * [`~PretrainedTokenizerOptions`](#module_tokenizers..PretrainedTokenizerOptions) : [PretrainedOptions](#PretrainedOptions)
        * [`~TextContent`](#module_tokenizers..TextContent) : Object
        * [`~ImageContent`](#module_tokenizers..ImageContent) : Object
        * [`~MessageContent`](#module_tokenizers..MessageContent) : TextContent | ImageContent | Object
        * [`~Message`](#module_tokenizers..Message) : Object
        * [`~BatchEncodingArrayItem`](#module_tokenizers..BatchEncodingArrayItem) : any
        * [`~BatchEncodingItem`](#module_tokenizers..BatchEncodingItem) : any
        * [`~BatchEncoding`](#module_tokenizers..BatchEncoding) : Object
        * [`~TokenizerCallOptions`](#module_tokenizers..TokenizerCallOptions) : Object
        * [`~PreTrainedTokenizerCallback`](#module_tokenizers..PreTrainedTokenizerCallback) : function
        * [`~ApplyChatTemplateOptions`](#module_tokenizers..ApplyChatTemplateOptions) : Object
        * [`~ApplyChatTemplateReturn`](#module_tokenizers..ApplyChatTemplateReturn) : any

* * *

## tokenizers.PreTrainedTokenizer

**Kind**: static class of [tokenizers](#module_tokenizers)  

* [.PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)
    * [`new PreTrainedTokenizer(tokenizerJSON, tokenizerConfig)`](#new_module_tokenizers.PreTrainedTokenizer_new)
    * _instance_
        * [`.convert_tokens_to_ids(tokens)`](#module_tokenizers.PreTrainedTokenizer+convert_tokens_to_ids) ⇒ any
        * [`._call(text, [options])`](#module_tokenizers.PreTrainedTokenizer+_call) ⇒ BatchEncoding.&lt;BatchEncodingItem.&lt;TText, TReturnTensor&gt;&gt;
        * [`._encode_text(text)`](#module_tokenizers.PreTrainedTokenizer+_encode_text) ⇒ Array | null
        * [`.tokenize(text, options)`](#module_tokenizers.PreTrainedTokenizer+tokenize) ⇒ Array
        * [`.encode(text, options)`](#module_tokenizers.PreTrainedTokenizer+encode) ⇒ Array
        * [`.batch_decode(batch, decode_args)`](#module_tokenizers.PreTrainedTokenizer+batch_decode) ⇒ Array
        * [`.decode(token_ids, [decode_args])`](#module_tokenizers.PreTrainedTokenizer+decode) ⇒ string
        * [`.decode_single(token_ids, decode_args)`](#module_tokenizers.PreTrainedTokenizer+decode_single) ⇒ string
        * [`.get_chat_template(options)`](#module_tokenizers.PreTrainedTokenizer+get_chat_template) ⇒ string
        * [`.apply_chat_template(conversation, [options])`](#module_tokenizers.PreTrainedTokenizer+apply_chat_template) ⇒ ApplyChatTemplateReturn.&lt;TTokenize, TReturnTensor, TReturnDict&gt;
    * _static_
        * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_tokenizers.PreTrainedTokenizer.from_pretrained) ⇒ Promise.&lt;PreTrainedTokenizer&gt;

* * *

### `new PreTrainedTokenizer(tokenizerJSON, tokenizerConfig)`

Create a new PreTrainedTokenizer instance.

  
    
      ParamTypeDescription
    
  
  

    tokenizerJSONObjectThe JSON of the tokenizer.

    
    tokenizerConfigObjectThe config of the tokenizer.

      

* * *

### `preTrainedTokenizer.convert_tokens_to_ids(tokens)` ⇒ any

Converts a token string (or a sequence of tokens) into a single integer id (or a sequence of ids), using the vocabulary.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: any - The token id or list of token ids.  

  
    
      ParamTypeDescription
    
  
  

    tokensTOne or several token(s) to convert to token id(s).

      

* * *

### `preTrainedTokenizer._call(text, [options])` ⇒ BatchEncoding.&lt;BatchEncodingItem.&lt;TText, TReturnTensor&gt;&gt;

Encode/tokenize the given text(s).

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: BatchEncoding.&lt;BatchEncodingItem.&lt;TText, TReturnTensor&gt;&gt; - Object to be passed to the model.  

  
    
      ParamTypeDescription
    
  
  

    textTTextThe text to tokenize.

    
    [options]TokenizerCallOptions.&lt;TText, TReturnTensor&gt;Additional tokenization options.

      

* * *

### `preTrainedTokenizer._encode_text(text)` ⇒ Array | null

Encodes a single text using the preprocessor pipeline of the tokenizer.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: Array | null - The encoded tokens.  

  
    
      ParamTypeDescription
    
  
  

    textstring | nullThe text to encode.

      

* * *

### `preTrainedTokenizer.tokenize(text, options)` ⇒ Array

Converts a string into a sequence of tokens.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: Array - The list of tokens.  

  
    
      ParamTypeDefaultDescription
    
  
  

    textstringThe sequence to be encoded.

    
    optionsObjectAn optional object containing the following properties:

    
    [options.pair]string | nullA second sequence to be encoded with the first.

    
    [options.add_special_tokens]booleanfalseWhether or not to add the special tokens associated with the corresponding model.

      

* * *

### `preTrainedTokenizer.encode(text, options)` ⇒ Array

Encodes a single text or a pair of texts using the model's tokenizer.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: Array - An array of token IDs representing the encoded text(s).  

  
    
      ParamTypeDefaultDescription
    
  
  

    textstringThe text to encode.

    
    optionsObjectAn optional object containing the following properties:

    
    [options.text_pair]string | nullnullThe optional second text to encode.

    
    [options.add_special_tokens]booleantrueWhether or not to add the special tokens associated with the corresponding model.

    
    [options.return_token_type_ids]boolean | nullWhether to return token_type_ids.

      

* * *

### `preTrainedTokenizer.batch_decode(batch, decode_args)` ⇒ Array

Decode a batch of tokenized sequences.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: Array - List of decoded sequences.  

  
    
      ParamTypeDescription
    
  
  

    batchArray | TensorList/Tensor of tokenized input sequences.

    
    decode_argsObject(Optional) Object with decoding arguments.

      

* * *

### `preTrainedTokenizer.decode(token_ids, [decode_args])` ⇒ string

Decodes a sequence of token IDs back to a string.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: string - The decoded string.  
**Throws**:

- Error If `token_ids` is not a non-empty array of integers.

  
    
      ParamTypeDefaultDescription
    
  
  

    token_idsArray | Array | TensorList/Tensor of token IDs to decode.

    
    [decode_args]Object{}
    
    [decode_args.skip_special_tokens]booleanfalseIf true, special tokens are removed from the output string.

    
    [decode_args.clean_up_tokenization_spaces]booleantrueIf true, spaces before punctuations and abbreviated forms are removed.

      

* * *

### `preTrainedTokenizer.decode_single(token_ids, decode_args)` ⇒ string

Decode a single list of token ids to a string.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: string - The decoded string  

  
    
      ParamTypeDefaultDescription
    
  
  

    token_idsArray | ArrayList of token ids to decode

    
    decode_argsObjectOptional arguments for decoding

    
    [decode_args.skip_special_tokens]booleanfalseWhether to skip special tokens during decoding

    
    [decode_args.clean_up_tokenization_spaces]boolean | nullWhether to clean up tokenization spaces during decoding.
If null, the value is set to this.decoder.cleanup if it exists, falling back to this.clean_up_tokenization_spaces if it exists, falling back to true.

      

* * *

### `preTrainedTokenizer.get_chat_template(options)` ⇒ string

Retrieve the chat template string used for tokenizing chat messages. This template is used
internally by the `apply_chat_template` method and can also be used externally to retrieve the model's chat
template for better generation tracking.

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: string - The chat template string.  

  
    
      ParamTypeDefaultDescription
    
  
  

    optionsObjectAn optional object containing the following properties:

    
    [options.chat_template]string | nullnullA Jinja template or the name of a template to use for this conversion.
It is usually not necessary to pass anything to this argument,
as the model&#39;s template will be used by default.

    
    [options.tools]ArrayA list of tools (callable functions) that will be accessible to the model. If the template does not
support function calling, this argument will have no effect. Each tool should be passed as a JSON Schema,
giving the name, description and argument types for the tool. See our
chat templating guide
for more information.

      

* * *

### `preTrainedTokenizer.apply_chat_template(conversation, [options])` ⇒ ApplyChatTemplateReturn.&lt;TTokenize, TReturnTensor, TReturnDict&gt;

Converts a list of message objects with `"role"` and `"content"` keys to a list of token
ids. This method is intended for use with chat models, and will read the tokenizer's chat_template attribute to
determine the format and control tokens to use when converting.

See [here](https://huggingface.co/docs/transformers/chat_templating) for more information.

**Example:** Applying a chat template to a conversation.

```javascript
import { AutoTokenizer } from "@huggingface/transformers";

const tokenizer = await AutoTokenizer.from_pretrained("Xenova/mistral-tokenizer-v1");

const chat = [
  { "role": "user", "content": "Hello, how are you?" },
  { "role": "assistant", "content": "I'm doing great. How can I help you today?" },
  { "role": "user", "content": "I'd like to show off how chat templating works!" },
]

const text = tokenizer.apply_chat_template(chat, { tokenize: false });
// "[INST] Hello, how are you? [/INST]I'm doing great. How can I help you today? [INST] I'd like to show off how chat templating works! [/INST]"

const input_ids = tokenizer.apply_chat_template(chat, { tokenize: true, return_tensor: false });
// [1, 733, 16289, 28793, 22557, 28725, 910, 460, 368, 28804, 733, 28748, 16289, 28793, 28737, 28742, 28719, 2548, 1598, 28723, 1602, 541, 315, 1316, 368, 3154, 28804, 2, 28705, 733, 16289, 28793, 315, 28742, 28715, 737, 298, 1347, 805, 910, 10706, 5752, 1077, 3791, 28808, 733, 28748, 16289, 28793]
```

**Kind**: instance method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: ApplyChatTemplateReturn.&lt;TTokenize, TReturnTensor, TReturnDict&gt; - The tokenized output.  

  
    
      ParamTypeDefaultDescription
    
  
  

    conversationArrayA list of message objects with &quot;role&quot; and &quot;content&quot; keys,
representing the chat history so far.

    
    [options]ObjectAn optional object containing the following properties:

    
    [options.chat_template]string | nullnullA Jinja template to use for this conversion. If
this is not passed, the model&#39;s chat template will be used instead.

    
    [options.tools]ArrayA list of tools (callable functions) that will be accessible to the model. If the template does not
support function calling, this argument will have no effect. Each tool should be passed as a JSON Schema,
giving the name, description and argument types for the tool. See our
chat templating guide
for more information.

    
    [options.documents]Array.&lt;Record&gt;A list of dicts representing documents that will be accessible to the model if it is performing RAG
(retrieval-augmented generation). If the template does not support RAG, this argument will have no
effect. We recommend that each document should be a dict containing &quot;title&quot; and &quot;text&quot; keys. Please
see the RAG section of the chat templating guide
for examples of passing documents with chat templates.

    
    [options.add_generation_prompt]booleanfalseWhether to end the prompt with the token(s) that indicate
the start of an assistant message. This is useful when you want to generate a response from the model.
Note that this argument will be passed to the chat template, and so it must be supported in the
template for this argument to have any effect.

    
    [options.tokenize]TTokenizetrueWhether to tokenize the output. If false, the output will be a string.

    
    [options.padding]booleanfalseWhether to pad sequences to the maximum length. Has no effect if tokenize is false.

    
    [options.truncation]booleanfalseWhether to truncate sequences to the maximum length. Has no effect if tokenize is false.

    
    [options.max_length]number | nullMaximum length (in tokens) to use for padding or truncation. Has no effect if tokenize is false.
If not specified, the tokenizer&#39;s max_length attribute will be used as a default.

    
    [options.return_tensor]TReturnTensortrueWhether to return the output as a Tensor or an Array. Has no effect if tokenize is false.

    
    [options.return_dict]TReturnDicttrueWhether to return a dictionary with named outputs. Has no effect if tokenize is false.

    
    [options.tokenizer_kwargs]Object{}Additional options to pass to the tokenizer.

      

* * *

### `PreTrainedTokenizer.from_pretrained(pretrained_model_name_or_path, options)` ⇒ Promise.&lt;PreTrainedTokenizer&gt;

Loads a pre-trained tokenizer from the given `pretrained_model_name_or_path`.

**Kind**: static method of [PreTrainedTokenizer](#module_tokenizers.PreTrainedTokenizer)  
**Returns**: Promise.&lt;PreTrainedTokenizer&gt; - A new instance of the `PreTrainedTokenizer` class.  
**Throws**:

- Error Throws an error if the tokenizer.json or tokenizer_config.json files are not found in the `pretrained_model_name_or_path`.

  
    
      ParamTypeDescription
    
  
  

    pretrained_model_name_or_pathstringThe path to the pre-trained tokenizer.

    
    optionsPretrainedTokenizerOptionsAdditional options for loading the tokenizer.

      

* * *

## `tokenizers.loadTokenizer(pretrained_model_name_or_path, options)` ⇒ Promise.&lt;Array&gt;

Loads a tokenizer from the specified path.

**Kind**: static method of [tokenizers](#module_tokenizers)  
**Returns**: Promise.&lt;Array&gt; - A promise that resolves with information about the loaded tokenizer.  

  
    
      ParamTypeDescription
    
  
  

    pretrained_model_name_or_pathstringThe path to the tokenizer directory.

    
    optionsPretrainedTokenizerOptionsAdditional options for loading the tokenizer.

      

* * *

## `tokenizers.prepareTensorForDecode(tensor)` ⇒ Array

Helper function to convert a tensor to a list before decoding.

**Kind**: static method of [tokenizers](#module_tokenizers)  
**Returns**: Array - The tensor as a list.  

  
    
      ParamTypeDescription
    
  
  

    tensorTensorThe tensor to convert.

      

* * *

## `tokenizers._build_translation_inputs(self, raw_inputs, tokenizer_options, generate_kwargs)` ⇒ Object

Helper function to build translation inputs for an `NllbTokenizer` or `M2M100Tokenizer`.

**Kind**: static method of [tokenizers](#module_tokenizers)  
**Returns**: Object - Object to be passed to the model.  

  
    
      ParamTypeDescription
    
  
  

    selfPreTrainedTokenizerThe tokenizer instance.

    
    raw_inputsstring | ArrayThe text to tokenize.

    
    tokenizer_optionsObjectOptions to be sent to the tokenizer

    
    generate_kwargsObjectGeneration options.

      

* * *

## `tokenizers~PretrainedTokenizerOptions` : [PretrainedOptions](#PretrainedOptions)

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *

## `tokenizers~TextContent` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    type&#x27;text&#x27;The type of content (must be &#39;text&#39;).

    
    textstringThe text content.

      

* * *

## `tokenizers~ImageContent` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    type&#x27;image&#x27;The type of content (must be &#39;image&#39;).

    
    [image]string | RawImageOptional URL or instance of the image.
Note: This works for SmolVLM. Qwen2VL and Idefics3 have different implementations.

      

* * *

## `tokenizers~MessageContent` : TextContent | ImageContent | Object

Base type for message content. This is a discriminated union that can be extended with additional content types.
Example: `@typedef {TextContent | ImageContent | AudioContent} MessageContent`

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *

## `tokenizers~Message` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    role&#x27;user&#x27; | &#x27;assistant&#x27; | &#x27;system&#x27; | stringThe role of the message.

    
    contentstring | ArrayThe content of the message. Can be a simple string or an array of content objects.

      

* * *

## `tokenizers~BatchEncodingArrayItem` : any

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *

## `tokenizers~BatchEncodingItem` : any

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *

## `tokenizers~BatchEncoding` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    input_idsTItemList of token ids to be fed to a model.

    
    attention_maskTItemList of indices specifying which tokens should be attended to by the model.

    
    [token_type_ids]TItemList of token type ids to be fed to a model.

      

* * *

## `tokenizers~TokenizerCallOptions` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDefaultDescription
    
  
  

    [text_pair]anyOptional second sequence to be encoded. If set, must be the same type as text.

    
    [padding]boolean | &#x27;max_length&#x27;falseWhether to pad the input sequences.

    
    [add_special_tokens]booleantrueWhether or not to add the special tokens associated with the corresponding model.

    
    [truncation]boolean | nullWhether to truncate the input sequences.

    
    [max_length]number | nullMaximum length of the returned list and optionally padding length.

    
    [return_tensor]TReturnTensortrueWhether to return the results as Tensors or arrays.

    
    [return_token_type_ids]boolean | nullWhether to return the token type ids.

      

* * *

## `tokenizers~PreTrainedTokenizerCallback` : function

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *

## `tokenizers~ApplyChatTemplateOptions` : Object

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  
**Properties**

  
    
      NameTypeDefaultDescription
    
  
  

    [chat_template]string | nullnullA Jinja template to use for this conversion.

    
    [tools]Array | nullA list of tools (callable functions) that will be accessible to the model.

    
    [documents]Array.&lt;Record&gt; | nullDocuments that will be accessible to the model.

    
    [add_generation_prompt]booleanfalseWhether to end the prompt with the token(s) that indicate the start of an assistant message.

    
    [tokenize]TTokenizetrueWhether to tokenize the output. If false, the output will be a string.

    
    [padding]booleanfalseWhether to pad sequences to the maximum length. Has no effect if tokenize is false.

    
    [truncation]booleanfalseWhether to truncate sequences to the maximum length. Has no effect if tokenize is false.

    
    [max_length]number | nullMaximum length (in tokens) to use for padding or truncation. Has no effect if tokenize is false.

    
    [return_tensor]TReturnTensortrueWhether to return the output as a Tensor or an Array. Has no effect if tokenize is false.

    
    [return_dict]TReturnDicttrueWhether to return a dictionary with named outputs. Has no effect if tokenize is false.

    
    [tokenizer_kwargs]Object{}Additional options to pass to the tokenizer.

      

* * *

## `tokenizers~ApplyChatTemplateReturn` : any

**Kind**: inner typedef of [tokenizers](#module_tokenizers)  

* * *
