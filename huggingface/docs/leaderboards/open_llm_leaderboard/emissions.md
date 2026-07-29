# CO2 calculation

## Function for CO2 calculation

To calculate `CO₂ Emissions for Evaluation (kg)` value, we use the following function. You can try to reproduce it yourself:

```python
def calculate_co2_emissions(total_evaluation_time_seconds: float | None) -> float:
    if total_evaluation_time_seconds is None or total_evaluation_time_seconds  Each result file have a `total_evaluation_time_seconds` field.

2. Assumes 8 NVIDIA H100 SXM GPUs with a combined power usage of 5.6 kilowatts (kW), based on each GPU’s maximum 0.7 kW consumption ([source](https://resources.nvidia.com/en-us-tensor-core/nvidia-tensor-core-gpu-datasheet)).

3. Uses an average of 269.8 grams of CO₂ per kilowatt-hour (g CO₂/kWh) for electricity in Virginia, based on U.S. Energy Information Administration data ([source](https://www.eia.gov/electricity/state/virginia/)).

4. Converts the evaluation time from seconds to hours, then calculates total energy usage in kWh.

5. Calculates emissions in grams by multiplying energy use (kWh) by the carbon intensity.

6. Finally, divides the total grams by 1,000 to convert to kilograms.

### FAQ
https://huggingface.co/docs/leaderboards/open_llm_leaderboard/faq.md

# FAQ

## Submissions

**Q: Do you keep track of who submits models?** 

A: Yes, we store information about which user submitted each model in the requests files here. This helps us prevent spam and encourages responsible submissions. Users are accountable for their submissions, as the community can identify who submitted each model.

**Q: Can I submit a model that requires `trust_remote_code=True`?**

A: We only accept models that have been integrated into a stable version of the `transformers` library to ensure the safety and stability of code executed on our cluster.

**Q: Are models of type X supported?**

A: For now, submission is limited to models that are included in a stable version of the transformers library.

**Q: Can I evaluate my model with a chat template?**

A: Sure! When submitting a model, you can choose whether to evaluate it using a chat template, which activates automatically for chat models.

**Q: How can I track the status of my model submission?**

A: You can monitor your model's status by checking the [Request file here](https://huggingface.co/datasets/open-llm-leaderboard/requests) or viewing the queues above the submit form.

**Q: What happens if my model disappears from all queues?**

A: A model’s disappearance typically indicates a failure. You can find your model in [Requests dataset here](https://huggingface.co/datasets/open-llm-leaderboard/requests) and check its status.

**Q: What causes an evaluation failure?**

A: Failures often stem from submission issues such as corrupted files or configuration errors. Please review the steps in About tab before submitting. Occasionally, failures are due to hardware or connectivity issues on our end.

**Q: How do I report an evaluation failure?**

A: Please create an issue in the [Community section]([https://huggingface.co/spaces/open-llm-leaderboard/open_llm_leaderboard/discussions), linking your model’s request file for further investigation. If the error is on our side, we will relaunch your model promptly.

*Do not re-upload your model under a different name as it will not resolve the issue.*

## Results

**Q: What information is available about my model's evaluation results?**

A: For each model, you can access:

- **Request File**: Status of the evaluation.
- **Contents Dataset:** A full dataset that contains information about all evaluated models. It's available [here](https://huggingface.co/datasets/open-llm-leaderboard/contents).
- **Details Dataset**: Comprehensive breakdown of scores and task examples. You can see all the Details datasets [here](https://huggingface.co/open-llm-leaderboard).

**Q: Why do some models appear multiple times in the leaderboard?**

A: Models may appear multiple times due to submissions under different commits or precision settings, like `float16` and `4bit`. You can check this by clicking on the `Precision` button under “column visibility” section on the main page. For evaluation, precision helps to assess the impact of quantization. 

*Duplicates with identical precision and commit should be reported.*

**Q: What is model flagging?**

A: Flagging helps report models that have unfair performance on the leaderboard. For example,   models that were trained on the evaluation data, models that are copies of other models not attributed properly, etc. 

*If your model is flagged incorrectly, you can open a discussion [here](https://huggingface.co/spaces/open-llm-leaderboard/open_llm_leaderboard/discussions) and defend your case.*

## Searching for a model

**Q: How do I search for models in the leaderboard?**

A: The search bar provides powerful filtering capabilities with several advanced features:

**Multiple Term Search**
- Combine Searches: Use semicolons (;) to combine multiple independent search terms.
- Stacked Results: Each term after the semicolon adds results to the previous search, creating a union of results rather than filtering by intersection.

Example: `llama; 7b` will find models containing "llama" OR models containing "7b."

**Special Field Search**

Use the `@` prefix to target specific fields:
- `@architecture:` - Search by model architecture.
- `@license:` - Filter by license type.
- `@precision:` - Filter by model precision.

Example: `@architecture:llama @license:apache` will find Llama models with an Apache license.

**Regex Support**
- Advanced Pattern Matching: Supports regular expressions for flexible search criteria.
- Automatic Detection: Regex mode is activated automatically when special regex characters are used.

Example: `llama-2-(7|13|70)b` matches `llama-2-7b`, `llama-2-13b`, and `llama-2-70b`.

**Combined Search**
- Combine and stack all features for precise results:

Example: `meta @architecture:llama; 7b @license:apache` will find:
- Models containing "meta" AND having the Llama architecture, OR
- Models containing "7b" AND having an Apache license.

**Real-Time Results**
- Dynamic Updates: The search is performed in real-time with debouncing for smooth performance.
- Highlighting: Results are visually emphasized in the table for easy identification.

## Editing submissions

**Q: How can I update or rename my submitted model?**

A: To update, open an issue with your model's exact name for removal from the leaderboard before resubmitting with the new commit hash. For renaming, check [community resources](https://huggingface.co/spaces/open-llm-leaderboard/open_llm_leaderboard/discussions/174) page and use @Weyaxi's tool to request changes, then link the pull request in a discussion for approval.

## Additional information

**Q: What does “Only Official Providers” button do?**

A: This button filters and displays models from a curated list of trusted and high-quality model providers. We have introduced it to help users easily identify and choose top-tier models. The current set of trusted authors includes well-known names such as EleutherAI, CohereForAI, MistralAI and many others.
The dataset is available [here](https://huggingface.co/datasets/open-llm-leaderboard/official-providers).

**Q: How can I view raw scores for each evaluation?**

A: The Leaderboard displays normalized scores by default to provide a fair comparison. Normalization adjusts scores so that the lower bound corresponds to the score of a random baseline, ensuring a fairer average. To view the non-normalized values, go to "table options", "Score Display", and click "Raw".

**Q: How are model categories differentiated?**

A: Categories are defined to reflect the specific training stages and methodologies applied to each model, ensuring comparisons are both fair and meaningful. Here's a breakdown of each category:

- **Pretrained Models:** These foundational models are initially trained on large datasets without task-specific tuning, serving as a versatile base for further development.
- **Continuously Pretrained Models:** These undergo additional training beyond initial pretraining to enhance their capabilities, often using more specialized data.
- **Fine-Tuned Models:** Specifically adjusted on targeted datasets, these models are optimized for particular tasks, improving performance in those areas.
- **Chat Models:** Tailored for interactive applications like chatbots, these models are trained to handle conversational contexts effectively.
- **Merge Models:** Combining multiple models or methods, these can show superior test results but do not always apply for real-world situations.

**Q: What are the leaderboard's intended uses?**

A: The leaderboard is ideal for:

1. Viewing rankings and scores of open pretrained models.
2. Experimenting with various fine-tuning and quantization techniques.
3. Comparing the performance of specific models within their categories.

**Q: Why don't you have closed-source models?**

A: The leaderboard focuses on open-source models to ensure transparency, reproducibility, and fairness. Closed-source models can change their APIs unpredictably, making it difficult to guarantee consistent and accurate scoring. Additionally, we rerun all evaluations on our cluster to maintain a uniform testing environment, which isn't possible with closed-source models.

**Q: I have another problem, help!**

A: Please, open an issue in the discussion tab, and we'll do our best to help you in a timely manner :

### About
https://huggingface.co/docs/leaderboards/open_llm_leaderboard/about.md

# About

With the plethora of large language models (LLMs) and chatbots being released week upon week, often with grandiose claims of their performance, it can be hard to filter out the genuine progress that is being made by the open-source community and which model is the current state of the art.

We wrote a release blog [here](https://huggingface.co/spaces/open-llm-leaderboard/blog) to explain why we introduced this leaderboard!

## Tasks

📈 We evaluate models on 6 key benchmarks using the [Eleuther AI Language Model Evaluation Harness](https://github.com/EleutherAI/lm-evaluation-harness) , a unified framework to test generative language models on a large number of different evaluation tasks.

- **IFEval** ([https://arxiv.org/abs/2311.07911](https://arxiv.org/abs/2311.07911)) – IFEval is a dataset designed to test a model's ability to follow explicit instructions, such as "include keyword x" or "use format y." The focus is on the model's adherence to formatting instructions rather than the content generated, allowing for the use of strict and rigorous metrics.
- **BBH (Big Bench Hard)** ([https://arxiv.org/abs/2210.09261](https://arxiv.org/abs/2210.09261)) – A subset of 23 challenging tasks from the BigBench dataset to evaluate language models. The tasks use objective metrics, are highly difficult, and have sufficient sample sizes for statistical significance. They include multistep arithmetic, algorithmic reasoning (e.g., boolean expressions, SVG shapes), language understanding (e.g., sarcasm detection, name disambiguation), and world knowledge. BBH performance correlates well with human preferences, providing valuable insights into model capabilities.
- **MATH** ([https://arxiv.org/abs/2103.03874](https://arxiv.org/abs/2103.03874)) – MATH is a compilation of high-school level competition problems gathered from several sources, formatted consistently using Latex for equations and Asymptote for figures. Generations must fit a very specific output format. We keep only level 5 MATH questions and call it MATH Lvl 5.
- **GPQA (Graduate-Level Google-Proof Q&A Benchmark)** ([https://arxiv.org/abs/2311.12022](https://arxiv.org/abs/2311.12022)) – GPQA is a highly challenging knowledge dataset with questions crafted by PhD-level domain experts in fields like biology, physics, and chemistry. These questions are designed to be difficult for laypersons but relatively easy for experts. The dataset has undergone multiple rounds of validation to ensure both difficulty and factual accuracy. Access to GPQA is restricted through gating mechanisms to minimize the risk of data contamination. Consequently, we do not provide plain text examples from this dataset, as requested by the authors.
- **MuSR (Multistep Soft Reasoning)** ([https://arxiv.org/abs/2310.16049](https://arxiv.org/abs/2310.16049)) – MuSR is a new dataset consisting of algorithmically generated complex problems, each around 1,000 words in length. The problems include murder mysteries, object placement questions, and team allocation optimizations. Solving these problems requires models to integrate reasoning with long-range context parsing. Few models achieve better than random performance on this dataset.
- **MMLU-PRO (Massive Multitask Language Understanding - Professional)** ([https://arxiv.org/abs/2406.01574](https://arxiv.org/abs/2406.01574)) – MMLU-Pro is a refined version of the MMLU dataset, which has been a standard for multiple-choice knowledge assessment. Recent research identified issues with the original MMLU, such as noisy data (some unanswerable questions) and decreasing difficulty due to advances in model capabilities and increased data contamination. MMLU-Pro addresses these issues by presenting models with 10 choices instead of 4, requiring reasoning on more questions, and undergoing expert review to reduce noise. As a result, MMLU-Pro is of higher quality and currently more challenging than the original.

For all these evaluations, a higher score is a better score. We chose these benchmarks as they test a variety of reasoning and general knowledge across a wide variety of fields in 0-shot and few-shot settings.

## Model Types

- 🟢 **Pretrained Model:** New, base models trained on a given text corpora using masked modeling.
- 🟩 **Continuously Pretrained Model:** New, base models continuously trained on further corpora (which may include IFT/chat data) using masked modeling.
- 🔶 **Fine-Tuned on Domain-Specific Datasets Model:** Pretrained models fine-tuned on more data.
- 💬 **Chat Models (RLHF, DPO, IFT, ...):** Chat-like fine-tunes using IFT (datasets of task instruction), RLHF, DPO (changing the model loss with an added policy), etc.
- 🤝 **Base Merges and Moerges Model:** Merges or MoErges, models which have been merged or fused without additional fine-tuning.

## Results

You can find:
- Detailed numerical results in the [`results` Hugging Face dataset](https://huggingface.co/datasets/open-llm-leaderboard/results/).
- Details on the input/outputs for the models in the `details` of each model, which you can access by clicking the 📄 emoji after the model name.
- Community queries and running status in the [`requests` Hugging Face dataset](https://huggingface.co/datasets/open-llm-leaderboard/requests).

If a model's name contains "Flagged", this indicates it has been flagged by the community, and should probably be ignored! Clicking the link will redirect you to the discussion about the model.

## Reproducibility

To reproduce our results, you can use our fork of [lm_eval](https://github.com/huggingface/lm-evaluation-harness/tree/main), as our PRs are not all merged in it at the moment.
```
git clone git@github.com:huggingface/lm-evaluation-harness.git
cd lm-evaluation-harness
git checkout main
pip install -e .
lm-eval --model_args="pretrained=,revision=,dtype=" --tasks=leaderboard  --batch_size=auto --output_path=
```
**Attention:** For instruction models add the `--apply_chat_template` and `fewshot_as_multiturn` option.

**Note:** You can expect results to vary slightly for different batch sizes because of padding.

### **Task Evaluations and Parameters**

**IFEval**:

- Task: "IFEval"
- Measure: Strict Accuracy at Instance and Prompt Levels (`inst_level_strict_acc,none` and `prompt_level_strict_acc,none`)
- Shots: 0-shot for both Instance-Level Strict Accuracy and Prompt-Level Strict Accuracy
- num_choices: 0 for both Strict Accuracy at Instance and Prompt Levels.
  
**Big Bench Hard (BBH)**:

- Overview Task: "BBH"
- Shots: 3-shot for each subtask
- Measure: Normalized Accuracy across all subtasks (`acc_norm,none`)
- List of subtasks with `num_choices`:
    - BBH Sports Understanding, num_choices=2
    - BBH Tracking Shuffled Objects (Three Objects), num_choices=3
    - BBH Navigate, num_choices=2
    - BBH Snarks, num_choices=2
    - BBH Date Understanding, num_choices=6
    - BBH Reasoning about Colored Objects, num_choices=18
    - BBH Object Counting, num_choices=19 (should be 18 but we added a “0” choice)
    - BBH Logical Deduction (Seven Objects), num_choices=7
    - BBH Geometric Shapes, num_choices=11
    - BBH Web of Lies, num_choices=2
    - BBH Movie Recommendation, num_choices=6
    - BBH Logical Deduction (Five Objects), num_choices=5
    - BBH Salient Translation Error Detection, num_choices=6
    - BBH Disambiguation QA, num_choices=3
    - BBH Temporal Sequences, num_choices=4
    - BBH Hyperbaton, num_choices=2
    - BBH Logical Deduction (Three Objects), num_choices=3
    - BBH Causal Judgement, num_choices=2
    - BBH Formal Fallacies, num_choices=2
    - BBH Tracking Shuffled Objects (Seven Objects), num_choices=7
    - BBH Ruin Names, num_choices=6
    - BBH Penguins in a Table, num_choices=5
    - BBH Boolean Expressions, num_choices=2
    - BBH Tracking Shuffled Objects (Five Objects), num_choices=5

**Math Challenges**:

- Task: "Math Level 5"
- Measure: Exact Match (`exact_match,none`)
- Shots: 4-shot
- num_choices: 0

**Generalized Purpose Question Answering (GPQA)**:

- Task: "GPQA"
- Measure: Normalized Accuracy (`acc_norm,none`)
- Shots: 0-shot
- num_choices: 4

**MuSR**:

- Overview Task: "MuSR"
- Measure: Normalized Accuracy across all subtasks (`acc_norm,none`)
- MuSR Murder Mysteries: 0-shot, num_choices: 2
- MuSR Object Placement: 0-shot, num_choices: 5
- MuSR Team Allocation: 0-shot, num_choices: 3

**MMLU-PRO**:

- Task: "MMLU-PRO"
- Measure: Accuracy (`acc,none`)
- Shots: 5-shot
- num_choices: 10

### Scores Normalization
https://huggingface.co/docs/leaderboards/open_llm_leaderboard/normalization.md

# Scores Normalization

This page explains how scores are normalized on the Open LLM Leaderboard for the six presented benchmarks. We can categorize all tasks into those with subtasks, those without subtasks, and generative evaluation.

[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/drive/1-aPrFJjwdifhVLxzJcsYXeebqNi_5vaw?usp=sharing)

**Note:** Click the button above to explore the scores normalization process in an interactive notebook (make a copy to edit).

## What is Normalization?
Normalization is the process of adjusting values measured on different scales to a common scale, making it possible to compare scores across different tasks. For the Open LLM Leaderboard, we normalize scores to:

1. Account for the varying difficulty and random guess baselines of different tasks.
2. Provide a consistent scale (0-100) for all tasks, enabling fair comparisons.
3. Ensure that improvements over random guessing are appropriately reflected in the scores.

## General Normalization Process

The basic normalization process involves two steps:
1. Subtracting the random baseline score (lower bound).
2. Scaling the result to a range of 0-100.

We use the following normalization function:

```python
def normalize_within_range(value, lower_bound, higher_bound):
    return (value - lower_bound) / (higher_bound - lower_bound)
