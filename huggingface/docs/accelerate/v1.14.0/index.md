# Accelerate

Accelerate is a library that enables the same PyTorch code to be run across any distributed configuration by adding just four lines of code! In short, training and inference at scale made simple, efficient and adaptable.

```diff
+ from accelerate import Accelerator
+ accelerator = Accelerator()

+ model, optimizer, training_dataloader, scheduler = accelerator.prepare(
+     model, optimizer, training_dataloader, scheduler
+ )

  for batch in training_dataloader:
      optimizer.zero_grad()
      inputs, targets = batch
      inputs = inputs.to(device)
      targets = targets.to(device)
      outputs = model(inputs)
      loss = loss_function(outputs, targets)
+     accelerator.backward(loss)
      optimizer.step()
      scheduler.step()
```

Built on `torch_xla` and `torch.distributed`, Accelerate takes care of the heavy lifting, so you don't have to write any custom code to adapt to these platforms.
Convert existing codebases to utilize [DeepSpeed](usage_guides/deepspeed), perform [fully sharded data parallelism](usage_guides/fsdp), and have automatic support for mixed-precision training! 

 

  To get a better idea of this process, make sure to check out the [Tutorials](basic_tutorials/overview)! 

This code can then be launched on any system through Accelerate's CLI interface:
```bash
accelerate launch {my_script.py}
```

  
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="./basic_tutorials/overview"
      >Tutorials
      Learn the basics and become familiar with using Accelerate. Start here if you are using Accelerate for the first time!
    
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="./usage_guides/explore"
      >How-to guides
      Practical guides to help you achieve a specific goal. Take a look at these guides to learn how to use Accelerate to solve real-world problems.
    
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="./concept_guides/gradient_synchronization"
      >Conceptual guides
      High-level explanations for building a better understanding of important topics such as avoiding subtle nuances and pitfalls in distributed training and DeepSpeed.
   
    <a class="!no-underline border dark:border-gray-700 p-5 rounded-lg shadow hover:shadow-lg" href="./package_reference/accelerator"
      >Reference
      Technical descriptions of how Accelerate classes and methods work.
