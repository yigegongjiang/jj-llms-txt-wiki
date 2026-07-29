### Template

If you want to update the content related to scaleway's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/scaleway.handlebars`.

### Logos

If you want to update scaleway's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `scaleway-light.png` and `scaleway-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# Scaleway

> [!TIP]
> All supported Scaleway models can be found [here](https://huggingface.co/models?inference_provider=scaleway&sort=trending)

    
        
        
    

    
        
        
    

Scaleway is a European cloud provider, serving latest LLM models through its [Generative APIs](https://www.scaleway.com/en/generative-apis/) alongside a complete cloud ecosystem.

## Supported tasks

### Chat Completion (LLM)

Find out more about Chat Completion (LLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=text-generation
    providersMapping={ {"scaleway":{"modelId":"zai-org/GLM-5.2","providerModelId":"glm-5.2"} } }
conversational />

### Chat Completion (VLM)

Find out more about Chat Completion (VLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=image-text-to-text
    providersMapping={ {"scaleway":{"modelId":"Qwen/Qwen3.6-35B-A3B","providerModelId":"qwen3.6-35b-a3b"} } }
conversational />

### Feature Extraction

Find out more about Feature Extraction [here](../tasks/feature-extraction).

<InferenceSnippet
    pipeline=feature-extraction
    providersMapping={ {"scaleway":{"modelId":"Qwen/Qwen3-Embedding-8B","providerModelId":"qwen3-embedding-8b"} } }
/>