```

## Normalizing Tasks without Subtasks
For tasks without subtasks (e.g., GPQA, MMLU-PRO), the normalization process is straightforward:
- Determine the lower bound (random guess baseline).
- Apply the normalization function.
- Scale to a percentage.

### Example: Normalizing GPQA Scores
GPQA has 4 `num_choices`, so the lower bound is 0.25 (1/`num_choices` = 1/4 = 0.25).

```python
raw_score = 0.6  # Example raw score
lower_bound = 0.25
higher_bound = 1.0

if raw_score < lower_bound:
    normalized_score = 0
else:
    normalized_score = normalize_within_range(raw_score, lower_bound, higher_bound) * 100

print(f"Normalized GPQA score: {normalized_score:.2f}")
# Output: Normalized GPQA score: 46.67
```

## Normalizing Tasks with Subtasks
For tasks with subtasks (e.g., MUSR, BBH), we follow these steps:
- Calculate the lower bound for each subtask.
- Normalize each subtask score.
- Average the normalized subtask scores.

### Example: Normalizing MUSR Scores

MUSR has three subtasks with different numbers of choices:
- MUSR murder mysteries, num_choices = 2, lower_bound = 0.5
- MUSR object placement, num_choices = 5, lower_bound = 0.2
- MUSR team allocation, num_choices = 3, lower_bound = 0.33

```python
subtasks = [
    {"name": "murder_mysteries", "raw_score": 0.7, "lower_bound": 0.5},
    {"name": "object_placement", "raw_score": 0.4, "lower_bound": 0.2},
    {"name": "team_allocation", "raw_score": 0.6, "lower_bound": 0.333}
]

