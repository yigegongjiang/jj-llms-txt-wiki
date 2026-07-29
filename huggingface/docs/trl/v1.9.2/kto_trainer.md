# KTO Trainer

[![All_models-KTO-blue](https://img.shields.io/badge/All_models-KTO-blue)](https://huggingface.co/models?other=kto,trl)

## Overview

TRL supports the Kahneman-Tversky Optimization (KTO) Trainer for training language models, as described in the paper [KTO: Model Alignment as Prospect Theoretic Optimization](https://huggingface.co/papers/2402.01306) by [Kawin Ethayarajh](https://huggingface.co/kawine), [Winnie Xu](https://huggingface.co/xwinxu), [Niklas Muennighoff](https://huggingface.co/Muennighoff), Dan Jurafsky, [Douwe Kiela](https://huggingface.co/douwekiela).

The abstract from the paper is the following:

> Kahneman & Tversky's prospect theory tells us that humans perceive random variables in a biased but well-defined manner; for example, humans are famously loss-averse. We show that objectives for aligning LLMs with human feedback implicitly incorporate many of these biases -- the success of these objectives (e.g., DPO) over cross-entropy minimization can partly be ascribed to them being human-aware loss functions (HALOs). However, the utility functions these methods attribute to humans still differ from those in the prospect theory literature. Using a Kahneman-Tversky model of human utility, we propose a HALO that directly maximizes the utility of generations instead of maximizing the log-likelihood of preferences, as current methods do. We call this approach Kahneman-Tversky Optimization (KTO), and it matches or exceeds the performance of preference-based methods at scales from 1B to 30B. Crucially, KTO does not need preferences -- only a binary signal of whether an output is desirable or undesirable for a given input. This makes it far easier to use in the real world, where preference data is scarce and expensive.

The official code can be found in [ContextualAI/HALOs](https://github.com/ContextualAI/HALOs).

This post-training method was contributed by [Kashif Rasul](https://huggingface.co/kashif), [Younes Belkada](https://huggingface.co/ybelkada), [Lewis Tunstall](https://huggingface.co/lewtun), Pablo Vicente, and later refactored by [Albert Villanova del Moral](https://huggingface.co/albertvillanova).

## Quick start

This example demonstrates how to train a model using the KTO method. We use the [Qwen 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) as the base model. We use the preference data from the [KTO Mix 14k](https://huggingface.co/datasets/trl-lib/kto-mix-14k). You can view the data in the dataset here:

<iframe
  src="https://huggingface.co/datasets/trl-lib/kto-mix-14k/embed/viewer/default/train?row=0"
  frameborder="0"
  width="100%"
  height="560px"
>

Below is the script to train the model:

```python
# train_kto.py
from datasets import load_dataset
from trl import KTOConfig, KTOTrainer
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2-0.5B-Instruct")
train_dataset = load_dataset("trl-lib/kto-mix-14k", split="train")

training_args = KTOConfig(output_dir="Qwen2-0.5B-KTO")
trainer = KTOTrainer(model=model, args=training_args, processing_class=tokenizer, train_dataset=train_dataset)
trainer.train()
```

Execute the script using the following command:

```bash
accelerate launch train_kto.py
```

Distributed across 8 x H100 GPUs, the training takes approximately 30 minutes. You can verify the training progress by checking the reward graph. An increasing trend in the reward margin indicates that the model is improving and generating better responses over time.

![kto qwen2 reward margin](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/kto-qwen2-reward-margin.png)

To see how the [trained model](https://huggingface.co/trl-lib/Qwen2-0.5B-KTO) performs, you can use the [Transformers Chat CLI](https://huggingface.co/docs/transformers/quicktour#chat-with-text-generation-models).

$ transformers chat trl-lib/Qwen2-0.5B-KTO
&lt;quentin_gallouedec&gt;:
What is the best programming language?

&lt;trl-lib/Qwen2-0.5B-KTO&gt;:
The best programming language can vary depending on individual preferences, industry-specific requirements, technical skills, and familiarity with the specific use case or task. Here are some widely-used programming languages that have been noted as popular and widely used:

Here are some other factors to consider when choosing a programming language for a project:

 1 JavaScript: JavaScript is at the heart of the web and can be used for building web applications, APIs, and interactive front-end applications like frameworks like React and Angular. It's similar to C, C++, and F# in syntax structure and is accessible and easy to learn, making it a popular choice for beginners and professionals alike.
 2 Java: Known for its object-oriented programming (OOP) and support for Java 8 and .NET, Java is used for developing enterprise-level software applications, high-performance games, as well as mobile apps, game development, and desktop applications.
 3 C++: Known for its flexibility and scalability, C++ offers comprehensive object-oriented programming and is a popular choice for high-performance computing and other technical fields. It's a powerful platform for building real-world applications and games at scale.
 4 Python: Developed by Guido van Rossum in 1991, Python is a high-level, interpreted, and dynamically typed language known for its simplicity, readability, and versatility.

## Expected dataset type and format

KTO requires an [unpaired preference](dataset_formats#unpaired-preference) dataset. Alternatively, you can provide a *paired* preference dataset (also known simply as a *preference dataset*). In this case, the trainer will automatically convert it to an unpaired format by separating the chosen and rejected responses, assigning `label = True` to the chosen completions and `label = False` to the rejected ones.

The [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) is compatible with both [standard](dataset_formats#standard) and [conversational](dataset_formats#conversational) dataset formats. When provided with a conversational dataset, the trainer will automatically apply the chat template to the dataset.

```python
# Standard format
unpaired_preference_example = {"prompt": "The sky is", "completion": " blue.", "label": True}

# Conversational format
unpaired_preference_example = {"prompt": [{"role": "user", "content": "What color is the sky?"}],
                               "completion": [{"role": "assistant", "content": "It is blue."}],
                               "label": True}
```

In theory, the dataset should contain at least one chosen and one rejected completion. However, some users have successfully run KTO using *only* chosen or only rejected data. If using only rejected data, it is advisable to adopt a conservative learning rate.

## Looking deeper into the KTO method

Kahneman-Tversky Optimization (KTO) is a training method designed to align a language model using only a binary signal of whether an output is *desirable* or *undesirable* for a given input, rather than pairs of preferred/dispreferred completions. Drawing on the prospect theory of Kahneman and Tversky, it defines a *human-aware loss* (HALO) that directly maximizes the utility of generations, weighting desirable and undesirable examples asymmetrically to reflect human loss aversion.

This section breaks down how KTO works in practice, covering the key steps: **preprocessing** and **loss computation**.

### Preprocessing and tokenization

During training, each example is expected to contain a prompt, a `completion`, and a boolean `label` indicating whether the completion is desirable (`True`) or undesirable (`False`). For more details on the expected formats, see [Dataset formats](dataset_formats).
The [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) tokenizes each input using the model's tokenizer.

### Computing the loss

The KL divergence term used by the KTO loss is estimated by pairing each prompt with a mismatched completion drawn from elsewhere in the batch. For this reason, loss types that estimate the KL term (all except `apo_zero_unpaired`) require a per-device train batch size greater than 1 and a sequential sampling strategy, so that the mismatched pairs are stable across a batch.

The loss used in KTO (Eq. 7 of the [paper](https://huggingface.co/papers/2402.01306)) is defined as follows:

$$
\mathcal{L}_{\mathrm{KTO}}(\theta) = \mathbb{E}_{(x,y)}\!\left[ w(y)\Big(1 - v(x, y)\Big) \right]
$$

where the value  \\( v(x, y) \\) is

$$
v(x, y) = \begin{cases}
\sigma\!\left(\beta\big(\log\frac{\pi_{\theta}(y\mid x)}{\pi_{\mathrm{ref}}(y\mid x)} - \mathrm{KL}\big)\right) & \text{if } y \text{ is desirable} \\
\sigma\!\left(\beta\big(\mathrm{KL} - \log\frac{\pi_{\theta}(y\mid x)}{\pi_{\mathrm{ref}}(y\mid x)}\big)\right) & \text{if } y \text{ is undesirable}
\end{cases}
$$

Here  \\( x \\) is the prompt,  \\( y \\) is the completion,  \\( \pi_{\theta} \\) is the policy model being trained,  \\( \pi_{\mathrm{ref}} \\) is the reference model,  \\( \sigma \\) is the sigmoid function,  \\( \beta > 0 \\) controls the deviation from the reference model, and  \\( \mathrm{KL} \\) is the estimated KL divergence term. The weight  \\( w(y) \\) is `desirable_weight` for desirable completions and `undesirable_weight` for undesirable ones, used to counter an imbalance between the number of desirable and undesirable examples.

#### Loss Types

| `loss_type=` | Description |
| --- | --- |
| `"kto"` (default) | The KTO loss from the [KTO](https://huggingface.co/papers/2402.01306) paper. A human-aware loss (HALO) based on Kahneman-Tversky prospect theory that maximizes the utility of desirable completions and minimizes it for undesirable ones, using an estimated KL divergence term as a reference point. |
| `"apo_zero_unpaired"` | The unpaired variant of the APO-zero loss from the [APO](https://huggingface.co/papers/2408.06266) paper. It increases the likelihood of desirable completions and decreases the likelihood of undesirable ones without estimating the KL divergence term. Use this loss when you believe the desirable completions are better than the model's default output. |

## Logged metrics

While training and evaluating, we record the following metrics:

* `global_step`: The total number of optimizer steps taken so far.
* `epoch`: The current epoch number, based on dataset iteration.
* `num_tokens`: The total number of tokens processed so far.
* `loss`: The average KTO loss over the current logging interval.
* `entropy`: The average entropy of the model's predicted token distribution over non-masked tokens.
* `kl`: The average estimated KL divergence between the policy and reference model, used as the reference point in the KTO loss.
* `learning_rate`: The current learning rate, which may change dynamically if a scheduler is used.
* `grad_norm`: The L2 norm of the gradients, computed before gradient clipping.
* `logits/chosen`: The average logit values assigned by the model to the tokens in the chosen (desirable) completion.
* `logits/rejected`: The average logit values assigned by the model to the tokens in the rejected (undesirable) completion.
* `logps/chosen`: The average log-probability assigned by the model to the tokens in the chosen (desirable) completion.
* `logps/rejected`: The average log-probability assigned by the model to the tokens in the rejected (undesirable) completion.
* `rewards/chosen`: The average implicit reward computed for the chosen (desirable) completion, computed as  \\( \beta \log \frac{\pi_{\theta}(y\mid x)}{\pi_{\mathrm{ref}}(y\mid x)} \\).
* `rewards/rejected`: The average implicit reward computed for the rejected (undesirable) completion, computed as  \\( \beta \log \frac{\pi_{\theta}(y\mid x)}{\pi_{\mathrm{ref}}(y\mid x)} \\).
* `rewards/margins`: The average implicit reward margin between the chosen and rejected completions.

## Customization

### Compatibility and constraints

Some argument combinations are intentionally restricted in the current [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) implementation:

* With `use_liger_kernel=True`:
  * only `loss_type="kto"` is supported (not `"apo_zero_unpaired"`),
  * `compute_metrics` is not supported,
  * `precompute_ref_log_probs=True` is not supported,
  * PEFT models are not supported.
* `sync_ref_model=True` is not supported when training with PEFT models that do not keep a standalone `ref_model`.
* `sync_ref_model=True` cannot be combined with `precompute_ref_log_probs=True`.
* `precompute_ref_log_probs=True` is not supported with `IterableDataset` (train or eval) or with vision datasets.
* Loss types that estimate the KL divergence term (all except `"apo_zero_unpaired"`) require `train_sampling_strategy="sequential"` and a per-device train batch size greater than 1.

### Model initialization

You can directly pass the kwargs of the `from_pretrained()` method to the [KTOConfig](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOConfig). For example, if you want to load a model in a different precision, analogous to

```python
model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2-0.5B-Instruct", dtype=torch.bfloat16)
```

you can do so by passing the `model_init_kwargs={"dtype": torch.bfloat16}` argument to the [KTOConfig](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOConfig).

```python
from trl import KTOConfig

training_args = KTOConfig(
    model_init_kwargs={"dtype": torch.bfloat16},
)
```

Note that all keyword arguments of `from_pretrained()` are supported.

### Train adapters with PEFT

We support tight integration with 🤗 PEFT library, allowing any user to conveniently train adapters and share them on the Hub, rather than training the entire model.

```python
from datasets import load_dataset
from trl import KTOTrainer
from peft import LoraConfig

dataset = load_dataset("trl-lib/kto-mix-14k", split="train")

trainer = KTOTrainer(
    "Qwen/Qwen2-0.5B-Instruct",
    train_dataset=dataset,
    peft_config=LoraConfig(),
)

trainer.train()
```

You can also continue training your `PeftModel`. For that, first load a `PeftModel` outside [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) and pass it directly to the trainer without the `peft_config` argument being passed.

> [!TIP]
> When training adapters, you typically use a higher learning rate than full fine-tuning since only new parameters are being learned.

### Train with Liger Kernel

Liger Kernel is a collection of Triton kernels for LLM training that boosts multi-GPU throughput by 20%, cuts memory use by 60% (enabling up to 4× longer context), and works seamlessly with tools like FlashAttention, PyTorch FSDP, and DeepSpeed. For more information, see [Liger Kernel Integration](liger_kernel_integration).

## Tool Calling with KTO

The [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) fully supports fine-tuning models with _tool calling_ capabilities. In this case, each dataset example should include:

* The conversation messages (prompt and completion), including any tool calls (`tool_calls`) and tool responses (`tool` role messages)
* The list of available tools in the `tools` column, typically provided as JSON schemas

For details on the expected dataset structure, see the [Dataset Format — Tool Calling](dataset_formats#tool-calling) section.

## Training Vision Language Models

[KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) fully supports training Vision-Language Models (VLMs). To train a VLM, provide a dataset with either an `image` column (single image per sample) or an `images` column (list of images per sample). For more information on the expected dataset structure, see the [Dataset Format — Vision Dataset](dataset_formats#vision-dataset) section.

```python
from trl import KTOConfig, KTOTrainer
from datasets import load_dataset

trainer = KTOTrainer(
    model="Qwen/Qwen2.5-VL-3B-Instruct",
    args=KTOConfig(max_length=None),
    train_dataset=load_dataset("trl-internal-testing/zen-image", "conversational_unpaired_preference", split="train"),
)
trainer.train()
```

> [!TIP]
> For VLMs, truncating may remove image tokens, leading to errors during training. To avoid this, set `max_length=None` in the [KTOConfig](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOConfig). This allows the model to process the full sequence length without truncating image tokens.
>
> ```python
> KTOConfig(max_length=None, ...)
> ```
>
> Only use `max_length` when you've verified that truncation won't remove image tokens for the entire dataset.

## Example script

We provide an example script to train a model using the KTO method. The script is available in [`trl/scripts/kto.py`](https://github.com/huggingface/trl/blob/main/trl/scripts/kto.py)

To test the KTO script with the [Qwen2 0.5B model](https://huggingface.co/Qwen/Qwen2-0.5B-Instruct) on the [UltraFeedback dataset](https://huggingface.co/datasets/trl-lib/kto-mix-14k), run the following command:

```bash
accelerate launch trl/scripts/kto.py \
    --model_name_or_path Qwen/Qwen2-0.5B-Instruct \
    --dataset_name trl-lib/kto-mix-14k \
    --num_train_epochs 1 \
    --output_dir Qwen2-0.5B-KTO
```

## Usage tips

### For Mixture of Experts Models: Enabling the auxiliary loss

MOEs are the most efficient if the load is about equally distributed between experts.  
To ensure that we train MOEs similarly during preference-tuning, it is beneficial to add the auxiliary loss from the load balancer to the final loss.

This option is enabled by setting `output_router_logits=True` in the model config (e.g. [MixtralConfig](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/mixtral#transformers.MixtralConfig)).  
To scale how much the auxiliary loss contributes to the total loss, use the hyperparameter `router_aux_loss_coef=...` (default: `0.001`) in the model config.

### Batch size recommendations

Use a per-step batch size that is at least 4, and an effective batch size between 16 and 128. Even if your effective batch size is large, if your per-step batch size is poor, then the KL estimate in KTO will be poor.

### Learning rate recommendations

Each choice of `beta` has a maximum learning rate it can tolerate before learning performance degrades. For the default setting of `beta = 0.1`, the learning rate should typically not exceed `1e-6` for most models. As `beta` decreases, the learning rate should also be reduced accordingly. In general, we strongly recommend keeping the learning rate between `5e-7` and `5e-6`. Even with small datasets, we advise against using a learning rate outside this range. Instead, opt for more epochs to achieve better results.

### Imbalanced data

The `desirable_weight` and `undesirable_weight` of the [KTOConfig](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOConfig) refer to the weights placed on the losses for desirable/positive and undesirable/negative examples.
By default, they are both 1. However, if you have more of one or the other, then you should upweight the less common type such that the ratio of (`desirable_weight`  \\(\times\\) number of positives) to (`undesirable_weight`  \\(\times\\) number of negatives) is in the range 1:1 to 4:3.

## KTOTrainer[[trl.KTOTrainer]]

- **model** (`str` or [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) or `PeftModel`) --
  Model to be trained. Can be either:

  - A string, being the *model id* of a pretrained model hosted inside a model repo on huggingface.co, or a
    path to a *directory* containing model weights saved using
    [save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `'./my_model_directory/'`. The model is loaded
    using `<ModelArchitecture>.from_pretrained` (where `<ModelArchitecture>` is derived from the model
    config) with the keyword arguments in `args.model_init_kwargs`. If `dtype` is not specified in
    `args.model_init_kwargs`, it defaults to `float32`. This differs from
    [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained), where (since Transformers v5) the dtype is inferred
    from the model config.
  - A [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) object. Only causal language models are supported.
  - A `PeftModel` object. Only causal language models are supported.
- **ref_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel), *optional*) --
  Reference model used to compute the reference log probabilities.

  - If provided, this model is used directly as the reference policy.
  - If `None`, the trainer will automatically use the initial policy corresponding to `model`, i.e. the model
    state before KTO training starts.
- **args** ([KTOConfig](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOConfig), *optional*) --
  Configuration for this trainer. If `None`, a default configuration is used.
- **data_collator** (`DataCollator`, *optional*) --
  Function to use to form a batch from a list of elements of the processed `train_dataset` or `eval_dataset`.
  Will default to `DataCollatorForUnpairedPreference` if the model is a language model
  and `DataCollatorForVisionUnpairedPreference` if the model is a vision-language
  model. Custom collators must truncate sequences before padding; the trainer does not apply post-collation
  truncation.
- **train_dataset** (`Dataset` or `IterableDataset`) --
  Dataset to use for training. This trainer supports [unpaired preference](#unpaired-preference) type. The
  format of the samples can be either:

  - [Standard](dataset_formats#standard): Each sample contains plain text.
  - [Conversational](dataset_formats#conversational): Each sample contains structured messages (e.g., role
    and content).

  When `train_dataset` is an `IterableDataset` (e.g. a streaming dataset), `max_steps` must be
  set in the training arguments, since its length cannot be inferred and the total number of training steps
  is required to bound the training loop and configure the learning rate scheduler.
- **eval_dataset** (`Dataset`, `IterableDataset`, `DatasetDict`, `IterableDatasetDict` or `dict[str, Dataset | IterableDataset]`) --
  Dataset to use for evaluation. It must meet the same requirements as `train_dataset`.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin), *optional*) --
  Processing class used to process the data. The padding side must be set to "left". If `None`, the
  processing class is loaded from the model's name with [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained). A
  padding token, `tokenizer.pad_token`, must be set. If the processing class has not set a padding token,
  `tokenizer.eos_token` will be used as the default.
- **compute_metrics** (`Callable[[EvalPrediction], dict]`, *optional*) --
  The function that will be used to compute metrics at evaluation. Must take a
  [EvalPrediction](https://huggingface.co/docs/transformers/v5.14.1/en/internal/trainer_utils#transformers.EvalPrediction) and return a dictionary string to metric values. When passing
  [SFTConfig](/docs/trl/v1.9.2/en/sft_trainer#trl.SFTConfig) with `batch_eval_metrics` set to `True`, your `compute_metrics` function must take a boolean
  `compute_result` argument. This will be triggered after the last eval batch to signal that the function
  needs to calculate and return the global summary statistics rather than accumulating the batch-level
  statistics.
- **callbacks** (list of [TrainerCallback](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) --
  List of callbacks to customize the training loop. Will add those to the list of default callbacks detailed
  in [here](https://huggingface.co/docs/transformers/main_classes/callback).

  If you want to remove one of the default callbacks used, use the [remove_callback](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.Trainer.remove_callback)
  method.
- **optimizers** (`tuple[torch.optim.Optimizer | None, torch.optim.lr_scheduler.LambdaLR | None]`, *optional*, defaults to `(None, None)`) --
  A tuple containing the optimizer and the scheduler to use. Will default to an instance of `AdamW` on your
  model and a scheduler given by [get_linear_schedule_with_warmup](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/optimizer_schedules#transformers.get_linear_schedule_with_warmup) controlled by `args`.
- **quantization_config** ([BitsAndBytesConfig](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/quantization#transformers.BitsAndBytesConfig), *optional*) --
  Quantization configuration used when loading the model from a model identifier. Combine with `peft_config`
  for QLoRA training. Ignored if the model is already instantiated.
- **peft_config** (`PeftConfig`, *optional*) --
  PEFT configuration used to wrap the model. If `None`, the model is not wrapped.

Trainer for Kahneman-Tversky Optimization (KTO) method. This algorithm was initially proposed in the paper [KTO:
Model Alignment as Prospect Theoretic Optimization](https://huggingface.co/papers/2402.01306). This class is a
wrapper around the [Trainer](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.Trainer) class and inherits all of its attributes and methods.

Example:

```python
>>> from trl import KTOTrainer
>>> from datasets import load_dataset

>>> dataset = load_dataset("trl-lib/kto-mix-14k", split="train")

>>> trainer = KTOTrainer(
...     model="Qwen/Qwen2.5-0.5B-Instruct",
...     train_dataset=dataset,
... )
>>> trainer.train()
```

- **resume_from_checkpoint** (`str` or `bool`, *optional*) --
  If a `str`, local path to a saved checkpoint as saved by a previous instance of `Trainer`. If a
  `bool` and equals `True`, load the last checkpoint in *args.output_dir* as saved by a previous instance
  of `Trainer`. If present, training will resume from the model/optimizer/scheduler states loaded here.
- **trial** (`optuna.Trial` or `dict[str, Any]`, *optional*) --
  The trial run or the hyperparameter dictionary for hyperparameter search.
- **ignore_keys_for_eval** (`list[str]`, *optional*) --
  A list of keys in the output of your model (if it is a dictionary) that should be ignored when
  gathering predictions for evaluation during the training.`~trainer_utils.TrainOutput`Object containing the global step count, training loss, and metrics.

Main training entry point.

Will save the model, so you can reload it using `from_pretrained()`.

Will only save from the main process.

- **commit_message** (`str`, *optional*, defaults to `"End of training"`) --
  Message to commit while pushing.
- **blocking** (`bool`, *optional*, defaults to `True`) --
  Whether the function should return only when the `git push` has finished.
- **token** (`str`, *optional*, defaults to `None`) --
  Token with write permission to overwrite Trainer's original args.
- **revision** (`str`, *optional*) --
  The git revision to commit from. Defaults to the head of the "main" branch.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments passed along to `~Trainer.create_model_card`.The URL of the repository where the model was pushed if `blocking=False`, or a `Future` object tracking the
progress of the commit if `blocking=True`.

Upload `self.model` and `self.processing_class` to the 🤗 model hub on the repo `self.args.hub_model_id`.

## KTOConfig[[trl.KTOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'sequential'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "model_init_kwargs", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "router_aux_loss_coef", "val": ": float = 0.001"}, {"name": "disable_dropout", "val": ": bool = True"}, {"name": "dataset_num_proc", "val": ": int | None = None"}, {"name": "max_length", "val": ": int | None = 1024"}, {"name": "pad_to_multiple_of", "val": ": int | None = None"}, {"name": "precompute_ref_log_probs", "val": ": bool = False"}, {"name": "precompute_ref_batch_size", "val": ": int | None = None"}, {"name": "loss_type", "val": ": str = 'kto'"}, {"name": "beta", "val": ": float = 0.1"}, {"name": "desirable_weight", "val": ": float = 1.0"}, {"name": "undesirable_weight", "val": ": float = 1.0"}, {"name": "activation_offloading", "val": ": bool = False"}, {"name": "sync_ref_model", "val": ": bool = False"}, {"name": "ref_model_mixup_alpha", "val": ": float = 0.6"}, {"name": "ref_model_sync_steps", "val": ": int = 512"}]}>
Parameters that control the model

- **model_init_kwargs** (`dict[str, Any]`, *optional*) --
  Keyword arguments for [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained), used when the `model`
  argument of the [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer) is provided as a string.
- **trust_remote_code** (`bool`, *optional*, defaults to `False`) --
  Whether to allow loading models and tokenizers that ship custom Python code from the Hub. Forwarded to
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForCausalLM.from_pretrained) and
  [from_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoProcessor.from_pretrained).
- **router_aux_loss_coef** (`float`, *optional*, defaults to `0.001`) --
  Coefficient of the load-balancing auxiliary loss. Only has an effect when training a Mixture-of-Experts
  (MoE) model; for other models it does nothing. The auxiliary loss is added to the training loss with this
  weight. Set to `0.0` to disable it.
- **disable_dropout** (`bool`, *optional*, defaults to `True`) --
  Whether to disable dropout in the model and reference model.

Parameters that control the data preprocessing

- **dataset_num_proc** (`int`, *optional*) --
  Number of processes to use for processing the dataset.
- **max_length** (`int` or `None`, *optional*, defaults to `1024`) --
  Maximum length of the tokenized sequence. Sequences longer than `max_length` are truncated from the right.
  If `None`, no truncation is applied.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set, the sequences will be padded to a multiple of this value.
- **precompute_ref_log_probs** (`bool`, *optional*, defaults to `False`) --
  Whether to precompute the reference model log probabilities for the entire training dataset before
  training. This allows to save memory during training, as the reference model does not need to be kept in
  memory.
- **precompute_ref_batch_size** (`int`, *optional*) --
  Batch size to use when precomputing reference model log probabilities. This can be set higher than the
  training batch size to speed up preprocessing. If `None`, defaults to `per_device_train_batch_size` for
  training and `per_device_eval_batch_size` for evaluation.

Parameters that control the training

- **loss_type** (`str`, *optional*, defaults to `"kto"`) --
  Type of loss to use. Possible values are:

  - `"kto"`: KTO loss from the [KTO](https://huggingface.co/papers/2402.01306) paper.
  - `"apo_zero_unpaired"`: Unpaired variant of APO-zero loss from the
    [APO](https://huggingface.co/papers/2408.06266) paper.

- **beta** (`float`, *optional*, defaults to `0.1`) --
  Parameter controlling the deviation from the reference model. Higher β means less deviation from the
  reference model.
- **desirable_weight** (`float`, *optional*, defaults to `1.0`) --
  Desirable losses are weighed by this factor to counter unequal number of desirable and undesirable pairs.
- **undesirable_weight** (`float`, *optional*, defaults to `1.0`) --
  Undesirable losses are weighed by this factor to counter unequal number of desirable and undesirable pairs.
- **activation_offloading** (`bool`, *optional*, defaults to `False`) --
  Whether to offload the activations to the CPU.
- **sync_ref_model** (`bool`, *optional*, defaults to `False`) --
  Whether to synchronize the reference model with the active model every `ref_model_sync_steps` steps, using
  the `ref_model_mixup_alpha` parameter. This synchronization originates from the
  [TR-DPO](https://huggingface.co/papers/2404.09656) paper. `sync_ref_model=True` is not yet compatible with
  PEFT or `precompute_ref_log_probs=True`.
- **ref_model_mixup_alpha** (`float`, *optional*, defaults to `0.6`) --
  α parameter from the TR-DPO paper, which controls the mix between the current policy and the previous
  reference policy during updates. The reference policy is updated according to the equation: `π_ref = α *
  π_θ + (1 - α) * π_ref_prev`. To use this parameter, you must set `sync_ref_model=True`.
- **ref_model_sync_steps** (`int`, *optional*, defaults to `512`) --
  τ parameter from the TR-DPO paper, which determines how frequently the current policy is synchronized with
  the reference policy. To use this parameter, you must set `sync_ref_model=True`.

Configuration class for the [KTOTrainer](/docs/trl/v1.9.2/en/kto_trainer#trl.KTOTrainer).

This class includes only the parameters that are specific to KTO training. For a full list of training arguments,
please refer to the [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments) documentation. Note that default values in this class may
differ from those in [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments).

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.14.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

> [!NOTE]
> These parameters have default values different from [TrainingArguments](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.TrainingArguments):
> - `logging_steps`: Defaults to `10` instead of `500`.
> - `gradient_checkpointing`: Defaults to `True` instead of `False`.
> - `bf16`: Defaults to `True` if `fp16` is not set, instead of `False`.
> - `learning_rate`: Defaults to `1e-6` instead of `5e-5`.
> - `train_sampling_strategy`: Defaults to `"sequential"` instead of `"random"`. Loss types
>   that estimate the KL divergence term (all except `"apo_zero_unpaired"`) require sequential
>   sampling because the KL completion for each example is precomputed against its neighbors in
>   a fixed-order batch; any other strategy breaks that pairing.
