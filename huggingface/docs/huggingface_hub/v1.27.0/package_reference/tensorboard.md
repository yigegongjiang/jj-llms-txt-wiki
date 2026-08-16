# TensorBoard logger

TensorBoard is a visualization toolkit for machine learning experimentation. TensorBoard allows tracking and visualizing
metrics such as loss and accuracy, visualizing the model graph, viewing histograms, displaying images and much more.
TensorBoard is well integrated with the Hugging Face Hub. The Hub automatically detects TensorBoard traces (such as
`tfevents`) when pushed to the Hub which starts an instance to visualize them. To get more information about TensorBoard
integration on the Hub, check out [this guide](https://huggingface.co/docs/hub/tensorboard).

To benefit from this integration, `huggingface_hub` provides a custom logger to push logs to the Hub. It works as a
drop-in replacement for [SummaryWriter](https://tensorboardx.readthedocs.io/en/latest/tensorboard.html) with no extra
code needed. Traces are still saved locally and a background job push them to the Hub at regular interval.

## HFSummaryWriter[[huggingface_hub.HFSummaryWriter]]

#### huggingface_hub.HFSummaryWriter[[huggingface_hub.HFSummaryWriter]]

```python
huggingface_hub.HFSummaryWriter(*args, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_tensorboard_logger.py#L45)

**Parameters:**

repo_id (`str`) : The id of the repo to which the logs will be pushed.

logdir (`str`, *optional*) : The directory where the logs will be written. If not specified, a local directory will be created by the underlying `SummaryWriter` object.

commit_every (`int` or `float`, *optional*) : The frequency (in minutes) at which the logs will be pushed to the Hub. Defaults to 5 minutes.

squash_history (`bool`, *optional*) : Whether to squash the history of the repo after each commit. Defaults to `False`. Squashing commits is useful to avoid degraded performances on the repo when it grows too large.

repo_type (`str`, *optional*) : The type of the repo to which the logs will be pushed. Defaults to "model".

repo_revision (`str`, *optional*) : The revision of the repo to which the logs will be pushed. Defaults to "main".

repo_private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

path_in_repo (`str`, *optional*) : The path to the folder in the repo where the logs will be pushed. Defaults to "tensorboard/".

repo_allow_patterns (`list[str]` or `str`, *optional*) : A list of patterns to include in the upload. Defaults to `"*.tfevents.*"`. Check out the [upload guide](https://huggingface.co/docs/huggingface_hub/guides/upload#upload-a-folder) for more details.

repo_ignore_patterns (`list[str]` or `str`, *optional*) : A list of patterns to exclude in the upload. Check out the [upload guide](https://huggingface.co/docs/huggingface_hub/guides/upload#upload-a-folder) for more details.

token (`str`, *optional*) : Authentication token. Will default to the stored token. See https://huggingface.co/settings/token for more details

kwargs : Additional keyword arguments passed to `SummaryWriter`.

Wrapper around the tensorboard's `SummaryWriter` to push training logs to the Hub.

Data is logged locally and then pushed to the Hub asynchronously. Pushing data to the Hub is done in a separate
thread to avoid blocking the training script. In particular, if the upload fails for any reason (e.g. a connection
issue), the main script will not be interrupted. Data is automatically pushed to the Hub every `commit_every`
minutes (default to every 5 minutes).

> [!WARNING]
> `HFSummaryWriter` is experimental. Its API is subject to change in the future without prior notice.

Examples:
```diff
# Taken from https://pytorch.org/docs/stable/tensorboard.html
- from torch.utils.tensorboard import SummaryWriter
+ from huggingface_hub import HFSummaryWriter

import numpy as np

- writer = SummaryWriter()
+ writer = HFSummaryWriter(repo_id="username/my-trained-model")

for n_iter in range(100):
    writer.add_scalar('Loss/train', np.random.random(), n_iter)
    writer.add_scalar('Loss/test', np.random.random(), n_iter)
    writer.add_scalar('Accuracy/train', np.random.random(), n_iter)
    writer.add_scalar('Accuracy/test', np.random.random(), n_iter)
```

```py
>>> from huggingface_hub import HFSummaryWriter

# Logs are automatically pushed every 15 minutes (5 by default) + when exiting the context manager
>>> with HFSummaryWriter(repo_id="test_hf_logger", commit_every=15) as logger:
...     logger.add_scalar("a", 1)
...     logger.add_scalar("b", 2)
```
