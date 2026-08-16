### Template

If you want to update the content related to ovhcloud's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/ovhcloud.handlebars`.

### Logos

If you want to update ovhcloud's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `ovhcloud-light.png` and `ovhcloud-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# OVHcloud AI Endpoints

> [!TIP]
> All supported OVHcloud AI Endpoints models can be found [here](https://huggingface.co/models?inference_provider=ovhcloud&sort=trending)

    
        
        
    

    
        
        
    

[OVHcloud AI Endpoints](https://www.ovhcloud.com/en/public-cloud/ai-endpoints/) is a managed AI inference service that provides access to a wide range of state-of-the-art machine learning models. As part of OVHcloud's Public Cloud offering, AI Endpoints enables developers to easily integrate AI capabilities into their applications with data sovereignty, privacy, and GDPR compliance.

## Supported tasks

### Chat Completion (LLM)

Find out more about Chat Completion (LLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=text-generation
    providersMapping={ {"ovhcloud":{"modelId":"openai/gpt-oss-120b","providerModelId":"gpt-oss-120b"} } }
conversational />

### Chat Completion (VLM)

Find out more about Chat Completion (VLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=image-text-to-text
    providersMapping={ {"ovhcloud":{"modelId":"Qwen/Qwen3.6-27B","providerModelId":"Qwen3.6-27B"} } }
conversational />
