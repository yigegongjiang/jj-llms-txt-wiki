# Trainable Tokens

The Trainable Tokens method provides a way to target specific token embeddings for fine-tuning without resorting to
training the full embedding matrix or using an adapter on the embedding matrix. It is based on the initial implementation from
[here](https://github.com/huggingface/peft/pull/1541).

The method only targets specific tokens and selectively trains the token indices you specify. Consequently the
required RAM will be lower and disk memory is also significantly lower than storing the full fine-tuned embedding matrix.

Some preliminary benchmarks acquired with [this script](https://github.com/huggingface/peft/blob/main/scripts/train_memory.py)
suggest that for `gemma-2-2b` (which has a rather large embedding matrix) you can save ~4 GiB VRAM with Trainable Tokens
over fully fine-tuning the embedding matrix. While LoRA will use comparable amounts of VRAM it might also target
tokens you don't want to be changed. Note that these are just indications and varying embedding matrix sizes might skew
these numbers a bit.

Note that this method does not add tokens for you, you have to add tokens to the tokenizer yourself and resize the
embedding matrix of the model accordingly. This method will only re-train the embeddings for the tokens you specify.
This method can also be used in conjunction with LoRA layers! See [the LoRA documentation](lora#efficiently-train-tokens-alongside-lora).

> [!TIP]
> Saving the model with [save_pretrained()](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel.save_pretrained) or retrieving the state dict using
> [get_peft_model_state_dict()](/docs/peft/v0.20.0/en/package_reference/functional#peft.get_peft_model_state_dict) when adding new tokens may save the full embedding matrix instead of only the difference
> as a precaution because the embedding matrix was resized. To save space you can disable this behavior by setting
> `save_embedding_layers=False` when calling `save_pretrained`. This is safe to do as long as you don't modify the
> embedding matrix through other means as well, as such changes will be not tracked by trainable tokens.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=TRAINABLE_TOKENS"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## TrainableTokensConfig[[peft.TrainableTokensConfig]]

#### peft.TrainableTokensConfig[[peft.TrainableTokensConfig]]

```python
peft.TrainableTokensConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, token_indices: list[int] = <factory>, target_modules: Optional[Union[list[str], str]] = None, init_weights: bool = True)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/trainable_tokens/config.py#L25)

**Parameters:**

token_indices (`list[int]`) : List of integers, signifying the indices of the tokens you want to be trainable. To find the index of a token with a tokenizer, you can tokenize the string and look at the returned `input_ids`. The closer the amount of indices is to the total amount of tokens, the less efficient this method gets.

target_modules (`Optional[Union[list[str], str]]`) : List of module names or regex expression of the module names to replace with our `TrainableTokensLayer`. If not defined, it will attempt to get the model's input embedding layer if the model has a `get_input_embeddings` method (transformer models usually do), if that fails the default is 'embed_tokens'. Other example targets are `embedding`, `encoder.embeddings` or `decoder.embeddings`.

init_weights (`bool`) : By default the new token weights are initialized to be the same as the respective token embeddings. This makes TrainableTokens a no-op when not trained. If set to `False` the weights will be random values. Do not change this setting unless you know exactly what you're doing.

Configuration for the `TrainableTokens` method.

Allows for training new tokens (and re-training existing ones) without training the full embedding matrix. By
marking a few select tokens (identified by their indices) trainable and leaving the rest untouched, this method can
be used to add new tokens or changing the embedding of existing tokens while saving on memory. Both storage as well
as working memory usage are reduced in contrast to training the embedding matrix fully.

Note that training with FSDP/DeepSpeed might not yet be fully supported.

## TrainableTokensModel[[peft.TrainableTokensModel]]

#### peft.TrainableTokensModel[[peft.TrainableTokensModel]]

```python
peft.TrainableTokensModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/trainable_tokens/model.py#L26)
