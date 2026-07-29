# OFT

    

Controlling Text-to-Image Diffusion by Orthogonal Finetuning

[Orthogonal Finetuning (OFT)](https://hf.co/papers/2306.07280) and [OFTv2](https://huggingface.co/papers/2506.19847) is a method developed for adapting text-to-image diffusion models. It works by reparameterizing the pretrained weight matrices with its orthogonal matrix to preserve information in the pretrained model. To reduce the number of parameters, OFT introduces a block-diagonal structure in the orthogonal matrix. The method primarily focuses on preserving a pretrained model's generative performance in the finetuned model. It tries to maintain the same cosine similarity ([hyperspherical energy](https://huggingface.co/papers/1805.09298)) between all pairwise neurons in a layer because this better captures the semantic information among neurons. This means OFT is more capable at preserving the subject and it is better for controllable generation (similar to [ControlNet](https://huggingface.co/docs/diffusers/using-diffusers/controlnet)).

The abstract from the paper is:

*Large text-to-image diffusion models have impressive capabilities in generating photorealistic images from text prompts. How to effectively guide or control these powerful models to perform different downstream tasks becomes an important open problem. To tackle this challenge, we introduce a principled finetuning method -- Orthogonal Finetuning (OFT), for adapting text-to-image diffusion models to downstream tasks. Unlike existing methods, OFT can provably preserve hyperspherical energy which characterizes the pairwise neuron relationship on the unit hypersphere. We find that this property is crucial for preserving the semantic generation ability of text-to-image diffusion models. To improve finetuning stability, we further propose Constrained Orthogonal Finetuning (COFT) which imposes an additional radius constraint to the hypersphere. Specifically, we consider two important finetuning text-to-image tasks: subject-driven generation where the goal is to generate subject-specific images given a few images of a subject and a text prompt, and controllable generation where the goal is to enable the model to take in additional control signals. We empirically show that our OFT framework outperforms existing methods in generation quality and convergence speed*.

OFT preserves the hyperspherical energy by learning an orthogonal transformation for neurons to keep the cosine similarity between them unchanged, potentially leading to less forgetting of previous learnt knowledge. In practice, this means taking the matrix product of an orthogonal matrix with the pretrained weight matrix. However, to be parameter-efficient, the orthogonal matrix is represented as a block-diagonal matrix with rank `r` blocks. Whereas LoRA reduces the number of trainable parameters with low-rank structures, OFT reduces the number of trainable parameters with a sparse block-diagonal matrix structure.

## Benchmark overview

<iframe
	src="https://peft-internal-testing-peft-method-comparison-embed.hf.space/?highlight[type]=OFT"
	frameborder="0"
	width="850"
	height="1000"
>

## Merge OFT weights into the base model

Similar to LoRA, the weights learned by OFT can be integrated into the pretrained weight matrices using the [`~OFTModel.merge_and_unload()` function. This function merges the adapter weights with the base model which allows you to effectively use the newly merged model as a standalone model.

## OFT Example Usage

For using OFT for quantized finetuning with [TRL](https://github.com/huggingface/trl) for `SFT`, `PPO`, or `DPO` fine-tuning, follow the following outline:

```py
from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig
from trl import SFTTrainer
from peft import OFTConfig

if use_quantization:
    bnb_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype=torch.bfloat16,
        bnb_4bit_use_double_quant=True,
        bnb_4bit_quant_storage=torch.bfloat16,
    )

model = AutoModelForCausalLM.from_pretrained(
    "model_name",
    quantization_config=bnb_config
)
tokenizer = AutoTokenizer.from_pretrained("model_name")

# Configure OFT
peft_config = OFTConfig(
    oft_block_size=32,
    use_cayley_neumann=True,
    target_modules="all-linear",
    bias="none",
    task_type="CAUSAL_LM"
)

trainer = SFTTrainer(
    model=model,
    train_dataset=ds['train'],
    peft_config=peft_config,
    processing_class=tokenizer,
    args=training_arguments,
    data_collator=collator,
)

trainer.train()
```

# API

## OFTConfig[[peft.OFTConfig]]

- **r** (`int`) --
  OFT rank, number of OFT blocks per injected layer. Bigger `r` results in more sparse update matrices with
  fewer trainable parameters. You can only specify either `r` or `oft_block_size`, but not both
  simultaneously, because `r` × `oft_block_size` = layer dimension. For simplicity, we let you specify either
  `r` or `oft_block_size` and infer the other one. Default set to `r = 0`, the user is advised to set the
  `oft_block_size` instead for better clarity.
- **oft_block_size** (`int`) -- OFT block size across different layers. Bigger `oft_block_size` results in more dense
  update matrices with more trainable parameters. Choose `oft_block_size` to be divisible by layer's input
  dimension (`in_features`), e.g., 4, 8, 16. You can only specify either `r` or `oft_block_size`, but not
  both simultaneously, because `r` × `oft_block_size` = layer dimension. For simplicity, we let you specify
  either `r` or `oft_block_size` and infer the other one. Default set to `oft_block_size = 32`.
- **use_cayley_neumann** (bool) -- Specifies whether to use the Cayley-Neumann parameterization (efficient but
  approximate) or the vanilla Cayley parameterization (exact but computationally expensive because of matrix
  inverse). We recommend to set it to `True` for better efficiency, but performance may be slightly worse
  because of the approximation error. Please test both settings (`True` and `False`) depending on your needs.
  Default is `False`.
- **module_dropout** (`float`) --
  The multiplicative dropout probability, by setting OFT blocks to identity during training, similar to the
  dropout layer in LoRA.
- **target_modules** (`Optional[Union[list[str], str]]`) --
  The names of the modules to apply the adapter to. If this is specified, only the modules with the specified
  names will be replaced. When passing a string, a regex match will be performed. When passing a list of
  strings, either an exact match will be performed or it is checked if the name of the module ends with any
  of the passed strings. If this is specified as 'all-linear', then all linear modules are chosen, excluding
  the output layer. If this is not specified, modules will be chosen according to the model architecture. If
  the architecture is not known, an error will be raised -- in this case, you should specify the target
  modules manually.
- **fan_in_fan_out** (`bool`) -- Set this to True if the layer to replace stores weight like (fan_in, fan_out).
- **bias** (`str`) -- Bias type for OFT. Can be 'none', 'all' or 'oft_only'. If 'all' or 'oft_only', the
  corresponding biases will be updated during training. Be aware that this means that, even when disabling
  the adapters, the model will not produce the same output as the base model would have without adaptation.
- **exclude_modules** (`Optional[Union[List[str], str]]`) --
  The names of the modules to not apply the adapter. When passing a string, a regex match will be performed.
  When passing a list of strings, either an exact match will be performed or it is checked if the name of the
  module ends with any of the passed strings.
- **init_weights** (`bool`) --
  Whether to perform initialization of OFT weights.
- **layers_to_transform** (`Union[List[int], int]`) --
  The layer indices to transform. If a list of ints is passed, it will apply the adapter to the layer indices
  that are specified in this list. If a single integer is passed, it will apply the transformations on the
  layer at this index.
- **layers_pattern** (`Optional[Union[List[str], str]]`) --
  The layer pattern name, used only if `layers_to_transform` is different from `None`. This should target the
  `nn.ModuleList` of the model, which is often called `'layers'` or `'h'`.
- **modules_to_save** (`List[str]`) --
  List of modules apart from adapter layers to be set as trainable and saved in the final checkpoint.
- **coft** (`bool`) --
  Whether to use the constrained variant of OFT or not, off by default.
- **eps** (`float`) --
  The control strength of COFT. The freedom of rotation. Only has an effect if `coft` is set to True.
- **block_share** (`bool`) --
  Whether to share the OFT parameters between blocks or not. This is `False` by default.

This is the configuration class to store the configuration of a [OFTModel](/docs/peft/v0.20.0/en/package_reference/oft#peft.OFTModel).

- **kwargs** (additional keyword arguments, *optional*) --
  Additional keyword arguments passed along to the child class initialization.

Check if the kwargs are valid for the configuration.

## OFTModel[[peft.OFTModel]]

- **model** (`torch.nn.Module`) -- The model to which the adapter tuner layers will be attached.
- **config** ([OFTConfig](/docs/peft/v0.20.0/en/package_reference/oft#peft.OFTConfig)) -- The configuration of the OFT model.
- **adapter_name** (`str`) -- The name of the adapter, defaults to `"default"`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.`torch.nn.Module`The OFT model.

Creates Orthogonal Finetuning model from a pretrained model. The method is described in
https://huggingface.co/papers/2306.07280

Example:
```py
>>> from diffusers import StableDiffusionPipeline
>>> from peft import OFTModel, OFTConfig

>>> config_te = OFTConfig(
...     r=8,
...     target_modules=["k_proj", "q_proj", "v_proj", "out_proj", "fc1", "fc2"],
...     module_dropout=0.0,
...     init_weights=True,
... )
>>> config_unet = OFTConfig(
...     r=8,
...     target_modules=[
...         "proj_in",
...         "proj_out",
...         "to_k",
...         "to_q",
...         "to_v",
...         "to_out.0",
...         "ff.net.0.proj",
...         "ff.net.2",
...     ],
...     module_dropout=0.0,
...     init_weights=True,
... )

>>> model = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5")
>>> model.text_encoder = OFTModel(model.text_encoder, config_te, "default")
>>> model.unet = OFTModel(model.unet, config_unet, "default")
```

**Attributes**:
- **model** (`~torch.nn.Module`) -- The model to be adapted.
- **peft_config** ([OFTConfig](/docs/peft/v0.20.0/en/package_reference/oft#peft.OFTConfig)): The configuration of the OFT model.
