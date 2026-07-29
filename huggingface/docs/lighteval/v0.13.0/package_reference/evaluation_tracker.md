# EvaluationTracker[[lighteval.logging.evaluation_tracker.EvaluationTracker]]

#### lighteval.logging.evaluation_tracker.EvaluationTracker[[lighteval.logging.evaluation_tracker.EvaluationTracker]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/evaluation_tracker.py#L95)

Tracks and manages evaluation results, metrics, and logging for model evaluations.

The EvaluationTracker coordinates multiple specialized loggers to track different aspects of model evaluation:

- Details Logger (DetailsLogger): Records per-sample evaluation details and predictions
- Metrics Logger (MetricsLogger): Tracks aggregate evaluation metrics and scores
- Versions Logger (VersionsLogger): Records task and dataset versions
- General Config Logger (GeneralConfigLogger): Stores overall evaluation configuration
- Task Config Logger (TaskConfigLogger): Maintains per-task configuration details

The tracker can save results locally and optionally push them to:
- Hugging Face Hub as datasets
- TensorBoard for visualization
- Trackio or Weights & Biases for experiment tracking

Example:
```python
tracker = EvaluationTracker(
    output_dir="./eval_results",
    push_to_hub=True,
    hub_results_org="my-org",
    save_details=True
)

# Log evaluation results
tracker.metrics_logger.add_metric("accuracy", 0.85)
tracker.details_logger.add_detail(task_name="qa", prediction="Paris")

# Save all results
tracker.save()
```

generate_final_dictlighteval.logging.evaluation_tracker.EvaluationTracker.generate_final_dicthttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/evaluation_tracker.py#L363[]dictDictionary containing all experiment information including config, results, versions, and summaries
Aggregates and returns all the logger's experiment information in a dictionary.

This function should be used to gather and display said information at the end of an evaluation run.

**Parameters:**

output_dir (str) : Local directory to save evaluation results and logs

results_path_template (str, optional) : Template for results directory structure. Example: "{output_dir}/results/{org}_{model}"

save_details (bool, defaults to True) : Whether to save detailed evaluation records

push_to_hub (bool, defaults to False) : Whether to push results to HF Hub

push_to_tensorboard (bool, defaults to False) : Whether to push metrics to TensorBoard

hub_results_org (str, optional) : HF Hub organization to push results to

tensorboard_metric_prefix (str, defaults to "eval") : Prefix for TensorBoard metrics

public (bool, defaults to False) : Whether to make Hub datasets public

nanotron_run_info (GeneralArgs, optional) : Nanotron model run information

use_wandb (bool, defaults to False) : Whether to log to Weights & Biases or Trackio if available

**Returns:**

`dict`

Dictionary containing all experiment information including config, results, versions, and summaries
#### push_to_hub[[lighteval.logging.evaluation_tracker.EvaluationTracker.push_to_hub]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/evaluation_tracker.py#L387)

Pushes the experiment details (all the model predictions for every step) to the hub.
#### recreate_metadata_card[[lighteval.logging.evaluation_tracker.EvaluationTracker.recreate_metadata_card]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/evaluation_tracker.py#L454)

Fully updates the details repository metadata card for the currently evaluated model

**Parameters:**

repo_id (str) : Details dataset repository path on the hub (`org/dataset`)
#### save[[lighteval.logging.evaluation_tracker.EvaluationTracker.save]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/evaluation_tracker.py#L247)

Saves the experiment information and results to files, and to the hub if requested.
