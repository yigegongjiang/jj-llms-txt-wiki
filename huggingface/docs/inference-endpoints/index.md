# Inference Endpoints

<img
    class="block dark:hidden w-full max-w-3xl h-auto mx-auto my-4"
    src="https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/illustration-static.svg"
    alt="Illustration of Inference Endpoints routing traffic from inference engines to a central endpoint"
/>
<img
    class="hidden dark:block w-full max-w-3xl h-auto mx-auto my-4"
    src="https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/illustration-dark-static.svg"
    alt="Illustration of Inference Endpoints routing traffic from inference engines to a central endpoint"
/>

Inference Endpoints is a managed service to deploy your AI model to production.
Here you'll find quickstarts, guides, tutorials, use cases and a lot more.

  
  <a
    class="!no-underline pb-8 pr-4 block rounded-xl border border-gray-200 dark:border-gray-800 bg-gradient-to-br from-blue-50 to-white dark:from-gray-900 dark:to-gray-800 hover:shadow-xl hover:-translate-y-1 transition-all leading-none flex flex-col h-full"
    href="./quick_start"
    >
    🔥 Quickstart
    
      Deploy a production ready AI model in minutes.
    
  

  <a 
    class="!no-underline pb-8 pr-4 block rounded-xl border border-gray-200 dark:border-gray-800 bg-gradient-to-br from-indigo-50 to-white dark:from-gray-900 dark:to-gray-800 hover:shadow-xl hover:-translate-y-1 transition-all leading-none flex flex-col h-full"
    href="./about"
    >
    🔍 How Inference Endpoints Works
    
      Understand the main components and benefits of Inference Endpoints.
    
  

  <a 
    class="!no-underline pb-8 pr-4 block rounded-xl border border-gray-200 dark:border-gray-800 bg-gradient-to-br from-red-50 to-white dark:from-gray-900 dark:to-gray-800 hover:shadow-xl hover:-translate-y-1 transition-all leading-none flex flex-col h-full"
    href="./guides/foundations"
    >
    📖 Guides
    
      Explore our guides to learn how to configure or enable specific features on the platform.
    
  

  <a
    class="!no-underline pb-8 pr-4 block rounded-xl border border-gray-200 dark:border-gray-800 bg-gradient-to-br from-green-50 to-white dark:from-gray-900 dark:to-gray-800 hover:shadow-xl hover:-translate-y-1 transition-all leading-none flex flex-col h-full"
    href="./tutorials/chat_bot"
    >
    🧑‍💻 Tutorials
    
      Step-by-step guides on common developer scenarios.
    
  

## Why use Inference Endpoints

Inference Endpoints makes deploying AI models to production a smooth experience. Instead of spending weeks configuring infrastructure, managing servers, and debugging deployment issues, you can focus on what matters most: your model and your users.

Our platform eliminates the complexity of AI infrastructure while providing enterprise-grade features that scale with your business needs. Whether you're a startup launching your first AI product or an enterprise team managing hundreds of models, Inference Endpoints provides the reliability, performance, and cost-efficiency you need.

**Key benefits include:**
- ⬇️ **Reduce operational overhead**: Eliminate the need for dedicated DevOps teams and infrastructure management, letting you focus on innovation.
- 🚀 **Scale with confidence**: Handle traffic spikes automatically without worrying about capacity planning or performance degradation.
- ⬇️ **Lower total cost of ownership**: Avoid the hidden costs of self-managed infrastructure including maintenance, monitoring, and security compliance.
- 💻  **Future-proof your AI stack**: Stay current with the latest frameworks and optimizations without managing complex upgrades.
- 🔥 **Focus on what matters**: Spend your time improving your models and building great user experiences, not managing servers.

## Key Features 
- 📦 **Fully managed infrastructure**: you don't need to worry about things like kubernetes, CUDA versions and configuring VPNs. Inference Endpoints deals with this under the hood so you can focus on deploying your model and serving customers as fast as possible.
- ↕️ **Autoscaling**: as there's more traffic to your model you'll need more firepower as well. Your Inference Endpoint scales up as traffic increases and down as it decreases to save you on unnecessary compute cost. 
- 👀 **Observability**: understand and debug what's going on in your model through logs & metrics.
- 🔥 **Integrated support for open-source Inference Engines**: Whether you want to deploy your model with vLLM, TGI or a custom container, we got you!
- 🤗 **Seamless integration with the Hugging Face Hub**: Downloading model weights fast and with the correct security policies is paramount when bringing an AI model to production. With Inference Endpoints, it's easy and safe.

## Further Reading

If you're considering using Inference Endpoints in production, read these two case studies:
- [Why we're switching to Hugging Face Inference Endpoints, and maybe you should too](https://huggingface.co/blog/mantis-case-study)
- [Investing in Performance: Fine-tune small models with LLM insights - a CFM case study](https://huggingface.co/blog/cfm-case-study)

You might also find these blogs helpful:
- [🤗 LLM suggestions in Argilla with HuggingFace Inference Endpoints](https://huggingface.co/blog/alvarobartt/argilla-suggestions-via-inference-endpoints)
- [Programmatically manage Inference Endpoints](https://www.philschmid.de/inference-endpoints-iac)
- [TGI Multi-LoRA: Deploy Once, Serve 30 models](https://huggingface.co/blog/multi-lora-serving)
- [Llama 3.1 - 405B, 70B & 8B with multilinguality and long context](https://huggingface.co/blog/llama31#hugging-face-inference-endpoints)
- [Deploy MusicGen in no time with Inference Endpoints](https://huggingface.co/blog/run-musicgen-as-an-api)

Or try out the [Quick Start](./quick_start)!
