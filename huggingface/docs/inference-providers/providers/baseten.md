### Template

If you want to update the content related to baseten's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/baseten.handlebars`.

### Logos

If you want to update baseten's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `baseten-light.png` and `baseten-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# Baseten

> [!TIP]
> All supported Baseten models can be found [here](https://huggingface.co/models?inference_provider=baseten&sort=trending)

    
        
        
    

    
        
        
    

Baseten provides on-demand frontier model APIs designed for production applications, not just experimentation. Built on the Baseten Inference Stack, these APIs deliver enterprise-grade performance and reliability with optimized inference for leading open-source models.

## Supported tasks

### Chat Completion (LLM)

Find out more about Chat Completion (LLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=text-generation
    providersMapping={ {"baseten":{"modelId":"deepseek-ai/DeepSeek-V4-Flash-0731","providerModelId":"deepseek-ai/DeepSeek-V4-Flash-0731"} } }
conversational />

### Chat Completion (VLM)

Find out more about Chat Completion (VLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=image-text-to-text
    providersMapping={ {"baseten":{"modelId":"moonshotai/Kimi-K3","providerModelId":"moonshotai/Kimi-K3"} } }
conversational />
