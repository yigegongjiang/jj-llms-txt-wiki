# BEFT: Bias-Efficient Fine-Tuning of Language Models in Low-Data Regimes

[BEFT](https://arxiv.org/abs/2509.15974) is a parameter efficient fine-tuning algorithm (PEFT) that only fine-tunes the added bias terms of value projections from pretrained transformer models. BEFT demonstrates that fine-tuning the added bias terms of value projections from pretrained transformers generally leads to a higher downstream performance in low-data regimes than fine-tuning the added bias terms of query/key projections.

BEFT currently has the following tradeoffs:

Pros:
- BEFT requires far fewer parameters than LoRA, while maintaining competitive or superior performance across tasks in low-data regimes.

Cons:
- In high-data regimes, BEFT may show limited effectiveness compared to LoRA and full-parameters fine-tuning.

If your use case belongs to the high-data regime, consider other PEFT methods such as LoRA.

The abstract from the paper is:

*Fine-tuning the bias terms of large language models (LLMs) has the potential to achieve unprecedented parameter efficiency while maintaining competitive performance, particularly in low-data regimes. However, the link between fine-tuning different bias terms (i.e., **b**q, **b**k, and **b**v in the query, key, or value projections) and downstream performance remains largely unclear to date. In this paper, we investigate the link between fine-tuning **b**q, **b**k, and **b**v with the performance of the downstream task. Our key finding is that directly fine-tuning **b**v generally leads to higher downstream performance in low-data regimes, in comparison to **b**q and **b**k. We extensively evaluate this unique property across a wide range of LLMs spanning encoder-only and decoder-only architectures up to 6.7B parameters (including bias-free LLMs). Our results provide strong evidence for the effectiveness of directly fine-tuning **b**v across various downstream tasks*.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=BEFT"
	frameborder="0"
	width="850"
	height="1000"
>

# API

## BeftConfig[[peft.BeftConfig]]

- **target_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is not specified, modules will be chosen according to the model
  architecture. If the architecture is not known, an error will be raised -- in this case, you should specify
  the target modules manually.
- **modules_to_save** (`Optional[List[str]]`) --
  List of modules apart from BEFT layers to be set as trainable and saved in the final checkpoint.
- **init_weights** (`bool`) --
  Whether to initialize the vectors in the BEFT layers, defaults to `True`. Setting this to `False` is
  discouraged.

This is the configuration class to store the configuration of a [BeftModel](/docs/peft/v0.20.0/en/package_reference/beft#peft.BeftModel).

## BeftModel[[peft.BeftModel]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **config** ([BeftConfig](/docs/peft/v0.20.0/en/package_reference/beft#peft.BeftConfig)) -- The configuration of the (BEFT) model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The (BEFT) model.

Creates a Infused Adapter by only fine-tuning the added bias terms of value projections from a pretrained
transformers model in low-training-data regimes (BEFT). The method is described in detail in
https://arxiv.org/abs/2509.15974

Example:

```py
>>> from transformers import AutoModelForSeq2SeqLM
>>> from peft import BeftModel, BeftConfig

>>> config = BeftConfig(
...     peft_type="Beft",
...     task_type="SEQ_2_SEQ_LM",
...     target_modules=["v"],
... )

>>> model = AutoModelForSeq2SeqLM.from_pretrained("t5-base")
>>> beft_model = BeftModel(model, config, adapter_name="default")
```

**Attributes**:
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The model to be adapted.
- **peft_config** ([BeftConfig](/docs/peft/v0.20.0/en/package_reference/beft#peft.BeftConfig)): The configuration of the (BEFT) model.
