# tokenizers

Tokenizers turn text into the integer ids a model understands, and
decode model output back into strings. Use `AutoTokenizer.from_pretrained()`
to load the right implementation for a model ID — the class is chosen from
the tokenizer's `tokenizer_config.json`.

For chat-trained models, `tokenizer.apply_chat_template()` renders an
OpenAI-style message list into the model's native prompt format.

## Classes

### AutoTokenizer

Helper class which is used to instantiate pretrained tokenizers with the `from_pretrained` function.
The chosen tokenizer class is determined by the type specified in the tokenizer config.

**Example:** Create an `AutoTokenizer` and use it to tokenize a sentence.
This will automatically detect the tokenizer type based on the tokenizer class defined in `tokenizer_config.json`.

```javascript
import { AutoTokenizer } from '@huggingface/transformers';

const tokenizer = await AutoTokenizer.from_pretrained('Xenova/bert-base-uncased');
const { input_ids } = await tokenizer('I love transformers!');
// Tensor {
//   data: BigInt64Array(6) [101n, 1045n, 2293n, 19081n, 999n, 102n],
//   dims: [1, 6],
//   type: 'int64',
//   size: 6,
// }
```

#### `AutoTokenizer.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the tokenizer classes of the library from a pretrained model.

The tokenizer class to instantiate is selected based on the `tokenizer_class` property of the config object
(either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model id* of a pretrained tokenizer hosted inside a model repo on huggingface.co.
  Valid model ids can be located at the root-level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing tokenizer files, e.g., `./my_model_directory/`.
- `options` ([`PretrainedTokenizerOptions`](./tokenizers#module_tokenizers.PretrainedTokenizerOptions)) — Additional options for loading the tokenizer.

**Returns:** `Promise`<[`PreTrainedTokenizer`](./tokenizers#module_tokenizers.PreTrainedTokenizer)> — The loaded tokenizer.

### PreTrainedTokenizer

`PreTrainedTokenizer` is the base class for all tokenizers in Transformers.js.

#### `PreTrainedTokenizer(text, [options])`

Encode/tokenize the given text(s).

**Parameters**

- `text` (`string` | `string[]`) — The text to tokenize.
- `options` ([`TokenizerCallOptions`](./tokenizers#module_tokenizers.TokenizerCallOptions)<`string` | `string[]`, `boolean`>) _optional_ — Additional tokenization options.

**Returns:** [`BatchEncoding`](./tokenizers#module_tokenizers.BatchEncoding)<[`BatchEncodingItem`](./tokenizers#module_tokenizers.BatchEncodingItem)<`string` | `string[]`, `boolean`>> — Object to be passed to the model.

#### `PreTrainedTokenizer.constructor(tokenizerJSON, tokenizerConfig)`

Create a new PreTrainedTokenizer instance.

**Parameters**

- `tokenizerJSON` (`Object`) — The JSON of the tokenizer.
- `tokenizerConfig` (`Object`) — The config of the tokenizer.

#### `PreTrainedTokenizer.from_pretrained(pretrained_model_name_or_path, options)`

Loads a pretrained tokenizer from the given `pretrained_model_name_or_path`.

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The path to the pretrained tokenizer.
- `options` ([`PretrainedTokenizerOptions`](./tokenizers#module_tokenizers.PretrainedTokenizerOptions)) — Additional options for loading the tokenizer.

**Returns:** `Promise`<[`PreTrainedTokenizer`](./tokenizers#module_tokenizers.PreTrainedTokenizer)> — A new instance of the `PreTrainedTokenizer` class.

**Throws**

- `Error` — Throws an error if the tokenizer.json or tokenizer_config.json files are not found in the `pretrained_model_name_or_path`.

#### `PreTrainedTokenizer.convert_tokens_to_ids(tokens)`

Converts a token string (or a sequence of tokens) into a single integer id (or a sequence of ids), using the vocabulary.

**Parameters**

- `tokens` (`string` | `string[]`) — One or several token(s) to convert to token id(s).

**Returns:** `number` | `number[]` — The token id or list of token ids.

#### `PreTrainedTokenizer.tokenize(text, options)`

Converts a string into a sequence of tokens.

**Parameters**

- `text` (`string`) — The sequence to be encoded.
- `options` (`Object`) — An optional object containing the following properties:
  - `pair` (`string` | `null`) _optional_ — A second sequence to be encoded with the first.
  - `add_special_tokens` (`boolean`) _optional_ — defaults to `false` — Whether or not to add the special tokens associated with the corresponding model.

**Returns:** `string[]` — The list of tokens.

#### `PreTrainedTokenizer.encode(text, options)`

Encodes a single text or a pair of texts using the model's tokenizer.

**Parameters**

- `text` (`string`) — The text to encode.
- `options` (`Object`) — An optional object containing the following properties:
  - `text_pair` (`string` | `null`) _optional_ — defaults to `null` — The optional second text to encode.
  - `add_special_tokens` (`boolean`) _optional_ — defaults to `true` — Whether or not to add the special tokens associated with the corresponding model.
  - `return_token_type_ids` (`boolean` | `null`) _optional_ — defaults to `null` — Whether to return token_type_ids.

**Returns:** `number[]` — An array of token IDs representing the encoded text(s).

#### `PreTrainedTokenizer.batch_decode(batch, decode_args)`

Decode a batch of tokenized sequences.

**Parameters**

- `batch` (`number[][]` | [`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — List/Tensor of tokenized input sequences.
- `decode_args` (`Object`) — (Optional) Object with decoding arguments.

**Returns:** `string[]` — List of decoded sequences.

#### `PreTrainedTokenizer.decode(token_ids, [decode_args])`

Decodes a sequence of token IDs back to a string.

**Parameters**

- `token_ids` (`number[]` | `bigint[]` | [`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — List/Tensor of token IDs to decode.
- `decode_args` (`Object`) _optional_ — defaults to `{}`
  - `skip_special_tokens` (`boolean`) _optional_ — defaults to `false` — If true, special tokens are removed from the output string.
  - `clean_up_tokenization_spaces` (`boolean`) _optional_ — defaults to `true` — If true, spaces before punctuation and abbreviated forms are removed.

**Returns:** `string` — The decoded string.

**Throws**

- `Error` — If `token_ids` is not a non-empty array of integers.

#### `PreTrainedTokenizer.decode_single(token_ids, decode_args)`

Decode a single list of token ids to a string.

**Parameters**

- `token_ids` (`number[]` | `bigint[]`) — List of token ids to decode
- `decode_args` (`Object`) — Optional arguments for decoding
  - `skip_special_tokens` (`boolean`) _optional_ — defaults to `false` — Whether to skip special tokens during decoding
  - `clean_up_tokenization_spaces` (`boolean` | `null`) _optional_ — defaults to `null` — Whether to clean up tokenization spaces during decoding.
    If null, the value is set to `this.decoder.cleanup` if it exists, falling back to `this.clean_up_tokenization_spaces` if it exists, falling back to `true`.

**Returns:** `string` — The decoded string

#### `PreTrainedTokenizer.get_chat_template(options)`

Retrieve the chat template string used for tokenizing chat messages. This template is used
internally by the `apply_chat_template` method and can also be used externally to retrieve the model's chat
template for better generation tracking.

**Parameters**

- `options` (`Object`) — An optional object containing the following properties:
  - `chat_template` (`string` | `null`) _optional_ — defaults to `null` — A Jinja template or the name of a template to use for this conversion.
    It is usually not necessary to pass anything to this argument,
    as the model's template will be used by default.
  - `tools` (`Object[]`) _optional_ — defaults to `null` — A list of tools (callable functions) that will be accessible to the model. If the template does not
    support function calling, this argument will have no effect. Each tool should be passed as a JSON Schema,
    giving the name, description and argument types for the tool. See our
    [chat templating guide](https://huggingface.co/docs/transformers/main/en/chat_templating#automated-function-conversion-for-tool-use)
    for more information.

**Returns:** `string` — The chat template string.

#### `PreTrainedTokenizer.apply_chat_template(conversation, [options])`

Converts a list of message objects with `"role"` and `"content"` keys to a list of token
ids. This method is intended for use with chat models, and will read the tokenizer's chat_template attribute to
determine the format and control tokens to use when converting.

See the [chat templating guide](https://huggingface.co/docs/transformers/chat_templating) for more information.

**Parameters**

- `conversation` ([`Message`](./tokenizers#module_tokenizers.Message)[]) — A list of message objects with `"role"` and `"content"` keys,
  representing the chat history so far.
- `options` ([`ApplyChatTemplateOptions`](./tokenizers#module_tokenizers.ApplyChatTemplateOptions)<`boolean`, `boolean`, `boolean`>) _optional_ — Options controlling
  template rendering and tokenization.

**Returns:** [`ApplyChatTemplateReturn`](./tokenizers#module_tokenizers.ApplyChatTemplateReturn)<`boolean`, `boolean`, `boolean`> — The tokenized output.

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
// "<s>[INST] Hello, how are you? [/INST]I'm doing great. How can I help you today?</s> [INST] I'd like to show off how chat templating works! [/INST]"

const input_ids = tokenizer.apply_chat_template(chat, { tokenize: true, return_tensor: false });
// [1, 733, 16289, 28793, 22557, 28725, 910, 460, 368, 28804, 733, 28748, 16289, 28793, 28737, 28742, 28719, 2548, 1598, 28723, 1602, 541, 315, 1316, 368, 3154, 28804, 2, 28705, 733, 16289, 28793, 315, 28742, 28715, 737, 298, 1347, 805, 910, 10706, 5752, 1077, 3791, 28808, 733, 28748, 16289, 28793]
```

## Type Definitions

### PretrainedTokenizerOptions

_Type:_ [`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)

### TextContent

**Properties**

- `type` (`'text'`) — The type of content (must be 'text').
- `text` (`string`) — The text content.

### ImageContent

**Properties**

- `type` (`'image'`) — The type of content (must be 'image').
- `image` (`string` | [`RawImage`](./utils/image#module_utils/image.RawImage)) _optional_ — Optional URL or instance of the image.

  Note: This works for SmolVLM. Qwen2VL and Idefics3 have different implementations.

### MessageContent

A single content block inside a chat message. Extend the union to add
custom types (e.g. `AudioContent`) when targeting a specific model.

_Type:_ [`TextContent`](./tokenizers#module_tokenizers.TextContent) | [`ImageContent`](./tokenizers#module_tokenizers.ImageContent) | `{ type: string & {}, [key: string]: any }`

### Message

**Properties**

- `role` (`'user'` | `'assistant'` | `'system'` | `string` & `{}`) — The role of the message.
- `content` (`string` | [`MessageContent`](./tokenizers#module_tokenizers.MessageContent)[]) — The content of the message. Can be a simple string or an array of content objects.

### BatchEncodingArrayItem

_Type:_ `number[]` | `number[][]`

### BatchEncodingItem

_Type:_ [`Tensor`](./utils/tensor#module_utils/tensor.Tensor) | [`BatchEncodingArrayItem`](./tokenizers#module_tokenizers.BatchEncodingArrayItem)<`string` | `string[]`>

### BatchEncoding

The object returned from `tokenizer(text)`. The fields are a `Tensor` by
default, or an `Array` when `return_tensor: false` is passed.

**Properties**

- `input_ids` (`any`) — Token ids to be fed to the model.
- `attention_mask` (`any`) — Mask indicating which tokens should be attended to (1) versus padded (0).
- `token_type_ids` (`any`) _optional_ — Segment ids, present only for tokenizers that distinguish sequence A vs B (e.g. BERT).

### TokenizerCallOptions

Options passed to `tokenizer(text, options)`.

**Properties**

- `text_pair` (`string` | `null` | `string[]`) _optional_ — defaults to `null` — Optional second sequence to be encoded. Must match the shape of `text` — string when `text` is a string, array when `text` is an array.
- `padding` (`boolean` | `'max_length'`) _optional_ — defaults to `false` — Whether to pad the input sequences.
- `add_special_tokens` (`boolean`) _optional_ — defaults to `true` — Whether or not to add the special tokens associated with the corresponding model.
- `truncation` (`boolean` | `null`) _optional_ — defaults to `null` — Whether to truncate the input sequences.
- `max_length` (`number` | `null`) _optional_ — defaults to `null` — Maximum length of the returned list and optionally padding length.
- `return_tensor` (`boolean`) _optional_ — defaults to `true` — Whether to return the results as Tensors or arrays.
- `return_token_type_ids` (`boolean` | `null`) _optional_ — defaults to `null` — Whether to return the token type ids.

### ApplyChatTemplateOptions

**Properties**

- `chat_template` (`string` | `null`) _optional_ — defaults to `null` — A Jinja template to use for this conversion. If omitted, the model's chat template is used.
- `tools` (`Object[]` | `null`) _optional_ — defaults to `null` — JSON Schema tool definitions exposed to templates that support function calling.
  See the [chat templating guide](https://huggingface.co/docs/transformers/main/en/chat_templating#automated-function-conversion-for-tool-use).
- `documents` (`Record`<`string`, `string`>[] | `null`) _optional_ — defaults to `null` — Documents exposed to templates that support retrieval-augmented generation.
  See the [RAG section](https://huggingface.co/docs/transformers/main/en/chat_templating#arguments-for-RAG) of the chat templating guide.
- `add_generation_prompt` (`boolean`) _optional_ — defaults to `false` — Whether to end the prompt with the token(s) that indicate the start of an assistant message.
  The template must support this argument for it to have any effect.
- `tokenize` (`boolean`) _optional_ — defaults to `true` — Whether to tokenize the output. If false, the output will be a string.
- `padding` (`boolean`) _optional_ — defaults to `false` — Whether to pad sequences to the maximum length. Has no effect if tokenize is false.
- `truncation` (`boolean`) _optional_ — defaults to `false` — Whether to truncate sequences to the maximum length. Has no effect if tokenize is false.
- `max_length` (`number` | `null`) _optional_ — defaults to `null` — Maximum length (in tokens) to use for padding or truncation. If omitted, the tokenizer's `max_length` is used.
  Has no effect if tokenize is false.
- `return_tensor` (`boolean`) _optional_ — defaults to `true` — Whether to return the output as a Tensor or an Array. Has no effect if tokenize is false.
- `return_dict` (`boolean`) _optional_ — defaults to `true` — Whether to return a dictionary with named outputs. Has no effect if tokenize is false.
- `tokenizer_kwargs` (`Object`) _optional_ — defaults to `{}` — Additional options to pass to the tokenizer.

### ApplyChatTemplateReturn

_Type:_ `string` | [`BatchEncodingItem`](./tokenizers#module_tokenizers.BatchEncodingItem)<`string`, `boolean`> | [`BatchEncoding`](./tokenizers#module_tokenizers.BatchEncoding)<[`BatchEncodingItem`](./tokenizers#module_tokenizers.BatchEncodingItem)<`string`, `boolean`>>

## Callbacks

### PreTrainedTokenizerCallback

**Parameters**

- `text` (`string` | `string[]`)
- `options` ([`TokenizerCallOptions`](./tokenizers#module_tokenizers.TokenizerCallOptions)<`string` | `string[]`, `boolean`>) _optional_

**Returns:** [`BatchEncoding`](./tokenizers#module_tokenizers.BatchEncoding)<[`BatchEncodingItem`](./tokenizers#module_tokenizers.BatchEncodingItem)<`string` | `string[]`, `boolean`>>