normalized_scores = []

for subtask in subtasks:
    if subtask["raw_score"] < subtask["lower_bound"]:
        normalized_score = 0
    else:
        normalized_score = normalize_within_range(
            subtask["raw_score"], 
            subtask["lower_bound"], 
            1.0
        ) * 100
    normalized_scores.append(normalized_score)
    print(f"{subtask['name']} normalized score: {normalized_score:.2f}")

overall_normalized_score = sum(normalized_scores) / len(normalized_scores)
print(f"Overall normalized MUSR score: {overall_normalized_score:.2f}")

# Output:
# murder_mysteries normalized score: 40.00
# object_placement normalized score: 25.00
# team_allocation normalized score: 40.00
# Overall normalized MUSR score: 35.00
```

## Generative Evaluations
Generative evaluations like MATH and IFEval require a different approach:
1. **MATH:** Uses exact match accuracy. The lower bound is effectively 0, as random guessing is unlikely to produce a correct answer.
2. **IFEval:**
    - For instance-level evaluation (`ifeval_inst`), we use strict accuracy.
    - For prompt-level evaluation (`ifeval_prompt`), we also use strict accuracy.
    - The lower bound for both is 0, as random generation is unlikely to produce correct answers.

This approach ensures that even for generative tasks, we can provide normalized scores that are comparable across different evaluations.

## Further Information
For more detailed information and examples, please refer to our [blog post](https://huggingface.co/spaces/open-llm-leaderboard/blog) on scores normalization.

If you have any questions or need clarification, please start a new discussion on [the Leaderboard page](https://huggingface.co/spaces/open-llm-leaderboard/open_llm_leaderboard/discussions).
