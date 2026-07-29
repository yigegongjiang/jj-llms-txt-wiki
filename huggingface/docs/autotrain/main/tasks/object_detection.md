# Object Detection

Object detection is a form of supervised learning where a model is trained to identify 
and categorize objects within images. AutoTrain simplifies the process, enabling you to
train a state-of-the-art object detection model by simply uploading labeled example images.

## Preparing your data

To ensure your object detection model trains effectively, follow these guidelines for preparing your data:

### Organizing Images

Prepare a zip file containing your images and metadata.jsonl.

```
Archive.zip
├── 0001.png
├── 0002.png
├── 0003.png
├── .
├── .
├── .
└── metadata.jsonl
```

Example for `metadata.jsonl`:

```
{"file_name": "0001.png", "objects": {"bbox": [[302.0, 109.0, 73.0, 52.0]], "category": [0]}}
{"file_name": "0002.png", "objects": {"bbox": [[810.0, 100.0, 57.0, 28.0]], "category": [1]}}
{"file_name": "0003.png", "objects": {"bbox": [[160.0, 31.0, 248.0, 616.0], [741.0, 68.0, 202.0, 401.0]], "category": [2, 2]}}
```

Please note that bboxes need to be in COCO format `[x, y, width, height]`.

### Image Requirements

- Format: Ensure all images are in JPEG, JPG, or PNG format.

- Quantity: Include at least 5 images to provide the model with sufficient examples for learning.

- Exclusivity: The zip file should exclusively contain images and metadata.jsonl.
No additional files or nested folders should be included.

Some points to keep in mind:

- The images must be jpeg, jpg or png.
- There should be at least 5 images per split.
- There must not be any other files in the zip file.
- There must not be any other folders inside the zip folder.

When train.zip is decompressed, it creates no folders: only images and metadata.jsonl.

## Parameters[[autotrain.trainers.object_detection.params.ObjectDetectionParams]]

#### autotrain.trainers.object_detection.params.ObjectDetectionParams[[autotrain.trainers.object_detection.params.ObjectDetectionParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/object_detection/params.py#L8)

ObjectDetectionParams is a configuration class for object detection training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to be used. Default is "google/vit-base-patch16-224".

username (Optional[str]) : Hugging Face Username.

lr (float) : Learning rate. Default is 5e-5.

epochs (int) : Number of training epochs. Default is 3.

batch_size (int) : Training batch size. Default is 8.

warmup_ratio (float) : Warmup proportion. Default is 0.1.

gradient_accumulation (int) : Gradient accumulation steps. Default is 1.

optimizer (str) : Optimizer to be used. Default is "adamw_torch".

scheduler (str) : Scheduler to be used. Default is "linear".

weight_decay (float) : Weight decay. Default is 0.0.

max_grad_norm (float) : Max gradient norm. Default is 1.0.

seed (int) : Random seed. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

logging_steps (int) : Number of steps between logging. Default is -1.

project_name (str) : Name of the project for output directory. Default is "project-name".

auto_find_batch_size (bool) : Whether to automatically find batch size. Default is False.

mixed_precision (Optional[str]) : Mixed precision type (fp16, bf16, or None).

save_total_limit (int) : Total number of checkpoints to save. Default is 1.

token (Optional[str]) : Hub Token for authentication.

push_to_hub (bool) : Whether to push the model to the Hugging Face Hub. Default is False.

eval_strategy (str) : Evaluation strategy. Default is "epoch".

image_column (str) : Name of the image column in the dataset. Default is "image".

objects_column (str) : Name of the target column in the dataset. Default is "objects".

log (str) : Logging method for experiment tracking. Default is "none".

image_square_size (Optional[int]) : Longest size to which the image will be resized, then padded to square. Default is 600.

early_stopping_patience (int) : Number of epochs with no improvement after which training will be stopped. Default is 5.

early_stopping_threshold (float) : Minimum change to qualify as an improvement. Default is 0.01.
