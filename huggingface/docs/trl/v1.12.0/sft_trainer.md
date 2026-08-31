# SFT Trainer

[![All_models-SFT-blue](https://img.shields.io/badge/All_models-SFT-blue)](https://huggingface.co/models?other=sft,trl) [![smol_course-Chapter_1-yellow](https://img.shields.io/badge/smol_course-Chapter_1-yellow)](https://github.com/huggingface/smol-course/tree/main/1_instruction_tuning)

## Overview

TRL supports the Supervised Fine-Tuning (SFT) Trainer for training language models.

[SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) doesn't natively support diffusion models like DiffusionGemma, but it can be easily extended to do so, see the [block-diffusion SFT example](https://github.com/huggingface/trl/blob/main/examples/sft_diffusion_gemma/sft_diffusion_gemma.py).

This post-training method was contributed by [Younes Belkada](https://huggingface.co/ybelkada).

## Quick start

This example demonstrates how to train a language model using the [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) from TRL. We train a [Qwen 3 0.6B](https://huggingface.co/Qwen/Qwen3-0.6B) model on the [Capybara dataset](https://huggingface.co/datasets/trl-lib/Capybara), a compact, diverse multi-turn dataset to benchmark reasoning and generalization.

```python
from trl import SFTTrainer
from datasets import load_dataset

trainer = SFTTrainer(
    model="Qwen/Qwen3-0.6B",
    train_dataset=load_dataset("trl-lib/Capybara", split="train"),
)
trainer.train()
```

## Expected dataset type and format

SFT supports both [language modeling](dataset_formats#language-modeling) and [prompt-completion](dataset_formats#prompt-completion) datasets. The [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) is compatible with both [standard](dataset_formats#standard) and [conversational](dataset_formats#conversational) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

```python
# Standard language modeling
{"text": "The sky is blue."}

# Conversational language modeling
{"messages": [{"role": "user", "content": "What color is the sky?"},
              {"role": "assistant", "content": "It is blue."}]}

# Standard prompt-completion
{"prompt": "The sky is",
 "completion": " blue."}

# Conversational prompt-completion
{"prompt": [{"role": "user", "content": "What color is the sky?"}],
 "completion": [{"role": "assistant", "content": "It is blue."}]}
```

If your dataset is not in one of these formats, you can preprocess it to convert it into the expected format. Here is an example with the [FreedomIntelligence/medical-o1-reasoning-SFT](https://huggingface.co/datasets/FreedomIntelligence/medical-o1-reasoning-SFT) dataset:

```python
from datasets import load_dataset

dataset = load_dataset("FreedomIntelligence/medical-o1-reasoning-SFT", "en")

def preprocess_function(example):
    return {
        "prompt": [{"role": "user", "content": example["Question"]}],
        "completion": [
            {"role": "assistant", "content": f"<think>{example['Complex_CoT']}</think>{example['Response']}"}
        ],
    }

dataset = dataset.map(preprocess_function, remove_columns=["Question", "Response", "Complex_CoT"])
print(next(iter(dataset["train"])))
```

```json
{
    "prompt": [
        {
            "content": "Given the symptoms of sudden weakness in the left arm and leg, recent long-distance travel, and the presence of swollen and tender right lower leg, what specific cardiac abnormality is most likely to be found upon further evaluation that could explain these findings?",
            "role": "user",
        }
    ],
    "completion": [
        {
            "content": "<think>Okay, let's see what's going on here. We've got sudden weakness [...] clicks into place!</think>The specific cardiac abnormality most likely to be found in [...] the presence of a PFO facilitating a paradoxical embolism.",
            "role": "assistant",
        }
    ],
}
```

## Looking deeper into the SFT method

Supervised Fine-Tuning (SFT) is the simplest and most commonly used method to adapt a language model to a target dataset. The model is trained in a fully supervised fashion using pairs of input and output sequences. The goal is to minimize the negative log-likelihood (NLL) of the target sequence, conditioning on the input.

This section breaks down how SFT works in practice, covering the key steps: **preprocessing**, **tokenization** and **loss computation**.

### Preprocessing and tokenization

During training, each example is expected to contain a **text field** or a **(prompt, completion)** pair, depending on the dataset format. For more details on the expected formats, see [Dataset formats](dataset_formats).
The [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) tokenizes each input using the model's tokenizer. If both prompt and completion are provided separately, they are concatenated before tokenization.

### Computing the loss

![sft_figure](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/sft_figure.png)

The loss used in SFT is the **token-level cross-entropy loss**, defined as:

$$
\mathcal{L}_{\text{SFT}}(\theta) = - \sum_{t=1}^{T} \log p_\theta(y_t \mid y_{<t}),
$$
  
where  \\( y_t \\) is the target token at timestep  \\( t \\), and the model is trained to predict the next token given the previous ones. In practice, padding tokens are masked out during loss computation.

> [!TIP]
> The paper [On the Generalization of SFT: A Reinforcement Learning Perspective with Reward Rectification](https://huggingface.co/papers/2508.05629) proposes an alternative loss function, called **Dynamic Fine-Tuning (DFT)**, which aims to improve generalization by rectifying the reward signal. This method can be enabled by setting `loss_type="dft"` in the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig). For more details, see [Paper Index - Dynamic Fine-Tuning](paper_index#on-the-generalization-of-sft-a-reinforcement-learning-perspective-with-reward-rectification).

> [!TIP]
> By default, [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) uses `loss_type="chunked_nll"`: same math as `"nll"`, but the `lm_head` projection skips ignored-label tokens and the cross-entropy is processed in chunks, so peak activation memory does not scale with the full vocab × seq_len logits tensor. To fall back to the standard path, set `loss_type="nll"`. When `use_liger_kernel=True`, the default automatically resolves to `"nll"` (the two paths are not compatible). See [Chunked cross-entropy for reducing peak memory usage](reducing_memory_usage#chunked-cross-entropy-for-reducing-peak-memory-usage).

### Label shifting and masking

During training, the loss is computed using a **one-token shift**: the model is trained to predict each token in the sequence based on all previous tokens. Specifically, the input sequence is shifted right by one position to form the target labels.
Padding tokens (if present) are ignored in the loss computation by applying an ignore index (default: `-100`) to the corresponding positions. This ensures that the loss focuses only on meaningful, non-padding tokens.

## Logged metrics

While training and evaluating, we record the following metrics:

* `global_step`: The total number of optimizer steps taken so far.
* `epoch`: The current epoch number, based on dataset iteration.
* `num_tokens`: The total number of tokens processed so far.
* `loss`: The average cross-entropy loss computed over non-masked tokens in the current logging interval.
* `entropy`: The average entropy of the model's predicted token distribution over non-masked tokens.
* `mean_token_accuracy`: The proportion of non-masked tokens for which the model’s top-1 prediction matches the ground truth token.
* `learning_rate`: The current learning rate, which may change dynamically if a scheduler is used.
* `grad_norm`: The L2 norm of the gradients, computed before gradient clipping.

## Customization

### Model initialization

You can directly pass the kwargs of the `from_pretrained()` method to the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig). For example, if you want to load a model in a different precision, analogous to

```python
model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen3-0.6B", dtype=torch.bfloat16)
```

you can do so by passing the `model_init_kwargs={"dtype": torch.bfloat16}` argument to the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig).

```python
from trl import SFTConfig

training_args = SFTConfig(
    model_init_kwargs={"dtype": torch.bfloat16},
)
```

Note that all keyword arguments of `from_pretrained()` are supported.

### Packing

[SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) supports _example packing_, where multiple examples are packed in the same input sequence to increase training efficiency. To enable packing, simply pass `packing=True` to the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig) constructor.

```python
training_args = SFTConfig(packing=True)
```

For more details on packing, see [Packing](reducing_memory_usage#packing).

### Train on assistant messages only

To train on assistant messages only, use a [conversational](dataset_formats#conversational) dataset and set `assistant_only_loss=True` in the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig). This setting ensures that loss is computed **only** on the assistant responses, ignoring user or system messages.

```python
training_args = SFTConfig(assistant_only_loss=True)
```

![train_on_assistant](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/train_on_assistant.png)

> [!WARNING]
> This functionality requires the chat template to include `&#123;% generation %&#125;` and `&#123;% endgeneration %&#125;` keywords. For known model families (e.g. Qwen3), TRL automatically patches the template when `assistant_only_loss=True`. See [Chat Templates](chat_templates#training-templates) for the full list of bundled training templates. For other models, check that your chat template includes these keywords. See [HuggingFaceTB/SmolLM3-3B](https://huggingface.co/HuggingFaceTB/SmolLM3-3B/blob/main/chat_template.jinja#L76-L82) for an example.

### Train on completion only

To train on completion only, use a [prompt-completion](dataset_formats#prompt-completion) dataset. By default, the trainer computes the loss on the completion tokens only, ignoring the prompt tokens. If you want to train on the full sequence, set `completion_only_loss=False` in the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig).

```python
from trl import SFTConfig, SFTTrainer
from datasets import load_dataset

# Load a prompt-completion dataset; loss is computed on the completion only by default
dataset = load_dataset("trl-lib/kto-mix-14k", split="train")

trainer = SFTTrainer(
    model="Qwen/Qwen2.5-0.5B-Instruct",
    args=SFTConfig(completion_only_loss=True),  # True by default for prompt-completion datasets
    train_dataset=dataset,
)
trainer.train()
```

![train_on_completion](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/train_on_completion.png)

> [!TIP]
> Training on completion only is compatible with training on assistant messages only. In this case, use a [conversational](dataset_formats#conversational) [prompt-completion](dataset_formats#prompt-completion) dataset and set `assistant_only_loss=True` in the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig).

### Train adapters with PEFT

We support tight integration with 🤗 PEFT library, allowing any user to conveniently train adapters and share them on the Hub, rather than training the entire model.

```python
from datasets import load_dataset
from trl import SFTTrainer
from peft import LoraConfig

dataset = load_dataset("trl-lib/Capybara", split="train")

trainer = SFTTrainer(
    "Qwen/Qwen3-0.6B",
    train_dataset=dataset,
    peft_config=LoraConfig(),
)

trainer.train()
```

You can also continue training your [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel). For that, first load a `PeftModel` outside [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) and pass it directly to the trainer without the `peft_config` argument being passed.

```python
from datasets import load_dataset
from trl import SFTTrainer
from peft import AutoPeftModelForCausalLM

model = AutoPeftModelForCausalLM.from_pretrained("trl-lib/Qwen3-4B-LoRA", is_trainable=True)
dataset = load_dataset("trl-lib/Capybara", split="train")

trainer = SFTTrainer(
    model=model,
    train_dataset=dataset,
)

trainer.train()
```

> [!TIP]
> When training adapters, you typically use a higher learning rate (≈1e‑4) since only new parameters are being learned.
>
> ```python
> SFTConfig(learning_rate=1e-4, ...)
> ```

### Train with Liger Kernel

Liger Kernel is a collection of Triton kernels for LLM training that boosts multi-GPU throughput by 20%, cuts memory use by 60% (enabling up to 4× longer context), and works seamlessly with tools like FlashAttention, PyTorch FSDP, and DeepSpeed. For more information, see [Liger Kernel Integration](liger_kernel_integration).

### Rapid Experimentation for SFT

RapidFire AI is an open-source experimentation engine that sits on top of TRL and lets you launch multiple SFT configurations at once, even on a single GPU. Instead of trying configurations sequentially, RapidFire lets you **see all their learning curves earlier, stop underperforming runs, and clone promising ones with new settings in flight** without restarting. For more information, see [RapidFire AI Integration](rapidfire_integration).

### Train with Unsloth

Unsloth is an open‑source framework for fine‑tuning and reinforcement learning that trains LLMs (like Llama, Mistral, Gemma, DeepSeek, and more) up to 2× faster with up to 70% less VRAM, while providing a streamlined, Hugging Face–compatible workflow for training, evaluation, and deployment. For more information, see [Unsloth Integration](unsloth_integration).

## Instruction tuning example

**Instruction tuning** teaches a base language model to follow user instructions and engage in conversations. This requires:

1. **Chat template**: Defines how to structure conversations into text sequences, including role markers (user/assistant), special tokens, and turn boundaries. Read more about chat templates in [Chat templates](https://huggingface.co/docs/transformers/chat_templating#templates).
2. **Conversational dataset**: Contains instruction-response pairs

This example shows how to transform the [Qwen 3 0.6B Base](https://huggingface.co/Qwen/Qwen3-0.6B-Base) model into an instruction-following model using the [Capybara dataset](https://huggingface.co/datasets/trl-lib/Capybara) and a chat template from [HuggingFaceTB/SmolLM3-3B](https://huggingface.co/HuggingFaceTB/SmolLM3-3B). The SFT Trainer automatically handles tokenizer updates and special token configuration.

```python
from trl import SFTConfig, SFTTrainer
from datasets import load_dataset

trainer = SFTTrainer(
    model="Qwen/Qwen3-0.6B-Base",
    args=SFTConfig(
        output_dir="Qwen3-0.6B-Instruct",
        chat_template_path="HuggingFaceTB/SmolLM3-3B",
    ),
    train_dataset=load_dataset("trl-lib/Capybara", split="train"),
)
trainer.train()
```

> [!WARNING]
> Some base models, like those from Qwen, have a predefined chat template in the model's tokenizer. In these cases, it is not necessary to apply `clone_chat_template()`, as the tokenizer already handles the formatting. However, it is necessary to align the EOS token with the chat template to ensure the model's responses terminate correctly. In these cases, specify `eos_token` in [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig); for example, for `Qwen/Qwen2.5-1.5B`, one should set `eos_token="<|im_end|>"`.

Once trained, your model can now follow instructions and engage in conversations using its new chat template.

```python
>>> from transformers import pipeline
>>> pipe = pipeline("text-generation", model="Qwen3-0.6B-Instruct/checkpoint-5000")
>>> prompt = "<|im_start|>user\nWhat is the capital of France? Answer in one word.<|im_end|>\n<|im_start|>assistant\n"
>>> response = pipe(prompt)
>>> response[0]["generated_text"]
'<|im_start|>user\nWhat is the capital of France? Answer in one word.<|im_end|>\n<|im_start|>assistant\nThe capital of France is Paris.'
```

Alternatively, use the structured conversation format (recommended):

```python
>>> prompt = [{"role": "user", "content": "What is the capital of France? Answer in one word."}]
>>> response = pipe(prompt)
>>> response[0]["generated_text"]
[{'role': 'user', 'content': 'What is the capital of France? Answer in one word.'}, {'role': 'assistant', 'content': 'The capital of France is Paris.'}]
```

## Tool Calling with SFT

The [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) fully supports fine-tuning models with _tool calling_ capabilities. In this case, each dataset example should include:

* The conversation messages, including any tool calls (`tool_calls`) and tool responses (`tool` role messages)
* The list of available tools in the `tools` column, typically provided as JSON schemas

For details on the expected dataset structure, see the [Dataset Format — Tool Calling](dataset_formats#tool-calling) section.

## Training Vision Language Models

[SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) fully supports training Vision-Language Models (VLMs). To train a VLM, provide a dataset with either an `image` column (single image per sample) or an `images` column (list of images per sample). For more information on the expected dataset structure, see the [Dataset Format — Vision Dataset](dataset_formats#vision-dataset) section.
An example of such a dataset is the [LLaVA Instruct Mix](https://huggingface.co/datasets/trl-lib/llava-instruct-mix).

```python
from trl import SFTConfig, SFTTrainer
from datasets import load_dataset

trainer = SFTTrainer(
    model="Qwen/Qwen2.5-VL-3B-Instruct",
    args=SFTConfig(max_length=None),
    train_dataset=load_dataset("trl-lib/llava-instruct-mix", split="train"),
)
trainer.train()
```

> [!TIP]
> For VLMs, truncating may remove image tokens, leading to errors during training. To avoid this, set `max_length=None` in the [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig). This allows the model to process the full sequence length without truncating image tokens.
>
> ```python
> SFTConfig(max_length=None, ...)
> ```
>
> Only use `max_length` when you've verified that truncation won't remove image tokens for the entire dataset.

## SFTTrainer[[trl.SFTTrainer]]

#### trl.SFTTrainer[[trl.SFTTrainer]]

```python
trl.SFTTrainer(model: str | PreTrainedModel | PeftModel, args: trl.trainer.sft_config.SFTConfig | transformers.training_args.TrainingArguments | None = None, data_collator: collections.abc.Callable[[list[typing.Any]], dict[str, typing.Any]] | None = None, train_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | None = None, eval_dataset: datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset | datasets.dataset_dict.DatasetDict | datasets.dataset_dict.IterableDatasetDict | dict[str, datasets.arrow_dataset.Dataset | datasets.iterable_dataset.IterableDataset] | None = None, processing_class: transformers.tokenization_utils_base.PreTrainedTokenizerBase | transformers.processing_utils.ProcessorMixin | None = None, compute_loss_func: collections.abc.Callable | None = None, compute_metrics: collections.abc.Callable[[transformers.trainer_utils.EvalPrediction], dict] | None = None, callbacks: list[transformers.trainer_callback.TrainerCallback] | None = None, optimizers: tuple = (None, None), optimizer_cls_and_kwargs: tuple[type[torch.optim.Optimizer], dict[str, typing.Any]] | None = None, preprocess_logits_for_metrics: collections.abc.Callable[[torch.Tensor, torch.Tensor], torch.Tensor] | None = None, quantization_config: BitsAndBytesConfig | None = None, peft_config: PeftConfig | None = None, formatting_func: collections.abc.Callable[[dict], str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/sft_trainer.py#L805)

**Parameters:**

model (`str` or [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) or [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) : Model to be trained. Can be either:  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a path to a *directory* containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded using `<ModelArchitecture>.from_pretrained` (where `<ModelArchitecture>` is derived from the model config) with the keyword arguments in `args.model_init_kwargs`. If `dtype` is not specified in `args.model_init_kwargs`, it defaults to `float32`. This differs from [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained), where (since Transformers v5) the dtype is inferred from the model config. - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported. - A [PeftModel](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel) object. Only causal language models are supported.

args ([SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig), *optional*) : Configuration for this trainer. If `None`, a default configuration is used.

data_collator (`DataCollator`, *optional*) : Function to use to form a batch from a list of elements of the processed `train_dataset` or `eval_dataset`. Will default to `DataCollatorForLanguageModeling`, or to `DataCollatorForVisionLanguageModeling` if the dataset contains images.

train_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset)) : Dataset to use for training. This trainer supports both [language modeling](#language-modeling) type and [prompt-completion](#prompt-completion) type. The format of the samples can be either:  - [Standard](dataset_formats#standard): Each sample contains plain text. - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role and content).  The trainer also supports pre-tokenized datasets, recognized by a required `input_ids` column. An optional `labels` column (`-100` on tokens excluded from the loss) is used as is if present; otherwise labels are built from the optional `assistant_masks` / `completion_mask` columns (which are folded in then dropped, `completion_mask` only when `completion_only_loss=True`), or default to a copy of `input_ids`. Sequences are truncated to `max_length` during preparation. With `skip_prepare_dataset=True`, preparation is skipped and the collator is expected to handle the dataset as is.  When `train_dataset` is an [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset) (e.g. a streaming dataset), `max_steps` must be set in the training arguments, since its length cannot be inferred and the total number of training steps is required to bound the training loop and configure the learning rate scheduler.

eval_dataset ([Dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.Dataset), [IterableDataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDataset), [DatasetDict](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.DatasetDict), [IterableDatasetDict](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.IterableDatasetDict) or `dict[str, Dataset | IterableDataset]`) : Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.

processing_class ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.16.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [ProcessorMixin](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) : Processing class used to process the data. If `None`, the processing class is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained). A padding token, `tokenizer.pad_token`, must be set. If the processing class has not set a padding token, `tokenizer.eos_token` will be used as the default.

compute_loss_func (`Callable`, *optional*) : A function that accepts the raw model outputs, labels, and the number of items in the entire accumulated batch (batch_size * gradient_accumulation_steps) and returns the loss. For example, see the default [loss function](https://github.com/huggingface/transformers/blob/052e652d6d53c2b26ffde87e039b723949a53493/src/transformers/trainer.py#L3618) used by `Trainer`.

compute_metrics (`Callable[[EvalPrediction], dict]`, *optional*) : The function that will be used to compute metrics at evaluation. Must take a [EvalPrediction](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.EvalPrediction) and return a dictionary string to metric values. When passing [SFTConfig](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTConfig) with `batch_eval_metrics` set to `True`, your `compute_metrics` function must take a boolean `compute_result` argument. This will be triggered after the last eval batch to signal that the function needs to calculate and return the global summary statistics rather than accumulating the batch-level statistics.

callbacks (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) : List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed in [here](https://huggingface.co/docs/transformers/main_classes/callback).  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer.remove_callback) method.

optimizers (`tuple[torch.optim.Optimizer | None, torch.optim.lr_scheduler.LambdaLR | None]`, *optional*, defaults to `(None, None)`) : A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your model and a scheduler given by [get_linear_schedule_with_warmup](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup) controlled by `args`.

optimizer_cls_and_kwargs (`tuple[Type[torch.optim.Optimizer], Dict[str, Any]]`, *optional*) : A tuple containing the optimizer class and keyword arguments to use. Overrides `optim` and `optim_args` in `args`. Incompatible with the `optimizers` argument.  Unlike `optimizers`, this argument avoids the need to place model parameters on the correct devices before initializing the Trainer.

preprocess_logits_for_metrics (`Callable[[torch.Tensor, torch.Tensor], torch.Tensor]`, *optional*) : A function that preprocess the logits right before caching them at each evaluation step. Must take two tensors, the logits and the labels, and return the logits once processed as desired. The modifications made by this function will be reflected in the predictions received by `compute_metrics`.  Note that the labels (second parameter) will be `None` if the dataset does not have them.

quantization_config ([BitsAndBytesConfig](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/quantization#transformers.BitsAndBytesConfig), *optional*) : Quantization configuration used when loading the model from a model identifier. Combine with `peft_config` for QLoRA training. Ignored if the model is already instantiated.

peft_config ([PeftConfig](https://huggingface.co/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig), *optional*) : PEFT configuration used to wrap the model. If `None`, the model is not wrapped.

formatting_func (`Callable`, *optional*) : Formatting function applied to the dataset before tokenization. Applying the formatting function explicitly converts the dataset into a [language modeling](#language-modeling) type.

Trainer for Supervised Fine-Tuning (SFT) method.

This class is a wrapper around the [Trainer](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.Trainer) class and inherits all of its attributes and methods.

Example:

```python
>>> from trl import SFTTrainer
>>> from datasets import load_dataset

>>> dataset = load_dataset("roneneldan/TinyStories", split="train[:1%]")

>>> trainer = SFTTrainer(
...     model="Qwen/Qwen2.5-0.5B-Instruct",
...     train_dataset=dataset,
... )
>>> trainer.train()
```

#### train[[trl.SFTTrainer.train]]

```python
train(resume_from_checkpoint: str | bool | None = None, trial: optuna.Trial | dict[str, Any] | None = None, ignore_keys_for_eval: list[str] | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L1350)

**Parameters:**

resume_from_checkpoint (`str` or `bool`, *optional*) : If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.

trial (`optuna.Trial` or `dict[str, Any]`, *optional*) : The trial run or the hyperparameter dictionary for hyperparameter search.

ignore_keys_for_eval (`list[str]`, *optional*) : A list of keys in the output of your model (if it is a dictionary) that should be ignored when gathering predictions for evaluation during the training.

**Returns:** `~trainer_utils.TrainOutput`

Object containing the global step count, training loss, and metrics.

Main training entry point.

#### save_model[[trl.SFTTrainer.save_model]]

```python
save_model(output_dir: str | None = None, _internal_call: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L3805)

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

#### push_to_hub[[trl.SFTTrainer.push_to_hub]]

```python
push_to_hub(commit_message: str | None = 'End of training', blocking: bool = True, token: str | None = None, revision: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/transformers/trainer.py#L4052)

**Parameters:**

commit_message (`str`, *optional*, defaults to `"End of training"`) : Message to commit while pushing.

blocking (`bool`, *optional*, defaults to `True`) : Whether the function should return only when the `git push` has finished.

token (`str`, *optional*, defaults to `None`) : Token with write permission to overwrite Trainer's original args.

revision (`str`, *optional*) : The git revision to commit from. Defaults to the head of the "main" branch.

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to `~Trainer.create_model_card`.

**Returns:**

The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.

## SFTConfig[[trl.SFTConfig]]

#### trl.SFTConfig[[trl.SFTConfig]]

```python
trl.SFTConfig(output_dir: str | None = None, per_device_train_batch_size: int = 8, num_train_epochs: float = 3.0, max_steps: int = -1, learning_rate: float = 2e-05, lr_scheduler_type: transformers.trainer_utils.SchedulerType | str = 'linear', lr_scheduler_kwargs: dict | str | None = None, warmup_steps: float = 0, optim: transformers.training_args.OptimizerNames | str = 'adamw_torch_fused', optim_args: str | None = None, weight_decay: float = 0.0, adam_beta1: float = 0.9, adam_beta2: float = 0.999, adam_epsilon: float = 1e-08, optim_target_modules: None | str | list[str] = None, gradient_accumulation_steps: int = 1, average_tokens_across_devices: bool = True, max_grad_norm: float = 1.0, label_smoothing_factor: float = 0.0, bf16: bool | None = None, fp16: bool = False, bf16_full_eval: bool = False, fp16_full_eval: bool = False, tf32: bool | None = None, gradient_checkpointing: bool = True, gradient_checkpointing_kwargs: dict[str, typing.Any] | str | None = None, torch_compile: bool = False, torch_compile_backend: str | None = None, torch_compile_mode: str | None = None, use_liger_kernel: bool = False, liger_kernel_config: dict[str, bool] | None = None, use_cache: bool = False, neftune_noise_alpha: float | None = None, torch_empty_cache_steps: int | None = None, auto_find_batch_size: bool = False, logging_strategy: transformers.trainer_utils.IntervalStrategy | str = 'steps', logging_steps: float = 10, logging_first_step: bool = False, log_on_each_node: bool = True, logging_nan_inf_filter: bool = True, include_num_input_tokens_seen: str | bool = 'no', log_level: str = 'passive', log_level_replica: str = 'warning', disable_tqdm: bool | None = None, report_to: None | str | list[str] = 'none', run_name: str | None = None, project: str = 'huggingface', trackio_space_id: str | None = None, trackio_bucket_id: str | None = None, trackio_static_space_id: typing.Union[str, NoneType, typing.Literal[False]] = None, eval_strategy: transformers.trainer_utils.IntervalStrategy | str = 'no', eval_steps: float | None = None, eval_delay: float = 0, per_device_eval_batch_size: int = 8, prediction_loss_only: bool = False, eval_on_start: bool = False, eval_do_concat_batches: bool = True, eval_use_gather_object: bool = False, eval_accumulation_steps: int | None = None, include_for_metrics: list = <factory>, batch_eval_metrics: bool = False, save_only_model: bool = False, save_strategy: transformers.trainer_utils.SaveStrategy | str = 'steps', save_steps: float = 500, save_on_each_node: bool = False, save_total_limit: int | None = None, enable_jit_checkpoint: bool = False, push_to_hub: bool = False, hub_token: str | None = None, hub_private_repo: bool | None = None, hub_model_id: str | None = None, hub_strategy: transformers.trainer_utils.HubStrategy | str = 'every_save', hub_always_push: bool = False, hub_revision: str | None = None, load_best_model_at_end: bool = False, metric_for_best_model: str | None = None, greater_is_better: bool | None = None, ignore_data_skip: bool = False, restore_callback_states_from_checkpoint: bool = False, full_determinism: bool = False, seed: int = 42, data_seed: int | None = None, use_cpu: bool = False, accelerator_config: dict | str | None = None, parallelism_config: accelerate.parallelism_config.ParallelismConfig | None = None, dataloader_drop_last: bool = False, dataloader_num_workers: int = 0, dataloader_pin_memory: bool = True, dataloader_persistent_workers: bool = False, dataloader_prefetch_factor: int | None = None, dataloader_multiprocessing_context: str | None = None, dataloader_in_order: bool = True, remove_unused_columns: bool = True, label_names: list[str] | None = None, train_sampling_strategy: str = 'random', length_column_name: str = 'length', ddp_find_unused_parameters: bool | None = None, ddp_bucket_cap_mb: int | None = None, ddp_broadcast_buffers: bool | None = None, ddp_static_graph: bool | None = None, ddp_backend: str | None = None, ddp_timeout: int = 1800, fsdp: str | None = None, fsdp_config: dict[str, typing.Any] | str | None = None, deepspeed: dict | str | None = None, debug: str | list[transformers.debug_utils.DebugOption] = '', skip_memory_metrics: bool = True, do_train: bool = False, do_eval: bool = False, do_predict: bool = False, resume_from_checkpoint: str | None = None, local_rank: int = -1, model_init_kwargs: dict[str, typing.Any] | str | None = None, router_aux_loss_coef: float = 0.001, trust_remote_code: bool = False, chat_template_path: str | None = None, dataset_text_field: str = 'text', dataset_kwargs: dict[str, typing.Any] | str | None = None, dataset_num_proc: int | None = None, eos_token: str | None = None, max_length: int | None = 1024, truncation_mode: str = 'keep_start', shuffle_dataset: bool = False, packing: bool = False, packing_strategy: str = 'bfd', padding_free: bool = False, pad_to_multiple_of: int | None = None, eval_packing: bool | None = None, completion_only_loss: bool | None = None, assistant_only_loss: bool = False, loss_type: str | None = None, activation_offloading: bool = False, pad_token: str | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/trainer/sft_config.py#L23)

**Parameters that control the model:**

model_init_kwargs (`dict[str, Any]`, *optional*) : Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model` argument of the [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer) is provided as a string.

trust_remote_code (`bool`, *optional*, defaults to `False`) : Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and [from_pretrained](https://huggingface.co/docs/transformers/v5.16.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).

router_aux_loss_coef (`float`, *optional*, defaults to `0.001`) : Coefficient of the load-balancing auxiliary loss. Only has an effect when training a Mixture-of-Experts (MoE) model; for other models it does nothing. The auxiliary loss is added to the training loss with this weight. Set to `0.0` to disable it.

chat_template_path (`str`, *optional*) : If specified, sets the model's chat template. This can either be the path to a tokenizer (local directory or Hugging Face Hub model) or a direct path to a Jinja template file. When using a Jinja file, you must ensure that any special tokens referenced in the template are added to the tokenizer and that the model's embedding layer is resized accordingly.

**Parameters that control the data preprocessing:**

dataset_text_field (`str`, *optional*, defaults to `"text"`) : Name of the column that contains text data in the dataset.

dataset_kwargs (`dict[str, Any]`, *optional*) : Dictionary of optional keyword arguments for the dataset preparation. The only supported key is `skip_prepare_dataset`. When the dataset contains images, `skip_prepare_dataset` is automatically treated as `True` regardless of the provided value, since preprocessing is done on the fly.

dataset_num_proc (`int`, *optional*) : Number of processes to use for processing the dataset.

eos_token (`str`, *optional*) : Token used to indicate the end of a turn or sequence. If `None`, it defaults to `processing_class.eos_token`.

max_length (`int` or `None`, *optional*, defaults to `1024`) : Maximum length of the tokenized sequence. Sequences longer than `max_length` are truncated from the left or right depending on `truncation_mode`. If `None`, no truncation is applied. When packing is enabled, this value sets the sequence length.

truncation_mode (`str`, *optional*, defaults to `"keep_start"`) : Truncation mode to use when the sequence exceeds `max_length`. The only supported value is `"keep_start"`. The `"keep_end"` value is deprecated and will be removed in v2.0.0.

shuffle_dataset (`bool`, *optional*, defaults to `False`) : Whether to shuffle the dataset.

packing (`bool`, *optional*, defaults to `False`) : Whether to group multiple sequences into fixed-length blocks to improve computational efficiency and reduce padding. Uses `max_length` to define sequence length.

packing_strategy (`str`, *optional*, defaults to `"bfd"`) : Strategy for packing sequences. Can be `"bfd"` (best-fit decreasing, truncates overflow), `"bfd_split"` (best-fit decreasing, splits overflow sequences), or `"wrapped"` (aggressive, cuts mid-sequence).

padding_free (`bool`, *optional*, defaults to `False`) : Whether to perform forward passes without padding by flattening all sequences in the batch into a single continuous sequence. This reduces memory usage by eliminating padding overhead. Currently, this is only supported with the FlashAttention 2 or 3, which can efficiently handle the flattened batch structure. When packing is enabled with strategy `"bfd"`, padding-free is enabled, regardless of the value of this parameter.

pad_to_multiple_of (`int`, *optional*) : If set, the sequences will be padded to a multiple of this value.

eval_packing (`bool`, *optional*) : Whether to pack the eval dataset. If `None`, uses the same value as `packing`.

**Parameters that control the training:**

completion_only_loss (`bool`, *optional*) : Whether to compute loss only on the completion part of the sequence. If set to `True`, loss is computed only on the completion, which is supported only for [prompt-completion](#prompt-completion) datasets. If `False`, loss is computed on the entire sequence. If `None` (default), the behavior depends on the dataset: loss is computed on the completion for [prompt-completion](#prompt-completion) datasets, and on the full sequence for [language modeling](#language-modeling) datasets.

assistant_only_loss (`bool`, *optional*, defaults to `False`) : Whether to compute loss only on the assistant part of the sequence. If set to `True`, loss is computed only on the assistant responses, which is supported only for [conversational](#conversational) datasets. If `False`, loss is computed on the entire sequence.

loss_type (`str`, *optional*, defaults to `"chunked_nll"`) : Type of loss to use. When left unset, it defaults to `"chunked_nll"`, except when `use_liger_kernel=True`, in which case it defaults to `"nll"`. Possible values are:  - `"nll"`: standard negative log-likelihood. - `"dft"`: Dynamic Fine-Tuning, as described in [this paper](https://huggingface.co/papers/2508.05629). - `"chunked_nll"`: same math as `"nll"`, but the `lm_head` projection is computed on non-ignored tokens only (positions with `labels == -100` are dropped before the matmul) and the cross-entropy is processed in chunks of tokens to reduce peak activation memory. Not compatible with `use_liger_kernel`. 

activation_offloading (`bool`, *optional*, defaults to `False`) : Whether to offload the activations to the CPU.

**Deprecated parameters:**

pad_token :   Parameter `pad_token` is deprecated and will be removed in version v2.0.0. Set `tokenizer.pad_token` directly and pass it as `processing_class` to the trainer instead.  

Configuration class for the [SFTTrainer](/docs/trl/v1.12.0/en/sft_trainer#trl.SFTTrainer).

This class includes only the parameters that are specific to SFT training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.16.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `2e-5` instead of `5e-5`.
