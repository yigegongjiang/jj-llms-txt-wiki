# Textual Inversion

Textual Inversion is a training method for personalizing models by learning new text embeddings from a few example images. The file produced from training is extremely small (a few KBs) and the new embeddings can be loaded into the text encoder.

`TextualInversionLoaderMixin` provides a function for loading Textual Inversion embeddings from Diffusers and Automatic1111 into the text encoder and loading a special token to activate the embeddings.

> [!TIP]
> To learn more about how to load Textual Inversion embeddings, see the [Textual Inversion](../../using-diffusers/textual_inversion_inference) loading guide.

## TextualInversionLoaderMixin[[diffusers.loaders.TextualInversionLoaderMixin]]

#### diffusers.loaders.TextualInversionLoaderMixin[[diffusers.loaders.TextualInversionLoaderMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/textual_inversion.py#L118)

Load Textual Inversion tokens and embeddings to the tokenizer and text encoder.

load_textual_inversiondiffusers.loaders.TextualInversionLoaderMixin.load_textual_inversionhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/textual_inversion.py#L271[{"name": "pretrained_model_name_or_path", "val": ": str | list[str] | dict[str, torch.Tensor] | list[dict[str, torch.Tensor]]"}, {"name": "token", "val": ": str | list[str] | None = None"}, {"name": "tokenizer", "val": ": 'PreTrainedTokenizer' | None = None"}, {"name": "text_encoder", "val": ": 'PreTrainedModel' | None = None"}, {"name": "**kwargs", "val": ""}]- **pretrained_model_name_or_path** (`str` or `os.PathLike` or `list[str or os.PathLike]` or `Dict` or `list[Dict]`) --
  Can be either one of the following or a list of them:

  - A string, the *model id* (for example `sd-concepts-library/low-poly-hd-logos-icons`) of a
    pretrained model hosted on the Hub.
  - A path to a *directory* (for example `./my_text_inversion_directory/`) containing the textual
    inversion weights.
  - A path to a *file* (for example `./my_text_inversions.pt`) containing textual inversion weights.
  - A [torch state
    dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict).

- **token** (`str` or `list[str]`, *optional*) --
  Override the token to use for the textual inversion weights. If `pretrained_model_name_or_path` is a
  list, then `token` must also be a list of equal length.
- **text_encoder** ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel), *optional*) --
  Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)).
  If not specified, function will take self.tokenizer.
- **tokenizer** ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer), *optional*) --
  A `CLIPTokenizer` to tokenize text. If not specified, function will take self.tokenizer.
- **weight_name** (`str`, *optional*) --
  Name of a custom weight file. This should be used when:

  - The saved textual inversion file is in 🤗 Diffusers format, but was saved under a specific weight
    name such as `text_inv.bin`.
  - The saved textual inversion file is in the Automatic1111 format.
- **cache_dir** (`str | os.PathLike`, *optional*) --
  Path to a directory where a downloaded pretrained model configuration is cached if the standard cache
  is not used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.

- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.
- **local_files_only** (`bool`, *optional*, defaults to `False`) --
  Whether to only load local model weights and configuration files or not. If set to `True`, the model
  won't be downloaded from the Hub.
