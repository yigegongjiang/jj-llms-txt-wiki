# TRL - Transformers Reinforcement Learning

TRL is a full stack library where we provide a set of tools to train transformer language models with methods like Supervised Fine-Tuning (SFT), Group Relative Policy Optimization (GRPO), Direct Preference Optimization (DPO), Reward Modeling, and more.
The library is integrated with 🤗 [transformers](https://github.com/huggingface/transformers).

## 🎉 What's New

**🌍 Multi-environment agentic RL:** [`GRPOTrainer`](grpo_trainer) now supports per-example environment selection and environment-owned rewards — mix multiple sandboxed task suites in one run and let each environment define its own scoring, with [Harbor](harbor) and [OpenEnv](openenv).

**🎯 KTO is now stable:** [`KTOTrainer`](kto_trainer) graduates to the stable API after a full alignment pass with [`DPOTrainer`](dpo_trainer).

## Taxonomy

Below is the current list of TRL trainers, organized by method type (⚡️ = vLLM support; 🧪 = experimental).

### Online methods

- [`GRPOTrainer`](grpo_trainer) ⚡️
- [`RLOOTrainer`](rloo_trainer) ⚡️
- [`OnlineDPOTrainer`](online_dpo_trainer) 🧪 ⚡️
- [`NashMDTrainer`](nash_md_trainer) 🧪 ⚡️
- [`PPOTrainer`](ppo_trainer) 🧪
- [`XPOTrainer`](xpo_trainer) 🧪 ⚡️

### Reward modeling

- [`RewardTrainer`](reward_trainer)
- [`PRMTrainer`](prm_trainer) 🧪

### Offline methods

- [`SFTTrainer`](sft_trainer)
- [`DPOTrainer`](dpo_trainer)
- [`KTOTrainer`](kto_trainer)
- [`BCOTrainer`](bco_trainer) 🧪
- [`CPOTrainer`](cpo_trainer) 🧪
- [`ORPOTrainer`](orpo_trainer) 🧪

### Knowledge distillation

- [`GKDTrainer`](gkd_trainer) 🧪
- [`MiniLLMTrainer`](minillm_trainer) 🧪

You can also explore TRL-related models, datasets, and demos in the [TRL Hugging Face organization](https://huggingface.co/trl-lib).

## Learn

Learn post-training with TRL and other libraries in 🤗 [smol course](https://github.com/huggingface/smol-course).

## Contents

The documentation is organized into the following sections:

- **Getting Started**: installation and quickstart guide.
- **Conceptual Guides**: dataset formats, training FAQ, and understanding logs.
- **How-to Guides**: reducing memory usage, speeding up training, distributing training, etc.
- **Integrations**: DeepSpeed, Liger Kernel, PEFT, etc.
- **Examples**: example overview, community tutorials, etc.
- **API**: trainers, utils, etc.

## Blog posts

  
    
      
      Published March 27, 2026
      TRL v1: Post-Training Library That Holds When the Field Invalidates Its Own Assumptions
    
    
      
      Published October 23, 2025
      Building the Open Agent Ecosystem Together: Introducing OpenEnv
    
    
      
      Published on August 7, 2025
      Vision Language Model Alignment in TRL ⚡️
    
    
      
      Published on June 3, 2025
      NO GPU left behind: Unlocking Efficiency with Co-located vLLM in TRL
    
    
      
      Published on May 25, 2025
      🐯 Liger GRPO meets TRL
    
    
      
      Published on January 28, 2025
      Open-R1: a fully open reproduction of DeepSeek-R1
    
    
      
      Published on July 10, 2024
      Preference Optimization for Vision Language Models with TRL
    
    
      
      Published on June 12, 2024
      Putting RL back in RLHF
    
    
      
      Published on September 29, 2023
      Finetune Stable Diffusion Models with DDPO via TRL
    
    
      
      Published on August 8, 2023
      Fine-tune Llama 2 with DPO
    
    
      
      Published on April 5, 2023
      StackLLaMA: A hands-on guide to train LLaMA with RLHF
   
    
      
      Published on March 9, 2023
      Fine-tuning 20B LLMs with RLHF on a 24GB consumer GPU
    
    
      
      Published on December 9, 2022
      Illustrating Reinforcement Learning from Human Feedback
    
  

## Talks

  
    
      
      Talk given on October 30, 2025
      Fine tuning with TRL
