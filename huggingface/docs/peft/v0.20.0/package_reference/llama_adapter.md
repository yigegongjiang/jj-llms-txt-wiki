# Llama-Adapter

    

LLaMA-Adapter: Efficient Fine-tuning of Language Models with Zero-init Attention

[Llama-Adapter](https://hf.co/papers/2303.16199) is a PEFT method specifically designed for turning Llama into an instruction-following model. The Llama model is frozen and only a set of adaptation prompts prefixed to the input instruction tokens are learned. Since randomly initialized modules inserted into the model can cause the model to lose some of its existing knowledge, Llama-Adapter uses zero-initialized attention with zero gating to progressively add the instructional prompts to the model.

The abstract from the paper is:

*We present LLaMA-Adapter, a lightweight adaption method to efficiently fine-tune LLaMA into an instruction-following model. Using 52K self-instruct demonstrations, LLaMA-Adapter only introduces 1.2M learnable parameters upon the frozen LLaMA 7B model, and costs less than one hour for fine-tuning on 8 A100 GPUs. Specifically, we adopt a set of learnable adaption prompts, and prepend them to the input text tokens at higher transformer layers. Then, a zero-init attention mechanism with zero gating is proposed, which adaptively injects the new instructional cues into LLaMA, while effectively preserves its pre-trained knowledge. With efficient training, LLaMA-Adapter generates high-quality responses, comparable to Alpaca with fully fine-tuned 7B parameters. Furthermore, our approach can be simply extended to multi-modal input, e.g., images, for image-conditioned LLaMA, which achieves superior reasoning capacity on ScienceQA. We release our code at https://github.com/ZrrSkywalker/LLaMA-Adapter*.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=ADAPTION_PROMPT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## AdaptionPromptConfig[[peft.AdaptionPromptConfig]]

#### peft.AdaptionPromptConfig[[peft.AdaptionPromptConfig]]

```python
peft.AdaptionPromptConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: str = None, adapter_len: int = None, adapter_layers: int = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/config.py#L25)

Stores the configuration of an [AdaptionPromptModel](/docs/peft/v0.20.0/en/package_reference/llama_adapter#peft.AdaptionPromptModel).

## AdaptionPromptModel[[peft.AdaptionPromptModel]]

#### peft.AdaptionPromptModel[[peft.AdaptionPromptModel]]

```python
peft.AdaptionPromptModel(model, configs: dict, adapter_name: str)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/model.py#L25)

Implements adaption prompts as described in https://huggingface.co/papers/2303.16199.

The top L attention modules are replaced with AdaptedAttention modules that wrap the original ones, but insert
trainable prompts with gates (for zero init).

Notes on the multi-adapter pattern:
- We store the states of different adapters by keeping a dictionary of AdaptedAttention modules indexed by adapter
  name.
- Every time we switch adapters, we remove the modules of the currently active adapter from the model, store them
  in the dictionary, and replace them with the modules of the new adapter.
- To avoid duplicated and potentially inconsistent state, the currently active adapter is always removed from the
  dictionary.
- Disabling the adapter would also result in the modules being removed from the model.

#### add_adapter[[peft.AdaptionPromptModel.add_adapter]]

```python
add_adapter(adapter_name: str, config: AdaptionPromptConfig)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/model.py#L60)

Add an adapter with the given name and config.

#### disable_adapter_layers[[peft.AdaptionPromptModel.disable_adapter_layers]]

```python
disable_adapter_layers()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/model.py#L115)

Disable adapter layers by swapping out AdaptedAttention modules.

#### enable_adapter_layers[[peft.AdaptionPromptModel.enable_adapter_layers]]

```python
enable_adapter_layers()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/model.py#L110)

Enable adapter layers by swapping in cached AdaptedAttention modules.

#### set_adapter[[peft.AdaptionPromptModel.set_adapter]]

```python
set_adapter(adapter_name: str, inference_mode: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/adaption_prompt/model.py#L95)

Set the model to use the adapter with the given name.
