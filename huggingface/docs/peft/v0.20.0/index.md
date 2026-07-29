# PEFT

🤗 PEFT (Parameter-Efficient Fine-Tuning) is a library for efficiently adapting large pretrained models to various downstream applications without fine-tuning all of a model's parameters because it is prohibitively costly. PEFT methods only fine-tune a small number of (extra) model parameters - significantly decreasing computational and storage costs - while yielding performance comparable to a fully fine-tuned model. This makes it more accessible to train and store large language models (LLMs) and other big models on consumer hardware.

PEFT is integrated with the Transformers, Diffusers, and Accelerate libraries to provide a faster and easier way to load, train, and use large models for inference.

  
    There are numerous methods to "adapt" existing models, often extensively integrating into the model. PEFT can be thought of as a framework for arbitrary methods of model adaption (modifying weights, wrapping layers, manipulating KV-caches, ...) while also serving as a reference implementation for many fine-tuning methods.
  
  

  
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="quicktour"
      >Quicktour
      Start here if you're new to 🤗 PEFT to get an overview of the library's main features, and how to train a model with a PEFT method.
    
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="./methods/overview"
      >Method overview
      Learn about the different categories of PEFT methods to get an orientation what to use with your model.
