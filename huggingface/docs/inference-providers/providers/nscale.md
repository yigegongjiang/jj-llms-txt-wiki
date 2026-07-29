### Template

If you want to update the content related to nscale's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/nscale.handlebars`.

### Logos

If you want to update nscale's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `nscale-light.png` and `nscale-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# Nscale

> [!TIP]
> All supported Nscale models can be found [here](https://huggingface.co/models?inference_provider=nscale&sort=trending)

    
        
        
    

    
        
        
    

Nscale is a vertically integrated AI cloud that delivers bespoke, sovereign AI infrastructure at scale.

Built on this foundation, Nscale's inference service empowers developers with a wide range of models and ready-to-use inference services that integrate into workflows without the need to manage the underlying infrastructure.

## Supported tasks

### Chat Completion (LLM)

Find out more about Chat Completion (LLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=text-generation
    providersMapping={ {"nscale":{"modelId":"meta-llama/Llama-3.1-8B-Instruct","providerModelId":"meta-llama/Llama-3.1-8B-Instruct"} } }
conversational />

### Chat Completion (VLM)

Find out more about Chat Completion (VLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=image-text-to-text
    providersMapping={ {"nscale":{"modelId":"meta-llama/Llama-4-Scout-17B-16E-Instruct","providerModelId":"meta-llama/Llama-4-Scout-17B-16E-Instruct"} } }
conversational />

### Text To Image

Find out more about Text To Image [here](../tasks/text-to-image).

<InferenceSnippet
    pipeline=text-to-image
    providersMapping={ {"nscale":{"modelId":"black-forest-labs/FLUX.1-schnell","providerModelId":"black-forest-labs/FLUX.1-schnell"} } }
/>
