# Single-HPU Training

Training on a single device is as simple as in Transformers:
- You need to replace the Transformers' [`Trainer`](https://huggingface.co/docs/transformers/main_classes/trainer) class with the [`GaudiTrainer`](https://huggingface.co/docs/optimum/habana/package_reference/trainer) class,
- You need to replace the Transformers' [`TrainingArguments`](https://huggingface.co/docs/transformers/main_classes/trainer#transformers.TrainingArguments) class with the [GaudiTrainingArguments](/docs/optimum.habana/main/en/package_reference/trainer#optimum.habana.GaudiTrainingArguments) class and add the following arguments:
    - `use_habana` to execute your script on an HPU,
    - `use_lazy_mode` to use lazy mode (recommended) or not (i.e. eager mode),
    - `gaudi_config_name` to give the name of (Hub) or the path to (local) your Gaudi configuration file.

To go further, we invite you to read our guides about [accelerating training](../usage_guides/accelerate_training) and [pretraining](../usage_guides/pretraining).
