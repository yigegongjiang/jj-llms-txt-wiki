# PPO Trainer

[![model badge](https://img.shields.io/badge/All_models-PPO-blue)](https://huggingface.co/models?other=ppo,trl)

TRL supports training LLMs with [Proximal Policy Optimization (PPO)](https://huggingface.co/papers/1707.06347).

References:

- [Fine-Tuning Language Models from Human Preferences](https://github.com/openai/lm-human-preferences)
- [Learning to Summarize from Human Feedback](https://github.com/openai/summarize-from-feedback)
- [The N Implementation Details of RLHF with PPO](https://huggingface.co/blog/the_n_implementation_details_of_rlhf_with_ppo)
- [The N+ Implementation Details of RLHF with PPO: A Case Study on TL;DR Summarization](https://huggingface.co/papers/2403.17031)

## Get started

To just run a PPO script to make sure the trainer can run, you can run the following command to train a PPO model with a dummy reward model.

```bash
python examples/scripts/ppo/ppo.py \
    --dataset_name trl-internal-testing/descriptiveness-sentiment-trl-style \
    --dataset_train_split descriptiveness \
    --learning_rate 3e-6 \
    --num_ppo_epochs 1 \
    --num_mini_batches 1 \
    --output_dir models/minimal/ppo \
    --per_device_train_batch_size 64 \
    --gradient_accumulation_steps 1 \
    --total_episodes 10000 \
    --model_name_or_path EleutherAI/pythia-1b-deduped \
    --sft_model_path EleutherAI/pythia-1b-deduped \
    --reward_model_path EleutherAI/pythia-1b-deduped \
    --missing_eos_penalty 1.0
```

## Explanation of the logged metrics

The logged metrics are as follows. Here is an example [tracked run at Weights and Biases](https://wandb.ai/huggingface/trl/runs/dd2o3g35)

- `eps`: Tracks the number of episodes per second.
- `objective/kl`: The mean Kullback-Leibler (KL) divergence between the current policy and reference policy.
- `objective/entropy`: The mean token-level entropy proxy on sampled rollouts, computed as `(-logprobs).sum(1).mean()` before PPO optimization.
- `objective/non_score_reward`: The mean reward from non-score-related sources, basically `beta * kl.sum(1)`, where `beta` is the KL penalty coefficient and `kl` is the per-token KL divergence.
- `objective/rlhf_reward`: The mean RLHF reward, which is `score - non_score_reward`.
- `objective/scores`: The mean scores returned by the reward model / environment.
- `policy/approxkl_avg`: The average approximate KL divergence between consecutive PPO policies. Note that this is not the same as `objective/kl`.
- `policy/clipfrac_avg`: The average fraction of policy updates that are clipped, indicating how often the policy updates are constrained to prevent large changes.
- `loss/policy_avg`: The average policy loss, indicating how well the policy is performing.
- `loss/value_avg`: The average value loss, indicating the difference between the predicted value and the actual reward.
- `val/clipfrac_avg`: The average fraction of value function updates that are clipped, similar to policy/clipfrac_avg but for the value function.
- `policy/entropy_avg`: The average categorical entropy of the current policy during PPO optimization, computed from `logits` each minibatch and averaged across PPO epochs/minibatches.
- `val/ratio`: The mean ratio of the current policy probability to the old policy probability, providing a measure of how much the policy has changed.
- `val/ratio_var`: The variance of the `val/ratio`, indicating the variability in policy changes.
- `val/num_eos_tokens`: The number of end-of-sequence (EOS) tokens generated, which can indicate the number of complete responses.
- `lr`: lr: The current learning rate used by the optimizer.
- `episode`: episode: The current episode count in the training process.

`objective/entropy` and `policy/entropy_avg` are intentionally different signals:

- `objective/entropy` is measured on the sampled behavior-policy rollouts used to build PPO advantages for the current update.
- `policy/entropy_avg` is measured during PPO optimization on the evolving policy (after gradient steps), so it can diverge from `objective/entropy`.

## Cookbook

- Debugging TIP: `objective/rlhf_reward`: this is the ultimate objective of the RLHF training. If training works as intended, this metric should keep going up.
- Debugging TIP: `val/ratio`: this number should float around 1.0, and it gets clipped by `--cliprange 0.2` with PPO's surrogate loss. So if this `ratio` is too high like 2.0 or 1000.0 or too small like 0.1, it means the updates between consecutive policies are too drastic. You should try understand why this is happening and try to fix it.
- Memory TIP: If you are running out of memory, you can try to reduce the `--per_device_train_batch_size` or increase the `--gradient_accumulation_steps` to reduce the memory footprint.
- Memory TIP: If you have multiple GPUs, you can also run training with DeepSpeed stage 3 to reduce the memory footprint `accelerate launch --config_file examples/accelerate_configs/deepspeed_zero3.yaml`.
- Usage TIP: We recommend to use the "EOS trick" via `--missing_eos_penalty`, which subtracts a static scalar penalty from the score of completions that do not end with an EOS token. This can help the model learn to generate more coherent completions.

## What is my model doing exactly?

To help you understand what your model is doing, we periodically log some sample completions from the model. Here is an example of a completion. In an example [tracked run at Weights and Biases](https://wandb.ai/huggingface/trl/runs/dd2o3g35), it looks like the following, allowing you to see the model's response at different stages of training. By default we generate `--num_sample_generations 10` during training, but you can customize the number of generations.

![ppov2_completions](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/ppov2_completions.gif)

In the logs the sampled generations look like

```txt
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━┓
┃ query                           ┃ model response                  ┃ score    ┃
┡━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━┩
│  SUBREDDIT: r/AskReddit         │  I'm in love with a friend, and │ 3.921875 │
│                                 │ I don't know how to get rid of  │          │
│ TITLE: How do you get someone   │ those feelings. I'm             │          │
│ out of your head?               │ desperate.<|endoftext|>[PAD][P… │          │
│                                 │                                 │          │
│ POST: Hi,                       │                                 │          │
│ I'm 22, and I have been with my │                                 │          │
│ girlfriend for 5 years now. We  │                                 │          │
│ recently moved together. We've  │                                 │          │
│ always loved each other         │                                 │          │
│ intensely.                      │                                 │          │
│                                 │                                 │          │
│ Problem, I recently started to  │                                 │          │
│ have feelings for an other      │                                 │          │
│ person (a friend). This person  │                                 │          │
│ has had a boyfriend for now 3   │                                 │          │
│ years, and has absolutely no    │                                 │          │
│ ideas. Those feelings were so   │                                 │          │
│ strong, it was hard to hide     │                                 │          │
│ them. After 2 months of me      │                                 │          │
│ being distant and really sad,   │                                 │          │
│ my girlfriend forced me to say  │                                 │          │
│ what was bothering me. I'm not  │                                 │          │
│ a good liar, and now she knows. │                                 │          │
│                                 │                                 │          │
│ We decided to give us a week    │                                 │          │
│ alone, I went to my parents.    │                                 │          │
│                                 │                                 │          │
│ Now, I'm completely lost. I     │                                 │          │
│ keep on thinking about this     │                                 │          │
│ person, and I hate that. I      │                                 │          │
│ would like for those feelings   │                                 │          │
│ to go away, to leave me alone.  │                                 │          │
│ But I can't.                    │                                 │          │
│                                 │                                 │          │
│ What do I do? It's been 3       │                                 │          │
│ months now, and I'm just        │                                 │          │
│ desperate.                      │                                 │          │
│                                 │                                 │          │
│ TL;DR:                          │                                 │          │
├─────────────────────────────────┼─────────────────────────────────┼──────────┤
│  SUBREDDIT: r/pettyrevenge      │  My mom woke me up with a loud  │ 6.84375  │
│                                 │ TV. I blasted Gangnam Style on  │          │
│ TITLE: So, my mom woke me up    │ repeat, with the bass cranked   │          │
│ with a loud TV.                 │ up as high as it could          │          │
│                                 │ go.<|endoftext|>[PAD][PAD][PAD… │          │
│ POST: She was in her living     │                                 │          │
│ room, watching TV. This was at  │                                 │          │
│ about 8:30 in the morning, and  │                                 │          │
│ she was exercising. She turned  │                                 │          │
│ the TV up extra loud to hear it │                                 │          │
│ over her excercycle, and woke   │                                 │          │
│ me up. I went in there asking   │                                 │          │
│ for her to turn it down. She    │                                 │          │
│ said she didn't have to; I      │                                 │          │
│ explained that I always used    │                                 │          │
│ headphones so she didn't have   │                                 │          │
│ to deal with my noise and that  │                                 │          │
│ she should give me a little     │                                 │          │
│ more respect, given that I paid │                                 │          │
│ rent at the time.               │                                 │          │
│                                 │                                 │          │
│ She disagreed. I went back to   │                                 │          │
│ my room, rather pissed off at   │                                 │          │
│ the lack of equality. I had no  │                                 │          │
│ lock on my door; but I had a    │                                 │          │
│ dresser right next to it, so I  │                                 │          │
│ pulled one of the drawers out   │                                 │          │
│ enough so that it caused the    │                                 │          │
│ door to not be openable. Then,  │                                 │          │
│ I turned my speakers up really  │                                 │          │
│ loud and blasted Gangnam Style  │                                 │          │
│ on repeat, with the bass        │                                 │          │
│ cranked up as high as it could  │                                 │          │
│ go.                             │                                 │          │
│                                 │                                 │          │
│ If you hate Gangnam Style for   │                                 │          │
│ being overplayed, you will see  │                                 │          │
│ why I chose that particular     │                                 │          │
│ song. I personally don't mind   │                                 │          │
│ it. But here's the thing about  │                                 │          │
│ my bass; it vibrates the walls, │                                 │          │
│ making one hell of a lot of     │                                 │          │
│ noise. Needless to say, my mom  │                                 │          │
│ was not pleased and shut off    │                                 │          │
│ the internet. But it was oh so  │                                 │          │
│ worth it.                       │                                 │          │
│                                 │                                 │          │
│ TL;DR:                          │                                 │          │
└─────────────────────────────────┴─────────────────────────────────┴──────────┘
```

## Implementation details

This PPO implementation is based on the [The N+ Implementation Details of RLHF with PPO: A Case Study on TL;DR Summarization](https://huggingface.co/papers/2403.17031).

## Benchmark experiments

To validate the PPO implementation works, we ran experiment on the 1B model. Here are the command we used to run the experiment. We take the SFT / RM models directly from [The N+ Implementation Details of RLHF with PPO: A Case Study on TL;DR Summarization](https://huggingface.co/papers/2403.17031).

```shell
accelerate launch --config_file examples/accelerate_configs/deepspeed_zero2.yaml \
    examples/scripts/ppo/ppo_tldr.py \
    --dataset_name trl-lib/tldr \
    --dataset_test_split validation \
    --output_dir models/minimal/ppo_tldr \
    --learning_rate 3e-6 \
    --per_device_train_batch_size 16 \
    --gradient_accumulation_steps 4 \
    --total_episodes 1000000 \
    --model_name_or_path EleutherAI/pythia-1b-deduped \
    --sft_model_path cleanrl/EleutherAI_pythia-1b-deduped__sft__tldr \
    --reward_model_path cleanrl/EleutherAI_pythia-1b-deduped__reward__tldr \
    --local_rollout_forward_batch_size 16 \
    --missing_eos_penalty 1.0 \
    --stop_token eos \
    --eval_strategy steps \
    --eval_steps 100
```

Checkpoints and experiment tracking are available at:

- [🤗 Model checkpoint](https://huggingface.co/trl-lib/ppo_tldr)
- [🐝 Tracked experiment](https://wandb.ai/huggingface/trl/runs/dd2o3g35)

The PPO checkpoint gets a 64.7% preferred rate vs the 33.0% preference rate of the SFT checkpoint (evaluated with GPT-4o mini as a judge). This is a good sign that the PPO training is working as intended.

Metrics:

![PPO v2](https://huggingface.co/datasets/trl-lib/documentation-images/resolve/main/ppov2.png)

```bash
# pip install openrlbenchmark==0.2.1a5
# see https://github.com/openrlbenchmark/openrlbenchmark#get-started for documentation
# to use it, change `?we=huggingface&wpn=trl` to your own project and `?tag=pr-1540` to your own tag
python -m openrlbenchmark.rlops_multi_metrics \
    --filters '?we=huggingface&wpn=trl&xaxis=train/episode&ceik=output_dir&cen=sft_model_path&metrics=train/objective/rlhf_reward&metrics=train/objective/scores&metrics=train/objective/kl&metrics=train/objective/non_score_reward&metrics=train/objective/entropy&metrics=train/policy/approxkl_avg&metrics=train/policy/clipfrac_avg&metrics=train/loss/policy_avg&metrics=train/loss/value_avg&metrics=train/val/clipfrac_avg&metrics=train/policy/entropy_avg&metrics=train/val/ratio&metrics=train/val/ratio_var&metrics=train/val/num_eos_tokens&metrics=train/lr&metrics=train/eps' \
        "cleanrl/EleutherAI_pythia-1b-deduped__sft__tldr?tag=pr-1540" \
    --env-ids models/minimal/ppo_tldr \
    --pc.ncols 4 \
    --pc.ncols-legend 1 \
    --pc.xlabel "Episode" \
    --output-filename benchmark/trl/pr-1540/ppo \
    --scan-history
```

## PPOTrainer[[trl.experimental.ppo.PPOTrainer]]

- **args** ([experimental.ppo.PPOConfig](/docs/trl/v1.9.2/en/ppo_trainer#trl.experimental.ppo.PPOConfig)) --
  Training arguments.
- **processing_class** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase), [BaseImageProcessor](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/image_processor#transformers.BaseImageProcessor), [FeatureExtractionMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin) or [ProcessorMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/processors#transformers.ProcessorMixin)) --
  Class to process the data.
- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Model to be trained. This is the policy model.
- **ref_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel), *optional*) --
  Reference model used to compute the KL divergence. If `None`, a copy of the policy model is created.
- **reward_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Reward model used to compute the rewards.
- **train_dataset** (`Dataset`) --
  Dataset for training.
- **value_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Value model used to predict the value of a state.
- **data_collator** ([DataCollatorWithPadding](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/data_collator#transformers.DataCollatorWithPadding), *optional*) --
  Data collator to batch and pad samples from the dataset. If `None`, a default data collator is created
  using the `processing_class`.
- **eval_dataset** (`Dataset` or `dict` of `Dataset`, *optional*) --
  Dataset for evaluation.
- **optimizers** (`tuple` of `torch.optim.Optimizer` and `torch.optim.lr_scheduler.LambdaLR`, *optional*, defaults to `(None, None)`) --
  Tuple containing the optimizer and the learning rate scheduler to use for training. If `None`, the
  optimizer and the learning rate scheduler are created using the
  [create_optimizer_and_scheduler](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/trainer#transformers.Trainer.create_optimizer_and_scheduler) method.
- **callbacks** (`list` of [TrainerCallback](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/callback#transformers.TrainerCallback), *optional*) --
  Callbacks to use during training.
- **peft_config** (`PeftConfig`, *optional*) --
  PEFT configuration to use PEFT for training. If `None`, PEFT is not used. If provided, the policy `model`
  will be wrapped with the specified PEFT adapter.
Trainer for Proximal Policy Optimization (PPO).

For details on PPO, see the paper: [Proximal Policy Optimization
Algorithms](https://huggingface.co/papers/1707.06347).

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

## PPOConfig[[trl.experimental.ppo.PPOConfig]]

"}, {"name": "batch_eval_metrics", "val": ": bool = False"}, {"name": "save_only_model", "val": ": bool = False"}, {"name": "save_strategy", "val": ": transformers.trainer_utils.SaveStrategy | str = 'steps'"}, {"name": "save_steps", "val": ": float = 500"}, {"name": "save_on_each_node", "val": ": bool = False"}, {"name": "save_total_limit", "val": ": int | None = None"}, {"name": "enable_jit_checkpoint", "val": ": bool = False"}, {"name": "push_to_hub", "val": ": bool = False"}, {"name": "hub_token", "val": ": str | None = None"}, {"name": "hub_private_repo", "val": ": bool | None = None"}, {"name": "hub_model_id", "val": ": str | None = None"}, {"name": "hub_strategy", "val": ": transformers.trainer_utils.HubStrategy | str = 'every_save'"}, {"name": "hub_always_push", "val": ": bool = False"}, {"name": "hub_revision", "val": ": str | None = None"}, {"name": "load_best_model_at_end", "val": ": bool = False"}, {"name": "metric_for_best_model", "val": ": str | None = None"}, {"name": "greater_is_better", "val": ": bool | None = None"}, {"name": "ignore_data_skip", "val": ": bool = False"}, {"name": "restore_callback_states_from_checkpoint", "val": ": bool = False"}, {"name": "full_determinism", "val": ": bool = False"}, {"name": "seed", "val": ": int = 42"}, {"name": "data_seed", "val": ": int | None = None"}, {"name": "use_cpu", "val": ": bool = False"}, {"name": "accelerator_config", "val": ": dict | str | None = None"}, {"name": "parallelism_config", "val": ": accelerate.parallelism_config.ParallelismConfig | None = None"}, {"name": "dataloader_drop_last", "val": ": bool = False"}, {"name": "dataloader_num_workers", "val": ": int = 0"}, {"name": "dataloader_pin_memory", "val": ": bool = True"}, {"name": "dataloader_persistent_workers", "val": ": bool = False"}, {"name": "dataloader_prefetch_factor", "val": ": int | None = None"}, {"name": "remove_unused_columns", "val": ": bool = True"}, {"name": "label_names", "val": ": list[str] | None = None"}, {"name": "train_sampling_strategy", "val": ": str = 'random'"}, {"name": "length_column_name", "val": ": str = 'length'"}, {"name": "ddp_find_unused_parameters", "val": ": bool | None = None"}, {"name": "ddp_bucket_cap_mb", "val": ": int | None = None"}, {"name": "ddp_broadcast_buffers", "val": ": bool | None = None"}, {"name": "ddp_static_graph", "val": ": bool | None = None"}, {"name": "ddp_backend", "val": ": str | None = None"}, {"name": "ddp_timeout", "val": ": int = 1800"}, {"name": "fsdp", "val": ": str | None = None"}, {"name": "fsdp_config", "val": ": dict[str, typing.Any] | str | None = None"}, {"name": "deepspeed", "val": ": dict | str | None = None"}, {"name": "debug", "val": ": str | list[transformers.debug_utils.DebugOption] = ''"}, {"name": "skip_memory_metrics", "val": ": bool = True"}, {"name": "do_train", "val": ": bool = False"}, {"name": "do_eval", "val": ": bool = False"}, {"name": "do_predict", "val": ": bool = False"}, {"name": "resume_from_checkpoint", "val": ": str | None = None"}, {"name": "warmup_ratio", "val": ": float | None = None"}, {"name": "logging_dir", "val": ": str | None = None"}, {"name": "local_rank", "val": ": int = -1"}, {"name": "dataset_num_proc", "val": ": int | None = None"}, {"name": "num_mini_batches", "val": ": int = 1"}, {"name": "total_episodes", "val": ": int | None = None"}, {"name": "local_rollout_forward_batch_size", "val": ": int = 64"}, {"name": "num_sample_generations", "val": ": int = 10"}, {"name": "response_length", "val": ": int = 53"}, {"name": "stop_token", "val": ": typing.Optional[typing.Literal['eos']] = None"}, {"name": "stop_token_id", "val": ": int | None = None"}, {"name": "temperature", "val": ": float = 0.7"}, {"name": "missing_eos_penalty", "val": ": float | None = None"}, {"name": "sft_model_path", "val": ": str = 'EleutherAI/pythia-160m'"}, {"name": "world_size", "val": ": int | None = None"}, {"name": "num_total_batches", "val": ": int | None = None"}, {"name": "micro_batch_size", "val": ": int | None = None"}, {"name": "local_batch_size", "val": ": int | None = None"}, {"name": "batch_size", "val": ": int | None = None"}, {"name": "local_mini_batch_size", "val": ": int | None = None"}, {"name": "mini_batch_size", "val": ": int | None = None"}, {"name": "reward_model_path", "val": ": str = 'EleutherAI/pythia-160m'"}, {"name": "model_adapter_name", "val": ": str | None = None"}, {"name": "ref_adapter_name", "val": ": str | None = None"}, {"name": "num_ppo_epochs", "val": ": int = 4"}, {"name": "whiten_rewards", "val": ": bool = False"}, {"name": "kl_coef", "val": ": float = 0.05"}, {"name": "kl_estimator", "val": ": typing.Literal['k1', 'k3'] = 'k1'"}, {"name": "cliprange", "val": ": float = 0.2"}, {"name": "vf_coef", "val": ": float = 0.1"}, {"name": "cliprange_value", "val": ": float = 0.2"}, {"name": "gamma", "val": ": float = 1.0"}, {"name": "lam", "val": ": float = 0.95"}, {"name": "ds3_gather_for_generation", "val": ": bool = True"}]}>
- **dataset_num_proc** (`int`, *optional*) --
  Number of processes to use for processing the dataset.
- **num_mini_batches** (`int`, *optional*, defaults to `1`) --
  Number of minibatches to split a batch into.
- **total_episodes** (`int`, *optional*) --
  Total number of episodes in the dataset.
- **local_rollout_forward_batch_size** (`int`, *optional*, defaults to `64`) --
  Per rank no grad forward pass in the rollout phase.
- **num_sample_generations** (`int`, *optional*, defaults to `10`) --
  Number of debugging samples generations (i.e., `generate_completions` calls) throughout training.
- **response_length** (`int`, *optional*, defaults to `53`) --
  Length of the response.
- **stop_token** (`str`, *optional*) --
  Specifies the stop token to use for text generation. This parameter is mutually exclusive with
  `stop_token_id`.

  - `None`: No stop token is applied, unless `stop_token_id` is specified.
  - `'eos'`: Uses the tokenizer's `eos_token`.

- **stop_token_id** (`int`, *optional*) --
  Specifies the ID of the stop token to use for text generation. If `None`, no stop token ID is applied,
  unless `stop_token` is specified. This parameter is mutually exclusive with `stop_token`.
- **temperature** (`float`, *optional*, defaults to `0.7`) --
  Sampling temperature.
- **missing_eos_penalty** (`float`, *optional*) --
  Penalty applied to the score when the model fails to generate an EOS token. This is useful to encourage to
  generate completions shorter than the maximum length (`max_new_tokens`). The penalty must be a positive
  value.
- **sft_model_path** (`str`, *optional*, defaults to `"EleutherAI/pythia-160m"`) --
  Path to the SFT model.
- **world_size** (`int`, *optional*) --
  Number of processes (GPUs) to use for the training.
- **num_total_batches** (`int`, *optional*) --
  Number of total batches to train.
- **micro_batch_size** (`int`, *optional*) --
  Micro batch size across devices (HF's `per_device_train_batch_size` * `world_size`).
- **local_batch_size** (`int`, *optional*) --
  Batch size per GPU (HF's `per_device_train_batch_size` * `gradient_accumulation_steps`).
- **batch_size** (`int`, *optional*) --
  Batch size across devices (HF's `per_device_train_batch_size` * `world_size` *
  `gradient_accumulation_steps`).
- **local_mini_batch_size** (`int`, *optional*) --
  Mini batch size per GPU.
- **mini_batch_size** (`int`, *optional*) --
  Mini batch size across GPUs.
- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether to push the model to the Hub after training.
- **reward_model_path** (`str`, *optional*, defaults to `"EleutherAI/pythia-160m"`) --
  Path to the reward model.
- **model_adapter_name** (`str`, *optional*) --
  Name of the train target PEFT adapter, when using LoRA with multiple adapters.
- **ref_adapter_name** (`str`, *optional*) --
  Name of the reference PEFT adapter, when using LoRA with multiple adapters.
- **num_ppo_epochs** (`int`, *optional*, defaults to `4`) --
  Number of epochs to train.
- **whiten_rewards** (`bool`, *optional*, defaults to `False`) --
  Whether to whiten the rewards.
- **kl_coef** (`float`, *optional*, defaults to `0.05`) --
  KL coefficient.
- **kl_estimator** (`Literal["k1", "k3"]`, *optional*, defaults to `"k1"`) --
  Which estimator for KL-Divergence to use from [Approximating KL
  Divergence](http://joschu.net/blog/kl-approx.html). Defaults to "k1", a straightforward, unbiased
  estimator. Can be set to "k3", an unbiased estimator with lower variance which "appears to be a strictly
  better estimator". Cannot be set to "k2", as it is used for logging purposes.
- **cliprange** (`float`, *optional*, defaults to `0.2`) --
  Clip range.
- **vf_coef** (`float`, *optional*, defaults to `0.1`) --
  Value function coefficient.
- **cliprange_value** (`float`, *optional*, defaults to `0.2`) --
  Clip range for the value function.
- **gamma** (`float`, *optional*, defaults to `1.0`) --
  Discount factor.
- **lam** (`float`, *optional*, defaults to `0.95`) --
  Lambda value for GAE.
- **ds3_gather_for_generation** (`bool`, *optional*, defaults to `True`) --
  This setting applies to DeepSpeed ZeRO-3. If enabled, the policy model weights are gathered for generation,
  improving generation speed. However, disabling this option allows training models that exceed the VRAM
  capacity of a single GPU, albeit at the cost of slower generation.

Configuration class for the [experimental.ppo.PPOTrainer](/docs/trl/v1.9.2/en/ppo_trainer#trl.experimental.ppo.PPOTrainer).

This class includes only the parameters that are specific to PPO training. For a full list of training arguments,
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
> - `learning_rate`: Defaults to `3e-6` instead of `5e-5`.

## PreTrainedModelWrapper[[trl.experimental.ppo.PreTrainedModelWrapper]]

- **pretrained_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to be wrapped.
- **parent_class** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The parent class of the model to be wrapped.
- **supported_args** (`list`) --
  The list of arguments that are supported by the wrapper class.

Wrapper for a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) implemented as a standard PyTorch `torch.nn.Module`.

This class provides a compatibility layer that preserves the key attributes and methods of the original
[PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel), while exposing a uniform interface consistent with PyTorch modules. It enables
seamless integration of pretrained Transformer models into custom training, evaluation, or inference workflows.

Add and load a reward modeling adapter. This method can only be used if the model is a `PeftModel` and if you
have initialized the model with the `reward_modeling_adapter_id` argument, pointing to the id of the reward
modeling adapter. The latest needs also to contain the score head in order to produce the reward.

Computes the reward score for a given input. The method has first to enable the adapter and then compute the
reward score. After that the model disables the reward modeling adapter and enables the default ppo adapter
again.

- **pretrained_model_name_or_path** (`str` or [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The path to the pretrained model or its name.
- ***model_args** (`list`, *optional*) --
  Additional positional arguments passed along to the underlying model's `from_pretrained` method.
- ****kwargs** (`dict`, *optional*) --
  Additional keyword arguments passed along to the underlying model's `from_pretrained` method. We also
  pre-process the kwargs to extract the arguments that are specific to the
  [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) class and the arguments that are specific to trl models. The kwargs
  also support `prepare_model_for_kbit_training` arguments from `peft` library.

Instantiates a new model from a pretrained model from `transformers`. The pretrained model is loaded using the
`from_pretrained` method of the [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) class. The arguments that are specific to the
[PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) class are passed along this method and filtered out from the `kwargs`
argument.

Post initialization method. This method is called after the model is instantiated and loaded from a checkpoint.
It can be used to perform additional operations such as loading the state_dict.

- ***args** (`list`, *optional*) --
  Positional arguments passed along to the underlying model's `push_to_hub` method.
- ****kwargs** (`dict`, *optional*) --
  Keyword arguments passed along to the underlying model's `push_to_hub` method.

Push the pretrained model to the hub. This method is a wrapper around
[push_to_hub](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.push_to_hub). Please refer to the documentation of
[push_to_hub](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.push_to_hub) for more information.

- ***args** (`list`, *optional*) --
  Positional arguments passed along to the underlying model's `save_pretrained` method.
- ****kwargs** (`dict`, *optional*) --
  Keyword arguments passed along to the underlying model's `save_pretrained` method.

Save the pretrained model to a directory. This method is a wrapper around
[save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained). Please refer to the documentation of
[save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained) for more information.

Return the state_dict of the pretrained model.

## AutoModelForCausalLMWithValueHead[[trl.experimental.ppo.AutoModelForCausalLMWithValueHead]]

An autoregressive model with a value head in addition to the language model head. This class inherits from
[experimental.ppo.PreTrainedModelWrapper](/docs/trl/v1.9.2/en/ppo_trainer#trl.experimental.ppo.PreTrainedModelWrapper) and wraps a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) class. The wrapper class
supports classic functions such as `from_pretrained`, `push_to_hub` and `generate`. To call a method of the wrapped
model, simply manipulate the `pretrained_model` attribute of this class.

Class attributes:
- **transformers_parent_class** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) -- The parent class of the wrapped model.
  This
  should be set to `transformers.AutoModelForCausalLM` for this class.
- **supported_args** (`tuple`) -- A tuple of strings that are used to identify the arguments that are supported
  by the `ValueHead` class. Currently, the supported args are:
  - **summary_dropout_prob** (`float`, `optional`, defaults to `None`) -- The dropout probability for the
    `ValueHead` class.
  - **v_head_initializer_range** (`float`, `optional`, defaults to `0.2`) -- The initializer range for the
    `ValueHead` if a specific initialization strategy is selected.
  - **v_head_init_strategy** (`str`, `optional`, defaults to `None`) -- The initialization strategy for the
    `ValueHead`. Currently, the supported strategies are:
    - **`None`** -- Initializes the weights of the `ValueHead` with a random distribution. This is the
      default strategy.
    - **"normal"** -- Initializes the weights of the `ValueHead` with a normal distribution.

- **pretrained_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to wrap. It should be a causal language model such as GPT2. or any model mapped inside the
  `AutoModelForCausalLM` class.
- **kwargs** (`dict`, `optional`) --
  Additional keyword arguments, that are passed to the `ValueHead` class.

Initializes the model.

- **input_ids** (*torch.LongTensor* of shape *(batch_size, sequence_length)*) --
  Indices of input sequence tokens in the vocabulary.
- **past_key_values** (*tuple(tuple(torch.FloatTensor))*, *optional*) --
  Contains pre-computed hidden-states (key and values in the attention blocks) as computed by the model
  (see *past_key_values* input) to speed up sequential decoding.
- **attention_mask** (*torch.FloatTensor* of shape *(batch_size, sequence_length)*, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **return_past_key_values** (bool) -- A flag indicating if the computed hidden-states should be returned.
- **kwargs** (*dict*, *optional*) --
  Additional keyword arguments, that are passed to the wrapped model.

Applies a forward pass to the wrapped model and returns the logits of the value head.

- ***args** (`list`, *optional*) --
  Positional arguments passed to the `generate` method of the wrapped model.
- ****kwargs** (`dict`, *optional*) --
  Keyword arguments passed to the `generate` method of the wrapped model.

A simple wrapper around the `generate` method of the wrapped model. Please refer to the
[`generate`](https://huggingface.co/docs/transformers/internal/generation_utils) method of the wrapped model
for more information about the supported arguments.

- ****kwargs** (`dict`, `optional`) --
  Additional keyword arguments, that are passed to the `ValueHead` class. These arguments can contain
  the `v_head_init_strategy` argument as well as the `v_head_initializer_range` argument.

Initializes the weights of the value head. The default initialization strategy is random. Users can pass a
different initialization strategy by passing the `v_head_init_strategy` argument when calling
`.from_pretrained`. Supported strategies are:
- `normal`: initializes the weights with a normal distribution.

## AutoModelForSeq2SeqLMWithValueHead[[trl.experimental.ppo.AutoModelForSeq2SeqLMWithValueHead]]

- **pretrained_model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to wrap. It should be a causal language model such as GPT2. or any model mapped inside the
  [AutoModelForSeq2SeqLM](https://huggingface.co/docs/transformers/v5.14.1/en/model_doc/auto#transformers.AutoModelForSeq2SeqLM) class.
- **kwargs** --
  Additional keyword arguments passed along to the `ValueHead` class.

A seq2seq model with a value head in addition to the language model head. This class inherits from
[experimental.ppo.PreTrainedModelWrapper](/docs/trl/v1.9.2/en/ppo_trainer#trl.experimental.ppo.PreTrainedModelWrapper) and wraps a [PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel) class. The wrapper class
supports classic functions such as `from_pretrained` and `push_to_hub` and also provides some additional
functionalities such as `generate`.

We call `generate` on the wrapped model.

We initialize the weights of the value head.
