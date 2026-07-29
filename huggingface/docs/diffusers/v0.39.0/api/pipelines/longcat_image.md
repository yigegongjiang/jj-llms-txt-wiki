# LongCat-Image

  

We introduce LongCat-Image, a pioneering open-source and bilingual (Chinese-English) foundation model for image generation, designed to address core challenges in multilingual text rendering, photorealism, deployment efficiency, and developer accessibility prevalent in current leading models.

### Key Features
- 🌟 **Exceptional Efficiency and Performance**: With only **6B parameters**, LongCat-Image surpasses numerous open-source models that are several times larger across multiple benchmarks, demonstrating the immense potential of efficient model design.
- 🌟 **Superior Editing Performance**: LongCat-Image-Edit model achieves state-of-the-art performance among open-source models, delivering leading instruction-following and image quality with superior visual consistency.
- 🌟 **Powerful Chinese Text Rendering**: LongCat-Image demonstrates superior accuracy and stability in rendering common Chinese characters compared to existing SOTA open-source models and achieves industry-leading coverage of the Chinese dictionary.
- 🌟 **Remarkable Photorealism**: Through an innovative data strategy and training framework, LongCat-Image achieves remarkable photorealism in generated images.
- 🌟 **Comprehensive Open-Source Ecosystem**: We provide a complete toolchain, from intermediate checkpoints to full training code, significantly lowering the barrier for further research and development.

For more details, please refer to the comprehensive [***LongCat-Image Technical Report***](https://arxiv.org/abs/2412.11963)

## Usage Example

```py
import torch
import diffusers
from diffusers import LongCatImagePipeline

weight_dtype = torch.bfloat16
pipe = LongCatImagePipeline.from_pretrained("meituan-longcat/LongCat-Image", torch_dtype=torch.bfloat16 )
pipe.to('cuda')
# pipe.enable_model_cpu_offload()

prompt = '一个年轻的亚裔女性，身穿黄色针织衫，搭配白色项链。她的双手放在膝盖上，表情恬静。背景是一堵粗糙的砖墙，午后的阳光温暖地洒在她身上，营造出一种宁静而温馨的氛围。镜头采用中距离视角，突出她的神态和服饰的细节。光线柔和地打在她的脸上，强调她的五官和饰品的质感，增加画面的层次感与亲和力。整个画面构图简洁，砖墙的纹理与阳光的光影效果相得益彰，突显出人物的优雅与从容。'
image = pipe(
    prompt,
    height=768,
    width=1344,
    guidance_scale=4.0,
    num_inference_steps=50,
    num_images_per_prompt=1,
    generator=torch.Generator("cpu").manual_seed(43),
    enable_cfg_renorm=True,
    enable_prompt_rewrite=True,
).images[0]
image.save(f'./longcat_image_t2i_example.png')
```

This pipeline was contributed by LongCat-Image Team. The original codebase can be found [here](https://github.com/meituan-longcat/LongCat-Image).

Available models:

  
    
      
        Models
        Type
        Description
        Download Link
      
    
    
      
        LongCat&#8209;Image
        Text&#8209;to&#8209;Image
        Final Release. The standard model for out&#8209;of&#8209;the&#8209;box inference.
        
          🤗&nbsp;Huggingface
        
      
      
        LongCat&#8209;Image&#8209;Dev
        Text&#8209;to&#8209;Image
        Development. Mid-training checkpoint, suitable for fine-tuning.
        
          🤗&nbsp;Huggingface
        
      
      
        LongCat&#8209;Image&#8209;Edit
        Image Editing
        Specialized model for image editing.
        
          🤗&nbsp;Huggingface
        
      
    
  

## LongCatImagePipeline[[diffusers.LongCatImagePipeline]]

#### diffusers.LongCatImagePipeline[[diffusers.LongCatImagePipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/longcat_image/pipeline_longcat_image.py#L205)

The pipeline for text-to-image generation.

- all
- __call__

## LongCatImagePipelineOutput[[diffusers.pipelines.longcat_image.LongCatImagePipelineOutput]]

#### diffusers.pipelines.longcat_image.LongCatImagePipelineOutput[[diffusers.pipelines.longcat_image.LongCatImagePipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/longcat_image/pipeline_output.py#L10)

Output class for Stable Diffusion pipelines.

**Parameters:**

images (`list[PIL.Image.Image]` or `np.ndarray`) : List of denoised PIL images of length `batch_size` or numpy array of shape `(batch_size, height, width, num_channels)`. PIL images or numpy array present the denoised images of the diffusion pipeline.
