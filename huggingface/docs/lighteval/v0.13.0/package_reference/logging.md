# Logging

## EvaluationTracker[[lighteval.logging.evaluation_tracker.EvaluationTracker]]
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

## GeneralConfigLogger[[lighteval.logging.info_loggers.GeneralConfigLogger]]
#### lighteval.logging.info_loggers.GeneralConfigLogger[[lighteval.logging.info_loggers.GeneralConfigLogger]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L48)

Tracks general configuration and runtime information for model evaluations.

This logger captures key configuration parameters, model details, and timing information
to ensure reproducibility and provide insights into the evaluation process.

log_args_infolighteval.logging.info_loggers.GeneralConfigLogger.log_args_infohttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L106[{"name": "num_fewshot_seeds", "val": ": int"}, {"name": "max_samples", "val": ": int | None"}, {"name": "job_id", "val": ": str"}]- **num_fewshot_seeds** (int) -- number of few-shot seeds.
- **max_samples** (int | None) -- maximum number of samples, if None, use all the samples available.
- **job_id** (str) -- job ID, used to retrieve logs.0
Logs the information about the arguments passed to the method.

**Parameters:**

lighteval_sha (str) : Git commit SHA of lighteval used for evaluation, enabling exact version reproducibility. Set to "?" if not in a git repository. 

num_fewshot_seeds (int) : Number of random seeds used for few-shot example sampling. - If  1: Multiple evaluations with different few-shot samplings (HELM-style) 

max_samples (int, optional) : Maximum number of samples to evaluate per task. Only used for debugging - truncates each task's dataset. 

job_id (int, optional) : Slurm job ID if running on a cluster. Used to cross-reference with scheduler logs. 

start_time (float) : Unix timestamp when evaluation started. Automatically set during logger initialization. 

end_time (float) : Unix timestamp when evaluation completed. Set by calling log_end_time(). 

total_evaluation_time_secondes (str) : Total runtime in seconds. Calculated as end_time - start_time. 

model_config (ModelConfig) : Complete model configuration settings. Contains model architecture, tokenizer, and generation parameters. 

model_name (str) : Name identifier for the evaluated model. Extracted from model_config.
#### log_model_info[[lighteval.logging.info_loggers.GeneralConfigLogger.log_model_info]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L123)

Logs the model information.

**Parameters:**

model_config : the model config used to initialize the model.

## DetailsLogger[[lighteval.logging.info_loggers.DetailsLogger]]
#### lighteval.logging.info_loggers.DetailsLogger[[lighteval.logging.info_loggers.DetailsLogger]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L138)

Logger for the experiment details.

Stores and logs experiment information both at the task and at the sample level.

aggregatelighteval.logging.info_loggers.DetailsLogger.aggregatehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L277[]
Hashes the details for each task and then for all tasks.

**Parameters:**

hashes (dict[str, list`Hash`) : Maps each task name to the list of all its samples' `Hash`.

compiled_hashes (dict[str, CompiledHash) : Maps each task name to its `CompiledHas`, an aggregation of all the individual sample hashes.

details (dict[str, list`Detail`]) : Maps each task name to the list of its samples' details. Example: winogrande: [sample1_details, sample2_details, ...]

compiled_details (dict[str, `CompiledDetail`]) : : Maps each task name to the list of its samples' compiled details.

compiled_details_over_all_tasks (CompiledDetailOverAllTasks) : Aggregated details over all the tasks.
#### log[[lighteval.logging.info_loggers.DetailsLogger.log]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L253)

Stores the relevant information for one sample of one task to the total list of samples stored in the DetailsLogger.

**Parameters:**

task_name (str) : Name of the current task of interest.

doc (Doc) : Current sample that we want to store.

model_response (ModelResponse) : Model outputs for the current sample

metrics (dict) : Model scores for said sample on the current task's metrics.

## MetricsLogger[[lighteval.logging.info_loggers.MetricsLogger]]
#### lighteval.logging.info_loggers.MetricsLogger[[lighteval.logging.info_loggers.MetricsLogger]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L309)

Logs the actual scores for each metric of each task.

aggregatelighteval.logging.info_loggers.MetricsLogger.aggregatehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L330[{"name": "task_dict", "val": ": dict"}, {"name": "bootstrap_iters", "val": ": int = 1000"}]- **task_dict** (dict[str, LightevalTask]) -- used to determine what aggregation function to use for each metric
- **bootstrap_iters** (int, optional) -- Number of runs used to run the statistical bootstrap. Defaults to 1000.0
Aggregate the metrics for each task and then for all tasks.

**Parameters:**

metrics_value (dict[str, dict[str, list[float]]]) : Maps each task to its dictionary of metrics to scores for all the example of the task. Example: {"winogrande|winogrande_xl": {"accuracy": [0.5, 0.5, 0.5, 0.5, 0.5, 0.5]}}

metric_aggregated (dict[str, dict[str, float]]) : Maps each task to its dictionary of metrics to aggregated scores over all the example of the task. Example: {"winogrande|winogrande_xl": {"accuracy": 0.5}}

## VersionsLogger[[lighteval.logging.info_loggers.VersionsLogger]]
#### lighteval.logging.info_loggers.VersionsLogger[[lighteval.logging.info_loggers.VersionsLogger]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L406)

Logger of the tasks versions.

Tasks can have a version number/date, which indicates what is the precise metric definition and dataset version used for an evaluation.

**Parameters:**

version (dict[str, int]) : Maps the task names with the task versions.

## TaskConfigLogger[[lighteval.logging.info_loggers.TaskConfigLogger]]
#### lighteval.logging.info_loggers.TaskConfigLogger[[lighteval.logging.info_loggers.TaskConfigLogger]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/logging/info_loggers.py#L425)

Logs the different parameters of the current `LightevalTask` of interest.

**Parameters:**

tasks_config (dict[str, LightevalTaskConfig]) : Maps each task to its associated `LightevalTaskConfig`
