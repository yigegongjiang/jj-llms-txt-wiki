### Template

If you want to update the content related to deepinfra's description, please edit the template file under `https://github.com/huggingface/hub-docs/tree/main/scripts/inference-providers/templates/providers/deepinfra.handlebars`.

### Logos

If you want to update deepinfra's logo, upload a file by opening a PR on https://huggingface.co/datasets/huggingface/documentation-images/tree/main/inference-providers/logos. Ping @wauplin and @celinah on the PR to let them know you uploaded a new logo.
Logos must be in .png format and be named `deepinfra-light.png` and `deepinfra-dark.png`. Visit https://huggingface.co/settings/theme to switch between light and dark mode and check that the logos are displayed correctly.

### Generation script

For more details, check out the `generate.ts` script: https://github.com/huggingface/hub-docs/blob/main/scripts/inference-providers/scripts/generate.ts.
--->

# DeepInfra

> [!TIP]
> All supported DeepInfra models can be found [here](https://huggingface.co/models?inference_provider=deepinfra&sort=trending)

    
        
        
    

    
        
        
    

DeepInfra is a serverless AI inference platform offering one of the most cost-effective pricing per token in the industry. With a catalog of over 100 models spanning LLMs, text-to-image, text-to-speech, speech-to-text, video generation, OCR, and more, DeepInfra makes it easy for developers to integrate a wide range of AI capabilities into their applications with minimal setup.

## Resources
- **Website**: https://deepinfra.com/
- **Documentation**: https://docs.deepinfra.com

## Supported tasks

### Automatic Speech Recognition

Find out more about Automatic Speech Recognition [here](../tasks/automatic-speech-recognition).

<InferenceSnippet
    pipeline=automatic-speech-recognition
    providersMapping={ {"deepinfra":{"modelId":"openai/whisper-large-v3","providerModelId":"openai/whisper-large-v3"} } }
/>

### Chat Completion (LLM)

Find out more about Chat Completion (LLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=text-generation
    providersMapping={ {"deepinfra":{"modelId":"deepseek-ai/DeepSeek-V4-Flash-0731","providerModelId":"deepseek-ai/DeepSeek-V4-Flash-0731"} } }
conversational />

### Chat Completion (VLM)

Find out more about Chat Completion (VLM) [here](../tasks/chat-completion).

<InferenceSnippet
    pipeline=image-text-to-text
    providersMapping={ {"deepinfra":{"modelId":"moonshotai/Kimi-K3","providerModelId":"moonshotai/Kimi-K3"} } }
conversational />

### Feature Extraction

Find out more about Feature Extraction [here](../tasks/feature-extraction).

<InferenceSnippet
    pipeline=feature-extraction
    providersMapping={ {"deepinfra":{"modelId":"Qwen/Qwen3-Embedding-0.6B","providerModelId":"Qwen/Qwen3-Embedding-0.6B"} } }
/>
