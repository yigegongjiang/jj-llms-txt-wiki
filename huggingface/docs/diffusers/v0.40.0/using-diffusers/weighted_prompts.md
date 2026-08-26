# Prompting

Prompts describes what a model should generate. Good prompts are detailed, specific, and structured and they generate better images and videos.

This guide shows you how to write effective prompts and introduces techniques that make them stronger.

## Writing good prompts

Every effective prompt needs three core elements.

1. Subject - what you want to generate. Start your prompt here.
2. Style - the medium or aesthetic. How should it look?
3. Context - details about actions, setting, and mood.

Use these elements as a structured narrative, not a keyword list. Modern models understand language better than keyword matching. Start simple, then add details.

Context is especially important for creating better prompts. Try adding lighting, artistic details, and mood.

  
    
    A cute cat lounges on a leaf in a pool during a peaceful summer afternoon, in lofi art style, illustration.
  
  
    
    A cute cat lounges on a floating leaf in a sparkling pool during a peaceful summer afternoon. Clear reflections ripple across the water, with sunlight casting soft, smooth highlights. The illustration is detailed and polished, with elegant lines and harmonious colors, evoking a relaxing, serene, and whimsical lofi mood, anime-inspired and visually comforting.
  

Be specific and add context. Use photography terms like lens type, focal length, camera angles, and depth of field.

> [!TIP]
> Try a [prompt enhancer](https://huggingface.co/models?sort=downloads&search=prompt+enhancer) to help improve your prompt structure.

## Prompt weighting

Prompt weighting makes some words stronger and others weaker. It scales attention scores so you control how much influence each concept has.

Diffusers handles this through `prompt_embeds` and `pooled_prompt_embeds` arguments which take scaled text embedding vectors. Use the [sd_embed](https://github.com/xhinker/sd_embed) library to generate these embeddings. It also supports longer prompts.

> [!NOTE]
> The sd_embed library only supports Stable Diffusion, Stable Diffusion XL, Stable Diffusion 3, Stable Cascade, and Flux. Prompt weighting doesn't necessarily help for newer models like Flux which already has very good prompt adherence.

```py
!uv pip install git+https://github.com/xhinker/sd_embed.git@main
```

Format weighted text with numerical multipliers or parentheses. More parentheses mean stronger weighting.

| format | multiplier |
|---|---|
| `(cat)` | increase by 1.1x |
| `((cat))` | increase by 1.21x |
| `(cat:1.5)` | increase by 1.5x |
| `(cat:0.5)` | decrease by 4x |

Create a weighted prompt and pass it to [get_weighted_text_embeddings_sdxl](https://github.com/xhinker/sd_embed/blob/4a47f71150a22942fa606fb741a1c971d95ba56f/src/sd_embed/embedding_funcs.py#L405) to generate embeddings.

> [!TIP]
> You could also pass negative prompts to `negative_prompt_embeds` and `negative_pooled_prompt_embeds`.

```py
import torch
from diffusers import DiffusionPipeline
from sd_embed.embedding_funcs import get_weighted_text_embeddings_sdxl

pipeline = DiffusionPipeline.from_pretrained(
    "Lykon/dreamshaper-xl-1-0", dtype=torch.bfloat16, device_map="cuda"
)

prompt = """
A (cute cat:1.4) lounges on a (floating leaf:1.2) in a (sparkling pool:1.1) during a peaceful summer afternoon.
Gentle ripples reflect pastel skies, while (sunlight:1.1) casts soft highlights. The illustration is smooth and polished
with elegant, sketchy lines and subtle gradients, evoking a ((whimsical, nostalgic, dreamy lofi atmosphere:2.0)), 
(anime-inspired:1.6), calming, comforting, and visually serene.
"""

prompt_embeds, _, pooled_prompt_embeds, *_ = get_weighted_text_embeddings_sdxl(pipeline, prompt=prompt)
```

Pass the embeddings to `prompt_embeds` and `pooled_prompt_embeds` to generate your image.

```py
image = pipeline(prompt_embeds=prompt_embeds, pooled_prompt_embeds=pooled_prompt_embeds).images[0]
```

  

Prompt weighting works with [Textual inversion](./textual_inversion_inference) and [DreamBooth](./dreambooth) adapters too.