- **hf_token** (`str` or *bool*, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from
  `diffusers-cli login` (stored in `~/.huggingface`) is used.
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier
  allowed by Git.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  The subfolder location of a model file within a larger model repository on the Hub or locally.
- **mirror** (`str`, *optional*) --
  Mirror source to resolve accessibility issues if you're downloading a model in China. We do not
  guarantee the timeliness or safety of the source, and you should refer to the mirror site for more
  information.0

Load Textual Inversion embeddings into the text encoder of [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline) (both 🤗 Diffusers and
Automatic1111 formats are supported).

Example:

To load a Textual Inversion embedding vector in 🤗 Diffusers format:

```py
from diffusers import StableDiffusionPipeline
import torch

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16).to("cuda")

pipe.load_textual_inversion("sd-concepts-library/cat-toy")

prompt = "A  backpack"

image = pipe(prompt, num_inference_steps=50).images[0]
image.save("cat-backpack.png")
```

To load a Textual Inversion embedding vector in Automatic1111 format, make sure to download the vector first
(for example from [civitAI](https://civitai.com/models/3036?modelVersionId=9857)) and then load the vector

locally:

```py
from diffusers import StableDiffusionPipeline
import torch

model_id = "stable-diffusion-v1-5/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16).to("cuda")

pipe.load_textual_inversion("./charturnerv2.pt", token="charturnerv2")

prompt = "charturnerv2, multiple views of the same character in the same outfit, a character turnaround of a woman wearing a black jacket and red shirt, best quality, intricate details."

image = pipe(prompt, num_inference_steps=50).images[0]
image.save("character.png")
```

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike` or `list[str or os.PathLike]` or `Dict` or `list[Dict]`) : Can be either one of the following or a list of them:  - A string, the *model id* (for example `sd-concepts-library/low-poly-hd-logos-icons`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_text_inversion_directory/`) containing the textual inversion weights. - A path to a *file* (for example `./my_text_inversions.pt`) containing textual inversion weights. - A [torch state dict](https://pytorch.org/tutorials/beginner/saving_loading_models.html#what-is-a-state-dict). 

token (`str` or `list[str]`, *optional*) : Override the token to use for the textual inversion weights. If `pretrained_model_name_or_path` is a list, then `token` must also be a list of equal length.

text_encoder ([CLIPTextModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTextModel), *optional*) : Frozen text-encoder ([clip-vit-large-patch14](https://huggingface.co/openai/clip-vit-large-patch14)). If not specified, function will take self.tokenizer.

tokenizer ([CLIPTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clip#transformers.CLIPTokenizer), *optional*) : A `CLIPTokenizer` to tokenize text. If not specified, function will take self.tokenizer.

weight_name (`str`, *optional*) : Name of a custom weight file. This should be used when:  - The saved textual inversion file is in 🤗 Diffusers format, but was saved under a specific weight name such as `text_inv.bin`. - The saved textual inversion file is in the Automatic1111 format.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

hf_token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

mirror (`str`, *optional*) : Mirror source to resolve accessibility issues if you're downloading a model in China. We do not guarantee the timeliness or safety of the source, and you should refer to the mirror site for more information.
#### maybe_convert_prompt[[diffusers.loaders.TextualInversionLoaderMixin.maybe_convert_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/textual_inversion.py#L123)

Processes prompts that include a special token corresponding to a multi-vector textual inversion embedding to
be replaced with multiple special tokens each corresponding to one of the vectors. If the prompt has no textual
inversion token or if the textual inversion token is a single vector, the input prompt is returned.

**Parameters:**

prompt (`str` or list of `str`) : The prompt or prompts to guide the image generation.

tokenizer (`PreTrainedTokenizer`) : The tokenizer responsible for encoding the prompt into input tokens.

**Returns:**

``str` or list of `str``

The converted prompt
#### unload_textual_inversion[[diffusers.loaders.TextualInversionLoaderMixin.unload_textual_inversion]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/loaders/textual_inversion.py#L467)

Unload Textual Inversion embeddings from the text encoder of [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline)

Example:
```py
from diffusers import AutoPipelineForText2Image
import torch

pipeline = AutoPipelineForText2Image.from_pretrained("stable-diffusion-v1-5/stable-diffusion-v1-5")

# Example 1
pipeline.load_textual_inversion("sd-concepts-library/gta5-artwork")
pipeline.load_textual_inversion("sd-concepts-library/moeb-style")

# Remove all token embeddings
pipeline.unload_textual_inversion()

# Example 2
pipeline.load_textual_inversion("sd-concepts-library/moeb-style")
pipeline.load_textual_inversion("sd-concepts-library/gta5-artwork")

# Remove just one token
pipeline.unload_textual_inversion("")

# Example 3: unload from SDXL
pipeline = AutoPipelineForText2Image.from_pretrained("stabilityai/stable-diffusion-xl-base-1.0")
embedding_path = hf_hub_download(
    repo_id="linoyts/web_y2k", filename="web_y2k_emb.safetensors", repo_type="model"
)

# load embeddings to the text encoders
state_dict = load_file(embedding_path)

# load embeddings of text_encoder 1 (CLIP ViT-L/14)
pipeline.load_textual_inversion(
    state_dict["clip_l"],
    tokens=["", ""],
    text_encoder=pipeline.text_encoder,
    tokenizer=pipeline.tokenizer,
)
# load embeddings of text_encoder 2 (CLIP ViT-G/14)
pipeline.load_textual_inversion(
    state_dict["clip_g"],
    tokens=["", ""],
    text_encoder=pipeline.text_encoder_2,
    tokenizer=pipeline.tokenizer_2,
)

# Unload explicitly from both text encoders and tokenizers
pipeline.unload_textual_inversion(
    tokens=["", ""], text_encoder=pipeline.text_encoder, tokenizer=pipeline.tokenizer
)
pipeline.unload_textual_inversion(
    tokens=["", ""], text_encoder=pipeline.text_encoder_2, tokenizer=pipeline.tokenizer_2
)
```
