# Reward Functions

This module contains some useful reward functions, primarily intended for use with the [GRPOTrainer](/docs/trl/v1.12.0/en/grpo_trainer#trl.GRPOTrainer) and [RLOOTrainer](/docs/trl/v1.12.0/en/rloo_trainer#trl.RLOOTrainer).

## accuracy_reward[[trl.rewards.accuracy_reward]]

#### trl.rewards.accuracy_reward[[trl.rewards.accuracy_reward]]

```python
trl.rewards.accuracy_reward(completions: list, solution: list, log_extra: collections.abc.Callable[[str, list], None] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/accuracy_rewards.py#L28)

**Parameters:**

completions (`list[list[dict[str, str]]]`) : List of completions to be evaluated. Each completion must be a list of one message, i.e. a dictionary containing the key `"content"` with the value being the text of the completion.

solution (`list[str]`) : List of the raw-text solutions to the questions/problems/prompts.

log_extra (`callable`, *optional*) : Callable to log extra columns to the completions table, provided automatically by the trainer. Defaults to `None` to allow calling the function directly outside of a trainer (e.g., for testing).

- ****kwargs** : Additional keyword arguments. This function does not use them, but they are required in the function signature to ensure compatibility with trainers like [GRPOTrainer](/docs/trl/v1.12.0/en/grpo_trainer#trl.GRPOTrainer).

Reward function that checks if the completion matches the ground truth.
- If both gold and prediction are parseable → use math verification.
- If gold is not parseable → return `None` to skip the example.

Example:
```python
>>> from trl.rewards import accuracy_reward

>>> solutions = [r"\frac{1}{3}", r"\frac{1}{3}"]
>>> completions = [
...     [{"role": "assistant", "content": r"My answer is \boxed{\frac{1}{3}}"}],
...     [{"role": "assistant", "content": r"My answer is \boxed{\frac{1}{2}}"}],
... ]
>>> accuracy_reward(completions, solutions)
[1.0, 0.0]
```

## reasoning_accuracy_reward[[trl.rewards.reasoning_accuracy_reward]]

#### trl.rewards.reasoning_accuracy_reward[[trl.rewards.reasoning_accuracy_reward]]

```python
trl.rewards.reasoning_accuracy_reward(completions: list, solution: list, reasoning_delimiters: list[str] | None = None, log_extra: collections.abc.Callable[[str, list], None] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/accuracy_rewards.py#L223)

**Parameters:**

completions (`list[list[dict[str, str]]]`) : List of completions to be evaluated. Each completion must be a list of one message, i.e. a dictionary containing the key `"content"` with the value being the text of the completion.

solution (`list[str]`) : List of the raw-text solutions to the questions/problems/prompts.

reasoning_delimiters (`list[str]]`, *optional*) : List of strings indicating where the reasoning content ends. The final answer is assumed to be after the last occurrence of any of these delimiters. If `None`, defaults to `["</think>"]`.

log_extra (`callable`, *optional*) : Callable to log extra columns to the completions table, provided automatically by the trainer. Defaults to `None` to allow calling the function directly outside of a trainer (e.g., for testing).

- ****kwargs** : Additional keyword arguments. This function does not use them, but they are required in the function signature to ensure compatibility with trainers like [GRPOTrainer](/docs/trl/v1.12.0/en/grpo_trainer#trl.GRPOTrainer).

Reward function that removes the reasoning content and checks if the final answer matches the ground truth.
- If both gold and prediction are parseable → use math verification.
- If gold is not parseable → return `None` to skip the example.

Example:
```python
>>> from trl.rewards import reasoning_accuracy_reward

>>> reasoning_delimiters = ["</think>"]
>>> solutions = [r"\frac{1}{3}", r"\frac{1}{3}", r"\frac{1}{3}"]
>>> completions = [
...     [
...         {
...             "role": "assistant",
...             "content": r"<think> Reasoning content </think> The final answer is \boxed{\frac{1}{3}}",
...         }
...     ],
...     [
...         {
...             "role": "assistant",
...             "content": r"<think> Reasoning content </think> The final answer is \boxed{\frac{1}{2}}",
...         }
...     ],
...     [
...         {
...             "role": "assistant",
...             "content": r"<think> Reasoning content with partial answers \boxed{\frac{1}{3}} but no final answer",
...         }
...     ],
... ]
>>> reasoning_accuracy_reward(completions, solutions, reasoning_delimiters=reasoning_delimiters)
[1.0, 0.0, 0.0]
```

## get_cosine_scaled_reward[[trl.rewards.get_cosine_scaled_reward]]

#### trl.rewards.get_cosine_scaled_reward[[trl.rewards.get_cosine_scaled_reward]]

```python
trl.rewards.get_cosine_scaled_reward(max_len: int, min_value_wrong: float = -1.0, max_value_wrong: float = -0.5, min_value_correct: float = 0.5, max_value_correct: float = 1.0)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/accuracy_rewards.py#L118)

**Parameters:**

max_len (`int`) : Maximum completion length (in tokens) used to normalize the cosine schedule, $L_{\max}$.

min_value_wrong (`float`, *optional*, defaults to `-1.0`) : Reward of a wrong completion at the shortest length.

max_value_wrong (`float`, *optional*, defaults to `-0.5`) : Reward of a wrong completion at the longest length.

min_value_correct (`float`, *optional*, defaults to `0.5`) : Reward of a correct completion at the longest length.

max_value_correct (`float`, *optional*, defaults to `1.0`) : Reward of a correct completion at the shortest length.

**Returns:** `Callable`

A reward function that takes completions, their solutions and token ids, and returns a list of rewards
(`None` for examples with an unparseable gold solution).

Reward function that scales a correctness reward by the completion length following a cosine schedule, to favor
concise reasoning. Reference: Appendix C.1 of the "Demystifying Long Chain-of-Thought Reasoning" paper
(https://huggingface.co/papers/2502.03373).

Correctness is determined by math verification (as in [accuracy_reward()](/docs/trl/v1.12.0/en/rewards#trl.rewards.accuracy_reward)), and the length is the number
of completion tokens. The reward interpolates along a cosine schedule between a short-completion and a
long-completion bound:

$$
R_{\text{cosine}}(y) = v_{\min} + \frac{1}{2}(v_{\max} - v_{\min})\left(1 + \cos\left(\frac{|y|}{L_{\max}}\pi\right)\right)
$$

For a **correct** completion, $(v_{\min}, v_{\max}) = (\texttt{min\_value\_correct}, \texttt{max\_value\_correct})$,
so a shorter completion is rewarded more. For a **wrong** completion, the bounds are swapped to
$(v_{\min}, v_{\max}) = (\texttt{max\_value\_wrong}, \texttt{min\_value\_wrong})$, so a shorter completion is
penalized more (a longer wrong completion is penalized less, preserving exploration). When the gold solution is not
parseable, the example is skipped (reward `None`), as in [accuracy_reward()](/docs/trl/v1.12.0/en/rewards#trl.rewards.accuracy_reward).

Example:
```python
>>> from trl.rewards import get_cosine_scaled_reward

>>> cosine_scaled_reward = get_cosine_scaled_reward(max_len=100)
>>> completions = [[{"content": r"\boxed{\frac{1}{3}}"}], [{"content": r"\boxed{\frac{1}{2}}"}]]
>>> solution = [r"\frac{1}{3}", r"\frac{1}{3}"]
>>> completion_ids = [[1] * 50, [1] * 50]  # both completions are 50 tokens, half of max_len
>>> cosine_scaled_reward(completions, solution, completion_ids)
[0.75, -0.75]
```

## think_format_reward[[trl.rewards.think_format_reward]]

#### trl.rewards.think_format_reward[[trl.rewards.think_format_reward]]

```python
trl.rewards.think_format_reward(completions: list, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/format_rewards.py#L18)

**Parameters:**

completions (`list[list[dict[str, str]]]`) : List of completions to be evaluated. Each completion must be a list of one message, i.e. a dictionary containing the key `"content"` with the value being the text of the completion.

- ****kwargs** : Additional keyword arguments. This function does not use them, but they are required in the function signature to ensure compatibility with trainers like [GRPOTrainer](/docs/trl/v1.12.0/en/grpo_trainer#trl.GRPOTrainer).

**Returns:** `list[float]`

A list of rewards, where each reward is 1.0 if the completion matches the expected format, otherwise 0.0.

Reward function that checks if the reasoning process is enclosed within `"<think>"` and `"</think>"` tags. The
function returns a reward of 1.0 if the format is correct, otherwise 0.0.

Example:
```python
>>> from trl.rewards import think_format_reward

>>> completions = [
...     [{"content": "<think>\nThis is my reasoning.\n</think>\nThis is my answer."}],
...     [{"content": "<think>\nThis is my reasoning.\nThis is my answer."}],
... ]
>>> think_format_reward(completions)
[1.0, 0.0]
```

## get_repetition_penalty_reward[[trl.rewards.get_repetition_penalty_reward]]

#### trl.rewards.get_repetition_penalty_reward[[trl.rewards.get_repetition_penalty_reward]]

```python
trl.rewards.get_repetition_penalty_reward(ngram_size: int = 3, max_penalty: float = -1.0)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/other_rewards.py#L18)

**Parameters:**

ngram_size (`int`, *optional*, defaults to `3`) : Size of the token n-grams to consider.

max_penalty (`float`, *optional*, defaults to `-1.0`) : Most negative penalty, applied to a fully repetitive completion. Must be non-positive.

**Returns:** `Callable`

A reward function that takes a list of completion token ids and returns a list of penalties (each in
`[max_penalty, 0.0]`).

Reward function that penalizes repeated n-grams in a completion, used to discourage degenerate, repetitive text
(a common failure mode and reward-hacking strategy when length- or format-shaping rewards are used). Reference:
Appendix C.2 of the "Demystifying Long Chain-of-Thought Reasoning" paper (https://huggingface.co/papers/2502.03373).

The penalty is proportional to the fraction of repeated n-grams in the completion:

$$
R_{\text{repetition}}(y) = \left(1 - \frac{\#\,\text{unique } n\text{-grams}}{\#\,\text{total } n\text{-grams}}\right) \times p
$$

where $p$ is `max_penalty`. A completion with no repeated n-gram gets a reward of `0.0`, while a fully repetitive
one approaches `max_penalty`. The n-grams are computed over the completion token ids (the paper applies the penalty
to repeated tokens), so the reward is tokenizer-defined and language-agnostic. Completions with fewer than
`ngram_size` tokens get a reward of `0.0`.

Example:
```python
>>> from trl.rewards import get_repetition_penalty_reward

>>> repetition_penalty = get_repetition_penalty_reward(ngram_size=2, max_penalty=-1.0)
>>> completion_ids = [[1, 2, 3, 4], [5, 5, 5, 5, 5]]
>>> repetition_penalty(completion_ids)
[0.0, -0.75]
```

## get_soft_overlong_punishment[[trl.rewards.get_soft_overlong_punishment]]

#### trl.rewards.get_soft_overlong_punishment[[trl.rewards.get_soft_overlong_punishment]]

```python
trl.rewards.get_soft_overlong_punishment(max_completion_len: int, soft_punish_cache: int)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/rewards/other_rewards.py#L83)

**Parameters:**

max_completion_len (`int`) : Maximum length of the completion,  \( L_{\max} \).

soft_punish_cache (`int`) : Minimum length of the completion,  \( L_{\text{cache}} \). If set to `0`, no minimum length is applied.

Reward function that penalizes overlong completions. It is used to penalize overlong completions, but not to reward
shorter completions. Reference: Eq. (13) from the DAPO paper (https://huggingface.co/papers/2503.14476)

$$
R_{\text{length}}(y) = \begin{cases}
0, & |y| \le L_{\max} - L_{\text{cache}} \\
\dfrac{(L_{\max} - L_{\text{cache}}) - |y|}{L_{\text{cache}}}, & L_{\max} - L_{\text{cache}} < |y| \le L_{\max} \\
-1, & L_{\max} < |y|
\end{cases}
$$

Example:
```python
>>> from trl.rewards import get_soft_overlong_punishment

>>> soft_overlong_punishment = get_soft_overlong_punishment(max_completion_len=100, soft_punish_cache=20)
>>> completion_ids = [[1] * 90]  # simulating a completion with 90 tokens. 90 is between 80 and 100.
>>> soft_overlong_punishment(completion_ids)
>>> [-0.5]
```
