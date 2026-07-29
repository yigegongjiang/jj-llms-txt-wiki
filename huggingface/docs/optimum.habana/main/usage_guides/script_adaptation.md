# Adapt a Transformers/Diffusers script to Intel Gaudi

🤗 Optimum for Intel Gaudi features HPU-optimized support for many of the latest 🤗 Transformers and Diffusers models.
To convert a script to use model optimized for a Gaudi device, simple adaptation can be performed.

## Transformers

Here is how to do a transformers script adaptation for Intel Gaudi:
```diff
- from transformers import Trainer, TrainingArguments
+ from optimum.habana import GaudiTrainer, GaudiTrainingArguments

# Define the training arguments
- training_args = TrainingArguments(
+ training_args = GaudiTrainingArguments(
+   use_habana=True,
+   use_lazy_mode=True,
+   gaudi_config_name=gaudi_config_name,
  ...
)

# Initialize our Trainer
- trainer = Trainer(
+ trainer = GaudiTrainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset
    ... # other arguments
)
```

where `gaudi_config_name` is the name of a model from the [Hub](https://huggingface.co/Habana) a path to a local Gaudi configuration file.
Gaudi configurations are stored as JSON files in model repositories but you can write your own.
More information can be found [here](../package_reference/gaudi_config).

## Diffusers

🤗 Optimum for Intel Gaudi also features HPU-optimized support for the 🤗 Diffusers library.
Thus, you can easily deploy Stable Diffusion on Gaudi for performing text-to-image generation.

Here is how to use it and the differences with the 🤗 Diffusers library:
```diff
- from diffusers import DDIMScheduler, StableDiffusionPipeline
+ from optimum.habana.diffusers import GaudiDDIMScheduler, GaudiStableDiffusionPipeline

model_name = "runwayml/stable-diffusion-v1-5"

- scheduler = DDIMScheduler.from_pretrained(model_name, subfolder="scheduler")
+ scheduler = GaudiDDIMScheduler.from_pretrained(model_name, subfolder="scheduler")

- pipeline = StableDiffusionPipeline.from_pretrained(
+ pipeline = GaudiStableDiffusionPipeline.from_pretrained(
    model_name,
    scheduler=scheduler,
+   use_habana=True,
+   use_hpu_graphs=True,
+   gaudi_config="Habana/stable-diffusion",
)

outputs = pipeline(
    ["An image of a squirrel in Picasso style"],
    num_images_per_prompt=16,
+   batch_size=4,
)
```
