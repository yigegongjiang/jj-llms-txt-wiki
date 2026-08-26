# Reformer

## Overview

The Reformer model was proposed in the paper [Reformer: The Efficient Transformer](https://huggingface.co/papers/2001.04451) by Nikita Kitaev, Łukasz Kaiser, Anselm Levskaya.

The abstract from the paper is the following:

*Large Transformer models routinely achieve state-of-the-art results on a number of tasks but training these models can
be prohibitively costly, especially on long sequences. We introduce two techniques to improve the efficiency of
Transformers. For one, we replace dot-product attention by one that uses locality-sensitive hashing, changing its
complexity from O(L^2) to O(Llog(L)), where L is the length of the sequence. Furthermore, we use reversible residual
layers instead of the standard residuals, which allows storing activations only once in the training process instead of
N times, where N is the number of layers. The resulting model, the Reformer, performs on par with Transformer models
while being much more memory-efficient and much faster on long sequences.*

This model was contributed by [patrickvonplaten](https://huggingface.co/patrickvonplaten). The Authors' code can be
found [here](https://github.com/google/trax/tree/master/trax/models/reformer).

## Usage tips

- Reformer does **not** work with *torch.nn.DataParallel* due to a bug in PyTorch, see [issue #36035](https://github.com/pytorch/pytorch/issues/36035).
- Use Axial position encoding (see below for more details). It's a mechanism to avoid having a huge positional encoding matrix (when the sequence length is very big) by factorizing it into smaller matrices.
- Replace traditional attention by LSH (local-sensitive hashing) attention (see below for more details). It's a technique to avoid computing the full product query-key in the attention layers.
- Avoid storing the intermediate results of each layer by using reversible transformer layers to obtain them during the backward pass (subtracting the residuals from the input of the next layer gives them back) or recomputing them for results inside a given layer (less efficient than storing them but saves memory).
- Compute the feedforward operations by chunks and not on the whole batch.

### Axial Positional Encodings

Axial Positional Encodings were first implemented in Google's [trax library](https://github.com/google/trax/blob/4d99ad4965bab1deba227539758d59f0df0fef48/trax/layers/research/position_encodings.py#L29)
and developed by the authors of this model's paper. In models that are treating very long input sequences, the
conventional position id encodings store an embeddings vector of size $d$ being the `config.hidden_size` for
every position $i, \ldots, n_s$, with $n_s$ being `config.max_embedding_size`. This means that having
a sequence length of $n_s = 2^{19} \approx 0.5M$ and a `config.hidden_size` of $d = 2^{10} \approx 1000$
would result in a position encoding matrix:

$$X_{i,j}, \text{ with } i \in \left[1,\ldots, d\right] \text{ and } j \in \left[1,\ldots, n_s\right]$$

which alone has over 500M parameters to store. Axial positional encodings factorize $X_{i,j}$ into two matrices:

$$X^{1}_{i,j}, \text{ with } i \in \left[1,\ldots, d^1\right] \text{ and } j \in \left[1,\ldots, n_s^1\right]$$

and

$$X^{2}_{i,j}, \text{ with } i \in \left[1,\ldots, d^2\right] \text{ and } j \in \left[1,\ldots, n_s^2\right]$$

with:

$$d = d^1 + d^2 \text{ and } n_s = n_s^1 \times n_s^2 .$$

Therefore the following holds:

$$X_{i,j} = \begin{cases}
X^{1}_{i, k}, & \text{if }\ i < d^1 \text{ with } k = j \mod n_s^1 \\
X^{2}_{i - d^1, l}, & \text{if } i \ge d^1 \text{ with } l = \lfloor\frac{j}{n_s^1}\rfloor
\end{cases}$$

Intuitively, this means that a position embedding vector $x_j \in \mathbb{R}^{d}$ is now the composition of two
factorized embedding vectors: $x^1_{k, l} + x^2_{l, k}$, where as the `config.max_embedding_size` dimension
$j$ is factorized into $k \text{ and } l$. This design ensures that each position embedding vector
$x_j$ is unique.

Using the above example again, axial position encoding with $d^1 = 2^9, d^2 = 2^9, n_s^1 = 2^9, n_s^2 = 2^{10}$
can drastically reduced the number of parameters from 500 000 000 to $2^{18} + 2^{19} \approx 780 000$ parameters, this means 85% less memory usage.

In practice, the parameter `config.axial_pos_embds_dim` is set to a tuple $(d^1, d^2)$ which sum has to be
equal to `config.hidden_size` and `config.axial_pos_shape` is set to a tuple $(n_s^1, n_s^2)$ which
product has to be equal to `config.max_embedding_size`, which during training has to be equal to the *sequence
length* of the `input_ids`.

### LSH Self Attention

In Locality sensitive hashing (LSH) self attention the key and query projection weights are tied. Therefore, the key
query embedding vectors are also tied. LSH self attention uses the locality sensitive hashing mechanism proposed in
[Practical and Optimal LSH for Angular Distance](https://huggingface.co/papers/1509.02897) to assign each of the tied key
query embedding vectors to one of `config.num_buckets` possible buckets. The premise is that the more "similar"
key query embedding vectors (in terms of *cosine similarity*) are to each other, the more likely they are assigned to
the same bucket.

The accuracy of the LSH mechanism can be improved by increasing `config.num_hashes` or directly the argument
`num_hashes` of the forward function so that the output of the LSH self attention better approximates the output
of the "normal" full self attention. The buckets are then sorted and chunked into query key embedding vector chunks
each of length `config.lsh_chunk_length`. For each chunk, the query embedding vectors attend to its key vectors
(which are tied to themselves) and to the key embedding vectors of `config.lsh_num_chunks_before` previous
neighboring chunks and `config.lsh_num_chunks_after` following neighboring chunks.

For more information, see the [original Paper](https://huggingface.co/papers/2001.04451) or this great [blog post](https://www.pragmatic.ml/reformer-deep-dive/).

Note that `config.num_buckets` can also be factorized into a list $(n_{\text{buckets}}^1,
n_{\text{buckets}}^2)$. This way instead of assigning the query key embedding vectors to one of $(1,\ldots,
n_{\text{buckets}})$ they are assigned to one of $(1-1,\ldots, n_{\text{buckets}}^1-1, \ldots,
1-n_{\text{buckets}}^2, \ldots, n_{\text{buckets}}^1-n_{\text{buckets}}^2)$. This is crucial for very long sequences to
save memory.

When training a model from scratch, it is recommended to leave `config.num_buckets=None`, so that depending on the
sequence length a good value for `num_buckets` is calculated on the fly. This value will then automatically be
saved in the config and should be reused for inference.

Using LSH self attention, the memory and time complexity of the query-key matmul operation can be reduced from
$\mathcal{O}(n_s \times n_s)$ to $\mathcal{O}(n_s \times \log(n_s))$, which usually represents the memory
and time bottleneck in a transformer model, with $n_s$ being the sequence length.

### Local Self Attention

Local self attention is essentially a "normal" self attention layer with key, query and value projections, but is
chunked so that in each chunk of length `config.local_chunk_length` the query embedding vectors only attends to
the key embedding vectors in its chunk and to the key embedding vectors of `config.local_num_chunks_before`
previous neighboring chunks and `config.local_num_chunks_after` following neighboring chunks.

Using Local self attention, the memory and time complexity of the query-key matmul operation can be reduced from
$\mathcal{O}(n_s \times n_s)$ to $\mathcal{O}(n_s \times \log(n_s))$, which usually represents the memory
and time bottleneck in a transformer model, with $n_s$ being the sequence length.

### Training

During training, we must ensure that the sequence length is set to a value that can be divided by the least common
multiple of `config.lsh_chunk_length` and `config.local_chunk_length` and that the parameters of the Axial
Positional Encodings are correctly set as described above. Reformer is very memory efficient so that the model can
easily be trained on sequences as long as 64000 tokens.

For training, the [ReformerModelWithLMHead](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerModelWithLMHead) should be used as follows:

```python
input_ids = tokenizer.encode("This is a sentence from the training data", return_tensors="pt").to(model.device)
loss = model(input_ids, labels=input_ids)[0]
```

## Resources

- [Text classification task guide](../tasks/sequence_classification)
- [Question answering task guide](../tasks/question_answering)
- [Causal language modeling task guide](../tasks/language_modeling)
- [Masked language modeling task guide](../tasks/masked_language_modeling)

## ReformerConfig[[transformers.ReformerConfig]]

#### transformers.ReformerConfig[[transformers.ReformerConfig]]

```python
transformers.ReformerConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, attention_head_size: int = 64, attn_layers: list[str] | tuple[str, ...] = ('local', 'lsh', 'local', 'lsh', 'local', 'lsh'), axial_norm_std: float = 1.0, axial_pos_embds: bool = True, axial_pos_shape: list[int] | tuple[int, ...] = (64, 64), axial_pos_embds_dim: list[int] | tuple[int, ...] = (64, 192), chunk_size_lm_head: int = 0, eos_token_id: int | list[int] | None = 2, feed_forward_size: int = 512, hash_seed: int | None = None, hidden_act: str = 'relu', hidden_dropout_prob: float | int = 0.05, hidden_size: int = 256, initializer_range: float = 0.02, is_decoder: bool = False, layer_norm_eps: float = 1e-12, local_num_chunks_before: int = 1, local_num_chunks_after: int = 0, local_attention_probs_dropout_prob: float | int = 0.05, local_attn_chunk_length: int = 64, lsh_attn_chunk_length: int | None = 64, lsh_attention_probs_dropout_prob: float | None = 0.0, lsh_num_chunks_before: int | None = 1, lsh_num_chunks_after: int | None = 0, max_position_embeddings: int = 4096, num_attention_heads: int = 12, num_buckets: int | list[int] | None = None, num_hashes: int = 1, vocab_size: int = 320, tie_word_embeddings: bool = False, use_cache: bool = True, classifier_dropout: float | int | None = None, bos_token_id: int | None = None, pad_token_id: int | None = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/configuration_reformer.py#L25)

**Parameters:**

attention_head_size (`int`, *optional*, defaults to 64) : Dimensionality of the projected key, query and value vectors

attn_layers (`list[str]`, *optional*, defaults to `["local", "lsh", "local", "lsh", "local", "lsh"]`) : List of attention layer types in ascending order. It can be chosen between a LSHSelfAttention layer (`"lsh"`) and a LocalSelfAttention layer (`"local"`). For more information on LSHSelfAttention layer, see [LSH Self Attention](reformer#lsh-self-attention). For more information on LocalSelfAttention layer, see [Local Self Attention](reformer#local-self-attention).

axial_norm_std (`float`, *optional*, defaults to 1.0) : The standard deviation of the normal_initializer for initializing the weight matrices of the axial positional encodings.

axial_pos_embds (`bool`, *optional*, defaults to `True`) : Whether or not to use axial position embeddings. For more information on how axial position embeddings work, see [Axial Position Encodings](reformer#axial-positional-encodings).

axial_pos_shape (`list[int]`, *optional*, defaults to `[64, 64]`) : The position dims of the axial position encodings. During training, the product of the position dims has to be equal to the sequence length. For more information on how axial position embeddings work, see [Axial Position Encodings](reformer#axial-positional-encodings).

axial_pos_embds_dim (`list[int]`, *optional*, defaults to `[64, 192]`) : The embedding dims of the axial position encodings. The sum of the embedding dims has to be equal to the hidden size. For more information on how axial position embeddings work, see [Axial Position Encodings](reformer#axial-positional-encodings).

chunk_size_lm_head (`int`, *optional*, defaults to 0) : The chunk size of the final language model feed forward head layer. A chunk size of 0 means that the feed forward layer is not chunked. A chunk size of n means that the feed forward layer processes n < sequence_length embeddings at a time. For more information on feed forward chunking, see [How does Feed Forward Chunking work?](../glossary#feed-forward-chunking).

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

feed_forward_size (`int`, *optional*, defaults to 512) : Dimensionality of the feed_forward layer in the residual attention block.

hash_seed (`int`, *optional*) : Seed that can be used to make local sensitive hashing in `LSHSelfAttention` deterministic. This should only be set for testing purposed. For evaluation and training purposes `hash_seed` should be left as `None` to ensure fully random rotations in local sensitive hashing scheme.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.05`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

is_decoder (`bool`, *optional*, defaults to `False`) : Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

local_num_chunks_before (`int`, *optional*, defaults to 1) : Number of previous neighbouring chunks to attend to in `LocalSelfAttention` layer to itself.

local_num_chunks_after (`int`, *optional*, defaults to 0) : Number of following neighbouring chunks to attend to in `LocalSelfAttention` layer in addition to itself.

local_attention_probs_dropout_prob (`float`, *optional*, defaults to 0.1) : The dropout ratio for the attention probabilities in `LocalSelfAttention`.

local_attn_chunk_length (`int`, *optional*, defaults to 64) : Length of each chunk in local attention layers.

lsh_attn_chunk_length (`int`, *optional*, defaults to 64) : Length of chunk which attends to itself in `LSHSelfAttention`. Chunking reduces memory complexity from sequence length x sequence length (self attention) to chunk length x chunk length x sequence length / chunk length (chunked self attention).

lsh_attention_probs_dropout_prob (`float`, *optional*, defaults to 0.1) : The dropout ratio for the attention probabilities in `LSHSelfAttention`.

lsh_num_chunks_before (`int`, *optional*, defaults to 1) : Number of previous neighbouring chunks to attend to in `LSHSelfAttention` layer to itself.

lsh_num_chunks_after (`int`, *optional*, defaults to 0) : Number of following neighbouring chunks to attend to in `LSHSelfAttention` layer to itself.

max_position_embeddings (`int`, *optional*, defaults to `4096`) : The maximum sequence length that this model might ever be used with.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

num_buckets (`int` or `list[int]`, *optional*) : Number of buckets, the key query vectors can be "hashed into" using the locality sensitive hashing scheme. Each query key vector is hashed into a hash in `1, ..., num_buckets`. The number of buckets can also be factorized into a list for improved memory complexity. In this case, each query key vector is hashed into a hash in `1-1, 1-2, ..., num_buckets[0]-1, ..., num_buckets[0]-num_buckets[1]` if `num_buckets` is factorized into two factors. The number of buckets (or the product the factors) should approximately equal sequence length / lsh_chunk_length. If `num_buckets` not set, a good value is calculated on the fly.

num_hashes (`int`, *optional*, defaults to 1) : Number of hashing rounds (e.g., number of random rotations) in Local Sensitive Hashing scheme. The higher `num_hashes`, the more accurate the `LSHSelfAttention` becomes, but also the more memory and time intensive the hashing becomes.

vocab_size (`int`, *optional*, defaults to `320`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

classifier_dropout (`Union[float, int]`, *optional*) : The dropout ratio for classifier.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a ReformerModel. It is used to instantiate a Reformer
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/reformer-crime-and-punishment](https://huggingface.co/google/reformer-crime-and-punishment)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import ReformerConfig, ReformerModel

>>> # Initializing a Reformer configuration
>>> configuration = ReformerConfig()

>>> # Initializing a Reformer model (with random weights)
>>> model = ReformerModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ReformerTokenizer[[transformers.ReformerTokenizer]]

#### transformers.ReformerTokenizer[[transformers.ReformerTokenizer]]

```python
transformers.ReformerTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, eos_token: str = '</s>', unk_token: str = '<unk>', _spm_precompiled_charsmap: str | None = None, additional_special_tokens: list | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/tokenization_reformer.py#L31)

**Parameters:**

vocab_file (`str`) : [SentencePiece](https://github.com/google/sentencepiece) file (generally has a *.spm* extension) that contains the vocabulary necessary to instantiate a tokenizer.

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

additional_special_tokens (`list[str]`, *optional*) : Additional special tokens used by the tokenizer.

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `vocab_file`.

Construct a Reformer tokenizer (backed by HuggingFace's tokenizers library). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=bpe#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### save_vocabulary[[transformers.ReformerTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_tokenizers.py#L509)

## ReformerTokenizerFast[[transformers.ReformerTokenizer]]

#### transformers.ReformerTokenizer[[transformers.ReformerTokenizer]]

```python
transformers.ReformerTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, eos_token: str = '</s>', unk_token: str = '<unk>', _spm_precompiled_charsmap: str | None = None, additional_special_tokens: list | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/tokenization_reformer.py#L31)

**Parameters:**

vocab_file (`str`) : [SentencePiece](https://github.com/google/sentencepiece) file (generally has a *.spm* extension) that contains the vocabulary necessary to instantiate a tokenizer.

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

additional_special_tokens (`list[str]`, *optional*) : Additional special tokens used by the tokenizer.

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `vocab_file`.

Construct a Reformer tokenizer (backed by HuggingFace's tokenizers library). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=bpe#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

## ReformerModel[[transformers.ReformerModel]]

#### transformers.ReformerModel[[transformers.ReformerModel]]

```python
transformers.ReformerModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L1931)

**Parameters:**

config ([ReformerModel](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Reformer Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ReformerModel.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, num_hashes: int | None = None, past_buckets_states: transformers.models.reformer.modeling_reformer.ReformerDynamicCache | None = None, use_cache: bool | None = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L1951)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. During training the input_ids sequence_length has to be a multiple of the relevant model's chunk lengths (lsh's, local's or both). During evaluation, the indices are automatically padded to be a multiple of the chunk length.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

num_hashes (`int`, *optional*) : The number of hashing rounds that should be performed during bucketing. Setting this argument overwrites the default defined in `config.num_hashes`.  For more information, see `num_hashes` in [ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig).

past_buckets_states (`ReformerDynamicCache`, *optional*) : List of `tuple(torch.LongTensor, torch.FloatTensor` of length `config.n_layers`, with the first element being the previous *buckets* of shape `(batch_size, num_heads, num_hashes, sequence_length)`) and the second being the previous *hidden_states* of shape `(batch_size, sequence_length, hidden_size)`).  Contains precomputed hidden-states and buckets (only relevant for LSH Self-Attention). Can be used to speed up sequential decoding.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `ReformerModelOutput` or `tuple(torch.FloatTensor)`

A `ReformerModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig)) and inputs.

The [ReformerModel](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_predict, hidden_size)`) -- Sequence of hidden-states at the last layer of the model.

  `num_predict` corresponds to `target_mapping.shape[1]`. If `target_mapping` is `None`, then `num_predict`
  corresponds to `sequence_length`.
- **past_buckets_states** (`list[tuple(torch.LongTensor, torch.FloatTensor)]`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- List of `tuple(torch.LongTensor, torch.FloatTensor` of length `config.n_layers`, with the first element
  being the previous *buckets* of shape `(batch_size, num_heads, num_hashes, sequence_length)`) and the
  second being the previous *hidden_states* of shape `(batch_size, sequence_length, hidden_size)`).

  Contains precomputed buckets and hidden-states that can be used (see `past_buckets_states` input) to speed
  up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## ReformerModelWithLMHead[[transformers.ReformerModelWithLMHead]]

#### transformers.ReformerModelWithLMHead[[transformers.ReformerModelWithLMHead]]

```python
transformers.ReformerModelWithLMHead(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2150)

**Parameters:**

config ([ReformerModelWithLMHead](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerModelWithLMHead)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Reformer Model with a `language modeling` head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ReformerModelWithLMHead.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, num_hashes: int | None = None, past_buckets_states: list[tuple[torch.Tensor]] | None = None, use_cache: bool | None = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2176)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. During training the input_ids sequence_length has to be a multiple of the relevant model's chunk lengths (lsh's, local's or both). During evaluation, the indices are automatically padded to be a multiple of the chunk length.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

num_hashes (`int`, *optional*) : The number of hashing rounds that should be performed during bucketing. Setting this argument overwrites the default defined in `config.num_hashes`.  For more information, see `num_hashes` in [ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig).

past_buckets_states (`list[tuple(torch.LongTensor, torch.FloatTensor)]`, *optional*) : List of `tuple(torch.LongTensor, torch.FloatTensor` of length `config.n_layers`, with the first element being the previous *buckets* of shape `(batch_size, num_heads, num_hashes, sequence_length)`) and the second being the previous *hidden_states* of shape `(batch_size, sequence_length, hidden_size)`).  Contains precomputed hidden-states and buckets (only relevant for LSH Self-Attention). Can be used to speed up sequential decoding.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[-100, 0, ..., config.vocab_size - 1]`. All labels set to `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or `tuple(torch.FloatTensor)`

A [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig)) and inputs.

The [ReformerModelWithLMHead](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerModelWithLMHead) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ReformerModelWithLMHead

>>> tokenizer = AutoTokenizer.from_pretrained("google/reformer-crime-and-punishment")
>>> model = ReformerModelWithLMHead.from_pretrained("google/reformer-crime-and-punishment")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")
>>> outputs = model(**inputs, labels=inputs["input_ids"])
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## ReformerForMaskedLM[[transformers.ReformerForMaskedLM]]

#### transformers.ReformerForMaskedLM[[transformers.ReformerForMaskedLM]]

```python
transformers.ReformerForMaskedLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2288)

**Parameters:**

config ([ReformerForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForMaskedLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Reformer Model with a `language modeling` head on top."

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ReformerForMaskedLM.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, num_hashes: int | None = None, labels: typing.Optional[torch.Tensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2308)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. During training the input_ids sequence_length has to be a multiple of the relevant model's chunk lengths (lsh's, local's or both). During evaluation, the indices are automatically padded to be a multiple of the chunk length.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

num_hashes (`int`, *optional*) : The number of hashing rounds that should be performed during bucketing. Setting this argument overwrites the default defined in `config.num_hashes`.  For more information, see `num_hashes` in [ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels    This example uses a false checkpoint since we don't have any available pretrained model for the masked language modeling task with the Reformer architecture.  

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`

A [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig)) and inputs.

The [ReformerForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ReformerForMaskedLM

>>> tokenizer = AutoTokenizer.from_pretrained("hf-internal-testing/tiny-random-reformer")
>>> model = ReformerForMaskedLM.from_pretrained("hf-internal-testing/tiny-random-reformer")

>>> # add mask_token
>>> tokenizer.add_special_tokens({"mask_token": "[MASK]"})
>>> inputs = tokenizer("The capital of France is [MASK].", return_tensors="pt")

>>> # resize model's embedding matrix
>>> model.resize_token_embeddings(new_num_tokens=model.config.vocab_size + 1)
>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # retrieve index of [MASK]
>>> mask_token_index = (inputs.input_ids == tokenizer.mask_token_id)[0].nonzero(as_tuple=True)[0]

>>> predicted_token_id = logits[0, mask_token_index].argmax(axis=-1)
>>> predicted_token = tokenizer.decode(predicted_token_id)
```

```python
>>> labels = tokenizer("The capital of France is Paris.", return_tensors="pt")["input_ids"]
>>> # mask labels of non-[MASK] tokens
>>> labels = torch.where(
...     inputs.input_ids == tokenizer.mask_token_id, labels[:, : inputs["input_ids"].shape[-1]], -100
... )

>>> outputs = model(**inputs, labels=labels)
>>> loss = round(outputs.loss.item(), 2)
```

## ReformerForSequenceClassification[[transformers.ReformerForSequenceClassification]]

#### transformers.ReformerForSequenceClassification[[transformers.ReformerForSequenceClassification]]

```python
transformers.ReformerForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2426)

**Parameters:**

config ([ReformerForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Reformer Model transformer with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ReformerForSequenceClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, num_hashes: int | None = None, labels: typing.Optional[torch.Tensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2440)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. During training the input_ids sequence_length has to be a multiple of the relevant model's chunk lengths (lsh's, local's or both). During evaluation, the indices are automatically padded to be a multiple of the chunk length.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

num_hashes (`int`, *optional*) : The number of hashing rounds that should be performed during bucketing. Setting this argument overwrites the default defined in `config.num_hashes`.  For more information, see `num_hashes` in [ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig).

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig)) and inputs.

The [ReformerForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ReformerForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("google/reformer-crime-and-punishment")
>>> model = ReformerForSequenceClassification.from_pretrained("google/reformer-crime-and-punishment")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> label = model.config.id2label[predicted_class_id]
```

```python
>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = ReformerForSequenceClassification.from_pretrained(
...     "google/reformer-crime-and-punishment", num_labels=num_labels
... )

>>> labels = torch.tensor(1)
>>> loss = model(**inputs, labels=labels).loss
```

## ReformerForQuestionAnswering[[transformers.ReformerForQuestionAnswering]]

#### transformers.ReformerForQuestionAnswering[[transformers.ReformerForQuestionAnswering]]

```python
transformers.ReformerForQuestionAnswering(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2577)

**Parameters:**

config ([ReformerForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForQuestionAnswering)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Reformer transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ReformerForQuestionAnswering.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, num_hashes: int | None = None, start_positions: typing.Optional[torch.Tensor] = None, end_positions: typing.Optional[torch.Tensor] = None, output_hidden_states: bool | None = None, output_attentions: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/reformer/modeling_reformer.py#L2589)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. During training the input_ids sequence_length has to be a multiple of the relevant model's chunk lengths (lsh's, local's or both). During evaluation, the indices are automatically padded to be a multiple of the chunk length.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

num_hashes (`int`, *optional*) : The number of hashing rounds that should be performed during bucketing. Setting this argument overwrites the default defined in `config.num_hashes`.  For more information, see `num_hashes` in [ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig).

start_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the start of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

end_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the end of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`

A [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ReformerConfig](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerConfig)) and inputs.

The [ReformerForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/reformer#transformers.ReformerForQuestionAnswering) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.
- **start_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-start scores (before SoftMax).
- **end_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-end scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, ReformerForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/reformer-crime-and-punishment")
>>> model = ReformerForQuestionAnswering.from_pretrained("google/reformer-crime-and-punishment")

>>> question, text = "Who was Jim Henson?", "Jim Henson was a nice puppet"

>>> inputs = tokenizer(question, text, return_tensors="pt")
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> answer_start_index = outputs.start_logits.argmax()
>>> answer_end_index = outputs.end_logits.argmax()

>>> predict_answer_tokens = inputs.input_ids[0, answer_start_index : answer_end_index + 1]
>>> tokenizer.decode(predict_answer_tokens, skip_special_tokens=True)
...

>>> # target is "nice puppet"
>>> target_start_index = torch.tensor([14])
>>> target_end_index = torch.tensor([15])

>>> outputs = model(**inputs, start_positions=target_start_index, end_positions=target_end_index)
>>> loss = outputs.loss
>>> round(loss.item(), 2)
...
```
