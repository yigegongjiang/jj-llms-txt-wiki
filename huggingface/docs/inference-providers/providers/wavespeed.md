### Template

If you want to update the content related to wavespeed's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/wavespeed.handlebars`.

### Logos

If you want to update wavespeed's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `wavespeed-light.png` and `wavespeed-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# WaveSpeed

> [!TIP]
> All supported WaveSpeed models can be found [here](https://huggingface.co/models?inference_provider=wavespeed&sort=trending)

    
        
        
    

    
        
        
    

[WaveSpeedAI](https://wavespeed.ai/) is a high-performance AI inference platform specializing in image and video generation. Built with cutting-edge infrastructure and optimization techniques, [WaveSpeedAI](https://wavespeed.ai/) provides fast, scalable, and cost-effective model serving for creative AI applications.

## Supported tasks

### Image To Image

Find out more about Image To Image [here](../tasks/image-to-image).

<InferenceSnippet
    pipeline=image-to-image
    providersMapping={ {"wavespeed":{"modelId":"black-forest-labs/FLUX.2-dev","providerModelId":"wavespeed-ai/flux-2-dev/edit"} } }
/>

### Text To Image

Find out more about Text To Image [here](../tasks/text-to-image).

<InferenceSnippet
    pipeline=text-to-image
    providersMapping={ {"wavespeed":{"modelId":"black-forest-labs/FLUX.1-dev","providerModelId":"wavespeed-ai/flux-dev"} } }
/>

### Text To Video

Find out more about Text To Video [here](../tasks/text-to-video).

<InferenceSnippet
    pipeline=text-to-video
    providersMapping={ {"wavespeed":{"modelId":"larryvrh/MiniMax-H3-Turbo-Lora","providerModelId":"wavespeed-ai/minimax-h3/text-to-video-lora"} } }
/>
