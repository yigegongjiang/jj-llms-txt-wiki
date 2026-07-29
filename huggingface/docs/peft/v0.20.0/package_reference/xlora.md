# X-LoRA

Mixture of LoRA Experts ([X-LoRA](https://huggingface.co/papers/2402.07148)) is a PEFT method enabling sparse or dense mixture of LoRA experts based on a high granularity (token, layer, sequence) scalings matrix. This leverages frozen LoRA adapters and a frozen base model to drastically reduces the number of parameters that need to be fine-tuned.

A unique aspect of X-LoRA is its versatility: it can be applied to any `transformers` base model with LoRA adapters. This means that, despite the mixture of experts strategy, no changes to the model code must be made.

The below graphic demonstrates how the scalings change for different prompts for each token. This highlights the activation of different adapters as the generation progresses and the sequence creates new context.

![Token-by-token scalings](https://github.com/EricLBuehler/xlora/raw/master/res/token_by_token_scalings.gif)

For each step, X-LoRA requires the base model to be run twice: first, to get hidden states without any LoRA adapters, and secondly, the hidden states are used to calculate scalings which are applied to the LoRA adapters and the model is run a second time. The output of the second run is the result of the model step.

Ultimately, X-LoRA allows the model to reflect upon its knowledge because of the dual forward pass scheme, and dynamically reconfigure the architecture.

The abstract from the paper is:

*We report a mixture of expert strategy to create fine-tuned large language models using a deep layer-wise token-level approach based on low-rank adaptation (LoRA). Starting with a set of pre-trained LoRA adapters, our gating strategy uses the hidden states to dynamically mix adapted layers, allowing the resulting X-LoRA model to draw upon different capabilities and create never-before-used deep layer-wise combinations to solve tasks. The design is inspired by the biological principles of universality and diversity, where neural network building blocks are reused in different hierarchical manifestations. Hence, the X-LoRA model can be easily implemented for any existing large language model (LLM) without a need for modifications of the underlying structure. We develop a tailored X-LoRA model that offers scientific capabilities including forward/inverse analysis tasks and enhanced reasoning capability, focused on biomaterial analysis, protein mechanics and design. The impact of this work include access to readily expandable and adaptable models with strong domain knowledge and the capability to integrate across areas of knowledge. Featuring experts in biology, mathematics, reasoning, bio-inspired materials, mechanics and materials, chemistry, protein biophysics, mechanics and quantum-mechanics based molecular properties, we conduct a series of physics-focused case studies. We examine knowledge recall, protein mechanics forward/inverse tasks, protein design, adversarial agentic modeling including ontological knowledge graph construction, as well as molecular design. The model is capable not only of making quantitative predictions of nanomechanical properties of proteins or quantum mechanical molecular properties, but also reasons over the results and correctly predicts likely mechanisms that explain distinct molecular behaviors.*.

Please cite X-LoRA as:
```bibtex
@article{10.1063/5.0203126,
    author = {Buehler, Eric L. and Buehler, Markus J.},
    title = "{X-LoRA: Mixture of low-rank adapter experts, a flexible framework for large language models with applications in protein mechanics and molecular design}",
    journal = {APL Machine Learning},
    volume = {2},
    number = {2},
    pages = {026119},
    year = {2024},
    month = {05},
    abstract = "{We report a mixture of expert strategy to create fine-tuned large language models using a deep layer-wise token-level approach based on low-rank adaptation (LoRA). Starting with a set of pre-trained LoRA adapters, our gating strategy uses the hidden states to dynamically mix adapted layers, allowing the resulting X-LoRA model to draw upon different capabilities and create never-before-used deep layer-wise combinations to solve tasks. The design is inspired by the biological principles of universality and diversity, where neural network building blocks are reused in different hierarchical manifestations. Hence, the X-LoRA model can be easily implemented for any existing large language model without a need for modifications of the underlying structure. We develop a tailored X-LoRA model that offers scientific capabilities, including forward/inverse analysis tasks and enhanced reasoning capability, focused on biomaterial analysis, protein mechanics, and design. The impact of this work includes access to readily expandable and adaptable models with strong domain knowledge and the capability to integrate across areas of knowledge. Featuring experts in biology, mathematics, reasoning, bio-inspired materials, mechanics and materials, chemistry, protein biophysics, mechanics, and quantum-mechanics based molecular properties, we conduct a series of physics-focused case studies. We examine knowledge recall, protein mechanics forward/inverse tasks, protein design, adversarial agentic modeling including ontological knowledge graph construction, and molecular design. The model is capable not only of making quantitative predictions of nanomechanical properties of proteins or quantum mechanical molecular properties but also reasoning over the results and correctly predicting likely mechanisms that explain distinct molecular behaviors.}",
    issn = {2770-9019},
    doi = {10.1063/5.0203126},
    url = {https://doi.org/10.1063/5.0203126},
    eprint = {https://pubs.aip.org/aip/aml/article-pdf/doi/10.1063/5.0203126/19964043/026119\_1\_5.0203126.pdf},
}
```

# API

## XLoraConfig[[peft.XLoraConfig]]

- **hidden_size** (`int`) --
  Hidden size of the base model.
- **adapters** (`dict`) --
  Mapping of adapter names to the LoRA adapter id, as per PeftModel.load_adapter. *They will be automatically
  loaded*, to use as LoRA experts. When using from_pretrained, pass the new adapters dict as a keyword
  argument.
- **enable_softmax** (`bool`, *optional*, defaults to `True`) --
  Enable softmax application for the X-LoRA classifier.
- **enable_softmax_topk** (`bool`, *optional*, defaults to `False`) --
  Enable softmax application for the top-k LoRA adapters. Mutually exclusive to `enable_softmax` and must
  only be set if `top_k_lora` is.
- **softmax_temperature** (`float`, *optional*, defaults to 1.0) --
  Softmax temperature, lower yields sharper predictions
- **layerwise_scalings** (`bool`, *optional*, defaults to `False`) --
  If True, generate scalings for each LoRA adapter (each layer). If this is False, then scalings will be
  broadcasted, the same, to each layer.
- **top_k_lora** (`int`, *optional*, defaults to None) --
  Sparsely select the top_k LoRA experts instead of the default dense method.
- **xlora_depth** (`int`, *optional*, defaults to 1) --
  Depth of the X-LoRA classifier.
- **xlora_size** (`int`, *optional*, defaults to 2048) --
  Hidden size of the X-LoRA classifier, irrelevant if `xlora_depth=1`.
- **xlora_dropout_p** (`float`, *optional*, defaults to 0.2) --
  Dropout probability of the X-LoRA classifier, irrelevant if `xlora_depth=1`.
- **use_trainable_adapters** (`bool`, *optional*, defaults to False) --
  Make the adapters trainable.
- **scaling_pass_value** (`float`, *optional*, defaults to 0) --
  Scaling pass value.
- **global_scaling_weight** (`float`, *optional*, defaults to 1) --
  Weight to multiply output of each LoRA adapter by.

This is the configuration class to store the configuration of a `XLoraModel`. When the config is reloaded, the
paths of the `adapters` field is disregarded in favor of the saved adapters. As such, only the keys matter during
loading.

## XLoraModel[[peft.XLoraModel]]

- **model** (`torch.nn.Module`) -- The model to be adapted.
- **config** ([XLoraConfig](/docs/peft/v0.20.0/en/package_reference/xlora#peft.XLoraConfig)) -- The configuration of the Lora model.
- **adapter_name** (`str`) -- The name of the adapter, does not affect the LoRA adapter names.`torch.nn.Module`The X-LoRA model.

Creates an X-LoRA (Mixture of LoRA experts), model from a pretrained transformers model. Currently, this X-LoRA
implementation only works with models with a transformer architecture.

The method is described in detail in https://huggingface.co/papers/2402.07148.

Example:
```py
>>> import torch
>>> from transformers import AutoModelForCausalLM, AutoConfig, BitsAndBytesConfig
>>> from peft import XLoraConfig, get_peft_model, prepare_model_for_kbit_training

>>> model_config = AutoConfig.from_pretrained("mistralai/Mistral-7B-Instruct-v0.1")
>>> config = XLoraConfig(
...     task_type="CAUSAL_LM",
...     hidden_size=model_config.hidden_size,
...     xlora_depth=4,
...     adapters={
...         "adapter_1": "./path/to/the/checkpoint/",
...         "adapter_2": "./path/to/the/checkpoint/",
...         "adapter_n": "./path/to/the/checkpoint/",
...     },
... )
>>> int8_config = BitsAndBytesConfig(load_in_8bit=True)
>>> model = AutoModelForCausalLM.from_pretrained(
...     "mistralai/Mistral-7B-Instruct-v0.1",
...     trust_remote_code=True,
...     attn_implementation="flash_attention_2",
...     device_map="cuda:0",
...     torch_dtype=torch.bfloat16,
...     quantization_config=int8_config,
... )
>>> model = prepare_model_for_kbit_training(model)
>>> xlora_model = get_peft_model(model, config)
```

Clear the scalings log.

Disable scalings logging, without clearing the log.

Enable scalings logging.

Returns bucketed scalings, bucketed by seq_len. Each value consists of the positions (the first) and the
associated tensors. The positions are paired with the associated tensors and give the position in the scaling
log.

Get the global LoRA weight.

Returns the latest scalings prediction, or None if no scalings have been predicted. The tensor is of shape
(batch_size, seq_len, n_layers, n_classes).

Returns a shallow (only copying the list itself not the tensors) copy of the list containing the scalings log.
Editing the list does not change the underlying log. The tensors are of shape (batch_size, seq_len, n_layers,
n_classes). The seq_len dim may vary with input dimension.

Set the global LoRA weight, a scalar to multiply the output of each LoRA adapter by. This is by default 1. This
is reflected in the config.

Set the scaling pass value, the value to set the scalings to during the scaling pass. If the value is None, the
scaling pass value will be 1/n where n is the number of adapters.

Sparsely select the specified top_k LoRA experts instead of the default dense method. Set to None to use dense.
This is reflected in the config.
